use std::collections::HashMap;
use std::io::{Read, Seek};

use quick_xml::Reader;
use quick_xml::events::Event;
use zip::ZipArchive;

use super::slide_parser::{parse_autofit_ratio, parse_line_end};
use super::xml_utils;
use crate::error::{PptxError, PptxResult};
use crate::model::presentation::ClrMap;
use crate::model::*;

/// Parse slideMaster XML into SlideMaster
pub fn parse_slide_master<R: Read + Seek>(
    xml: &str,
    rels: &HashMap<String, String>,
    archive: &mut ZipArchive<R>,
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

    // Spacing parsing state for txStyles
    let mut in_ln_spc = false;
    let mut in_spc_bef = false;
    let mut in_spc_aft = false;

    // Shape parsing state
    let mut in_sp_tree = false;
    let mut current_shape: Option<MasterShapeBuilder> = None;
    let mut in_nv_pr = false;
    let mut in_shape_tx_body = false;
    let mut in_shape_lst_style = false;
    let mut shape_current_lvl: Option<usize> = None;
    let mut shape_para_defaults: Option<ParagraphDefaults> = None;
    let mut shape_run_defaults: Option<RunDefaults> = None;
    let mut in_shape_def_rpr = false;
    let mut shape_current_color: Option<Color> = None;
    let mut in_shape_ln_spc = false;
    let mut in_shape_spc_bef = false;
    let mut in_shape_spc_aft = false;
    let mut in_shape_ln = false;

    // Background parsing state
    let mut in_bg_pr = false;
    let mut in_bg_blip_fill = false;
    let mut bg_blip_rel_id: Option<String> = None;
    let mut bg_solid_color: Option<Color> = None;
    let mut in_bg_grad_fill = false;
    let mut bg_grad_stops: Vec<GradientStop> = Vec::new();
    let mut bg_grad_angle: f64 = 0.0;
    let mut bg_grad_type = GradientType::Linear;
    let mut bg_gs_pos: f64 = 0.0;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.push(local.clone());

                match local.as_str() {
                    // Background properties
                    "bgPr" => in_bg_pr = true,
                    "blipFill" if in_bg_pr => in_bg_blip_fill = true,
                    "blip" if in_bg_blip_fill => {
                        for attr in e.attributes().flatten() {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            if key.ends_with("embed") {
                                bg_blip_rel_id =
                                    Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }
                    "solidFill" if in_bg_pr && !in_bg_blip_fill => {}
                    "gradFill" if in_bg_pr => {
                        in_bg_grad_fill = true;
                        bg_grad_stops.clear();
                        bg_grad_angle = 0.0;
                        bg_grad_type = GradientType::Linear;
                    }
                    "path" if in_bg_grad_fill => {
                        if let Some(val) = xml_utils::attr_str(e, "path") {
                            bg_grad_type = GradientType::from_path_attr(&val);
                        }
                    }
                    "gs" if in_bg_grad_fill => {
                        bg_gs_pos = xml_utils::attr_str(e, "pos")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| v / 100_000.0)
                            .unwrap_or(0.0);
                    }
                    "srgbClr" if in_bg_pr && !in_bg_blip_fill && !in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::rgb(val);
                            if in_bg_grad_fill {
                                bg_grad_stops.push(GradientStop {
                                    position: bg_gs_pos,
                                    color,
                                });
                            } else {
                                bg_solid_color = Some(color);
                            }
                        }
                    }
                    "schemeClr" if in_bg_pr && !in_bg_blip_fill && !in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::theme(val);
                            if in_bg_grad_fill {
                                bg_grad_stops.push(GradientStop {
                                    position: bg_gs_pos,
                                    color,
                                });
                            } else {
                                bg_solid_color = Some(color);
                            }
                        }
                    }
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
                    // Spacing containers inside lvlNpPr
                    "lnSpc" if current_lvl.is_some() && !in_def_rpr => {
                        in_ln_spc = true;
                    }
                    "spcBef" if current_lvl.is_some() && !in_def_rpr => {
                        in_spc_bef = true;
                    }
                    "spcAft" if current_lvl.is_some() && !in_def_rpr => {
                        in_spc_aft = true;
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
                    "txBody" if current_shape.is_some() && !in_tx_styles => {
                        in_shape_tx_body = true;
                    }
                    "bodyPr" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            if let Some(anchor) = xml_utils::attr_str(e, "anchor") {
                                sb.text_vertical_align = VerticalAlign::from_ooxml(&anchor);
                                sb.text_vertical_align_explicit = true;
                            }
                            if let Some(anchor_ctr) = xml_utils::attr_str(e, "anchorCtr") {
                                sb.text_anchor_center = anchor_ctr == "1" || anchor_ctr == "true";
                            }
                            if let Some(rot) = xml_utils::attr_str(e, "rot") {
                                sb.text_rotation_deg = rot.parse::<f64>().unwrap_or(0.0) / 60_000.0;
                            }
                            if let Some(vert) = xml_utils::attr_str(e, "vert") {
                                sb.vertical_text_explicit = true;
                                sb.vertical_text = if vert == "horz" { None } else { Some(vert) };
                            }
                            if let Some(v) = xml_utils::attr_str(e, "lIns") {
                                sb.text_margins.left = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_left_explicit = true;
                            }
                            if let Some(v) = xml_utils::attr_str(e, "tIns") {
                                sb.text_margins.top = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_top_explicit = true;
                            }
                            if let Some(v) = xml_utils::attr_str(e, "rIns") {
                                sb.text_margins.right = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_right_explicit = true;
                            }
                            if let Some(v) = xml_utils::attr_str(e, "bIns") {
                                sb.text_margins.bottom = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_bottom_explicit = true;
                            }
                            if let Some(wrap) = xml_utils::attr_str(e, "wrap") {
                                sb.text_word_wrap = wrap != "none";
                                sb.text_word_wrap_explicit = true;
                            }
                        }
                    }
                    "normAutofit" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            let font_scale = parse_autofit_ratio(e, "fontScale");
                            let line_spacing_reduction = parse_autofit_ratio(e, "lnSpcReduction");
                            sb.text_auto_fit = AutoFit::Normal {
                                font_scale,
                                line_spacing_reduction,
                            };
                        }
                    }
                    "noAutofit" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::NoAutoFit;
                        }
                    }
                    "spAutoFit" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::Shrink;
                        }
                    }
                    "lstStyle" if in_shape_tx_body => {
                        in_shape_lst_style = true;
                    }
                    s if in_shape_lst_style && is_lvl_ppr(s) => {
                        let lvl = parse_lvl_index(s);
                        shape_current_lvl = Some(lvl);
                        let mut pd = ParagraphDefaults::default();
                        parse_lvl_ppr_attrs(e, &mut pd);
                        shape_para_defaults = Some(pd);
                    }
                    "defRPr" if in_shape_lst_style && shape_current_lvl.is_some() => {
                        in_shape_def_rpr = true;
                        let mut rd = RunDefaults::default();
                        parse_def_rpr_attrs(e, &mut rd);
                        shape_run_defaults = Some(rd);
                    }
                    "lnSpc"
                        if in_shape_lst_style
                            && shape_current_lvl.is_some()
                            && !in_shape_def_rpr =>
                    {
                        in_shape_ln_spc = true;
                    }
                    "spcBef"
                        if in_shape_lst_style
                            && shape_current_lvl.is_some()
                            && !in_shape_def_rpr =>
                    {
                        in_shape_spc_bef = true;
                    }
                    "spcAft"
                        if in_shape_lst_style
                            && shape_current_lvl.is_some()
                            && !in_shape_def_rpr =>
                    {
                        in_shape_spc_aft = true;
                    }
                    "srgbClr" if in_shape_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            shape_current_color = Some(Color::rgb(val));
                        }
                    }
                    "schemeClr" if in_shape_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            shape_current_color = Some(Color::theme(val));
                        }
                    }
                    "nvPr" if current_shape.is_some() => {
                        in_nv_pr = true;
                    }
                    "ln" if current_shape.is_some() && depth.iter().any(|d| d == "spPr") => {
                        in_shape_ln = true;
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.width = xml_utils::attr_str(e, "w")
                                .map(|w| Emu::parse_emu(&w).to_pt())
                                .unwrap_or(0.0);
                            sb.border.cap = match xml_utils::attr_str(e, "cap").as_deref() {
                                Some("rnd") => LineCap::Round,
                                Some("flat") => LineCap::Flat,
                                _ => LineCap::Square,
                            };
                            sb.border.compound = match xml_utils::attr_str(e, "cmpd").as_deref() {
                                Some("dbl") => CompoundLine::Double,
                                Some("thickThin") => CompoundLine::ThickThin,
                                Some("thinThick") => CompoundLine::ThinThick,
                                Some("tri") => CompoundLine::Triple,
                                _ => CompoundLine::Single,
                            };
                            sb.border.alignment = match xml_utils::attr_str(e, "algn").as_deref() {
                                Some("in") => LineAlignment::Inset,
                                _ => LineAlignment::Center,
                            };
                            sb.border.join = LineJoin::Miter;
                            sb.border.miter_limit = None;
                            sb.border.no_fill = false;
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();

                match local.as_str() {
                    // Background blip (Empty variant)
                    "blip" if in_bg_blip_fill => {
                        for attr in e.attributes().flatten() {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            if key.ends_with("embed") {
                                bg_blip_rel_id =
                                    Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }
                    // Background gradient direction (Empty variant)
                    "lin" if in_bg_grad_fill => {
                        if let Some(ang) = xml_utils::attr_str(e, "ang") {
                            bg_grad_angle = ang.parse::<f64>().unwrap_or(0.0) / 60_000.0;
                        }
                        bg_grad_type = GradientType::Linear;
                    }
                    "bodyPr" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            if let Some(anchor) = xml_utils::attr_str(e, "anchor") {
                                sb.text_vertical_align = VerticalAlign::from_ooxml(&anchor);
                                sb.text_vertical_align_explicit = true;
                            }
                            if let Some(anchor_ctr) = xml_utils::attr_str(e, "anchorCtr") {
                                sb.text_anchor_center = anchor_ctr == "1" || anchor_ctr == "true";
                            }
                            if let Some(rot) = xml_utils::attr_str(e, "rot") {
                                sb.text_rotation_deg = rot.parse::<f64>().unwrap_or(0.0) / 60_000.0;
                            }
                            if let Some(vert) = xml_utils::attr_str(e, "vert") {
                                sb.vertical_text_explicit = true;
                                sb.vertical_text = if vert == "horz" { None } else { Some(vert) };
                            }
                            if let Some(v) = xml_utils::attr_str(e, "lIns") {
                                sb.text_margins.left = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_left_explicit = true;
                            }
                            if let Some(v) = xml_utils::attr_str(e, "tIns") {
                                sb.text_margins.top = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_top_explicit = true;
                            }
                            if let Some(v) = xml_utils::attr_str(e, "rIns") {
                                sb.text_margins.right = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_right_explicit = true;
                            }
                            if let Some(v) = xml_utils::attr_str(e, "bIns") {
                                sb.text_margins.bottom = Emu::parse_emu(&v).to_pt();
                                sb.text_margin_bottom_explicit = true;
                            }
                            if let Some(wrap) = xml_utils::attr_str(e, "wrap") {
                                sb.text_word_wrap = wrap != "none";
                                sb.text_word_wrap_explicit = true;
                            }
                        }
                    }
                    "normAutofit" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            let font_scale = parse_autofit_ratio(e, "fontScale");
                            let line_spacing_reduction = parse_autofit_ratio(e, "lnSpcReduction");
                            sb.text_auto_fit = AutoFit::Normal {
                                font_scale,
                                line_spacing_reduction,
                            };
                        }
                    }
                    "noAutofit" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::NoAutoFit;
                        }
                    }
                    "spAutoFit" if current_shape.is_some() && in_shape_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::Shrink;
                        }
                    }
                    // Background gradient path type (Empty variant)
                    "path" if in_bg_grad_fill => {
                        if let Some(val) = xml_utils::attr_str(e, "path") {
                            bg_grad_type = GradientType::from_path_attr(&val);
                        }
                    }
                    // Background solid/gradient color (Empty variant)
                    "srgbClr" if in_bg_pr && !in_bg_blip_fill && !in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::rgb(val);
                            if in_bg_grad_fill && depth.iter().any(|d| d == "gs") {
                                bg_grad_stops.push(GradientStop {
                                    position: bg_gs_pos,
                                    color,
                                });
                            } else {
                                bg_solid_color = Some(color);
                            }
                        }
                    }
                    "schemeClr" if in_bg_pr && !in_bg_blip_fill && !in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::theme(val);
                            if in_bg_grad_fill && depth.iter().any(|d| d == "gs") {
                                bg_grad_stops.push(GradientStop {
                                    position: bg_gs_pos,
                                    color,
                                });
                            } else {
                                bg_solid_color = Some(color);
                            }
                        }
                    }
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
                    "latin" if in_shape_def_rpr => {
                        if let Some(rd) = shape_run_defaults.as_mut()
                            && let Some(typeface) = xml_utils::attr_str(e, "typeface")
                        {
                            rd.font_latin = Some(typeface);
                        }
                    }
                    "ea" if in_shape_def_rpr => {
                        if let Some(rd) = shape_run_defaults.as_mut()
                            && let Some(typeface) = xml_utils::attr_str(e, "typeface")
                        {
                            rd.font_ea = Some(typeface);
                        }
                    }
                    "cs" if in_shape_def_rpr => {
                        if let Some(rd) = shape_run_defaults.as_mut()
                            && let Some(typeface) = xml_utils::attr_str(e, "typeface")
                        {
                            rd.font_cs = Some(typeface);
                        }
                    }
                    "srgbClr" if in_shape_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(rd) = shape_run_defaults.as_mut()
                        {
                            rd.color = Some(Color::rgb(val));
                        }
                    }
                    "schemeClr" if in_shape_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(rd) = shape_run_defaults.as_mut()
                        {
                            rd.color = Some(Color::theme(val));
                        }
                    }
                    "spcPct"
                        if in_shape_lst_style
                            && shape_current_lvl.is_some()
                            && (in_shape_ln_spc || in_shape_spc_bef || in_shape_spc_aft) =>
                    {
                        if let Some(val_str) = xml_utils::attr_str(e, "val")
                            && let Ok(val) = val_str.parse::<f64>()
                        {
                            let spacing = SpacingValue::Percent(val / 100_000.0);
                            if let Some(pd) = shape_para_defaults.as_mut() {
                                if in_shape_ln_spc {
                                    pd.line_spacing = Some(spacing);
                                } else if in_shape_spc_bef {
                                    pd.space_before = Some(spacing);
                                } else if in_shape_spc_aft {
                                    pd.space_after = Some(spacing);
                                }
                            }
                        }
                    }
                    "spcPts"
                        if in_shape_lst_style
                            && shape_current_lvl.is_some()
                            && (in_shape_ln_spc || in_shape_spc_bef || in_shape_spc_aft) =>
                    {
                        if let Some(val_str) = xml_utils::attr_str(e, "val")
                            && let Ok(val) = val_str.parse::<f64>()
                        {
                            let spacing = SpacingValue::Points(val / 100.0);
                            if let Some(pd) = shape_para_defaults.as_mut() {
                                if in_shape_ln_spc {
                                    pd.line_spacing = Some(spacing);
                                } else if in_shape_spc_bef {
                                    pd.space_before = Some(spacing);
                                } else if in_shape_spc_aft {
                                    pd.space_after = Some(spacing);
                                }
                            }
                        }
                    }
                    // Font in defRPr
                    "latin" if in_def_rpr => {
                        if let Some(rd) = current_run_defaults.as_mut()
                            && let Some(typeface) = xml_utils::attr_str(e, "typeface")
                        {
                            rd.font_latin = Some(typeface);
                        }
                    }
                    "ea" if in_def_rpr => {
                        if let Some(rd) = current_run_defaults.as_mut()
                            && let Some(typeface) = xml_utils::attr_str(e, "typeface")
                        {
                            rd.font_ea = Some(typeface);
                        }
                    }
                    "cs" if in_def_rpr => {
                        if let Some(rd) = current_run_defaults.as_mut()
                            && let Some(typeface) = xml_utils::attr_str(e, "typeface")
                        {
                            rd.font_cs = Some(typeface);
                        }
                    }
                    // Color elements (Empty variant) inside defRPr
                    "srgbClr" if in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(rd) = current_run_defaults.as_mut()
                        {
                            rd.color = Some(Color::rgb(val));
                        }
                    }
                    "schemeClr" if in_def_rpr => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(rd) = current_run_defaults.as_mut()
                        {
                            rd.color = Some(Color::theme(val));
                        }
                    }
                    "srgbClr" if in_shape_ln => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(sb) = current_shape.as_mut()
                        {
                            sb.border.color = Color::rgb(val);
                        }
                    }
                    "schemeClr" if in_shape_ln => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(sb) = current_shape.as_mut()
                        {
                            sb.border.color = Color::theme(val);
                        }
                    }
                    // Spacing percentage/points (inside lnSpc/spcBef/spcAft)
                    "spcPct"
                        if current_lvl.is_some() && (in_ln_spc || in_spc_bef || in_spc_aft) =>
                    {
                        if let Some(val_str) = xml_utils::attr_str(e, "val")
                            && let Ok(val) = val_str.parse::<f64>()
                        {
                            let spacing = SpacingValue::Percent(val / 100_000.0);
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
                    "spcPts"
                        if current_lvl.is_some() && (in_ln_spc || in_spc_bef || in_spc_aft) =>
                    {
                        if let Some(val_str) = xml_utils::attr_str(e, "val")
                            && let Ok(val) = val_str.parse::<f64>()
                        {
                            let spacing = SpacingValue::Points(val / 100.0);
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
                    // Empty lvlNpPr (no children)
                    s if tx_style_kind.is_some() && is_lvl_ppr(s) => {
                        let lvl = parse_lvl_index(s);
                        let mut pd = ParagraphDefaults::default();
                        parse_lvl_ppr_attrs(e, &mut pd);
                        store_level_defaults(&tx_style_kind, &mut master.tx_styles, lvl, pd);
                    }
                    // Empty defRPr (no children)
                    "defRPr" if current_lvl.is_some() && !in_def_rpr => {
                        let mut rd = RunDefaults::default();
                        parse_def_rpr_attrs(e, &mut rd);
                        if let Some(pd) = current_para_defaults.as_mut() {
                            pd.def_run_props = Some(rd);
                        }
                    }
                    // Position/size for shapes — only inside <a:xfrm>
                    "off" if current_shape.is_some() && depth.iter().any(|d| d == "xfrm") => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.position.x =
                                Emu::parse_emu(&xml_utils::attr_str(e, "x").unwrap_or_default());
                            sb.position.y =
                                Emu::parse_emu(&xml_utils::attr_str(e, "y").unwrap_or_default());
                        }
                    }
                    "ext" if current_shape.is_some() && depth.iter().any(|d| d == "xfrm") => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.size.width =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cx").unwrap_or_default());
                            sb.size.height =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cy").unwrap_or_default());
                        }
                    }
                    "noFill" if in_shape_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.style = BorderStyle::None;
                            sb.border.width = 0.0;
                            sb.border.no_fill = true;
                        }
                    }
                    "prstDash" if in_shape_ln => {
                        if let Some(sb) = current_shape.as_mut()
                            && let Some(val) = xml_utils::attr_str(e, "val")
                        {
                            sb.border.style = match val.as_str() {
                                "solid" => BorderStyle::Solid,
                                "dash" | "lgDash" | "sysDash" => BorderStyle::Dashed,
                                "dot" | "sysDot" | "lgDashDot" | "lgDashDotDot" | "sysDashDot"
                                | "sysDashDotDot" => BorderStyle::Dotted,
                                _ => BorderStyle::Solid,
                            };
                            sb.border.dash_style = match val.as_str() {
                                "solid" => DashStyle::Solid,
                                "dash" => DashStyle::Dash,
                                "dot" => DashStyle::Dot,
                                "dashDot" => DashStyle::DashDot,
                                "lgDash" => DashStyle::LongDash,
                                "lgDashDot" => DashStyle::LongDashDot,
                                "lgDashDotDot" => DashStyle::LongDashDotDot,
                                "sysDash" => DashStyle::SystemDash,
                                "sysDot" => DashStyle::SystemDot,
                                "sysDashDot" => DashStyle::SystemDashDot,
                                "sysDashDotDot" => DashStyle::SystemDashDotDot,
                                _ => DashStyle::Solid,
                            };
                        }
                    }
                    "round" if in_shape_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.join = LineJoin::Round;
                        }
                    }
                    "bevel" if in_shape_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.join = LineJoin::Bevel;
                        }
                    }
                    "miter" if in_shape_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.join = LineJoin::Miter;
                            sb.border.miter_limit = xml_utils::attr_str(e, "lim")
                                .and_then(|v| v.parse::<f64>().ok())
                                .map(|v| v / 100_000.0);
                        }
                    }
                    "headEnd" if in_shape_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.head_end = parse_line_end(e);
                        }
                    }
                    "tailEnd" if in_shape_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.tail_end = parse_line_end(e);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.pop();

                match local.as_str() {
                    "blipFill" if in_bg_blip_fill => in_bg_blip_fill = false,
                    "gradFill" if in_bg_grad_fill => {
                        in_bg_grad_fill = false;
                    }
                    "bgPr" => {
                        in_bg_pr = false;
                        if let Some(rel_id) = bg_blip_rel_id.take() {
                            if let Some(target) = rels.get(&rel_id) {
                                let path = resolve_master_rel_path("ppt/slideMasters", target);
                                if let Ok(mut entry) = archive.by_name(&path) {
                                    let mut buf = Vec::new();
                                    let _ = std::io::Read::read_to_end(&mut entry, &mut buf);
                                    if !buf.is_empty() {
                                        let ct = bg_mime_from_ext(&path);
                                        master.background = Some(Fill::Image(ImageFill {
                                            rel_id,
                                            data: buf,
                                            content_type: ct,
                                        }));
                                    }
                                }
                            }
                        } else if let Some(color) = bg_solid_color.take() {
                            master.background = Some(Fill::Solid(SolidFill { color }));
                        } else if !bg_grad_stops.is_empty() {
                            master.background = Some(Fill::Gradient(GradientFill {
                                gradient_type: std::mem::take(&mut bg_grad_type),
                                stops: std::mem::take(&mut bg_grad_stops),
                                angle: bg_grad_angle,
                            }));
                        }
                    }
                    "txStyles" => {
                        in_tx_styles = false;
                        tx_style_kind = None;
                    }
                    "txBody" => in_shape_tx_body = false,
                    "lstStyle" => in_shape_lst_style = false,
                    "titleStyle" | "bodyStyle" | "otherStyle" => {
                        tx_style_kind = None;
                    }
                    "defRPr" if in_def_rpr => {
                        in_def_rpr = false;
                        // Assign color from Start+child pattern
                        if let (Some(color), Some(rd)) =
                            (current_color.take(), current_run_defaults.as_mut())
                            && rd.color.is_none()
                        {
                            rd.color = Some(color);
                        }
                        if let Some(pd) = current_para_defaults.as_mut() {
                            pd.def_run_props = current_run_defaults.take();
                        }
                    }
                    "defRPr" if in_shape_def_rpr => {
                        in_shape_def_rpr = false;
                        if let (Some(color), Some(rd)) =
                            (shape_current_color.take(), shape_run_defaults.as_mut())
                            && rd.color.is_none()
                        {
                            rd.color = Some(color);
                        }
                        if let Some(pd) = shape_para_defaults.as_mut() {
                            pd.def_run_props = shape_run_defaults.take();
                        }
                    }
                    s if is_lvl_ppr(s) && current_lvl.is_some() => {
                        if let (Some(pd), Some(lvl)) = (current_para_defaults.take(), current_lvl) {
                            store_level_defaults(&tx_style_kind, &mut master.tx_styles, lvl, pd);
                        }
                        current_lvl = None;
                    }
                    s if in_shape_lst_style && is_lvl_ppr(s) => {
                        if let (Some(pd), Some(lvl)) =
                            (shape_para_defaults.take(), shape_current_lvl)
                        {
                            store_shape_level_defaults(&mut current_shape, lvl, pd);
                        }
                        shape_current_lvl = None;
                    }
                    "srgbClr" | "schemeClr" if in_def_rpr => {
                        if let Some(color) = current_color.take()
                            && let Some(rd) = current_run_defaults.as_mut()
                        {
                            rd.color = Some(color);
                        }
                    }
                    // End of spacing containers
                    "lnSpc" => {
                        in_ln_spc = false;
                        in_shape_ln_spc = false;
                    }
                    "spcBef" => {
                        in_spc_bef = false;
                        in_shape_spc_bef = false;
                    }
                    "spcAft" => {
                        in_spc_aft = false;
                        in_shape_spc_aft = false;
                    }
                    "spTree" => in_sp_tree = false,
                    "nvPr" => in_nv_pr = false,
                    "ln" if in_shape_ln => {
                        in_shape_ln = false;
                        if let Some(sb) = current_shape.as_mut()
                            && !sb.border.no_fill
                            && sb.border.width > 0.0
                            && matches!(sb.border.style, BorderStyle::None)
                        {
                            sb.border.style = BorderStyle::Solid;
                        }
                    }
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
        "lvl1pPr"
            | "lvl2pPr"
            | "lvl3pPr"
            | "lvl4pPr"
            | "lvl5pPr"
            | "lvl6pPr"
            | "lvl7pPr"
            | "lvl8pPr"
            | "lvl9pPr"
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
        pd.margin_left = Some(Emu::parse_emu(&mar_l).to_pt());
    }
    if let Some(indent) = xml_utils::attr_str(e, "indent") {
        pd.indent = Some(Emu::parse_emu(&indent).to_pt());
    }
}

