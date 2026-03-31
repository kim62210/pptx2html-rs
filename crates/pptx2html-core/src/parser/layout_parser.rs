use std::collections::HashMap;
use std::io::{Read, Seek};

use quick_xml::Reader;
use quick_xml::events::Event;
use zip::ZipArchive;

use super::master_parser::{
    is_lvl_ppr, parse_def_rpr_attrs, parse_lvl_index, parse_lvl_ppr_attrs, parse_placeholder_attrs,
};
use super::slide_parser::parse_line_end;
use super::xml_utils;
use crate::error::{PptxError, PptxResult};
use crate::model::*;

/// Parse slideLayout XML into SlideLayout
pub fn parse_slide_layout<R: Read + Seek>(
    xml: &str,
    rels: &HashMap<String, String>,
    archive: &mut ZipArchive<R>,
) -> PptxResult<SlideLayout> {
    let mut reader = Reader::from_str(xml);
    let mut layout = SlideLayout::default();

    let mut depth: Vec<String> = Vec::new();
    let mut in_sp_tree = false;
    let mut current_shape: Option<LayoutShapeBuilder> = None;
    let mut in_nv_pr = false;
    let mut in_tx_body = false;
    let mut in_lst_style = false;
    let mut current_lvl: Option<usize> = None;
    let mut current_para_defaults: Option<ParagraphDefaults> = None;
    let mut current_run_defaults: Option<RunDefaults> = None;
    let mut in_def_rpr = false;
    let mut current_color: Option<Color> = None;
    let mut in_ln_spc = false;
    let mut in_spc_bef = false;
    let mut in_spc_aft = false;
    let mut in_ln = false;

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

    // Parse root element attributes
    // We handle them in the first Start event for sldLayout

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.push(local.clone());

                match local.as_str() {
                    "sldLayout" => {
                        // Root element attributes
                        if let Some(t) = xml_utils::attr_str(e, "type") {
                            layout.layout_type = Some(t);
                        }
                        if let Some(show) = xml_utils::attr_str(e, "showMasterSp") {
                            layout.show_master_sp = show != "0" && show != "false";
                        }
                    }
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
                    "srgbClr" if in_bg_pr && !in_bg_blip_fill => {
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
                    "schemeClr" if in_bg_pr && !in_bg_blip_fill => {
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
                    "sp" if in_sp_tree => {
                        current_shape = Some(LayoutShapeBuilder::default());
                    }
                    "txBody" if current_shape.is_some() => {
                        in_tx_body = true;
                    }
                    "bodyPr" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            if let Some(anchor) = xml_utils::attr_str(e, "anchor") {
                                sb.text_vertical_align = VerticalAlign::from_ooxml(&anchor);
                                sb.text_vertical_align_explicit = true;
                            }
                            if let Some(wrap) = xml_utils::attr_str(e, "wrap") {
                                sb.text_word_wrap = wrap != "none";
                                sb.text_word_wrap_explicit = true;
                            }
                        }
                    }
                    "normAutofit" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            let font_scale = xml_utils::attr_str(e, "fontScale")
                                .and_then(|s| s.parse::<f64>().ok())
                                .map(|v| v / 100000.0);
                            let line_spacing_reduction = xml_utils::attr_str(e, "lnSpcReduction")
                                .and_then(|s| s.parse::<f64>().ok())
                                .map(|v| v / 100000.0);
                            sb.text_auto_fit = AutoFit::Normal {
                                font_scale,
                                line_spacing_reduction,
                            };
                        }
                    }
                    "noAutofit" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::NoAutoFit;
                        }
                    }
                    "spAutoFit" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::Shrink;
                        }
                    }
                    "lstStyle" if in_tx_body => {
                        in_lst_style = true;
                    }
                    s if in_lst_style && is_lvl_ppr(s) => {
                        let lvl = parse_lvl_index(s);
                        current_lvl = Some(lvl);
                        let mut pd = ParagraphDefaults::default();
                        parse_lvl_ppr_attrs(e, &mut pd);
                        current_para_defaults = Some(pd);
                    }
                    "defRPr" if in_lst_style && current_lvl.is_some() => {
                        in_def_rpr = true;
                        let mut rd = RunDefaults::default();
                        parse_def_rpr_attrs(e, &mut rd);
                        current_run_defaults = Some(rd);
                    }
                    "lnSpc" if in_lst_style && current_lvl.is_some() && !in_def_rpr => {
                        in_ln_spc = true;
                    }
                    "spcBef" if in_lst_style && current_lvl.is_some() && !in_def_rpr => {
                        in_spc_bef = true;
                    }
                    "spcAft" if in_lst_style && current_lvl.is_some() && !in_def_rpr => {
                        in_spc_aft = true;
                    }
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
                    "nvPr" if current_shape.is_some() => {
                        in_nv_pr = true;
                    }
                    "ln" if current_shape.is_some() && depth.iter().any(|d| d == "spPr") => {
                        in_ln = true;
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
                    "clrMapOvr" => {
                        // Will check child elements
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
                    "lin" if in_bg_grad_fill => {
                        if let Some(ang) = xml_utils::attr_str(e, "ang") {
                            bg_grad_angle = ang.parse::<f64>().unwrap_or(0.0) / 60_000.0;
                        }
                        bg_grad_type = GradientType::Linear;
                    }
                    "bodyPr" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            if let Some(anchor) = xml_utils::attr_str(e, "anchor") {
                                sb.text_vertical_align = VerticalAlign::from_ooxml(&anchor);
                                sb.text_vertical_align_explicit = true;
                            }
                            if let Some(wrap) = xml_utils::attr_str(e, "wrap") {
                                sb.text_word_wrap = wrap != "none";
                                sb.text_word_wrap_explicit = true;
                            }
                        }
                    }
                    "path" if in_bg_grad_fill => {
                        if let Some(val) = xml_utils::attr_str(e, "path") {
                            bg_grad_type = GradientType::from_path_attr(&val);
                        }
                    }
                    "normAutofit" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            let font_scale = xml_utils::attr_str(e, "fontScale")
                                .and_then(|s| s.parse::<f64>().ok())
                                .map(|v| v / 100000.0);
                            let line_spacing_reduction = xml_utils::attr_str(e, "lnSpcReduction")
                                .and_then(|s| s.parse::<f64>().ok())
                                .map(|v| v / 100000.0);
                            sb.text_auto_fit = AutoFit::Normal {
                                font_scale,
                                line_spacing_reduction,
                            };
                        }
                    }
                    "noAutofit" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::NoAutoFit;
                        }
                    }
                    "spAutoFit" if current_shape.is_some() && in_tx_body => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = AutoFit::Shrink;
                        }
                    }
                    "srgbClr" if in_bg_pr && !in_bg_blip_fill => {
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
                    "schemeClr" if in_bg_pr && !in_bg_blip_fill => {
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
                    "srgbClr" if in_ln => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(sb) = current_shape.as_mut()
                        {
                            sb.border.color = Color::rgb(val);
                        }
                    }
                    "schemeClr" if in_ln => {
                        if let Some(val) = xml_utils::attr_str(e, "val")
                            && let Some(sb) = current_shape.as_mut()
                        {
                            sb.border.color = Color::theme(val);
                        }
                    }
                    // Placeholder in shape
                    "ph" if in_nv_pr && current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.placeholder = Some(parse_placeholder_attrs(e));
                        }
                    }
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
                    "spcPct"
                        if in_lst_style
                            && current_lvl.is_some()
                            && (in_ln_spc || in_spc_bef || in_spc_aft) =>
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
                        if in_lst_style
                            && current_lvl.is_some()
                            && (in_ln_spc || in_spc_bef || in_spc_aft) =>
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
                    // ClrMapOverride with masterClrMapping (use master's ClrMap)
                    "masterClrMapping" => {
                        layout.clr_map_ovr = Some(ClrMapOverride::UseMaster);
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
                    // overrideClrMapping (override ClrMap)
                    "overrideClrMapping" => {
                        let mut clr_map = crate::model::presentation::ClrMap::default();
                        for attr in e.attributes().flatten() {
                            let key = xml_utils::local_name(attr.key.as_ref());
                            let val = String::from_utf8_lossy(&attr.value);
                            clr_map.set(key, &val);
                        }
                        layout.clr_map_ovr = Some(ClrMapOverride::Override(clr_map));
                    }
                    "noFill" if in_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.style = BorderStyle::None;
                            sb.border.width = 0.0;
                            sb.border.no_fill = true;
                        }
                    }
                    "prstDash" if in_ln => {
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
                    "round" if in_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.join = LineJoin::Round;
                        }
                    }
                    "bevel" if in_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.join = LineJoin::Bevel;
                        }
                    }
                    "miter" if in_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.join = LineJoin::Miter;
                            sb.border.miter_limit = xml_utils::attr_str(e, "lim")
                                .and_then(|v| v.parse::<f64>().ok())
                                .map(|v| v / 100_000.0);
                        }
                    }
                    "headEnd" if in_ln => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.border.head_end = parse_line_end(e);
                        }
                    }
                    "tailEnd" if in_ln => {
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
                                let path = resolve_layout_rel_path("ppt/slideLayouts", target);
                                if let Ok(mut entry) = archive.by_name(&path) {
                                    let mut buf = Vec::new();
                                    let _ = std::io::Read::read_to_end(&mut entry, &mut buf);
                                    if !buf.is_empty() {
                                        let ct = layout_bg_mime(&path);
                                        layout.background = Some(Fill::Image(ImageFill {
                                            rel_id,
                                            data: buf,
                                            content_type: ct,
                                        }));
                                    }
                                }
                            }
                        } else if let Some(color) = bg_solid_color.take() {
                            layout.background = Some(Fill::Solid(SolidFill { color }));
                        } else if !bg_grad_stops.is_empty() {
                            layout.background = Some(Fill::Gradient(GradientFill {
                                gradient_type: std::mem::take(&mut bg_grad_type),
                                stops: std::mem::take(&mut bg_grad_stops),
                                angle: bg_grad_angle,
                            }));
                        }
                    }
                    "spTree" => in_sp_tree = false,
                    "nvPr" => in_nv_pr = false,
                    "txBody" => in_tx_body = false,
                    "lstStyle" => in_lst_style = false,
                    "defRPr" if in_def_rpr => {
                        if let Some(color) = current_color.take()
                            && let Some(rd) = current_run_defaults.as_mut()
                        {
                            rd.color = Some(color);
                        }
                        in_def_rpr = false;
                    }
                    "lnSpc" if in_ln_spc => in_ln_spc = false,
                    "spcBef" if in_spc_bef => in_spc_bef = false,
                    "spcAft" if in_spc_aft => in_spc_aft = false,
                    s if in_lst_style && is_lvl_ppr(&local) => {
                        if let Some(pd) = current_para_defaults.take() {
                            let mut pd = pd;
                            pd.def_run_props = current_run_defaults.take();
                            if let Some(sb) = current_shape.as_mut() {
                                store_shape_level_defaults(sb, s, pd);
                            }
                        }
                        current_lvl = None;
                    }
                    "ln" if in_ln => {
                        in_ln = false;
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
                            layout.shapes.push(sb.build());
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

    Ok(layout)
}

