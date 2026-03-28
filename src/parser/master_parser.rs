use std::collections::HashMap;
use std::io::{Read, Seek};

use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

use crate::error::{PptxError, PptxResult};
use crate::model::*;
use crate::model::presentation::ClrMap;
use super::xml_utils;

/// Parse slideMaster XML into SlideMaster
pub fn parse_slide_master<R: Read + Seek>(
    xml: &str,
    _rels: &HashMap<String, String>,
    _archive: &mut ZipArchive<R>,
) -> PptxResult<SlideMaster> {
    let mut reader = Reader::from_str(xml);
    let mut master = SlideMaster::default();

    let mut depth: Vec<String> = Vec::new();
    let mut in_tx_styles = false;
    let mut tx_style_kind: Option<TxStyleKind> = None;
    let mut current_lvl: Option<usize> = None;
    let mut current_para_defaults: Option<ParagraphDefaults> = None;
    let mut current_run_defaults: Option<RunDefaults> = None;
    let mut in_def_rpr = false;
    let mut current_color: Option<Color> = None;

    // Shape parsing state
    let mut in_sp_tree = false;
    let mut current_shape: Option<MasterShapeBuilder> = None;
    let mut in_nv_pr = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.push(local.clone());

                match local.as_str() {
                    "spTree" => in_sp_tree = true,
                    "txStyles" => in_tx_styles = true,
                    "titleStyle" if in_tx_styles => {
                        tx_style_kind = Some(TxStyleKind::Title);
                    }
                    "bodyStyle" if in_tx_styles => {
                        tx_style_kind = Some(TxStyleKind::Body);
                    }
                    "otherStyle" if in_tx_styles => {
                        tx_style_kind = Some(TxStyleKind::Other);
                    }
                    // Level paragraph properties (lvl1pPr .. lvl9pPr)
                    s if tx_style_kind.is_some() && is_lvl_ppr(s) => {
                        let lvl = parse_lvl_index(s);
                        current_lvl = Some(lvl);
                        let mut pd = ParagraphDefaults::default();
                        parse_lvl_ppr_attrs(e, &mut pd);
                        current_para_defaults = Some(pd);
                    }
                    "defRPr" if current_lvl.is_some() => {
                        in_def_rpr = true;
                        let mut rd = RunDefaults::default();
                        parse_def_rpr_attrs(e, &mut rd);
                        current_run_defaults = Some(rd);
                    }
                    // Color elements inside defRPr
                    "srgbClr" if in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            current_color = Some(Color::rgb(val));
                        }
                    }
                    "schemeClr" if in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            current_color = Some(Color::theme(val));
                        }
                    }
                    // Shape in spTree
                    "sp" if in_sp_tree && !in_tx_styles => {
                        current_shape = Some(MasterShapeBuilder::default());
                    }
                    "nvPr" if current_shape.is_some() => {
                        in_nv_pr = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();

                match local.as_str() {
                    // ClrMap
                    "clrMap" => {
                        master.clr_map = parse_clr_map_element(e);
                    }
                    // Placeholder in shape
                    "ph" if in_nv_pr && current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.placeholder = Some(parse_placeholder_attrs(e));
                        }
                    }
                    // Font in defRPr
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
                    // Color elements (Empty variant) inside defRPr
                    "srgbClr" if in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if let Some(rd) = current_run_defaults.as_mut() {
                                rd.color = Some(Color::rgb(val));
                            }
                        }
                    }
                    "schemeClr" if in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if let Some(rd) = current_run_defaults.as_mut() {
                                rd.color = Some(Color::theme(val));
                            }
                        }
                    }
                    // Empty lvlNpPr (no children)
                    s if tx_style_kind.is_some() && is_lvl_ppr(s) => {
                        let lvl = parse_lvl_index(s);
                        let mut pd = ParagraphDefaults::default();
                        parse_lvl_ppr_attrs(e, &mut pd);
                        store_level_defaults(
                            &tx_style_kind,
                            &mut master.tx_styles,
                            lvl,
                            pd,
                        );
                    }
                    // Empty defRPr (no children)
                    "defRPr" if current_lvl.is_some() && !in_def_rpr => {
                        let mut rd = RunDefaults::default();
                        parse_def_rpr_attrs(e, &mut rd);
                        if let Some(pd) = current_para_defaults.as_mut() {
                            pd.def_run_props = Some(rd);
                        }
                    }
                    // Position/size for shapes
                    "off" if current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.position.x = Emu::from_str(&xml_utils::attr_str(e, "x").unwrap_or_default());
                            sb.position.y = Emu::from_str(&xml_utils::attr_str(e, "y").unwrap_or_default());
                        }
                    }
                    "ext" if current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.size.width = Emu::from_str(&xml_utils::attr_str(e, "cx").unwrap_or_default());
                            sb.size.height = Emu::from_str(&xml_utils::attr_str(e, "cy").unwrap_or_default());
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.pop();

                match local.as_str() {
                    "txStyles" => {
                        in_tx_styles = false;
                        tx_style_kind = None;
                    }
                    "titleStyle" | "bodyStyle" | "otherStyle" => {
                        tx_style_kind = None;
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
                    s if is_lvl_ppr(s) && current_lvl.is_some() => {
                        if let Some(pd) = current_para_defaults.take() {
                            store_level_defaults(
                                &tx_style_kind,
                                &mut master.tx_styles,
                                current_lvl.unwrap(),
                                pd,
                            );
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
                    "spTree" => in_sp_tree = false,
                    "nvPr" => in_nv_pr = false,
                    "sp" if current_shape.is_some() => {
                        if let Some(sb) = current_shape.take() {
                            master.shapes.push(sb.build());
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

    Ok(master)
}

// Helper types and functions

enum TxStyleKind {
    Title,
    Body,
    Other,
}

pub fn is_lvl_ppr(s: &str) -> bool {
    matches!(
        s,
        "lvl1pPr" | "lvl2pPr" | "lvl3pPr" | "lvl4pPr" | "lvl5pPr"
        | "lvl6pPr" | "lvl7pPr" | "lvl8pPr" | "lvl9pPr"
    )
}

pub fn parse_lvl_index(s: &str) -> usize {
    // "lvl1pPr" -> 0, "lvl2pPr" -> 1, ...
    s.chars()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .map(|d| (d as usize).saturating_sub(1))
        .unwrap_or(0)
}

pub fn parse_lvl_ppr_attrs(e: &quick_xml::events::BytesStart<'_>, pd: &mut ParagraphDefaults) {
    if let Some(algn) = xml_utils::attr_str(e, "algn") {
        pd.alignment = Some(Alignment::from_ooxml(&algn));
    }
    if let Some(mar_l) = xml_utils::attr_str(e, "marL") {
        pd.margin_left = Some(Emu::from_str(&mar_l).to_pt());
    }
    if let Some(indent) = xml_utils::attr_str(e, "indent") {
        pd.indent = Some(Emu::from_str(&indent).to_pt());
    }
}

pub fn parse_def_rpr_attrs(e: &quick_xml::events::BytesStart<'_>, rd: &mut RunDefaults) {
    if let Some(sz) = xml_utils::attr_str(e, "sz") {
        rd.font_size = sz.parse::<f64>().ok().map(|v| v / 100.0);
    }
    if let Some(b) = xml_utils::attr_str(e, "b") {
        rd.bold = Some(b == "1" || b == "true");
    }
    if let Some(i) = xml_utils::attr_str(e, "i") {
        rd.italic = Some(i == "1" || i == "true");
    }
}

/// Parse <p:clrMap .../> attributes into ClrMap
pub fn parse_clr_map_element(e: &quick_xml::events::BytesStart<'_>) -> ClrMap {
    let mut clr_map = ClrMap::default();
    for attr in e.attributes().flatten() {
        let key = xml_utils::local_name(attr.key.as_ref());
        let val = String::from_utf8_lossy(&attr.value);
        clr_map.set(key, &val);
    }
    clr_map
}

/// Parse <p:ph type="..." idx="..."/> placeholder attributes
pub fn parse_placeholder_attrs(e: &quick_xml::events::BytesStart<'_>) -> PlaceholderInfo {
    let mut info = PlaceholderInfo::default();
    if let Some(t) = xml_utils::attr_str(e, "type") {
        info.ph_type = PlaceholderType::from_ooxml(&t);
    }
    if let Some(idx) = xml_utils::attr_str(e, "idx") {
        info.idx = idx.parse::<u32>().ok();
    }
    info
}

fn store_level_defaults(
    kind: &Option<TxStyleKind>,
    tx_styles: &mut TxStyles,
    lvl: usize,
    pd: ParagraphDefaults,
) {
    if lvl >= 9 {
        return;
    }
    let list_style = match kind {
        Some(TxStyleKind::Title) => {
            tx_styles.title_style.get_or_insert_with(ListStyle::default)
        }
        Some(TxStyleKind::Body) => {
            tx_styles.body_style.get_or_insert_with(ListStyle::default)
        }
        Some(TxStyleKind::Other) => {
            tx_styles.other_style.get_or_insert_with(ListStyle::default)
        }
        None => return,
    };
    list_style.levels[lvl] = Some(pd);
}

#[derive(Default)]
struct MasterShapeBuilder {
    position: Position,
    size: Size,
    placeholder: Option<PlaceholderInfo>,
}

impl MasterShapeBuilder {
    fn build(self) -> Shape {
        Shape {
            position: self.position,
            size: self.size,
            placeholder: self.placeholder,
            ..Default::default()
        }
    }
}