pub fn parse_def_rpr_attrs(e: &quick_xml::events::BytesStart<'_>, rd: &mut RunDefaults) {
    if let Some(sz) = xml_utils::attr_str(e, "sz") {
        rd.font_size = sz.parse::<f64>().ok().map(|v| v / 100.0);
    }
    if let Some(spc) = xml_utils::attr_str(e, "spc") {
        rd.letter_spacing = spc.parse::<f64>().ok().map(|v| v / 100.0);
    }
    if let Some(baseline) = xml_utils::attr_str(e, "baseline") {
        rd.baseline = baseline.parse::<i32>().ok();
    }
    if let Some(cap) = xml_utils::attr_str(e, "cap") {
        rd.capitalization = Some(TextCapitalization::from_ooxml(&cap));
    }
    if let Some(u) = xml_utils::attr_str(e, "u") {
        rd.underline = Some(UnderlineType::from_ooxml(&u));
    }
    if let Some(strike) = xml_utils::attr_str(e, "strike") {
        rd.strikethrough = Some(StrikethroughType::from_ooxml(&strike));
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
        Some(TxStyleKind::Title) => tx_styles.title_style.get_or_insert_with(ListStyle::default),
        Some(TxStyleKind::Body) => tx_styles.body_style.get_or_insert_with(ListStyle::default),
        Some(TxStyleKind::Other) => tx_styles.other_style.get_or_insert_with(ListStyle::default),
        None => return,
    };
    list_style.levels[lvl] = Some(pd);
}