#[derive(Default)]
struct LayoutShapeBuilder {
    position: Position,
    size: Size,
    placeholder: Option<PlaceholderInfo>,
    list_style: Option<ListStyle>,
    text_vertical_align: VerticalAlign,
    text_vertical_align_explicit: bool,
    text_word_wrap: bool,
    text_word_wrap_explicit: bool,
    text_auto_fit: AutoFit,
    border: Border,
}

impl LayoutShapeBuilder {
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
            border: self.border,
            text_body: if self.list_style.is_some()
                || self.text_vertical_align_explicit
                || self.text_word_wrap_explicit
                || !matches!(self.text_auto_fit, AutoFit::None)
            {
                Some(TextBody {
                    list_style: self.list_style,
                    vertical_align: self.text_vertical_align,
                    vertical_align_explicit: self.text_vertical_align_explicit,
                    word_wrap,
                    word_wrap_explicit: self.text_word_wrap_explicit,
                    auto_fit: self.text_auto_fit,
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
    shape: &mut LayoutShapeBuilder,
    lvl_tag: &str,
    pd: ParagraphDefaults,
) {
    let lvl = parse_lvl_index(lvl_tag);
    if lvl >= 9 {
        return;
    }
    let list_style = shape.list_style.get_or_insert_with(ListStyle::default);
    list_style.levels[lvl] = Some(pd);
}

fn resolve_layout_rel_path(base_dir: &str, target: &str) -> String {
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

fn layout_bg_mime(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        _ => "image/png",
    }
    .to_string()
}
