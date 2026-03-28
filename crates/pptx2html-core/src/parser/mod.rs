//! PPTX ZIP/XML parser
//! PPTX = ZIP archive containing OOXML (PresentationML) XML files

mod layout_parser;
pub mod master_parser;
mod relationships;
mod slide_parser;
mod theme_parser;
mod xml_utils;

use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::Path;

use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

use crate::error::{PptxError, PptxResult};
use crate::model::{Emu, ListStyle, Presentation, Size};

pub struct PptxParser;

impl PptxParser {
    /// Parse PPTX from file path
    pub fn parse_file(path: &Path) -> PptxResult<Presentation> {
        let data = std::fs::read(path)?;
        Self::parse_bytes(&data)
    }

    /// Parse PPTX from byte data
    pub fn parse_bytes(data: &[u8]) -> PptxResult<Presentation> {
        let cursor = Cursor::new(data);
        let mut archive = ZipArchive::new(cursor)?;

        let mut presentation = Presentation::default();

        // 1. Parse slide size, slide rel IDs, and default text style from presentation.xml
        let pres_xml = Self::read_entry(&mut archive, "ppt/presentation.xml")?;
        let (slide_size, slide_rel_ids, default_text_style) = Self::parse_presentation_xml(&pres_xml)?;
        presentation.slide_size = slide_size;
        presentation.default_text_style = default_text_style;

        // 2. Parse presentation.xml.rels (all relationships)
        let rels_xml = Self::read_entry(&mut archive, "ppt/_rels/presentation.xml.rels")?;
        let pres_rels = relationships::parse_relationships(&rels_xml)?;

        // 3. Parse themes
        let theme_paths = collect_targets_by_type(&pres_rels, "theme");
        for theme_target in &theme_paths {
            let theme_full = normalize_ppt_path(theme_target);
            if let Ok(theme_xml) = Self::read_entry(&mut archive, &theme_full) {
                presentation.themes.push(theme_parser::parse_theme(&theme_xml)?);
            }
        }

        // 4. Parse slide masters
        let master_targets = collect_targets_by_type(&pres_rels, "slideMaster");
        // Map: master target path -> index in presentation.masters
        let mut master_path_to_idx: HashMap<String, usize> = HashMap::new();

        for master_target in &master_targets {
            let master_full = normalize_ppt_path(master_target);
            let master_xml = match Self::read_entry(&mut archive, &master_full) {
                Ok(xml) => xml,
                Err(_) => continue,
            };

            let master_rels_path = Self::rels_path_for(&master_full);
            let master_rels = if let Ok(rels_xml) = Self::read_entry(&mut archive, &master_rels_path) {
                relationships::parse_relationships(&rels_xml)?
            } else {
                HashMap::new()
            };

            let mut master = master_parser::parse_slide_master(&master_xml, &master_rels, &mut archive)?;

            // Find which theme this master references
            let theme_ref = find_target_by_type(&master_rels, "theme");
            if let Some(theme_target) = theme_ref {
                let theme_full_path = resolve_relative_path(&master_full, &theme_target);
                master.theme_idx = theme_paths.iter().position(|tp| {
                    normalize_ppt_path(tp) == theme_full_path
                }).unwrap_or(0);
            }

            let idx = presentation.masters.len();
            let canonical = canonical_part_name(master_target);
            master_path_to_idx.insert(canonical, idx);
            presentation.masters.push(master);
        }

        // Backward compat: copy ClrMap from first master into presentation
        if let Some(first_master) = presentation.masters.first() {
            if !first_master.clr_map.is_empty() {
                presentation.clr_map = first_master.clr_map.clone();
            }
        }

        // 5. Parse slide layouts
        // Layouts are referenced from master .rels files
        let mut layout_path_to_idx: HashMap<String, usize> = HashMap::new();

        for master_target in &master_targets {
            let master_full = normalize_ppt_path(master_target);
            let master_rels_path = Self::rels_path_for(&master_full);
            let master_rels = if let Ok(rels_xml) = Self::read_entry(&mut archive, &master_rels_path) {
                relationships::parse_relationships(&rels_xml)?
            } else {
                continue;
            };

            let master_canonical = canonical_part_name(master_target);
            let master_idx = master_path_to_idx.get(&master_canonical).copied().unwrap_or(0);

            let layout_targets = collect_targets_by_type(&master_rels, "slideLayout");
            for layout_target in &layout_targets {
                let layout_full = resolve_relative_path(&master_full, layout_target);
                let layout_canonical = canonical_part_name(&layout_full.replace("ppt/", ""));

                if layout_path_to_idx.contains_key(&layout_canonical) {
                    continue;
                }

                let layout_xml = match Self::read_entry(&mut archive, &layout_full) {
                    Ok(xml) => xml,
                    Err(_) => continue,
                };

                let layout_rels_path = Self::rels_path_for(&layout_full);
                let layout_rels = if let Ok(rels_xml) = Self::read_entry(&mut archive, &layout_rels_path) {
                    relationships::parse_relationships(&rels_xml)?
                } else {
                    HashMap::new()
                };

                let mut layout = layout_parser::parse_slide_layout(&layout_xml, &layout_rels, &mut archive)?;
                layout.master_idx = master_idx;

                let idx = presentation.layouts.len();
                layout_path_to_idx.insert(layout_canonical, idx);
                presentation.layouts.push(layout);
            }
        }

        // 6. Parse slides
        for rel_id in &slide_rel_ids {
            if let Some(slide_path) = pres_rels.get(rel_id) {
                let full_path = normalize_ppt_path(slide_path);
                if let Ok(slide_xml) = Self::read_entry(&mut archive, &full_path) {
                    let slide_rels_path = Self::rels_path_for(&full_path);
                    let slide_rels = if let Ok(rels_xml) =
                        Self::read_entry(&mut archive, &slide_rels_path)
                    {
                        relationships::parse_relationships(&rels_xml)?
                    } else {
                        HashMap::new()
                    };

                    let mut slide = slide_parser::parse_slide(
                        &slide_xml,
                        &slide_rels,
                        &mut archive,
                    )?;

                    // Find which layout this slide references
                    let layout_ref = find_target_by_type(&slide_rels, "slideLayout");
                    if let Some(layout_target) = layout_ref {
                        let layout_full = resolve_relative_path(&full_path, &layout_target);
                        let layout_canonical = canonical_part_name(&layout_full.replace("ppt/", ""));
                        slide.layout_idx = layout_path_to_idx.get(&layout_canonical).copied();
                    }

                    presentation.slides.push(slide);
                }
            }
        }

        // 7. Build presentation (already populated)
        Ok(presentation)
    }