#[derive(Default)]
struct MasterShapeBuilder {
    position: Position,
    size: Size,
    placeholder: Option<PlaceholderInfo>,
    list_style: Option<ListStyle>,
    text_vertical_align: VerticalAlign,
    text_vertical_align_explicit: bool,
    text_anchor_center: bool,
    text_rotation_deg: f64,
    text_margins: TextMargins,
    text_margin_top_explicit: bool,
    text_margin_bottom_explicit: bool,
    text_margin_left_explicit: bool,
    text_margin_right_explicit: bool,
    text_word_wrap: bool,
    text_word_wrap_explicit: bool,
    text_auto_fit: AutoFit,
    vertical_text: Option<String>,
    vertical_text_explicit: bool,
    border: Border,
}

impl MasterShapeBuilder {
    fn build(self) -> Shape {
        let word_wrap = if self.text_word_wrap_explicit {
            self.text_word_wrap
        } else {
            true
        };
        Shape {
            position: self.position,
            size: self.size,
            placeholder: self.placeholder,
            vertical_text: self.vertical_text,
            vertical_text_explicit: self.vertical_text_explicit,
            border: self.border,
            text_body: if self.list_style.is_some()
                || self.text_vertical_align_explicit
                || self.text_anchor_center
                || self.text_rotation_deg != 0.0
                || self.text_margin_top_explicit
                || self.text_margin_bottom_explicit
                || self.text_margin_left_explicit
                || self.text_margin_right_explicit
                || self.text_word_wrap_explicit
                || !matches!(self.text_auto_fit, AutoFit::None)
            {
                Some(TextBody {
                    list_style: self.list_style,
                    vertical_align: self.text_vertical_align,
                    vertical_align_explicit: self.text_vertical_align_explicit,
                    anchor_center: self.text_anchor_center,
                    text_rotation_deg: self.text_rotation_deg,
                    margin_top_explicit: self.text_margin_top_explicit,
                    margin_bottom_explicit: self.text_margin_bottom_explicit,
                    margin_left_explicit: self.text_margin_left_explicit,
                    margin_right_explicit: self.text_margin_right_explicit,
                    word_wrap,
                    word_wrap_explicit: self.text_word_wrap_explicit,
                    auto_fit: self.text_auto_fit,
                    margins: self.text_margins,
                    ..Default::default()
                })
            } else {
                None
            },
            ..Default::default()
        }
    }
}

fn store_shape_level_defaults(
    shape: &mut Option<MasterShapeBuilder>,
    lvl: usize,
    pd: ParagraphDefaults,
) {
    if lvl >= 9 {
        return;
    }
    if let Some(shape) = shape.as_mut() {
        let list_style = shape.list_style.get_or_insert_with(ListStyle::default);
        list_style.levels[lvl] = Some(pd);
    }
}

fn resolve_master_rel_path(base_dir: &str, target: &str) -> String {
    if !target.contains("../") {
        return format!("{base_dir}/{target}");
    }
    let mut parts: Vec<&str> = base_dir.split('/').collect();
    for segment in target.split('/') {
        if segment == ".." {
            parts.pop();
        } else if !segment.is_empty() && segment != "." {
            parts.push(segment);
        }
    }
    parts.join("/")
}

fn bg_mime_from_ext(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "emf" => "image/x-emf",
        "wmf" => "image/x-wmf",
        _ => "image/png",
    }
    .to_string()
}