    /// Read a ZIP entry
    fn read_entry<R: Read + std::io::Seek>(
        archive: &mut ZipArchive<R>,
        name: &str,
    ) -> PptxResult<String> {
        let mut file = archive
            .by_name(name)
            .map_err(|_| PptxError::MissingFile(name.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Build relationships file path: ppt/slides/slide1.xml -> ppt/slides/_rels/slide1.xml.rels
    fn rels_path_for(path: &str) -> String {
        if let Some((dir, file)) = path.rsplit_once('/') {
            format!("{dir}/_rels/{file}.rels")
        } else {
            format!("_rels/{path}.rels")
        }
    }

    /// Extract slide size, slide relationship IDs, and defaultTextStyle from presentation.xml
    fn parse_presentation_xml(xml: &str) -> PptxResult<(Size, Vec<String>, Option<ListStyle>)> {
        let mut reader = Reader::from_str(xml);
        let mut slide_size = Size::default();
        let mut slide_rel_ids = Vec::new();

        // defaultTextStyle parsing state
        let mut in_default_text_style = false;
        let mut default_text_style = ListStyle::default();
        let mut has_default_text_style = false;
        let mut current_lvl: Option<usize> = None;
        let mut current_para_defaults: Option<crate::model::ParagraphDefaults> = None;
        let mut current_run_defaults: Option<crate::model::RunDefaults> = None;
        let mut in_def_rpr = false;
        let mut current_color: Option<crate::model::Color> = None;
        let mut in_ln_spc = false;
        let mut in_spc_bef = false;
        let mut in_spc_aft = false;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let local = xml_utils::local_name(name.as_ref());
                    match local {
                        "defaultTextStyle" => {
                            in_default_text_style = true;
                            has_default_text_style = true;
                        }
                        // Level paragraph properties inside defaultTextStyle
                        s if in_default_text_style && master_parser::is_lvl_ppr(s) => {
                            let lvl = master_parser::parse_lvl_index(s);
                            current_lvl = Some(lvl);
                            let mut pd = crate::model::ParagraphDefaults::default();
                            master_parser::parse_lvl_ppr_attrs(e, &mut pd);
                            current_para_defaults = Some(pd);
                        }
                        "defRPr" if in_default_text_style && current_lvl.is_some() => {
                            in_def_rpr = true;
                            let mut rd = crate::model::RunDefaults::default();
                            master_parser::parse_def_rpr_attrs(e, &mut rd);
                            current_run_defaults = Some(rd);
                        }
                        // Spacing containers inside defaultTextStyle lvlNpPr
                        "lnSpc" if in_default_text_style && current_lvl.is_some() && !in_def_rpr => {
                            in_ln_spc = true;
                        }
                        "spcBef" if in_default_text_style && current_lvl.is_some() && !in_def_rpr => {
                            in_spc_bef = true;
                        }
                        "spcAft" if in_default_text_style && current_lvl.is_some() && !in_def_rpr => {
                            in_spc_aft = true;
                        }
                        // Color elements inside defRPr
                        "srgbClr" if in_def_rpr => {
                            if let Some(val) = xml_utils::attr_str(e, "val") {
                                current_color = Some(crate::model::Color::rgb(val));
                            }
                        }
                        "schemeClr" if in_def_rpr => {
                            if let Some(val) = xml_utils::attr_str(e, "val") {
                                current_color = Some(crate::model::Color::theme(val));
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    let local = xml_utils::local_name(name.as_ref());
                    match local {
                        "sldSz" => {
                            for attr in e.attributes().flatten() {
                                let key = xml_utils::local_name(attr.key.as_ref());
                                let val = String::from_utf8_lossy(&attr.value);
                                match key {
                                    "cx" => slide_size.width = Emu::from_str(&val),
                                    "cy" => slide_size.height = Emu::from_str(&val),
                                    _ => {}
                                }
                            }
                        }
                        "sldId" => {
                            for attr in e.attributes().flatten() {
                                let key = std::str::from_utf8(attr.key.as_ref())
                                    .unwrap_or("");
                                if key.ends_with("id") && key.contains(':') {
                                    let val = String::from_utf8_lossy(&attr.value);
                                    slide_rel_ids.push(val.to_string());
                                }
                            }
                        }
                        // Empty lvlNpPr inside defaultTextStyle
                        s if in_default_text_style && master_parser::is_lvl_ppr(s) => {
                            let lvl = master_parser::parse_lvl_index(s);
                            if lvl < 9 {
                                let mut pd = crate::model::ParagraphDefaults::default();
                                master_parser::parse_lvl_ppr_attrs(e, &mut pd);
                                default_text_style.levels[lvl] = Some(pd);
                            }
                        }
                        // Empty defRPr inside defaultTextStyle
                        "defRPr" if in_default_text_style && current_lvl.is_some() && !in_def_rpr => {
                            let mut rd = crate::model::RunDefaults::default();
                            master_parser::parse_def_rpr_attrs(e, &mut rd);
                            if let Some(pd) = current_para_defaults.as_mut() {
                                pd.def_run_props = Some(rd);
                            }
                        }
                        // Spacing percentage/points inside defaultTextStyle lvlNpPr
                        "spcPct" if in_default_text_style && current_lvl.is_some() && (in_ln_spc || in_spc_bef || in_spc_aft) => {
                            if let Some(val_str) = xml_utils::attr_str(e, "val") {
                                if let Ok(val) = val_str.parse::<f64>() {
                                    let spacing = crate::model::SpacingValue::Percent(val / 100_000.0);
                                    if let Some(pd) = current_para_defaults.as_mut() {
                                        if in_ln_spc {
                                            pd.line_spacing = Some(spacing);
                                        } else if in_spc_bef {
                                            pd.space_before = Some(spacing);
                                        } else if in_spc_aft {
                                            pd.space_after = Some(spacing);
                                        }
                                    }
                                }
                            }
                        }
                        "spcPts" if in_default_text_style && current_lvl.is_some() && (in_ln_spc || in_spc_bef || in_spc_aft) => {
                            if let Some(val_str) = xml_utils::attr_str(e, "val") {
                                if let Ok(val) = val_str.parse::<f64>() {
                                    let spacing = crate::model::SpacingValue::Points(val / 100.0);
                                    if let Some(pd) = current_para_defaults.as_mut() {
                                        if in_ln_spc {
                                            pd.line_spacing = Some(spacing);
                                        } else if in_spc_bef {
                                            pd.space_before = Some(spacing);
                                        } else if in_spc_aft {
                                            pd.space_after = Some(spacing);
                                        }
                                    }
                                }
                            }
                        }
                        // Font inside defRPr
                        "latin" if in_def_rpr => {
                            if let Some(rd) = current_run_defaults.as_mut() {
                                if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                                    rd.font_latin = Some(typeface);
                                }
                            }
                        }
                        "ea" if in_def_rpr => {
                            if let Some(rd) = current_run_defaults.as_mut() {
                                if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                                    rd.font_ea = Some(typeface);
                                }
                            }
                        }
                        // Color (Empty variant) inside defRPr
                        "srgbClr" if in_def_rpr => {
                            if let Some(val) = xml_utils::attr_str(e, "val") {
                                if let Some(rd) = current_run_defaults.as_mut() {
                                    rd.color = Some(crate::model::Color::rgb(val));
                                }
                            }
                        }
                        "schemeClr" if in_def_rpr => {
                            if let Some(val) = xml_utils::attr_str(e, "val") {
                                if let Some(rd) = current_run_defaults.as_mut() {
                                    rd.color = Some(crate::model::Color::theme(val));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    let qname = e.name();
                    let local = xml_utils::local_name(qname.as_ref());
                    match local {
                        "defaultTextStyle" => {
                            in_default_text_style = false;
                        }
                        "defRPr" if in_def_rpr => {
                            in_def_rpr = false;
                            // Assign color from Start+child pattern
                            if let (Some(color), Some(rd)) = (current_color.take(), current_run_defaults.as_mut()) {
                                if rd.color.is_none() {
                                    rd.color = Some(color);
                                }
                            }
                            if let Some(pd) = current_para_defaults.as_mut() {
                                pd.def_run_props = current_run_defaults.take();
                            }
                        }
                        // End of spacing containers
                        "lnSpc" if in_default_text_style => {
                            in_ln_spc = false;
                        }
                        "spcBef" if in_default_text_style => {
                            in_spc_bef = false;
                        }
                        "spcAft" if in_default_text_style => {
                            in_spc_aft = false;
                        }
                        s if in_default_text_style && master_parser::is_lvl_ppr(s) && current_lvl.is_some() => {
                            if let Some(pd) = current_para_defaults.take() {
                                let lvl = current_lvl.unwrap();
                                if lvl < 9 {
                                    default_text_style.levels[lvl] = Some(pd);
                                }
                            }
                            current_lvl = None;
                        }
                        "srgbClr" | "schemeClr" if in_def_rpr => {
                            if let Some(color) = current_color.take() {
                                if let Some(rd) = current_run_defaults.as_mut() {
                                    rd.color = Some(color);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(PptxError::Xml(e)),
                _ => {}
            }
        }

        let dts = if has_default_text_style {
            Some(default_text_style)
        } else {
            None
        };

        Ok((slide_size, slide_rel_ids, dts))
    }
}

// -- Helper functions --

/// Collect all relationship targets whose Type URL contains the given substring
fn collect_targets_by_type(rels: &HashMap<String, String>, type_fragment: &str) -> Vec<String> {
    // The relationships HashMap maps rId -> target path.
    // For type-based filtering, we need the full rels. But our current parser
    // only stores Id->Target. We use a name-based heuristic instead.
    let mut targets: Vec<String> = Vec::new();
    for target in rels.values() {
        let lower = target.to_lowercase();
        match type_fragment {
            "theme" if lower.contains("theme") && lower.ends_with(".xml") => {
                targets.push(target.clone());
            }
            "slideMaster" if lower.contains("slidemaster") => {
                targets.push(target.clone());
            }
            "slideLayout" if lower.contains("slidelayout") => {
                targets.push(target.clone());
            }
            "slide" if lower.contains("slide") && !lower.contains("master") && !lower.contains("layout") => {
                targets.push(target.clone());
            }
            _ => {}
        }
    }
    targets.sort();
    targets
}

/// Find single target by type fragment
fn find_target_by_type(rels: &HashMap<String, String>, type_fragment: &str) -> Option<String> {
    let results = collect_targets_by_type(rels, type_fragment);
    results.into_iter().next()
}

/// Normalize a target path relative to ppt/ directory
fn normalize_ppt_path(target: &str) -> String {
    if target.starts_with("ppt/") {
        target.to_string()
    } else {
        format!("ppt/{target}")
    }
}

/// Resolve a relative path from a base path
/// e.g. base="ppt/slideMasters/slideMaster1.xml", rel="../slideLayouts/slideLayout1.xml"
///   -> "ppt/slideLayouts/slideLayout1.xml"
fn resolve_relative_path(base: &str, rel: &str) -> String {
    if !rel.contains("..") {
        // Simple case: relative to same directory or already absolute-ish
        if rel.starts_with("ppt/") {
            return rel.to_string();
        }
        if let Some((dir, _)) = base.rsplit_once('/') {
            return format!("{dir}/{rel}");
        }
        return rel.to_string();
    }

    // Handle ../ navigation
    let base_parts: Vec<&str> = base.split('/').collect();
    // Remove filename from base
    let mut base_dir: Vec<&str> = base_parts[..base_parts.len() - 1].to_vec();

    for segment in rel.split('/') {
        if segment == ".." {
            base_dir.pop();
        } else {
            base_dir.push(segment);
        }
    }

    base_dir.join("/")
}

/// Extract canonical part name from a target path (just the filename without path)
fn canonical_part_name(path: &str) -> String {
    path.rsplit_once('/')
        .map(|(_, name)| name.to_string())
        .unwrap_or_else(|| path.to_string())
        .to_lowercase()
}
