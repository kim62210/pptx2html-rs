use std::collections::HashMap;
use std::io::{Read, Seek};

use log::warn;
use quick_xml::Reader;
use quick_xml::events::Event;
use zip::ZipArchive;

use super::chart_parser;
use super::master_parser::{is_lvl_ppr, parse_def_rpr_attrs, parse_lvl_index, parse_lvl_ppr_attrs};
use super::relationships;
use super::xml_utils;
use crate::error::{PptxError, PptxResult};
use crate::model::*;

/// Parse slide XML
pub fn parse_slide<R: Read + Seek>(
    xml: &str,
    rels: &HashMap<String, String>,
    archive: &mut ZipArchive<R>,
) -> PptxResult<Slide> {
    let mut reader = Reader::from_str(xml);
    let mut slide = Slide::default();
    let mut depth: Vec<String> = Vec::new();

    let mut current_shape: Option<ShapeBuilder> = None;
    let mut current_paragraph: Option<ParagraphBuilder> = None;
    let mut current_run: Option<RunBuilder> = None;
    let mut in_text = false;

    // Fill/Border/Color parsing state
    let mut current_color: Option<Color> = None;
    let mut in_sp_pr = false;
    let mut in_ln = false;
    let mut in_r_pr = false;
    let mut in_nv_pr = false;
    let mut in_grad_fill = false;
    let mut grad_stops: Vec<GradientStop> = Vec::new();
    let mut grad_angle: f64 = 0.0;
    let mut grad_type = GradientType::Linear;
    let mut current_gs_pos: f64 = 0.0;

    // Paragraph spacing nesting state
    let mut in_ln_spc = false;
    let mut in_spc_bef = false;
    let mut in_spc_aft = false;

    // Paragraph-level defRPr nesting state
    let mut in_para_def_rpr = false;

    let mut in_shape_lst_style = false;
    let mut in_shape_def_rpr = false;
    let mut in_shape_ln_spc = false;
    let mut in_shape_spc_bef = false;
    let mut in_shape_spc_aft = false;
    let mut current_shape_lvl: Option<usize> = None;
    let mut current_shape_para_defaults: Option<ParagraphDefaults> = None;
    let mut current_shape_run_defaults: Option<RunDefaults> = None;

    // Bullet color nesting state
    let mut in_bu_clr = false;

    // Shape style reference (<p:style>) state
    let mut in_p_style = false;
    let mut p_style_builder: Option<ShapeStyleRef> = None;
    let mut p_style_current_ref: Option<String> = None;
    let mut p_style_idx: Option<String> = None;

    // Table parsing state
    let mut in_graphic_frame = false;
    let mut in_tbl = false;
    let mut in_tr = false;
    let mut in_tc = false;
    let mut in_tc_pr = false;
    let mut tc_border_side: Option<String> = None;
    let mut table_builder: Option<TableBuilder> = None;
    let mut current_row: Option<TableRowBuilder> = None;
    let mut current_cell: Option<TableCellBuilder> = None;
    let mut cell_paragraphs: Vec<TextParagraph> = Vec::new();
    let mut cell_paragraph: Option<ParagraphBuilder> = None;
    let mut cell_run: Option<RunBuilder> = None;
    let mut in_cell_text = false;
    let mut in_cell_r_pr = false;
    let mut in_cell_bu_clr = false;

    // Group shape parsing state
    let mut grp_stack: Vec<GroupContext> = Vec::new();
    let mut in_grp_sp_pr = false;

    // Adjust value parsing state
    let mut in_av_lst = false;

    // Custom geometry parsing state
    let mut in_cust_geom = false;
    let mut in_cust_geom_path = false;
    let mut cust_geom_paths: Vec<GeometryPath> = Vec::new();
    let mut cust_geom_cmds: Vec<PathCommand> = Vec::new();
    let mut cust_geom_path_w: f64 = 0.0;
    let mut cust_geom_path_h: f64 = 0.0;
    let mut cust_geom_path_fill = PathFill::Norm;
    let mut cust_geom_pts: Vec<(f64, f64)> = Vec::new();
    let mut in_cust_geom_cmd: Option<String> = None;
    let mut cust_geom_guides: HashMap<String, f64> = HashMap::new();
    let mut cust_geom_text_rect: Option<GeomRect> = None;
    let mut cust_geom_handles: Vec<AdjustHandle> = Vec::new();
    let mut cust_geom_connection_sites: Vec<ConnectionSite> = Vec::new();
    let mut current_xy_handle: Option<XYAdjustHandle> = None;
    let mut current_polar_handle: Option<PolarAdjustHandle> = None;
    let mut current_connection_site: Option<ConnectionSite> = None;

    // Text shadow and highlight parsing state
    let mut in_effect_lst = false;
    let mut in_outer_shdw = false;
    let mut outer_shdw_blur: f64 = 0.0;
    let mut outer_shdw_dist: f64 = 0.0;
    let mut outer_shdw_dir: f64 = 0.0;
    let mut in_highlight = false;

    // Shape-level effectLst parsing state
    let mut in_shape_effect_lst = false;
    let mut in_shape_outer_shdw = false;
    let mut shape_shdw_blur: f64 = 0.0;
    let mut shape_shdw_dist: f64 = 0.0;
    let mut shape_shdw_dir: f64 = 0.0;
    let mut shape_shdw_alpha: f64 = 1.0;
    let mut in_shape_glow = false;
    let mut shape_glow_rad: f64 = 0.0;
    let mut shape_glow_alpha: f64 = 1.0;
    let mut shape_effect_color: Option<Color> = None;

    // Background fill state
    let mut in_bg_pr = false;
    let mut in_bg_blip_fill = false;
    let mut bg_blip_rel_id: Option<String> = None;
    let mut bg_solid_color: Option<Color> = None;
    let mut in_bg_grad_fill = false;
    let mut bg_grad_stops: Vec<GradientStop> = Vec::new();
    let mut bg_grad_angle: f64 = 0.0;
    let mut bg_grad_type = GradientType::Linear;
    let mut bg_gs_pos: f64 = 0.0;

    // Chart detection state
    let mut in_graphic_data = false;
    let mut graphic_data_is_chart = false;
    // Raw XML capture buffer for unsupported graphicData content
    let mut capturing_raw_xml = false;
    let mut raw_xml_buf = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.push(local.clone());

                if local == "cNvPr" && current_shape.is_some() {
                    parse_shape_identity(e, &mut current_shape);
                    continue;
                }
                if local == "stCxn" && current_shape.as_ref().is_some_and(|s| s.is_connector) {
                    parse_connector_ref(e, &mut current_shape, true);
                    continue;
                }
                if local == "endCxn" && current_shape.as_ref().is_some_and(|s| s.is_connector) {
                    parse_connector_ref(e, &mut current_shape, false);
                    continue;
                }

                // Capture raw XML inside graphicData for unresolved content
                if capturing_raw_xml && local != "graphicData" {
                    raw_xml_buf.push('<');
                    raw_xml_buf.push_str(std::str::from_utf8(e.name().as_ref()).unwrap_or(&local));
                    for attr in e.attributes().flatten() {
                        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                        let val = std::str::from_utf8(&attr.value).unwrap_or("");
                        raw_xml_buf.push(' ');
                        raw_xml_buf.push_str(key);
                        raw_xml_buf.push_str("=\"");
                        raw_xml_buf.push_str(val);
                        raw_xml_buf.push('"');
                    }
                    raw_xml_buf.push('>');
                }

                match local.as_str() {
                    // ── Group shape ──
                    "grpSp" => {
                        grp_stack.push(GroupContext {
                            shapes: Vec::new(),
                            position: Position::default(),
                            size: Size::default(),
                            child_offset: Position::default(),
                            child_extent: Size::default(),
                        });
                    }
                    // Group shape properties
                    "grpSpPr" if !grp_stack.is_empty() => {
                        in_grp_sp_pr = true;
                    }
                    // ── Background properties ──
                    "bgPr" => {
                        in_bg_pr = true;
                    }
                    // ── Graphic frame (tables/charts) ──
                    "graphicFrame" => {
                        in_graphic_frame = true;
                        current_shape = Some(ShapeBuilder::default());
                    }
                    // Graphic data with URI (detect charts / SmartArt / OLE / Math)
                    "graphicData" if in_graphic_frame => {
                        in_graphic_data = true;
                        if let Some(uri) = xml_utils::attr_str(e, "uri") {
                            if uri.contains("chart") {
                                graphic_data_is_chart = true;
                                if let Some(sb) = current_shape.as_mut() {
                                    sb.is_chart = true;
                                }
                            } else if uri.contains("diagram") || uri.contains("dgm") {
                                warn!("SmartArt diagram detected — rendering placeholder");
                                if let Some(sb) = current_shape.as_mut() {
                                    sb.unsupported_content = Some("SmartArt".to_string());
                                    sb.unresolved_type = Some(slide::UnresolvedType::SmartArt);
                                }
                                capturing_raw_xml = true;
                                raw_xml_buf.clear();
                            } else if uri.contains("oleObject") {
                                warn!("OLE object detected — rendering placeholder");
                                if let Some(sb) = current_shape.as_mut() {
                                    sb.unsupported_content = Some("OLE Object".to_string());
                                    sb.unresolved_type = Some(slide::UnresolvedType::OleObject);
                                }
                                capturing_raw_xml = true;
                                raw_xml_buf.clear();
                            } else if uri.contains("math") || uri.contains("omml") {
                                warn!("Math equation detected — rendering placeholder");
                                if let Some(sb) = current_shape.as_mut() {
                                    sb.unsupported_content = Some("Math Equation".to_string());
                                    sb.unresolved_type = Some(slide::UnresolvedType::MathEquation);
                                }
                                capturing_raw_xml = true;
                                raw_xml_buf.clear();
                            }
                        }
                    }
                    // Table start
                    "tbl" if in_graphic_frame => {
                        in_tbl = true;
                        table_builder = Some(TableBuilder::default());
                    }
                    // Table properties (bandRow, bandCol, firstRow, etc.)
                    "tblPr" if in_tbl => {
                        if let Some(ref mut tb) = table_builder {
                            let parse_bool = |val: &str| val == "1" || val == "true";
                            if let Some(v) = xml_utils::attr_str(e, "bandRow") {
                                tb.band_row = parse_bool(&v);
                            }
                            if let Some(v) = xml_utils::attr_str(e, "bandCol") {
                                tb.band_col = parse_bool(&v);
                            }
                            if let Some(v) = xml_utils::attr_str(e, "firstRow") {
                                tb.first_row = parse_bool(&v);
                            }
                            if let Some(v) = xml_utils::attr_str(e, "lastRow") {
                                tb.last_row = parse_bool(&v);
                            }
                            if let Some(v) = xml_utils::attr_str(e, "firstCol") {
                                tb.first_col = parse_bool(&v);
                            }
                            if let Some(v) = xml_utils::attr_str(e, "lastCol") {
                                tb.last_col = parse_bool(&v);
                            }
                        }
                    }
                    // Table column width (open-tag variant: <a:gridCol w="..."></a:gridCol>)
                    "gridCol" if in_tbl => {
                        if let Some(ref mut tb) = table_builder {
                            let w = xml_utils::attr_str(e, "w")
                                .map(|v| Emu::parse_emu(&v).to_px())
                                .unwrap_or(0.0);
                            tb.col_widths.push(w);
                        }
                    }
                    // Table row
                    "tr" if in_tbl => {
                        in_tr = true;
                        let h = xml_utils::attr_str(e, "h")
                            .map(|v| Emu::parse_emu(&v).to_px())
                            .unwrap_or(0.0);
                        current_row = Some(TableRowBuilder {
                            height: h,
                            cells: Vec::new(),
                        });
                    }
                    // Table cell
                    "tc" if in_tr => {
                        in_tc = true;
                        let col_span = xml_utils::attr_str(e, "gridSpan")
                            .and_then(|v| v.parse::<u32>().ok())
                            .unwrap_or(1);
                        let row_span = xml_utils::attr_str(e, "rowSpan")
                            .and_then(|v| v.parse::<u32>().ok())
                            .unwrap_or(1);
                        let v_merge = xml_utils::attr_str(e, "vMerge")
                            .map(|v| v == "1" || v == "true")
                            .unwrap_or(false);
                        current_cell = Some(TableCellBuilder {
                            text_body: None,
                            fill: Fill::None,
                            border_left: Border::default(),
                            border_right: Border::default(),
                            border_top: Border::default(),
                            border_bottom: Border::default(),
                            col_span,
                            row_span,
                            v_merge,
                            margin_left: 7.2,   // OOXML default 91440 EMU
                            margin_right: 7.2,  // OOXML default 91440 EMU
                            margin_top: 3.6,    // OOXML default 45720 EMU
                            margin_bottom: 3.6, // OOXML default 45720 EMU
                            vertical_align: VerticalAlign::Top,
                        });
                        cell_paragraphs.clear();
                    }
                    // Table cell properties
                    "tcPr" if in_tc => {
                        in_tc_pr = true;
                        if let Some(ref mut cell) = current_cell {
                            // Cell margins (marL, marR, marT, marB in EMU)
                            if let Some(v) = xml_utils::attr_str(e, "marL") {
                                cell.margin_left = Emu::parse_emu(&v).to_pt();
                            }
                            if let Some(v) = xml_utils::attr_str(e, "marR") {
                                cell.margin_right = Emu::parse_emu(&v).to_pt();
                            }
                            if let Some(v) = xml_utils::attr_str(e, "marT") {
                                cell.margin_top = Emu::parse_emu(&v).to_pt();
                            }
                            if let Some(v) = xml_utils::attr_str(e, "marB") {
                                cell.margin_bottom = Emu::parse_emu(&v).to_pt();
                            }
                            // Vertical alignment (anchor attribute)
                            if let Some(v) = xml_utils::attr_str(e, "anchor") {
                                cell.vertical_align = VerticalAlign::from_ooxml(&v);
                            }
                        }
                    }
                    // Table cell border elements inside tcPr
                    "lnL" | "lnR" | "lnT" | "lnB" if in_tc_pr => {
                        tc_border_side = Some(local.clone());
                        if let Some(w) = xml_utils::attr_str(e, "w") {
                            let width = Emu::parse_emu(&w).to_pt();
                            if let Some(ref mut cell) = current_cell {
                                if local == "lnL" {
                                    cell.border_left.width = width;
                                } else if local == "lnR" {
                                    cell.border_right.width = width;
                                } else if local == "lnT" {
                                    cell.border_top.width = width;
                                } else if local == "lnB" {
                                    cell.border_bottom.width = width;
                                }
                            }
                        }
                    }
                    // Dash style inside table cell border
                    "prstDash" if in_tc_pr && tc_border_side.is_some() => {
                        if let Some(ref mut cell) = current_cell
                            && let Some(val) = xml_utils::attr_str(e, "val")
                        {
                            let border_style = match val.as_str() {
                                "solid" => BorderStyle::Solid,
                                "dash" | "lgDash" | "sysDash" => BorderStyle::Dashed,
                                "dot" | "sysDot" | "lgDashDot" | "lgDashDotDot" => {
                                    BorderStyle::Dotted
                                }
                                _ => BorderStyle::Solid,
                            };
                            let dash = match val.as_str() {
                                "solid" => DashStyle::Solid,
                                "dash" => DashStyle::Dash,
                                "dot" => DashStyle::Dot,
                                "dashDot" => DashStyle::DashDot,
                                "lgDash" => DashStyle::LongDash,
                                "lgDashDot" => DashStyle::LongDashDot,
                                "lgDashDotDot" => DashStyle::LongDashDotDot,
                                "sysDash" => DashStyle::SystemDash,
                                "sysDot" => DashStyle::SystemDot,
                                _ => DashStyle::Solid,
                            };
                            if let Some(border_side) = tc_border_side.as_deref() {
                                if border_side == "lnL" {
                                    cell.border_left.style = border_style;
                                    cell.border_left.dash_style = dash;
                                } else if border_side == "lnR" {
                                    cell.border_right.style = border_style;
                                    cell.border_right.dash_style = dash;
                                } else if border_side == "lnT" {
                                    cell.border_top.style = border_style;
                                    cell.border_top.dash_style = dash;
                                } else if border_side == "lnB" {
                                    cell.border_bottom.style = border_style;
                                    cell.border_bottom.dash_style = dash;
                                }
                            }
                        }
                    }
                    // Paragraph inside table cell
                    "p" if in_tc => {
                        cell_paragraph = Some(ParagraphBuilder::default());
                    }
                    // Paragraph properties inside table cell
                    "pPr" if in_tc && cell_paragraph.is_some() => {
                        parse_para_props(e, &mut cell_paragraph);
                    }
                    // Paragraph-level defRPr inside table cell paragraph
                    "defRPr" if in_tc && cell_paragraph.is_some() && cell_run.is_none() => {
                        in_para_def_rpr = true;
                        if let Some(pb) = cell_paragraph.as_mut() {
                            apply_paragraph_def_rpr(pb, e);
                        }
                    }
                    // Spacing containers inside table cell
                    "lnSpc" if in_tc && cell_paragraph.is_some() => {
                        in_ln_spc = true;
                    }
                    "spcBef" if in_tc && cell_paragraph.is_some() => {
                        in_spc_bef = true;
                    }
                    "spcAft" if in_tc && cell_paragraph.is_some() => {
                        in_spc_aft = true;
                    }
                    // Bullet color inside table cell
                    "buClr" if in_tc && cell_paragraph.is_some() => {
                        in_cell_bu_clr = true;
                    }
                    // Run inside table cell
                    "r" if in_tc && cell_paragraph.is_some() => {
                        cell_run = Some(RunBuilder::default());
                    }
                    // Run properties inside table cell
                    "rPr" if in_tc && cell_run.is_some() => {
                        in_cell_r_pr = true;
                        parse_run_props(e, &mut cell_run);
                    }
                    // Text content inside table cell
                    "t" if in_tc && cell_run.is_some() => {
                        in_cell_text = true;
                    }

                    // ── Regular shape handling ──
                    // Shape start
                    "sp" | "pic" | "cxnSp" => {
                        current_shape = Some(ShapeBuilder::default());
                        if let Some(sb) = &mut current_shape {
                            if local == "pic" {
                                sb.is_picture = true;
                            }
                            if local == "cxnSp" {
                                sb.is_connector = true;
                            }
                        }
                    }
                    // Non-visual properties (contains placeholder)
                    "nvPr" if current_shape.is_some() => {
                        in_nv_pr = true;
                    }
                    // Shape properties
                    "spPr" if current_shape.is_some() => {
                        in_sp_pr = true;
                    }
                    // Transform (rotation, flip)
                    "xfrm" if in_sp_pr => {
                        if let Some(sb) = current_shape.as_mut() {
                            apply_shape_transform(sb, e);
                        }
                    }
                    // Line/border
                    "ln" if in_sp_pr => {
                        in_ln = true;
                        if let Some(sb) = &mut current_shape {
                            if let Some(w) = xml_utils::attr_str(e, "w") {
                                sb.border_width = Emu::parse_emu(&w).to_pt();
                            } else {
                                sb.border_width = 0.0;
                            }
                            sb.border_cap = match xml_utils::attr_str(e, "cap").as_deref() {
                                Some("rnd") => LineCap::Round,
                                Some("flat") => LineCap::Flat,
                                _ => LineCap::Square,
                            };
                            sb.border_compound = match xml_utils::attr_str(e, "cmpd").as_deref() {
                                Some("dbl") => CompoundLine::Double,
                                Some("thickThin") => CompoundLine::ThickThin,
                                Some("thinThick") => CompoundLine::ThinThick,
                                Some("tri") => CompoundLine::Triple,
                                _ => CompoundLine::Single,
                            };
                            sb.border_alignment = match xml_utils::attr_str(e, "algn").as_deref() {
                                Some("in") => LineAlignment::Inset,
                                _ => LineAlignment::Center,
                            };
                        }
                    }
                    // Text body (non-table)
                    "txBody" if !in_tc => {
                        if let Some(sb) = &mut current_shape {
                            sb.has_text_body = true;
                        }
                    }
                    // bodyPr — text area properties
                    "bodyPr" if current_shape.is_some() && !in_tc => {
                        parse_body_pr(e, &mut current_shape);
                    }
                    "lstStyle" if current_shape.is_some() && !in_tc => {
                        in_shape_lst_style = true;
                    }
                    s if in_shape_lst_style && is_lvl_ppr(s) => {
                        let lvl = parse_lvl_index(s);
                        current_shape_lvl = Some(lvl);
                        let mut pd = ParagraphDefaults::default();
                        parse_lvl_ppr_attrs(e, &mut pd);
                        current_shape_para_defaults = Some(pd);
                    }
                    "defRPr" if in_shape_lst_style && current_shape_lvl.is_some() => {
                        in_shape_def_rpr = true;
                        let mut rd = RunDefaults::default();
                        parse_def_rpr_attrs(e, &mut rd);
                        current_shape_run_defaults = Some(rd);
                    }
                    "lnSpc"
                        if in_shape_lst_style
                            && current_shape_lvl.is_some()
                            && !in_shape_def_rpr =>
                    {
                        in_shape_ln_spc = true;
                    }
                    "spcBef"
                        if in_shape_lst_style
                            && current_shape_lvl.is_some()
                            && !in_shape_def_rpr =>
                    {
                        in_shape_spc_bef = true;
                    }
                    "spcAft"
                        if in_shape_lst_style
                            && current_shape_lvl.is_some()
                            && !in_shape_def_rpr =>
                    {
                        in_shape_spc_aft = true;
                    }
                    // normAutofit — shrink text to fit (child of bodyPr)
                    "normAutofit" if current_shape.is_some() && !in_tc => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = parse_shape_auto_fit(local.as_str(), e);
                        }
                    }
                    "noAutofit" if current_shape.is_some() && !in_tc => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = parse_shape_auto_fit(local.as_str(), e);
                        }
                    }
                    // spAutoFit — resize shape to fit text (child of bodyPr)
                    "spAutoFit" if current_shape.is_some() && !in_tc => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = parse_shape_auto_fit(local.as_str(), e);
                        }
                    }
                    // Paragraph (non-table)
                    "p" if current_shape.is_some() && !in_tc => {
                        current_paragraph = Some(ParagraphBuilder::default());
                    }
                    // Paragraph properties
                    "pPr" if current_paragraph.is_some() && !in_tc => {
                        parse_para_props(e, &mut current_paragraph);
                    }
                    // Paragraph-level defRPr (default run properties inside pPr)
                    "defRPr" if current_paragraph.is_some() && !in_tc && current_run.is_none() => {
                        in_para_def_rpr = true;
                        if let Some(pb) = current_paragraph.as_mut() {
                            apply_paragraph_def_rpr(pb, e);
                        }
                    }
                    // Paragraph spacing containers (non-table)
                    "lnSpc" if current_paragraph.is_some() && !in_tc => {
                        in_ln_spc = true;
                    }
                    "spcBef" if current_paragraph.is_some() && !in_tc => {
                        in_spc_bef = true;
                    }
                    "spcAft" if current_paragraph.is_some() && !in_tc => {
                        in_spc_aft = true;
                    }
                    // Bullet color container (non-table)
                    "buClr" if current_paragraph.is_some() && !in_tc => {
                        in_bu_clr = true;
                    }
                    // Text run (non-table)
                    "r" if current_paragraph.is_some() && !in_tc => {
                        current_run = Some(RunBuilder::default());
                    }
                    // Run properties (non-table)
                    "rPr" if current_run.is_some() && !in_tc => {
                        in_r_pr = true;
                        parse_run_props(e, &mut current_run);
                    }
                    "hlinkClick" if in_r_pr || in_cell_r_pr => {
                        if let Some(rel_id) = hyperlink_rel_id(e) {
                            let target = rels.get(&rel_id).cloned();
                            if let Some(rb) = current_run.as_mut() {
                                rb.hyperlink = target.clone();
                            }
                            if let Some(rb) = cell_run.as_mut() {
                                rb.hyperlink = target;
                            }
                        }
                    }
                    // Text content (non-table)
                    "t" if current_run.is_some() && !in_tc => {
                        in_text = true;
                    }
                    // Shape style reference (<p:style>)
                    "style" if current_shape.is_some() && !in_sp_pr => {
                        in_p_style = true;
                        p_style_builder = Some(ShapeStyleRef::default());
                    }
                    // Style ref children (lnRef, fillRef, effectRef, fontRef)
                    "lnRef" | "fillRef" | "effectRef" | "fontRef" if in_p_style => {
                        p_style_current_ref = Some(local.clone());
                        p_style_idx = xml_utils::attr_str(e, "idx");
                    }
                    // Gradient fill
                    "gradFill" if in_sp_pr && !in_ln => {
                        in_grad_fill = true;
                        grad_stops.clear();
                        grad_angle = 0.0;
                        grad_type = GradientType::Linear;
                    }
                    // Gradient stop
                    "gs" if in_grad_fill => {
                        current_gs_pos = xml_utils::attr_str(e, "pos")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| v / 100_000.0)
                            .unwrap_or(0.0);
                    }
                    // Gradient path type (Start variant — has fillToRect child)
                    "path" if in_bg_grad_fill => {
                        if let Some(val) = xml_utils::attr_str(e, "path") {
                            bg_grad_type = GradientType::from_path_attr(&val);
                        }
                    }
                    "path" if in_grad_fill => {
                        if let Some(val) = xml_utils::attr_str(e, "path") {
                            grad_type = GradientType::from_path_attr(&val);
                        }
                    }
                    // Color element (Start — may have child modifiers)
                    "srgbClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if in_shape_outer_shdw || in_shape_glow {
                                shape_effect_color = Some(Color::rgb(val));
                            } else {
                                current_color = Some(Color::rgb(val));
                            }
                        }
                    }
                    "schemeClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if in_shape_outer_shdw || in_shape_glow {
                                shape_effect_color = Some(Color::theme(val));
                            } else {
                                current_color = Some(Color::theme(val));
                            }
                        }
                    }
                    "prstClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if in_shape_outer_shdw || in_shape_glow {
                                shape_effect_color = Some(Color::preset(val));
                            } else {
                                current_color = Some(Color::preset(val));
                            }
                        }
                    }
                    "sysClr" => {
                        if in_shape_outer_shdw || in_shape_glow {
                            if let Some(val) = xml_utils::attr_str(e, "val") {
                                shape_effect_color = Some(Color::system(val));
                            } else if let Some(val) = xml_utils::attr_str(e, "lastClr") {
                                shape_effect_color = Some(Color::rgb(val));
                            }
                        } else if let Some(val) = xml_utils::attr_str(e, "val") {
                            current_color = Some(Color::system(val));
                        } else if let Some(val) = xml_utils::attr_str(e, "lastClr") {
                            current_color = Some(Color::rgb(val));
                        }
                    }
                    // ── Text break (<a:br>) ──
                    "br" if current_paragraph.is_some() && !in_tc => {
                        let br_run = RunBuilder {
                            is_break: true,
                            text: "\n".to_string(),
                            ..Default::default()
                        };
                        if let Some(pb) = current_paragraph.as_mut() {
                            pb.runs.push(br_run.build());
                        }
                    }
                    "br" if in_tc && cell_paragraph.is_some() => {
                        let br_run = RunBuilder {
                            is_break: true,
                            text: "\n".to_string(),
                            ..Default::default()
                        };
                        if let Some(pb) = cell_paragraph.as_mut() {
                            pb.runs.push(br_run.build());
                        }
                    }
                    // ── Adjust values (<a:avLst>) ──
                    "avLst" if in_sp_pr && current_shape.is_some() => {
                        in_av_lst = true;
                    }
                    "gdLst" if in_cust_geom => {
                        in_av_lst = true;
                    }
                    // ── Text highlight (<a:highlight>) ──
                    "highlight" if in_r_pr || in_cell_r_pr => {
                        in_highlight = true;
                    }
                    // ── Effect list (<a:effectLst>) for text shadow ──
                    "effectLst" if in_r_pr || in_cell_r_pr => {
                        in_effect_lst = true;
                    }
                    // ── Outer shadow inside text effectLst ──
                    "outerShdw" if in_effect_lst => {
                        in_outer_shdw = true;
                        outer_shdw_blur = xml_utils::attr_str(e, "blurRad")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| Emu(v as i64).to_pt())
                            .unwrap_or(0.0);
                        outer_shdw_dist = xml_utils::attr_str(e, "dist")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| Emu(v as i64).to_pt())
                            .unwrap_or(0.0);
                        outer_shdw_dir = xml_utils::attr_str(e, "dir")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| v / 60_000.0)
                            .unwrap_or(0.0);
                    }
                    // ── Shape-level effect list (<a:effectLst> inside <p:spPr>) ──
                    "effectLst" if in_sp_pr && current_shape.is_some() => {
                        in_shape_effect_lst = true;
                    }
                    // ── Shape-level outer shadow ──
                    "outerShdw" if in_shape_effect_lst => {
                        in_shape_outer_shdw = true;
                        shape_shdw_blur = xml_utils::attr_str(e, "blurRad")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| Emu(v as i64).to_pt())
                            .unwrap_or(0.0);
                        shape_shdw_dist = xml_utils::attr_str(e, "dist")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| Emu(v as i64).to_pt())
                            .unwrap_or(0.0);
                        shape_shdw_dir = xml_utils::attr_str(e, "dir")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| v / 60_000.0)
                            .unwrap_or(0.0);
                        shape_shdw_alpha = 1.0;
                    }
                    // ── Shape-level glow ──
                    "glow" if in_shape_effect_lst => {
                        in_shape_glow = true;
                        shape_glow_rad = xml_utils::attr_str(e, "rad")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| Emu(v as i64).to_pt())
                            .unwrap_or(0.0);
                        shape_glow_alpha = 1.0;
                    }
                    // ── Background gradFill ──
                    "gradFill" if in_bg_pr => {
                        in_bg_grad_fill = true;
                        bg_grad_stops.clear();
                        bg_grad_angle = 0.0;
                        bg_grad_type = GradientType::Linear;
                    }
                    // ── Background gradient stop ──
                    "gs" if in_bg_grad_fill => {
                        bg_gs_pos = xml_utils::attr_str(e, "pos")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| v / 100_000.0)
                            .unwrap_or(0.0);
                    }
                    // ── Background blipFill ──
                    "blipFill" if in_bg_pr => {
                        in_bg_blip_fill = true;
                    }
                    // Image reference (Start variant — blip with child elements)
                    "blip" => {
                        for attr in e.attributes().flatten() {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            if key.ends_with("embed") {
                                let rel_id = String::from_utf8_lossy(&attr.value).to_string();
                                if in_bg_blip_fill {
                                    bg_blip_rel_id = Some(rel_id);
                                } else if let Some(sb) = current_shape.as_mut() {
                                    sb.image_rel_id = Some(rel_id);
                                }
                            }
                        }
                    }
                    // ── Preset geometry (<a:prstGeom>) — Start variant ──
                    // In real PPTX files, prstGeom is usually a Start event
                    // (e.g., <a:prstGeom prst="ellipse"><a:avLst/></a:prstGeom>)
                    "prstGeom" if current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut()
                            && let Some(prst) = xml_utils::attr_str(e, "prst")
                        {
                            sb.preset_geometry = Some(prst);
                        }
                    }
                    // ── Custom geometry (<a:custGeom>) — Start variant ──
                    "custGeom" if in_sp_pr && current_shape.is_some() => {
                        in_cust_geom = true;
                        cust_geom_paths.clear();
                        cust_geom_guides.clear();
                        cust_geom_text_rect = None;
                        cust_geom_handles.clear();
                        cust_geom_connection_sites.clear();
                    }
                    // Path inside custGeom pathLst
                    "path" if in_cust_geom => {
                        in_cust_geom_path = true;
                        cust_geom_cmds.clear();
                        cust_geom_path_w = xml_utils::attr_str(e, "w")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        cust_geom_path_h = xml_utils::attr_str(e, "h")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        cust_geom_path_fill = match xml_utils::attr_str(e, "fill").as_deref() {
                            Some("none") => PathFill::None,
                            Some("lighten") => PathFill::Lighten,
                            Some("darken") => PathFill::Darken,
                            Some("lightenLess") => PathFill::LightenLess,
                            Some("darkenLess") => PathFill::DarkenLess,
                            _ => PathFill::Norm,
                        };
                    }
                    // Path drawing commands (Start variants with child <a:pt> elements)
                    "moveTo" | "lnTo" | "cubicBezTo" | "quadBezTo" if in_cust_geom_path => {
                        in_cust_geom_cmd = Some(local.clone());
                        cust_geom_pts.clear();
                    }
                    // arcTo as Start element (some generators emit it with children)
                    "arcTo" if in_cust_geom_path => {
                        let wr = xml_utils::attr_str(e, "wR")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let hr = xml_utils::attr_str(e, "hR")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let st_ang = xml_utils::attr_str(e, "stAng")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let sw_ang = xml_utils::attr_str(e, "swAng")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        cust_geom_cmds.push(PathCommand::ArcTo {
                            wr,
                            hr,
                            start_angle: st_ang,
                            swing_angle: sw_ang,
                        });
                    }
                    "rect" if in_cust_geom => {
                        let left = xml_utils::attr_str(e, "l")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let top = xml_utils::attr_str(e, "t")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let right = xml_utils::attr_str(e, "r")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let bottom = xml_utils::attr_str(e, "b")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        cust_geom_text_rect = Some(GeomRect {
                            left,
                            top,
                            right,
                            bottom,
                        });
                    }
                    "ahXY" if in_cust_geom => {
                        current_xy_handle = Some(XYAdjustHandle {
                            gd_ref_x: xml_utils::attr_str(e, "gdRefX"),
                            gd_ref_y: xml_utils::attr_str(e, "gdRefY"),
                            min_x: xml_utils::attr_str(e, "minX")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            max_x: xml_utils::attr_str(e, "maxX")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            min_y: xml_utils::attr_str(e, "minY")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            max_y: xml_utils::attr_str(e, "maxY")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            pos_x: 0.0,
                            pos_y: 0.0,
                        });
                    }
                    "ahPolar" if in_cust_geom => {
                        current_polar_handle = Some(PolarAdjustHandle {
                            gd_ref_r: xml_utils::attr_str(e, "gdRefR"),
                            gd_ref_ang: xml_utils::attr_str(e, "gdRefAng"),
                            min_r: xml_utils::attr_str(e, "minR")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            max_r: xml_utils::attr_str(e, "maxR")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            min_ang: xml_utils::attr_str(e, "minAng")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            max_ang: xml_utils::attr_str(e, "maxAng")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides)),
                            pos_x: 0.0,
                            pos_y: 0.0,
                        });
                    }
                    "cxn" if in_cust_geom => {
                        current_connection_site = Some(ConnectionSite {
                            x: 0.0,
                            y: 0.0,
                            angle: xml_utils::attr_str(e, "ang")
                                .as_deref()
                                .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                                .unwrap_or(0.0),
                        });
                    }
                    // close as Start element
                    "close" if in_cust_geom_path => {
                        cust_geom_cmds.push(PathCommand::Close);
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();

                if local == "cNvPr" && current_shape.is_some() {
                    parse_shape_identity(e, &mut current_shape);
                    continue;
                }
                if local == "stCxn" && current_shape.as_ref().is_some_and(|s| s.is_connector) {
                    parse_connector_ref(e, &mut current_shape, true);
                    continue;
                }
                if local == "endCxn" && current_shape.as_ref().is_some_and(|s| s.is_connector) {
                    parse_connector_ref(e, &mut current_shape, false);
                    continue;
                }

                // Capture self-closing elements inside graphicData
                if capturing_raw_xml {
                    raw_xml_buf.push('<');
                    raw_xml_buf.push_str(std::str::from_utf8(e.name().as_ref()).unwrap_or(&local));
                    for attr in e.attributes().flatten() {
                        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                        let val = std::str::from_utf8(&attr.value).unwrap_or("");
                        raw_xml_buf.push(' ');
                        raw_xml_buf.push_str(key);
                        raw_xml_buf.push_str("=\"");
                        raw_xml_buf.push_str(val);
                        raw_xml_buf.push('"');
                    }
                    raw_xml_buf.push_str("/>");
                }

                match local.as_str() {
                    // Table column width
                    "gridCol" if in_tbl => {
                        if let Some(ref mut tb) = table_builder {
                            let w = xml_utils::attr_str(e, "w")
                                .map(|v| Emu::parse_emu(&v).to_px())
                                .unwrap_or(0.0);
                            tb.col_widths.push(w);
                        }
                    }
                    // Paragraph properties inside table cell (Empty variant)
                    "pPr" if in_tc && cell_paragraph.is_some() => {
                        parse_para_props(e, &mut cell_paragraph);
                    }
                    // Paragraph-level defRPr inside table cell (Empty variant)
                    "defRPr" if in_tc && cell_paragraph.is_some() && cell_run.is_none() => {
                        if let Some(pb) = cell_paragraph.as_mut() {
                            apply_paragraph_def_rpr(pb, e);
                        }
                    }
                    // Run properties inside table cell (Empty variant)
                    "rPr" if in_tc && cell_run.is_some() => {
                        parse_run_props(e, &mut cell_run);
                    }
                    // Shape position/size — only inside <a:xfrm> (or group grpSpPr).
                    // <a:off> and <a:ext> also appear inside <a:extLst> (extension
                    // lists) where they carry a `uri` attribute but no cx/cy.
                    // Parsing those would overwrite the shape size with zeros.
                    "off" if depth_contains(&depth, "xfrm") || in_grp_sp_pr => {
                        if in_grp_sp_pr {
                            // Inside grpSpPr: "off" under xfrm is group position,
                            // "chOff" is handled separately below
                            if let Some(gc) = grp_stack.last_mut() {
                                let x = Emu::parse_emu(
                                    &xml_utils::attr_str(e, "x").unwrap_or_default(),
                                );
                                let y = Emu::parse_emu(
                                    &xml_utils::attr_str(e, "y").unwrap_or_default(),
                                );
                                // Check if this is inside chOff or the outer xfrm off
                                if depth_contains(&depth, "chOff") {
                                    gc.child_offset = Position { x, y };
                                } else {
                                    gc.position = Position { x, y };
                                }
                            }
                        } else if let Some(sb) = current_shape.as_mut() {
                            sb.position.x =
                                Emu::parse_emu(&xml_utils::attr_str(e, "x").unwrap_or_default());
                            sb.position.y =
                                Emu::parse_emu(&xml_utils::attr_str(e, "y").unwrap_or_default());
                        }
                    }
                    "ext" if depth_contains(&depth, "xfrm") || in_grp_sp_pr => {
                        if in_grp_sp_pr {
                            if let Some(gc) = grp_stack.last_mut() {
                                let cx = Emu::parse_emu(
                                    &xml_utils::attr_str(e, "cx").unwrap_or_default(),
                                );
                                let cy = Emu::parse_emu(
                                    &xml_utils::attr_str(e, "cy").unwrap_or_default(),
                                );
                                if depth_contains(&depth, "chExt") {
                                    gc.child_extent = Size {
                                        width: cx,
                                        height: cy,
                                    };
                                } else {
                                    gc.size = Size {
                                        width: cx,
                                        height: cy,
                                    };
                                }
                            }
                        } else if let Some(sb) = current_shape.as_mut() {
                            sb.size.width =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cx").unwrap_or_default());
                            sb.size.height =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cy").unwrap_or_default());
                        }
                    }
                    // Child offset/extent for group (self-closing variant)
                    "chOff" if in_grp_sp_pr => {
                        if let Some(gc) = grp_stack.last_mut() {
                            gc.child_offset.x =
                                Emu::parse_emu(&xml_utils::attr_str(e, "x").unwrap_or_default());
                            gc.child_offset.y =
                                Emu::parse_emu(&xml_utils::attr_str(e, "y").unwrap_or_default());
                        }
                    }
                    "chExt" if in_grp_sp_pr => {
                        if let Some(gc) = grp_stack.last_mut() {
                            gc.child_extent.width =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cx").unwrap_or_default());
                            gc.child_extent.height =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cy").unwrap_or_default());
                        }
                    }
                    // Transform (Empty variant, e.g. connector with no children)
                    "xfrm" if in_sp_pr => {
                        if let Some(sb) = current_shape.as_mut() {
                            apply_shape_transform(sb, e);
                        }
                    }
                    // Preset geometry
                    "prstGeom" => {
                        if let Some(sb) = current_shape.as_mut()
                            && let Some(prst) = xml_utils::attr_str(e, "prst")
                        {
                            sb.preset_geometry = Some(prst);
                        }
                    }
                    // bodyPr (Empty variant)
                    "bodyPr" if current_shape.is_some() && !in_tc => {
                        parse_body_pr(e, &mut current_shape);
                    }
                    // normAutofit (Empty variant — self-closing tag)
                    "normAutofit" if current_shape.is_some() && !in_tc => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = parse_shape_auto_fit(local.as_str(), e);
                        }
                    }
                    "noAutofit" if current_shape.is_some() && !in_tc => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = parse_shape_auto_fit(local.as_str(), e);
                        }
                    }
                    // spAutoFit (Empty variant)
                    "spAutoFit" if current_shape.is_some() && !in_tc => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.text_auto_fit = parse_shape_auto_fit(local.as_str(), e);
                        }
                    }
                    // Paragraph properties (Empty variant, non-table)
                    "pPr" if current_paragraph.is_some() && !in_tc => {
                        parse_para_props(e, &mut current_paragraph);
                    }
                    // Paragraph-level defRPr (Empty variant — self-closing, no children)
                    "defRPr" if current_paragraph.is_some() && !in_tc && current_run.is_none() => {
                        if let Some(pb) = current_paragraph.as_mut() {
                            apply_paragraph_def_rpr(pb, e);
                        }
                    }
                    // Run properties (Empty variant, non-table)
                    "rPr" if current_run.is_some() && !in_tc => {
                        parse_run_props(e, &mut current_run);
                    }
                    "hlinkClick" if in_r_pr || in_cell_r_pr => {
                        if let Some(rel_id) = hyperlink_rel_id(e) {
                            let target = rels.get(&rel_id).cloned();
                            if let Some(rb) = current_run.as_mut() {
                                rb.hyperlink = target.clone();
                            }
                            if let Some(rb) = cell_run.as_mut() {
                                rb.hyperlink = target;
                            }
                        }
                    }
                    // noFill -- explicit transparent
                    "noFill" => {
                        if in_ln {
                            if let Some(sb) = &mut current_shape {
                                sb.border_style = BorderStyle::None;
                                sb.border_width = 0.0;
                                sb.border_no_fill = true;
                            }
                        } else if in_sp_pr && let Some(sb) = &mut current_shape {
                            sb.fill = Fill::NoFill;
                        }
                    }
                    // Line dash style
                    "prstDash" if in_ln => {
                        if let Some(sb) = &mut current_shape
                            && let Some(val) = xml_utils::attr_str(e, "val")
                        {
                            sb.border_style = match val.as_str() {
                                "solid" => BorderStyle::Solid,
                                "dash" | "lgDash" | "sysDash" => BorderStyle::Dashed,
                                "dot" | "sysDot" | "lgDashDot" | "lgDashDotDot" | "sysDashDot"
                                | "sysDashDotDot" => BorderStyle::Dotted,
                                _ => BorderStyle::Solid,
                            };
                            sb.dash_style = match val.as_str() {
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
                    // Line join styles (inside <a:ln>)
                    "round" if in_ln => {
                        if let Some(sb) = &mut current_shape {
                            sb.border_join = LineJoin::Round;
                        }
                    }
                    "bevel" if in_ln => {
                        if let Some(sb) = &mut current_shape {
                            sb.border_join = LineJoin::Bevel;
                        }
                    }
                    "miter" if in_ln => {
                        if let Some(sb) = &mut current_shape {
                            sb.border_join = LineJoin::Miter;
                            sb.miter_limit = xml_utils::attr_str(e, "lim")
                                .and_then(|v| v.parse::<f64>().ok())
                                .map(|v| v / 100_000.0);
                        }
                    }
                    // Line ending: head arrow (<a:headEnd>)
                    "headEnd" if in_ln => {
                        if let Some(sb) = &mut current_shape {
                            sb.head_end = parse_line_end(e);
                        }
                    }
                    // Line ending: tail arrow (<a:tailEnd>)
                    "tailEnd" if in_ln => {
                        if let Some(sb) = &mut current_shape {
                            sb.tail_end = parse_line_end(e);
                        }
                    }
                    // Gradient direction (linear)
                    "lin" if in_bg_grad_fill => {
                        if let Some(ang) = xml_utils::attr_str(e, "ang") {
                            bg_grad_angle = ang.parse::<f64>().unwrap_or(0.0) / 60_000.0;
                        }
                        bg_grad_type = GradientType::Linear;
                    }
                    "lin" if in_grad_fill => {
                        if let Some(ang) = xml_utils::attr_str(e, "ang") {
                            // OOXML angle: in 1/60000 degree units
                            grad_angle = ang.parse::<f64>().unwrap_or(0.0) / 60_000.0;
                        }
                        grad_type = GradientType::Linear;
                    }
                    // Gradient path type (radial/rectangular/shape)
                    "path" if in_bg_grad_fill => {
                        if let Some(val) = xml_utils::attr_str(e, "path") {
                            bg_grad_type = GradientType::from_path_attr(&val);
                        }
                    }
                    "path" if in_grad_fill => {
                        if let Some(val) = xml_utils::attr_str(e, "path") {
                            grad_type = GradientType::from_path_attr(&val);
                        }
                    }
                    // Style ref elements (Empty variant -- self-closing with color child)
                    "lnRef" | "fillRef" | "effectRef" | "fontRef" if in_p_style => {
                        // Self-closing style ref with no child color
                        if let Some(idx_val) = xml_utils::attr_str(e, "idx") {
                            assign_style_ref_no_color(&local, &idx_val, &mut p_style_builder);
                        }
                    }
                    // Color element (Empty — simple color without modifiers)
                    "srgbClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::rgb(val);
                            if in_shape_outer_shdw || in_shape_glow {
                                shape_effect_color = Some(color);
                            } else if in_highlight {
                                if in_cell_r_pr {
                                    if let Some(rb) = cell_run.as_mut() {
                                        rb.highlight = Some(color);
                                    }
                                } else if in_r_pr && let Some(rb) = current_run.as_mut() {
                                    rb.highlight = Some(color);
                                }
                            } else if in_outer_shdw {
                                current_color = Some(color);
                            } else if in_cell_bu_clr {
                                if let Some(pb) = cell_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_shape_def_rpr {
                                if let Some(rd) = current_shape_run_defaults.as_mut() {
                                    rd.color = Some(color);
                                }
                            } else if in_tc_pr {
                                assign_tc_color(color, &tc_border_side, &mut current_cell);
                            } else if in_cell_r_pr {
                                if let Some(rb) = cell_run.as_mut() {
                                    rb.color = color;
                                }
                            } else if in_bu_clr {
                                if let Some(pb) = current_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else if in_bg_pr && !in_bg_blip_fill {
                                if in_bg_grad_fill && depth_contains(&depth, "gs") {
                                    bg_grad_stops.push(GradientStop {
                                        position: bg_gs_pos,
                                        color,
                                    });
                                } else {
                                    bg_solid_color = Some(color);
                                }
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr,
                                    in_ln,
                                    in_r_pr,
                                    in_grad_fill,
                                    current_gs_pos,
                                    &mut current_shape,
                                    &mut current_run,
                                    &mut grad_stops,
                                );
                            }
                        }
                    }
                    "schemeClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::theme(val);
                            if in_shape_outer_shdw || in_shape_glow {
                                shape_effect_color = Some(color);
                            } else if in_highlight {
                                if in_cell_r_pr {
                                    if let Some(rb) = cell_run.as_mut() {
                                        rb.highlight = Some(color);
                                    }
                                } else if in_r_pr && let Some(rb) = current_run.as_mut() {
                                    rb.highlight = Some(color);
                                }
                            } else if in_outer_shdw {
                                current_color = Some(color);
                            } else if in_cell_bu_clr {
                                if let Some(pb) = cell_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_tc_pr {
                                assign_tc_color(color, &tc_border_side, &mut current_cell);
                            } else if in_cell_r_pr {
                                if let Some(rb) = cell_run.as_mut() {
                                    rb.color = color;
                                }
                            } else if in_bu_clr {
                                if let Some(pb) = current_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else if in_bg_pr && !in_bg_blip_fill {
                                if in_bg_grad_fill && depth_contains(&depth, "gs") {
                                    bg_grad_stops.push(GradientStop {
                                        position: bg_gs_pos,
                                        color,
                                    });
                                } else {
                                    bg_solid_color = Some(color);
                                }
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr,
                                    in_ln,
                                    in_r_pr,
                                    in_grad_fill,
                                    current_gs_pos,
                                    &mut current_shape,
                                    &mut current_run,
                                    &mut grad_stops,
                                );
                            }
                        }
                    }
                    "prstClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::preset(val);
                            if in_shape_outer_shdw || in_shape_glow {
                                shape_effect_color = Some(color);
                            } else if in_cell_bu_clr {
                                if let Some(pb) = cell_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_tc_pr {
                                assign_tc_color(color, &tc_border_side, &mut current_cell);
                            } else if in_cell_r_pr {
                                if let Some(rb) = cell_run.as_mut() {
                                    rb.color = color;
                                }
                            } else if in_bu_clr {
                                if let Some(pb) = current_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else if in_bg_pr && !in_bg_blip_fill {
                                if in_bg_grad_fill && depth_contains(&depth, "gs") {
                                    bg_grad_stops.push(GradientStop {
                                        position: bg_gs_pos,
                                        color,
                                    });
                                } else {
                                    bg_solid_color = Some(color);
                                }
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr,
                                    in_ln,
                                    in_r_pr,
                                    in_grad_fill,
                                    current_gs_pos,
                                    &mut current_shape,
                                    &mut current_run,
                                    &mut grad_stops,
                                );
                            }
                        }
                    }
                    "sysClr" => {
                        let color = if let Some(val) = xml_utils::attr_str(e, "lastClr") {
                            Color::rgb(val)
                        } else if let Some(val) = xml_utils::attr_str(e, "val") {
                            Color::system(val)
                        } else {
                            Color::none()
                        };
                        if !color.is_none() {
                            if in_shape_outer_shdw || in_shape_glow {
                                shape_effect_color = Some(color);
                            } else if in_cell_bu_clr {
                                if let Some(pb) = cell_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_tc_pr {
                                assign_tc_color(color, &tc_border_side, &mut current_cell);
                            } else if in_cell_r_pr {
                                if let Some(rb) = cell_run.as_mut() {
                                    rb.color = color;
                                }
                            } else if in_bu_clr {
                                if let Some(pb) = current_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else if in_bg_pr && !in_bg_blip_fill {
                                if in_bg_grad_fill && depth_contains(&depth, "gs") {
                                    bg_grad_stops.push(GradientStop {
                                        position: bg_gs_pos,
                                        color,
                                    });
                                } else {
                                    bg_solid_color = Some(color);
                                }
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr,
                                    in_ln,
                                    in_r_pr,
                                    in_grad_fill,
                                    current_gs_pos,
                                    &mut current_shape,
                                    &mut current_run,
                                    &mut grad_stops,
                                );
                            }
                        }
                    }
                    // Color modifiers (Empty tags)
                    "tint" | "shade" | "alpha" | "lumMod" | "lumOff" | "satMod" | "satOff"
                    | "hueMod" | "hueOff" | "comp" | "inv" | "gray" => {
                        let val = xml_utils::attr_str(e, "val").and_then(|v| v.parse::<i32>().ok());
                        if let Some(modifier) = ColorModifier::from_ooxml(&local, val) {
                            if in_shape_outer_shdw || in_shape_glow {
                                if let Some(ref mut color) = shape_effect_color {
                                    color.modifiers.push(modifier);
                                }
                            } else if let Some(ref mut color) = current_color {
                                color.modifiers.push(modifier);
                            }
                        }
                    }
                    // Placeholder
                    "ph" if in_nv_pr && current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.placeholder = Some(super::master_parser::parse_placeholder_attrs(e));
                        }
                    }
                    // Image reference (Empty variant)
                    "blip" => {
                        for attr in e.attributes().flatten() {
                            let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                            if key.ends_with("embed") {
                                let rel_id = String::from_utf8_lossy(&attr.value).to_string();
                                if in_bg_blip_fill {
                                    bg_blip_rel_id = Some(rel_id);
                                } else if let Some(sb) = current_shape.as_mut() {
                                    sb.image_rel_id = Some(rel_id);
                                }
                            }
                        }
                    }
                    // Font (table cell, paragraph defRPr, or regular run)
                    "latin" | "ea" | "cs" => {
                        assign_typeface(
                            &local,
                            e,
                            &mut cell_run,
                            in_shape_def_rpr,
                            &mut current_shape_run_defaults,
                            in_para_def_rpr,
                            if in_tc {
                                cell_paragraph.as_mut()
                            } else {
                                current_paragraph.as_mut()
                            },
                            &mut current_run,
                        );
                    }
                    // Spacing percentage / points (inside lnSpc/spcBef/spcAft) — cell or regular
                    "spcPct" | "spcPts" => {
                        if let Some(spacing) = parse_spacing_tag(&local, e) {
                            if in_shape_lst_style {
                                assign_spacing_defaults(
                                    current_shape_para_defaults.as_mut(),
                                    spacing,
                                    in_shape_ln_spc,
                                    in_shape_spc_bef,
                                    in_shape_spc_aft,
                                );
                            } else {
                                assign_spacing_paragraph(
                                    if in_tc {
                                        cell_paragraph.as_mut()
                                    } else {
                                        current_paragraph.as_mut()
                                    },
                                    spacing,
                                    in_ln_spc,
                                    in_spc_bef,
                                    in_spc_aft,
                                );
                            }
                        }
                    }
                    // Bullet font (cell or regular)
                    "buFont" => {
                        let target = if in_tc {
                            &mut cell_paragraph
                        } else {
                            &mut current_paragraph
                        };
                        if let Some(pb) = target.as_mut()
                            && let Some(typeface) = xml_utils::attr_str(e, "typeface")
                        {
                            pb.bu_font = Some(typeface);
                        }
                    }
                    // Bullet size (percentage, cell or regular)
                    "buSzPct" => {
                        let target = if in_tc {
                            &mut cell_paragraph
                        } else {
                            &mut current_paragraph
                        };
                        if let Some(pb) = target.as_mut()
                            && let Some(val_str) = xml_utils::attr_str(e, "val")
                            && let Ok(val) = val_str.parse::<f64>()
                        {
                            pb.bu_size_pct = Some(val / 100_000.0);
                        }
                    }
                    // Bullet size (points, cell or regular)
                    "buSzPts" => {
                        let target = if in_tc {
                            &mut cell_paragraph
                        } else {
                            &mut current_paragraph
                        };
                        if let Some(pb) = target.as_mut()
                            && let Some(val_str) = xml_utils::attr_str(e, "val")
                            && let Ok(val) = val_str.parse::<f64>()
                        {
                            // Store as negative to distinguish from pct in rendering
                            // (points stored directly, renderer handles it)
                            pb.bu_size_pct = Some(-(val / 100.0));
                        }
                    }
                    // Bullet (cell or regular)
                    "buNone" => {
                        let target = if in_tc {
                            &mut cell_paragraph
                        } else {
                            &mut current_paragraph
                        };
                        if let Some(pb) = target.as_mut() {
                            pb.bullet = Some(Bullet::None);
                        }
                    }
                    "buChar" => {
                        let target = if in_tc {
                            &mut cell_paragraph
                        } else {
                            &mut current_paragraph
                        };
                        if let Some(pb) = target.as_mut()
                            && let Some(ch) = xml_utils::attr_str(e, "char")
                        {
                            pb.bullet = Some(Bullet::Char(BulletChar {
                                char: ch,
                                font: pb.bu_font.take(),
                                size_pct: pb.bu_size_pct.take(),
                                color: pb.bu_color.take(),
                            }));
                        }
                    }
                    "buAutoNum" => {
                        let target = if in_tc {
                            &mut cell_paragraph
                        } else {
                            &mut current_paragraph
                        };
                        if let Some(pb) = target.as_mut() {
                            let num_type = xml_utils::attr_str(e, "type")
                                .unwrap_or_else(|| "arabicPeriod".to_string());
                            let start_at = xml_utils::attr_str(e, "startAt")
                                .and_then(|v| v.parse::<i32>().ok());
                            pb.bullet = Some(Bullet::AutoNum(BulletAutoNum {
                                num_type,
                                start_at,
                                font: pb.bu_font.take(),
                                size_pct: pb.bu_size_pct.take(),
                                color: pb.bu_color.take(),
                            }));
                        }
                    }
                    // ── Adjust value guide (<a:gd>) inside avLst ──
                    "gd" if in_av_lst => {
                        if let (Some(name), Some(fmla)) = (
                            xml_utils::attr_str(e, "name"),
                            xml_utils::attr_str(e, "fmla"),
                        ) {
                            let val = parse_guide_formula_value(&fmla, &cust_geom_guides);
                            if in_cust_geom {
                                cust_geom_guides.insert(name, val);
                            } else if let Some(sb) = current_shape.as_mut() {
                                sb.adjust_values.insert(name, val);
                            }
                        }
                    }
                    // ── Custom geometry: point element (<a:pt/>) ──
                    "pt" if in_cust_geom_cmd.is_some() => {
                        let x = xml_utils::attr_str(e, "x")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let y = xml_utils::attr_str(e, "y")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        cust_geom_pts.push((x, y));
                    }
                    // ── Custom geometry: self-closing arcTo ──
                    "arcTo" if in_cust_geom_path => {
                        let wr = xml_utils::attr_str(e, "wR")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let hr = xml_utils::attr_str(e, "hR")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let st_ang = xml_utils::attr_str(e, "stAng")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let sw_ang = xml_utils::attr_str(e, "swAng")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        cust_geom_cmds.push(PathCommand::ArcTo {
                            wr,
                            hr,
                            start_angle: st_ang,
                            swing_angle: sw_ang,
                        });
                    }
                    "rect" if in_cust_geom => {
                        let left = xml_utils::attr_str(e, "l")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let top = xml_utils::attr_str(e, "t")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let right = xml_utils::attr_str(e, "r")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let bottom = xml_utils::attr_str(e, "b")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        cust_geom_text_rect = Some(GeomRect {
                            left,
                            top,
                            right,
                            bottom,
                        });
                    }
                    "pos" if in_cust_geom => {
                        let x = xml_utils::attr_str(e, "x")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        let y = xml_utils::attr_str(e, "y")
                            .as_deref()
                            .map(|v| resolve_custom_geom_value(v, &cust_geom_guides))
                            .unwrap_or(0.0);
                        if let Some(handle) = current_xy_handle.as_mut() {
                            handle.pos_x = x;
                            handle.pos_y = y;
                        } else if let Some(handle) = current_polar_handle.as_mut() {
                            handle.pos_x = x;
                            handle.pos_y = y;
                        } else if let Some(cxn) = current_connection_site.as_mut() {
                            cxn.x = x;
                            cxn.y = y;
                        }
                    }
                    // ── Custom geometry: self-closing close ──
                    "close" if in_cust_geom_path => {
                        cust_geom_cmds.push(PathCommand::Close);
                    }
                    // ── Custom geometry: self-closing path (no commands) ──
                    "path" if in_cust_geom => {
                        let w = xml_utils::attr_str(e, "w")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let h = xml_utils::attr_str(e, "h")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let fill = match xml_utils::attr_str(e, "fill").as_deref() {
                            Some("none") => PathFill::None,
                            Some("lighten") => PathFill::Lighten,
                            Some("darken") => PathFill::Darken,
                            Some("lightenLess") => PathFill::LightenLess,
                            Some("darkenLess") => PathFill::DarkenLess,
                            _ => PathFill::Norm,
                        };
                        cust_geom_paths.push(GeometryPath {
                            width: w,
                            height: h,
                            commands: Vec::new(),
                            fill,
                        });
                    }
                    // ── Image crop (<a:srcRect>) ──
                    "srcRect" if current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut() {
                            let l = xml_utils::attr_str(e, "l")
                                .and_then(|v| v.parse::<f64>().ok())
                                .map(|v| v / 100_000.0)
                                .unwrap_or(0.0);
                            let t = xml_utils::attr_str(e, "t")
                                .and_then(|v| v.parse::<f64>().ok())
                                .map(|v| v / 100_000.0)
                                .unwrap_or(0.0);
                            let r = xml_utils::attr_str(e, "r")
                                .and_then(|v| v.parse::<f64>().ok())
                                .map(|v| v / 100_000.0)
                                .unwrap_or(0.0);
                            let b = xml_utils::attr_str(e, "b")
                                .and_then(|v| v.parse::<f64>().ok())
                                .map(|v| v / 100_000.0)
                                .unwrap_or(0.0);
                            if l > 0.0 || t > 0.0 || r > 0.0 || b > 0.0 {
                                sb.crop = Some(CropRect {
                                    left: l,
                                    top: t,
                                    right: r,
                                    bottom: b,
                                });
                            }
                        }
                    }
                    // ── Chart reference inside graphicData ──
                    "chart" if in_graphic_data && graphic_data_is_chart => {
                        if let Some(sb) = current_shape.as_mut() {
                            if let Some(rel_id) = xml_utils::attr_str(e, "id") {
                                sb.chart_rel_id = Some(rel_id);
                            }
                        }
                    }
                    // ── Text break (Empty variant) ──
                    "br" if current_paragraph.is_some() && !in_tc => {
                        let br_run = RunBuilder {
                            is_break: true,
                            text: "\n".to_string(),
                            ..Default::default()
                        };
                        if let Some(pb) = current_paragraph.as_mut() {
                            pb.runs.push(br_run.build());
                        }
                    }
                    "br" if in_tc && cell_paragraph.is_some() => {
                        let br_run = RunBuilder {
                            is_break: true,
                            text: "\n".to_string(),
                            ..Default::default()
                        };
                        if let Some(pb) = cell_paragraph.as_mut() {
                            pb.runs.push(br_run.build());
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) if capturing_raw_xml => {
                raw_xml_buf.push_str(&e.unescape().unwrap_or_default());
                // Fall through to regular text handlers
                if in_cell_text {
                    if let Some(rb) = &mut cell_run {
                        rb.text.push_str(&e.unescape().unwrap_or_default());
                    }
                } else if in_text && let Some(rb) = &mut current_run {
                    rb.text.push_str(&e.unescape().unwrap_or_default());
                }
            }
            Ok(Event::Text(ref e)) if in_cell_text => {
                if let Some(rb) = &mut cell_run {
                    rb.text.push_str(&e.unescape().unwrap_or_default());
                }
            }
            Ok(Event::Text(ref e)) if in_text => {
                if let Some(rb) = &mut current_run {
                    rb.text.push_str(&e.unescape().unwrap_or_default());
                }
            }
            Ok(Event::End(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.pop();

                // Capture closing tags for raw XML (before graphicData end stops capture)
                if capturing_raw_xml && local != "graphicData" {
                    raw_xml_buf.push_str("</");
                    raw_xml_buf.push_str(std::str::from_utf8(e.name().as_ref()).unwrap_or(&local));
                    raw_xml_buf.push('>');
                }

                match local.as_str() {
                    // ── Table cell text end events ──
                    "t" if in_cell_text => {
                        in_cell_text = false;
                    }
                    "rPr" if in_cell_r_pr => {
                        in_cell_r_pr = false;
                    }
                    "r" if in_tc && cell_paragraph.is_some() => {
                        if let (Some(pb), Some(rb)) = (&mut cell_paragraph, cell_run.take()) {
                            pb.runs.push(rb.build());
                        }
                    }
                    "p" if in_tc => {
                        if let Some(pb) = cell_paragraph.take() {
                            cell_paragraphs.push(pb.build());
                        }
                    }
                    "buClr" if in_cell_bu_clr => {
                        in_cell_bu_clr = false;
                    }
                    // End of table cell border sides
                    "lnL" | "lnR" | "lnT" | "lnB" if in_tc_pr => {
                        tc_border_side = None;
                    }
                    // End of table cell properties
                    "tcPr" => {
                        in_tc_pr = false;
                    }
                    // End of table cell
                    "tc" => {
                        if let Some(mut cell) = current_cell.take() {
                            if !cell_paragraphs.is_empty() {
                                cell.text_body = Some(TextBody {
                                    paragraphs: std::mem::take(&mut cell_paragraphs),
                                    list_style: None,
                                    ..Default::default()
                                });
                            }
                            if let Some(ref mut row) = current_row {
                                row.cells.push(cell.build());
                            }
                        }
                        in_tc = false;
                        cell_paragraph = None;
                        cell_run = None;
                        in_cell_text = false;
                        in_cell_r_pr = false;
                    }
                    // End of table row
                    "tr" => {
                        if let Some(row) = current_row.take()
                            && let Some(tb) = table_builder.as_mut()
                        {
                            tb.rows.push(row.build());
                        }
                        in_tr = false;
                    }
                    // End of table
                    "tbl" => {
                        in_tbl = false;
                    }
                    // End of graphic frame — finalize table or chart shape
                    "graphicFrame" => {
                        if graphic_data_is_chart {
                            // Chart: build a chart shape
                            if let Some(mut sb) = current_shape.take() {
                                if let Some(rel_id) = sb.chart_rel_id.as_ref()
                                    && let Some(target) = rels.get(rel_id)
                                {
                                    let path = resolve_rel_path("ppt/slides", target);
                                    if let Ok(chart_xml) = read_archive_entry(archive, &path) {
                                        sb.chart_direct_spec =
                                            chart_parser::parse_chart(&chart_xml).ok().flatten();

                                        let chart_rels_path = rels_path_for(&path);
                                        if let Ok(chart_rels_xml) =
                                            read_archive_entry(archive, &chart_rels_path)
                                            && let Ok(chart_rels) =
                                                relationships::parse_relationships(&chart_rels_xml)
                                        {
                                            for preview_target in chart_rels.values() {
                                                let preview_path = resolve_relative_file_path(
                                                    &path,
                                                    preview_target,
                                                );
                                                let preview_ext = preview_path
                                                    .rsplit('.')
                                                    .next()
                                                    .unwrap_or("")
                                                    .to_lowercase();
                                                if !matches!(
                                                    preview_ext.as_str(),
                                                    "png"
                                                        | "jpg"
                                                        | "jpeg"
                                                        | "gif"
                                                        | "bmp"
                                                        | "tif"
                                                        | "tiff"
                                                        | "svg"
                                                        | "emf"
                                                        | "wmf"
                                                ) {
                                                    continue;
                                                }
                                                let preview_mime =
                                                    mime_from_extension(&preview_path);
                                                if let Ok(preview_bytes) =
                                                    read_archive_bytes(archive, &preview_path)
                                                    && !preview_bytes.is_empty()
                                                {
                                                    sb.chart_preview_mime = Some(preview_mime);
                                                    sb.chart_preview_image = Some(preview_bytes);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                                let shape = sb.build();
                                if !grp_stack.is_empty() {
                                    if let Some(gc) = grp_stack.last_mut() {
                                        gc.shapes.push(shape);
                                    }
                                } else {
                                    slide.shapes.push(shape);
                                }
                            }
                        } else if current_shape
                            .as_ref()
                            .is_some_and(|sb| sb.unsupported_content.is_some())
                        {
                            // Unsupported content (SmartArt, OLE, Math): build placeholder shape
                            if let Some(sb) = current_shape.take() {
                                let shape = sb.build();
                                if !grp_stack.is_empty() {
                                    if let Some(gc) = grp_stack.last_mut() {
                                        gc.shapes.push(shape);
                                    }
                                } else {
                                    slide.shapes.push(shape);
                                }
                            }
                        } else if let (Some(sb), Some(tb)) =
                            (current_shape.take(), table_builder.take())
                        {
                            let table_data = tb.build();
                            let shape = Shape {
                                position: sb.position,
                                size: sb.size,
                                shape_type: ShapeType::Table(table_data),
                                ..Default::default()
                            };
                            if !grp_stack.is_empty() {
                                if let Some(gc) = grp_stack.last_mut() {
                                    gc.shapes.push(shape);
                                }
                            } else {
                                slide.shapes.push(shape);
                            }
                        }
                        in_graphic_frame = false;
                        in_graphic_data = false;
                        graphic_data_is_chart = false;
                    }
                    // ── Group shape end ──
                    "grpSp" => {
                        if let Some(gc) = grp_stack.pop() {
                            let group_data = GroupData {
                                child_offset: gc.child_offset,
                                child_extent: gc.child_extent,
                            };
                            let shape = Shape {
                                position: gc.position,
                                size: gc.size,
                                shape_type: ShapeType::Group(gc.shapes, group_data),
                                ..Default::default()
                            };
                            // Nested groups: push to parent group
                            if let Some(parent) = grp_stack.last_mut() {
                                parent.shapes.push(shape);
                            } else {
                                slide.shapes.push(shape);
                            }
                        }
                    }
                    "grpSpPr" => {
                        in_grp_sp_pr = false;
                    }

                    // ── Custom geometry end events ──
                    "moveTo" | "lnTo" if in_cust_geom_cmd.as_deref() == Some(&local) => {
                        if let Some((x, y)) = cust_geom_pts.first() {
                            let cmd = if local == "moveTo" {
                                PathCommand::MoveTo { x: *x, y: *y }
                            } else {
                                PathCommand::LineTo { x: *x, y: *y }
                            };
                            cust_geom_cmds.push(cmd);
                        }
                        in_cust_geom_cmd = None;
                        cust_geom_pts.clear();
                    }
                    "cubicBezTo" if in_cust_geom_cmd.as_deref() == Some("cubicBezTo") => {
                        if cust_geom_pts.len() >= 3 {
                            cust_geom_cmds.push(PathCommand::CubicBezTo {
                                x1: cust_geom_pts[0].0,
                                y1: cust_geom_pts[0].1,
                                x2: cust_geom_pts[1].0,
                                y2: cust_geom_pts[1].1,
                                x: cust_geom_pts[2].0,
                                y: cust_geom_pts[2].1,
                            });
                        }
                        in_cust_geom_cmd = None;
                        cust_geom_pts.clear();
                    }
                    "quadBezTo" if in_cust_geom_cmd.as_deref() == Some("quadBezTo") => {
                        if cust_geom_pts.len() >= 2 {
                            cust_geom_cmds.push(PathCommand::QuadBezTo {
                                x1: cust_geom_pts[0].0,
                                y1: cust_geom_pts[0].1,
                                x: cust_geom_pts[1].0,
                                y: cust_geom_pts[1].1,
                            });
                        }
                        in_cust_geom_cmd = None;
                        cust_geom_pts.clear();
                    }
                    "path" if in_cust_geom_path => {
                        in_cust_geom_path = false;
                        cust_geom_paths.push(GeometryPath {
                            width: cust_geom_path_w,
                            height: cust_geom_path_h,
                            commands: std::mem::take(&mut cust_geom_cmds),
                            fill: cust_geom_path_fill.clone(),
                        });
                    }
                    "ahXY" if current_xy_handle.is_some() => {
                        if let Some(handle) = current_xy_handle.take() {
                            cust_geom_handles.push(AdjustHandle::XY(handle));
                        }
                    }
                    "ahPolar" if current_polar_handle.is_some() => {
                        if let Some(handle) = current_polar_handle.take() {
                            cust_geom_handles.push(AdjustHandle::Polar(handle));
                        }
                    }
                    "cxn" if current_connection_site.is_some() => {
                        if let Some(cxn) = current_connection_site.take() {
                            cust_geom_connection_sites.push(cxn);
                        }
                    }
                    "custGeom" if in_cust_geom => {
                        in_cust_geom = false;
                        if let Some(sb) = current_shape.as_mut() {
                            sb.custom_geometry = Some(CustomGeometry {
                                paths: std::mem::take(&mut cust_geom_paths),
                                text_rect: cust_geom_text_rect.take(),
                                adjust_handles: std::mem::take(&mut cust_geom_handles),
                                connection_sites: std::mem::take(&mut cust_geom_connection_sites),
                            });
                        }
                        cust_geom_guides.clear();
                        cust_geom_text_rect = None;
                    }

                    // ── New state end events ──
                    "avLst" | "gdLst" => in_av_lst = false,
                    "blipFill" if in_bg_blip_fill => {
                        in_bg_blip_fill = false;
                    }
                    "bgPr" => {
                        in_bg_pr = false;
                        // Load background image if blipFill was present
                        if let Some(rel_id) = bg_blip_rel_id.take() {
                            if let Some(target) = rels.get(&rel_id) {
                                let path = resolve_rel_path("ppt/slides", target);
                                if let Ok(mut entry) = archive.by_name(&path) {
                                    let mut buf = Vec::new();
                                    let _ = entry.read_to_end(&mut buf);
                                    if !buf.is_empty() {
                                        let content_type = mime_from_extension(&path);
                                        slide.background = Some(Fill::Image(ImageFill {
                                            rel_id,
                                            data: buf,
                                            content_type,
                                        }));
                                    }
                                }
                            }
                        } else if let Some(color) = bg_solid_color.take() {
                            // Background solid fill
                            slide.background = Some(Fill::Solid(SolidFill { color }));
                        } else if !bg_grad_stops.is_empty() {
                            // Background gradient fill
                            slide.background = Some(Fill::Gradient(GradientFill {
                                gradient_type: std::mem::take(&mut bg_grad_type),
                                stops: std::mem::take(&mut bg_grad_stops),
                                angle: bg_grad_angle,
                            }));
                        }
                    }
                    "graphicData" => {
                        in_graphic_data = false;
                        if capturing_raw_xml {
                            capturing_raw_xml = false;
                            if let Some(sb) = current_shape.as_mut()
                                && !raw_xml_buf.is_empty()
                            {
                                sb.raw_xml_capture = Some(raw_xml_buf.clone());
                            }
                            raw_xml_buf.clear();
                        }
                    }
                    "effectLst" if in_effect_lst => in_effect_lst = false,
                    "effectLst" if in_shape_effect_lst => in_shape_effect_lst = false,
                    "outerShdw" if in_outer_shdw => {
                        in_outer_shdw = false;
                        if let Some(color) = current_color.take() {
                            let shadow = TextShadow {
                                color,
                                blur_rad: outer_shdw_blur,
                                dist: outer_shdw_dist,
                                dir: outer_shdw_dir,
                            };
                            if in_cell_r_pr {
                                if let Some(rb) = cell_run.as_mut() {
                                    rb.shadow = Some(shadow);
                                }
                            } else if in_r_pr && let Some(rb) = current_run.as_mut() {
                                rb.shadow = Some(shadow);
                            }
                        }
                    }
                    "outerShdw" if in_shape_outer_shdw => {
                        in_shape_outer_shdw = false;
                        if let Some(sb) = current_shape.as_mut() {
                            let color = shape_effect_color
                                .take()
                                .unwrap_or_else(|| Color::rgb("000000"));
                            sb.shape_outer_shadow = Some(OuterShadow {
                                blur_radius: shape_shdw_blur,
                                distance: shape_shdw_dist,
                                direction: shape_shdw_dir,
                                color,
                                alpha: shape_shdw_alpha,
                            });
                        }
                    }
                    "glow" if in_shape_glow => {
                        in_shape_glow = false;
                        if let Some(sb) = current_shape.as_mut() {
                            let color = shape_effect_color
                                .take()
                                .unwrap_or_else(|| Color::rgb("FFC000"));
                            sb.shape_glow = Some(GlowEffect {
                                radius: shape_glow_rad,
                                color,
                                alpha: shape_glow_alpha,
                            });
                        }
                    }
                    "highlight" if in_highlight => {
                        in_highlight = false;
                        if let Some(color) = current_color.take() {
                            if in_cell_r_pr {
                                if let Some(rb) = cell_run.as_mut() {
                                    rb.highlight = Some(color);
                                }
                            } else if in_r_pr && let Some(rb) = current_run.as_mut() {
                                rb.highlight = Some(color);
                            }
                        }
                    }

                    // ── Original shape end events ──
                    "t" => in_text = false,
                    "rPr" => in_r_pr = false,
                    "defRPr" if in_para_def_rpr => {
                        // Assign accumulated color to paragraph defRPr (table cell or regular)
                        if let Some(color) = current_color.take() {
                            if in_tc {
                                if let Some(pb) = cell_paragraph.as_mut() {
                                    pb.def_rpr_color = Some(color);
                                }
                            } else if let Some(pb) = current_paragraph.as_mut() {
                                pb.def_rpr_color = Some(color);
                            }
                        }
                        in_para_def_rpr = false;
                    }
                    "defRPr" if in_shape_def_rpr => {
                        if let Some(pd) = current_shape_para_defaults.as_mut() {
                            pd.def_run_props = current_shape_run_defaults.take();
                        }
                        in_shape_def_rpr = false;
                    }
                    // End of paragraph spacing containers
                    "lnSpc" if in_shape_ln_spc => in_shape_ln_spc = false,
                    "lnSpc" => in_ln_spc = false,
                    "spcBef" if in_shape_spc_bef => in_shape_spc_bef = false,
                    "spcBef" => in_spc_bef = false,
                    "spcAft" if in_shape_spc_aft => in_shape_spc_aft = false,
                    "spcAft" => in_spc_aft = false,
                    "lstStyle" if in_shape_lst_style => in_shape_lst_style = false,
                    s if in_shape_lst_style && is_lvl_ppr(s) => {
                        if let (Some(pd), Some(lvl)) =
                            (current_shape_para_defaults.take(), current_shape_lvl)
                        {
                            store_shape_level_defaults(&mut current_shape, lvl, pd);
                        }
                        current_shape_lvl = None;
                    }
                    "buClr" => in_bu_clr = false,
                    "r" => {
                        if let (Some(pb), Some(rb)) = (&mut current_paragraph, current_run.take()) {
                            pb.runs.push(rb.build());
                        }
                    }
                    "p" if current_shape.is_some() => {
                        if let (Some(sb), Some(pb)) = (&mut current_shape, current_paragraph.take())
                        {
                            sb.paragraphs.push(pb.build());
                        }
                    }
                    // End of color element — assign to target after applying modifiers
                    "srgbClr" | "schemeClr" | "prstClr" | "sysClr" => {
                        if let Some(color) = current_color.take() {
                            if in_highlight {
                                // Highlight color goes to run
                                if in_cell_r_pr {
                                    if let Some(rb) = cell_run.as_mut() {
                                        rb.highlight = Some(color);
                                    }
                                } else if in_r_pr && let Some(rb) = current_run.as_mut() {
                                    rb.highlight = Some(color);
                                }
                            } else if in_outer_shdw {
                                // Shadow color: don't consume, let outerShdw End handler use it
                                current_color = Some(color);
                            } else if in_shape_outer_shdw || in_shape_glow {
                                // Shape effect color: store for End handler
                                shape_effect_color = Some(color);
                            } else if in_para_def_rpr {
                                // Paragraph default run color is applied when defRPr closes.
                                current_color = Some(color);
                            } else if in_cell_bu_clr {
                                if let Some(pb) = cell_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_tc_pr {
                                assign_tc_color(color, &tc_border_side, &mut current_cell);
                            } else if in_cell_r_pr {
                                if let Some(rb) = cell_run.as_mut() {
                                    rb.color = color;
                                }
                            } else if in_bu_clr {
                                // Bullet color
                                if let Some(pb) = current_paragraph.as_mut() {
                                    pb.bu_color = Some(color);
                                }
                            } else if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else if in_bg_pr && !in_bg_blip_fill {
                                // Background solid/gradient fill color
                                if in_bg_grad_fill && depth_contains(&depth, "gs") {
                                    bg_grad_stops.push(GradientStop {
                                        position: bg_gs_pos,
                                        color,
                                    });
                                } else {
                                    bg_solid_color = Some(color);
                                }
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr,
                                    in_ln,
                                    in_r_pr,
                                    in_grad_fill,
                                    current_gs_pos,
                                    &mut current_shape,
                                    &mut current_run,
                                    &mut grad_stops,
                                );
                            }
                        }
                    }
                    // End of style ref children
                    "lnRef" | "fillRef" | "effectRef" | "fontRef" if in_p_style => {
                        // If no child color was found, still record the idx
                        if p_style_current_ref.is_some() {
                            if let Some(idx_val) = p_style_idx.take() {
                                // Only create if builder doesn't already have this ref set
                                // (it's already set if a color child was processed)
                                ensure_style_ref(&local, &idx_val, &mut p_style_builder);
                            }
                            p_style_current_ref = None;
                        }
                    }
                    // End of <p:style>
                    "style" if in_p_style => {
                        in_p_style = false;
                        if let Some(sb) = current_shape.as_mut() {
                            sb.style_ref = p_style_builder.take();
                        }
                    }
                    // End of background gradient fill
                    "gradFill" if in_bg_grad_fill => {
                        in_bg_grad_fill = false;
                        // bg_grad_stops will be consumed when bgPr ends
                    }
                    // End of gradient fill
                    "gradFill" if in_grad_fill => {
                        in_grad_fill = false;
                        if let Some(sb) = &mut current_shape {
                            sb.fill = Fill::Gradient(GradientFill {
                                gradient_type: std::mem::take(&mut grad_type),
                                stops: std::mem::take(&mut grad_stops),
                                angle: grad_angle,
                            });
                        }
                    }
                    // End of line/border
                    "ln" if in_ln => {
                        in_ln = false;
                    }
                    // End of non-visual properties
                    "nvPr" => {
                        in_nv_pr = false;
                    }
                    // End of shape properties
                    "spPr" => {
                        in_sp_pr = false;
                    }
                    // End of shape
                    "sp" | "pic" | "cxnSp" => {
                        if let Some(mut sb) = current_shape.take() {
                            // For non-picture shapes with blipFill (image-filled rectangles etc.),
                            // load the image data and set Fill::Image before building
                            if !sb.is_picture
                                && let Some(ref rel_id) = sb.image_rel_id
                                && let Some(target) = rels.get(rel_id)
                            {
                                let path = resolve_rel_path("ppt/slides", target);
                                if let Ok(mut entry) = archive.by_name(&path) {
                                    let mut buf = Vec::new();
                                    let _ = entry.read_to_end(&mut buf);
                                    if !buf.is_empty() {
                                        let content_type = mime_from_extension(&path);
                                        sb.fill = Fill::Image(ImageFill {
                                            rel_id: rel_id.clone(),
                                            data: buf,
                                            content_type,
                                        });
                                    }
                                }
                            }
                            let mut shape = sb.build();
                            // Load image data for picture shapes
                            if let ShapeType::Picture(pic) = &mut shape.shape_type
                                && let Some(target) = rels.get(&pic.rel_id)
                            {
                                // Resolve relative paths (e.g., "../media/image1.png")
                                let path = resolve_rel_path("ppt/slides", target);
                                if let Ok(mut entry) = archive.by_name(&path) {
                                    let mut buf = Vec::new();
                                    let _ = entry.read_to_end(&mut buf);
                                    pic.data = buf;
                                    // Detect content type from extension
                                    if pic.content_type.is_empty() {
                                        pic.content_type = mime_from_extension(&path);
                                    }
                                }
                            }
                            // Add shape to group or slide
                            if !grp_stack.is_empty() {
                                if let Some(gc) = grp_stack.last_mut() {
                                    gc.shapes.push(shape);
                                }
                            } else {
                                slide.shapes.push(shape);
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

    Ok(slide)
}

/// Assign parsed color to appropriate target based on context
#[allow(clippy::too_many_arguments)]
fn assign_color(
    color: Color,
    depth: &[String],
    in_sp_pr: bool,
    in_ln: bool,
    in_r_pr: bool,
    in_grad_fill: bool,
    gs_pos: f64,
    shape: &mut Option<ShapeBuilder>,
    run: &mut Option<RunBuilder>,
    grad_stops: &mut Vec<GradientStop>,
) {
    // Run text color MUST be checked first — rPr is nested inside sp,
    // so in_sp_pr can be true simultaneously with in_r_pr.
    if in_r_pr || depth_contains(depth, "rPr") {
        if let Some(rb) = run.as_mut() {
            rb.color = color;
        }
        return;
    }

    // Gradient stop color
    if in_grad_fill && depth_contains(depth, "gs") {
        grad_stops.push(GradientStop {
            position: gs_pos,
            color,
        });
        return;
    }

    // Border color
    if in_ln {
        if let Some(sb) = shape.as_mut() {
            sb.border_color = color;
            if matches!(sb.border_style, BorderStyle::None) {
                sb.border_style = BorderStyle::Solid;
            }
        }
        return;
    }

    // Shape fill color (solidFill in spPr)
    if in_sp_pr && depth_contains(depth, "solidFill") {
        if let Some(sb) = shape.as_mut() {
            sb.fill = Fill::Solid(SolidFill { color });
        }
        return;
    }

    // Other cases inside spPr (no solidFill ancestor)
    if in_sp_pr && let Some(sb) = shape.as_mut() {
        sb.fill = Fill::Solid(SolidFill { color });
    }
}

fn depth_contains(depth: &[String], tag: &str) -> bool {
    depth.iter().any(|d| d == tag)
}

/// Assign color to a style ref element (lnRef/fillRef/effectRef/fontRef)
fn assign_style_ref_color(
    ref_kind: &str,
    idx_str: &str,
    color: Color,
    builder: &mut Option<ShapeStyleRef>,
) {
    let builder = match builder.as_mut() {
        Some(b) => b,
        None => return,
    };
    match ref_kind {
        "fillRef" => {
            builder.fill_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color,
            });
        }
        "lnRef" => {
            builder.ln_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color,
            });
        }
        "effectRef" => {
            builder.effect_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color,
            });
        }
        "fontRef" => {
            builder.font_ref = Some(FontRef {
                idx: idx_str.to_string(),
                color,
            });
        }
        _ => {}
    }
}

/// Ensure style ref exists (set idx but no color override, for End events when no child color was present)
fn ensure_style_ref(ref_kind: &str, idx_str: &str, builder: &mut Option<ShapeStyleRef>) {
    let builder = match builder.as_mut() {
        Some(b) => b,
        None => return,
    };
    match ref_kind {
        "fillRef" if builder.fill_ref.is_none() => {
            builder.fill_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color: Color::none(),
            });
        }
        "lnRef" if builder.ln_ref.is_none() => {
            builder.ln_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color: Color::none(),
            });
        }
        "effectRef" if builder.effect_ref.is_none() => {
            builder.effect_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color: Color::none(),
            });
        }
        "fontRef" if builder.font_ref.is_none() => {
            builder.font_ref = Some(FontRef {
                idx: idx_str.to_string(),
                color: Color::none(),
            });
        }
        _ => {}
    }
}

/// Assign style ref with no color (Empty variant self-closing)
fn assign_style_ref_no_color(ref_kind: &str, idx_str: &str, builder: &mut Option<ShapeStyleRef>) {
    let builder = match builder.as_mut() {
        Some(b) => b,
        None => return,
    };
    match ref_kind {
        "fillRef" => {
            builder.fill_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color: Color::none(),
            });
        }
        "lnRef" => {
            builder.ln_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color: Color::none(),
            });
        }
        "effectRef" => {
            builder.effect_ref = Some(StyleRef {
                idx: idx_str.parse::<u32>().unwrap_or(0),
                color: Color::none(),
            });
        }
        "fontRef" => {
            builder.font_ref = Some(FontRef {
                idx: idx_str.to_string(),
                color: Color::none(),
            });
        }
        _ => {}
    }
}

/// Parse <a:headEnd> or <a:tailEnd> attributes into a LineEnd
pub(crate) fn parse_line_end(e: &quick_xml::events::BytesStart<'_>) -> Option<LineEnd> {
    let end_type = match xml_utils::attr_str(e, "type").as_deref() {
        Some("arrow") => LineEndType::Arrow,
        Some("triangle") => LineEndType::Triangle,
        Some("stealth") => LineEndType::Stealth,
        Some("diamond") => LineEndType::Diamond,
        Some("oval") => LineEndType::Oval,
        Some("none") | None => return None,
        Some(_) => return None,
    };
    let width = match xml_utils::attr_str(e, "w").as_deref() {
        Some("sm") => LineEndSize::Small,
        Some("lg") => LineEndSize::Large,
        _ => LineEndSize::Medium,
    };
    let length = match xml_utils::attr_str(e, "len").as_deref() {
        Some("sm") => LineEndSize::Small,
        Some("lg") => LineEndSize::Large,
        _ => LineEndSize::Medium,
    };
    Some(LineEnd {
        end_type,
        width,
        length,
    })
}

fn parse_guide_formula_value(fmla: &str, guides: &HashMap<String, f64>) -> f64 {
    let tokens: Vec<&str> = fmla.split_whitespace().collect();
    if tokens.is_empty() {
        return 0.0;
    }

    let resolve = |token: &str| resolve_custom_geom_value(token, guides);

    match tokens[0] {
        "val" => tokens.get(1).map(|v| resolve(v)).unwrap_or(0.0),
        "+-" => {
            if tokens.len() >= 4 {
                resolve(tokens[1]) + resolve(tokens[2]) - resolve(tokens[3])
            } else {
                0.0
            }
        }
        "*/" => {
            if tokens.len() >= 4 {
                let numerator = resolve(tokens[1]) * resolve(tokens[2]);
                let denominator = resolve(tokens[3]);
                if denominator.abs() < f64::EPSILON {
                    0.0
                } else {
                    numerator / denominator
                }
            } else {
                0.0
            }
        }
        "+/" => {
            if tokens.len() >= 4 {
                let numerator = resolve(tokens[1]) + resolve(tokens[2]);
                let denominator = resolve(tokens[3]);
                if denominator.abs() < f64::EPSILON {
                    0.0
                } else {
                    numerator / denominator
                }
            } else {
                0.0
            }
        }
        "pin" => {
            if tokens.len() >= 4 {
                let low = resolve(tokens[1]);
                let value = resolve(tokens[2]);
                let high = resolve(tokens[3]);
                value.max(low).min(high)
            } else {
                0.0
            }
        }
        "min" => {
            if tokens.len() >= 3 {
                resolve(tokens[1]).min(resolve(tokens[2]))
            } else {
                0.0
            }
        }
        "max" => {
            if tokens.len() >= 3 {
                resolve(tokens[1]).max(resolve(tokens[2]))
            } else {
                0.0
            }
        }
        "?:" => {
            if tokens.len() >= 4 {
                if resolve(tokens[1]).abs() >= f64::EPSILON {
                    resolve(tokens[2])
                } else {
                    resolve(tokens[3])
                }
            } else {
                0.0
            }
        }
        "abs" => {
            if tokens.len() >= 2 {
                resolve(tokens[1]).abs()
            } else {
                0.0
            }
        }
        "sqrt" => {
            if tokens.len() >= 2 {
                resolve(tokens[1]).max(0.0).sqrt()
            } else {
                0.0
            }
        }
        "mod" => {
            if tokens.len() >= 4 {
                let x = resolve(tokens[1]);
                let y = resolve(tokens[2]);
                let z = resolve(tokens[3]);
                (x * x + y * y + z * z).sqrt()
            } else {
                0.0
            }
        }
        "sin" => {
            if tokens.len() >= 3 {
                let scale = resolve(tokens[1]);
                let angle = ooxml_angle_to_radians(resolve(tokens[2]));
                scale * angle.sin()
            } else {
                0.0
            }
        }
        "cos" => {
            if tokens.len() >= 3 {
                let scale = resolve(tokens[1]);
                let angle = ooxml_angle_to_radians(resolve(tokens[2]));
                scale * angle.cos()
            } else {
                0.0
            }
        }
        "cat2" => {
            if tokens.len() >= 4 {
                let scale = resolve(tokens[1]);
                let y = resolve(tokens[2]);
                let z = resolve(tokens[3]);
                scale * z.atan2(y).cos()
            } else {
                0.0
            }
        }
        "sat2" => {
            if tokens.len() >= 4 {
                let scale = resolve(tokens[1]);
                let y = resolve(tokens[2]);
                let z = resolve(tokens[3]);
                scale * z.atan2(y).sin()
            } else {
                0.0
            }
        }
        "at2" => {
            if tokens.len() >= 3 {
                let x = resolve(tokens[1]);
                let y = resolve(tokens[2]);
                y.atan2(x).to_degrees() * 60_000.0
            } else {
                0.0
            }
        }
        "tan" => {
            if tokens.len() >= 3 {
                let scale = resolve(tokens[1]);
                let angle = ooxml_angle_to_radians(resolve(tokens[2]));
                scale * angle.tan()
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

fn ooxml_angle_to_radians(angle: f64) -> f64 {
    (angle / 60_000.0).to_radians()
}

fn resolve_custom_geom_value(raw: &str, guides: &HashMap<String, f64>) -> f64 {
    raw.parse::<f64>()
        .ok()
        .or_else(|| guides.get(raw).copied())
        .unwrap_or(0.0)
}

/// Parse bodyPr attributes
fn parse_body_pr(e: &quick_xml::events::BytesStart<'_>, shape: &mut Option<ShapeBuilder>) {
    if let Some(sb) = shape.as_mut() {
        // Vertical alignment
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
        // Inner margins (EMU → pt)
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
        // Word wrap
        if let Some(wrap) = xml_utils::attr_str(e, "wrap") {
            sb.text_word_wrap = wrap != "none";
            sb.text_word_wrap_explicit = true;
        }
        // Vertical text direction
        if let Some(vert) = xml_utils::attr_str(e, "vert") {
            sb.vertical_text_explicit = true;
            sb.vertical_text = if vert == "horz" { None } else { Some(vert) };
        }
    }
}

fn apply_shape_transform(sb: &mut ShapeBuilder, e: &quick_xml::events::BytesStart<'_>) {
    if let Some(rot) = xml_utils::attr_str(e, "rot") {
        sb.rotation = rot.parse::<f64>().unwrap_or(0.0) / 60000.0;
    }
    if let Some(fh) = xml_utils::attr_str(e, "flipH") {
        sb.flip_h = fh == "1" || fh == "true";
    }
    if let Some(fv) = xml_utils::attr_str(e, "flipV") {
        sb.flip_v = fv == "1" || fv == "true";
    }
}

fn parse_shape_auto_fit(local: &str, e: &quick_xml::events::BytesStart<'_>) -> AutoFit {
    match local {
        "normAutofit" => AutoFit::Normal {
            font_scale: parse_autofit_ratio(e, "fontScale"),
            line_spacing_reduction: parse_autofit_ratio(e, "lnSpcReduction"),
        },
        "noAutofit" => AutoFit::NoAutoFit,
        "spAutoFit" => AutoFit::Shrink,
        _ => AutoFit::None,
    }
}

fn apply_paragraph_def_rpr(pb: &mut ParagraphBuilder, e: &quick_xml::events::BytesStart<'_>) {
    if let Some(sz) = xml_utils::attr_str(e, "sz") {
        pb.def_rpr_font_size = sz.parse::<f64>().ok().map(|v| v / 100.0);
    }
    if let Some(spc) = xml_utils::attr_str(e, "spc") {
        pb.def_rpr_letter_spacing = spc.parse::<f64>().ok().map(|v| v / 100.0);
    }
    if let Some(baseline) = xml_utils::attr_str(e, "baseline") {
        pb.def_rpr_baseline = baseline.parse::<i32>().ok();
    }
    if let Some(cap) = xml_utils::attr_str(e, "cap") {
        pb.def_rpr_capitalization = Some(TextCapitalization::from_ooxml(&cap));
    }
    if let Some(u) = xml_utils::attr_str(e, "u") {
        pb.def_rpr_underline = Some(UnderlineType::from_ooxml(&u));
    }
    if let Some(strike) = xml_utils::attr_str(e, "strike") {
        pb.def_rpr_strikethrough = Some(StrikethroughType::from_ooxml(&strike));
    }
    if let Some(b) = xml_utils::attr_str(e, "b") {
        pb.def_rpr_bold = Some(b == "1" || b == "true");
    }
    if let Some(i) = xml_utils::attr_str(e, "i") {
        pb.def_rpr_italic = Some(i == "1" || i == "true");
    }
}

pub(crate) fn parse_autofit_ratio(
    e: &quick_xml::events::BytesStart<'_>,
    attr: &str,
) -> Option<f64> {
    xml_utils::attr_str(e, attr)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|v| (v / 100000.0).clamp(0.0, 1.0))
}

fn parse_spacing_tag(local: &str, e: &quick_xml::events::BytesStart<'_>) -> Option<SpacingValue> {
    let value = xml_utils::attr_str(e, "val")?.parse::<f64>().ok()?;
    Some(match local {
        "spcPct" => SpacingValue::Percent(value / 100_000.0),
        "spcPts" => SpacingValue::Points(value / 100.0),
        _ => return None,
    })
}

fn assign_spacing_defaults(
    target: Option<&mut ParagraphDefaults>,
    spacing: SpacingValue,
    in_ln_spc: bool,
    in_spc_bef: bool,
    in_spc_aft: bool,
) {
    if let Some(target) = target {
        if in_ln_spc {
            target.line_spacing = Some(spacing);
        } else if in_spc_bef {
            target.space_before = Some(spacing);
        } else if in_spc_aft {
            target.space_after = Some(spacing);
        }
    }
}

fn assign_spacing_paragraph(
    target: Option<&mut ParagraphBuilder>,
    spacing: SpacingValue,
    in_ln_spc: bool,
    in_spc_bef: bool,
    in_spc_aft: bool,
) {
    if let Some(target) = target {
        if in_ln_spc {
            target.line_spacing = Some(spacing);
        } else if in_spc_bef {
            target.space_before = Some(spacing);
        } else if in_spc_aft {
            target.space_after = Some(spacing);
        }
    }
}

fn assign_typeface_to_run(run: &mut RunBuilder, local: &str, typeface: String) {
    match local {
        "latin" => run.font_latin = Some(typeface),
        "ea" => run.font_ea = Some(typeface),
        "cs" => run.font_cs = Some(typeface),
        _ => {}
    }
}

fn assign_typeface_to_defaults(defaults: &mut RunDefaults, local: &str, typeface: String) {
    match local {
        "latin" => defaults.font_latin = Some(typeface),
        "ea" => defaults.font_ea = Some(typeface),
        "cs" => defaults.font_cs = Some(typeface),
        _ => {}
    }
}

fn assign_typeface_to_paragraph(paragraph: &mut ParagraphBuilder, local: &str, typeface: String) {
    match local {
        "latin" => paragraph.def_rpr_font_latin = Some(typeface),
        "ea" => paragraph.def_rpr_font_ea = Some(typeface),
        "cs" => paragraph.def_rpr_font_cs = Some(typeface),
        _ => {}
    }
}

fn assign_typeface(
    local: &str,
    e: &quick_xml::events::BytesStart<'_>,
    cell_run: &mut Option<RunBuilder>,
    in_shape_def_rpr: bool,
    shape_run_defaults: &mut Option<RunDefaults>,
    in_para_def_rpr: bool,
    paragraph_target: Option<&mut ParagraphBuilder>,
    current_run: &mut Option<RunBuilder>,
) {
    let Some(typeface) = xml_utils::attr_str(e, "typeface") else {
        return;
    };

    if let Some(run) = cell_run.as_mut() {
        assign_typeface_to_run(run, local, typeface);
    } else if in_shape_def_rpr {
        if let Some(defaults) = shape_run_defaults.as_mut() {
            assign_typeface_to_defaults(defaults, local, typeface);
        }
    } else if in_para_def_rpr {
        if let Some(paragraph) = paragraph_target {
            assign_typeface_to_paragraph(paragraph, local, typeface);
        }
    } else if let Some(run) = current_run.as_mut() {
        assign_typeface_to_run(run, local, typeface);
    }
}

fn parse_shape_identity(e: &quick_xml::events::BytesStart<'_>, shape: &mut Option<ShapeBuilder>) {
    if let Some(sb) = shape.as_mut() {
        sb.id = xml_utils::attr_str(e, "id")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0);
        sb.name = xml_utils::attr_str(e, "name").unwrap_or_default();
    }
}

fn parse_connector_ref(
    e: &quick_xml::events::BytesStart<'_>,
    shape: &mut Option<ShapeBuilder>,
    is_start: bool,
) {
    if let Some(sb) = shape.as_mut() {
        let connection = ConnectionRef {
            shape_id: xml_utils::attr_str(e, "id")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(0),
            site_idx: xml_utils::attr_str(e, "idx")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0),
        };
        if is_start {
            sb.start_connection = Some(connection);
        } else {
            sb.end_connection = Some(connection);
        }
    }
}

/// Parse pPr (paragraph properties)
fn parse_para_props(e: &quick_xml::events::BytesStart<'_>, para: &mut Option<ParagraphBuilder>) {
    if let Some(pb) = para.as_mut() {
        if let Some(algn) = xml_utils::attr_str(e, "algn") {
            pb.alignment = Alignment::from_ooxml(&algn);
        }
        if let Some(rtl) = xml_utils::attr_str(e, "rtl") {
            pb.rtl = rtl == "1" || rtl == "true";
        }
        if let Some(lvl) = xml_utils::attr_str(e, "lvl") {
            pb.level = lvl.parse::<u32>().unwrap_or(0);
        }
        if let Some(indent) = xml_utils::attr_str(e, "indent") {
            pb.indent = Some(Emu::parse_emu(&indent).to_pt());
        }
        if let Some(mar_l) = xml_utils::attr_str(e, "marL") {
            pb.margin_left = Some(Emu::parse_emu(&mar_l).to_pt());
        }
    }
}

/// Parse rPr (run properties)
fn parse_run_props(e: &quick_xml::events::BytesStart<'_>, run: &mut Option<RunBuilder>) {
    if let Some(rb) = run.as_mut() {
        if let Some(sz) = xml_utils::attr_str(e, "sz") {
            rb.font_size = sz.parse::<f64>().ok().map(|v| v / 100.0);
        }
        if let Some(b) = xml_utils::attr_str(e, "b") {
            rb.bold = b == "1" || b == "true";
        }
        if let Some(i) = xml_utils::attr_str(e, "i") {
            rb.italic = i == "1" || i == "true";
        }
        if let Some(u) = xml_utils::attr_str(e, "u") {
            rb.underline = UnderlineType::from_ooxml(&u);
        }
        if let Some(strike) = xml_utils::attr_str(e, "strike") {
            rb.strikethrough = StrikethroughType::from_ooxml(&strike);
        }
        if let Some(cap) = xml_utils::attr_str(e, "cap") {
            rb.capitalization = TextCapitalization::from_ooxml(&cap);
        }
        if let Some(baseline) = xml_utils::attr_str(e, "baseline") {
            rb.baseline = baseline.parse::<i32>().ok();
        }
        if let Some(spc) = xml_utils::attr_str(e, "spc") {
            // spc is in 1/100 pt units
            rb.letter_spacing = spc.parse::<f64>().ok().map(|v| v / 100.0);
        }
    }
}

fn hyperlink_rel_id(e: &quick_xml::events::BytesStart<'_>) -> Option<String> {
    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
        if key.ends_with("id") && key.contains(':') {
            return Some(String::from_utf8_lossy(&attr.value).to_string());
        }
    }
    None
}

// ── Builder pattern ──

#[derive(Default)]
struct ShapeBuilder {
    id: u32,
    name: String,
    position: Position,
    size: Size,
    rotation: f64,
    flip_h: bool,
    flip_v: bool,
    paragraphs: Vec<TextParagraph>,
    has_text_body: bool,
    is_picture: bool,
    image_rel_id: Option<String>,
    preset_geometry: Option<String>,
    adjust_values: HashMap<String, f64>,
    // Fill/Border
    fill: Fill,
    border_width: f64,
    border_color: Color,
    border_style: BorderStyle,
    border_no_fill: bool,
    dash_style: DashStyle,
    border_cap: LineCap,
    border_compound: CompoundLine,
    border_alignment: LineAlignment,
    border_join: LineJoin,
    miter_limit: Option<f64>,
    head_end: Option<LineEnd>,
    tail_end: Option<LineEnd>,
    // bodyPr
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
    text_list_style: Option<ListStyle>,
    vertical_text: Option<String>,
    vertical_text_explicit: bool,
    // Image cropping
    crop: Option<CropRect>,
    // Placeholder and style reference (parsed as None for now)
    placeholder: Option<PlaceholderInfo>,
    style_ref: Option<ShapeStyleRef>,
    // Chart detection
    is_chart: bool,
    chart_rel_id: Option<String>,
    chart_direct_spec: Option<ChartSpec>,
    chart_preview_image: Option<Vec<u8>>,
    chart_preview_mime: Option<String>,
    // Unsupported content type (SmartArt, OLE, Math)
    unsupported_content: Option<String>,
    // Typed classification for unresolved element
    unresolved_type: Option<slide::UnresolvedType>,
    // Raw XML captured from graphicData for unresolved content
    raw_xml_capture: Option<String>,
    // Shape-level effects
    shape_outer_shadow: Option<OuterShadow>,
    shape_glow: Option<GlowEffect>,
    // Custom geometry
    custom_geometry: Option<CustomGeometry>,
    // Connection shape (cxnSp) — defaults to line if no preset geometry
    is_connector: bool,
    start_connection: Option<ConnectionRef>,
    end_connection: Option<ConnectionRef>,
}

impl ShapeBuilder {
    fn build(self) -> Shape {
        let shape_type = if let Some(label) = self.unsupported_content {
            ShapeType::Unsupported(slide::UnsupportedData {
                label,
                element_type: self
                    .unresolved_type
                    .unwrap_or(slide::UnresolvedType::SmartArt),
                raw_xml: self.raw_xml_capture,
            })
        } else if self.is_chart {
            ShapeType::Chart(ChartData {
                rel_id: self.chart_rel_id.unwrap_or_default(),
                preview_image: self.chart_preview_image,
                preview_mime: self.chart_preview_mime,
                direct_spec: self.chart_direct_spec,
            })
        } else if self.is_picture {
            ShapeType::Picture(PictureData {
                rel_id: self.image_rel_id.unwrap_or_default(),
                crop: self.crop,
                ..Default::default()
            })
        } else if let Some(geom) = self.custom_geometry {
            ShapeType::CustomGeom(geom)
        } else if let Some(ref prst) = self.preset_geometry {
            match prst.as_str() {
                "rect" => ShapeType::Rectangle,
                "roundRect" => ShapeType::RoundedRectangle,
                "ellipse" => ShapeType::Ellipse,
                "triangle" | "rtTriangle" => ShapeType::Triangle,
                other => ShapeType::Custom(other.to_string()),
            }
        } else if self.is_connector {
            // cxnSp without preset geometry defaults to a straight line
            ShapeType::Custom("line".to_string())
        } else {
            ShapeType::TextBox
        };

        let text_body = if self.has_text_body {
            let word_wrap = if self.text_word_wrap_explicit {
                self.text_word_wrap
            } else {
                true
            };
            Some(TextBody {
                paragraphs: self.paragraphs,
                list_style: self.text_list_style,
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
            })
        } else {
            None
        };

        let border = Border {
            width: self.border_width,
            color: self.border_color,
            style: if self.border_no_fill {
                // Explicit <a:noFill/> inside <a:ln>: keep None
                BorderStyle::None
            } else if self.border_width > 0.0 && matches!(self.border_style, BorderStyle::None) {
                BorderStyle::Solid
            } else {
                self.border_style
            },
            dash_style: self.dash_style,
            cap: self.border_cap,
            compound: self.border_compound,
            alignment: self.border_alignment,
            join: self.border_join,
            miter_limit: self.miter_limit,
            head_end: self.head_end,
            tail_end: self.tail_end,
            no_fill: self.border_no_fill,
        };

        let adjust_values = if self.adjust_values.is_empty() {
            None
        } else {
            Some(self.adjust_values)
        };

        let effects = ShapeEffects {
            outer_shadow: self.shape_outer_shadow,
            glow: self.shape_glow,
        };

        Shape {
            id: self.id,
            name: self.name,
            position: self.position,
            size: self.size,
            rotation: self.rotation,
            flip_h: self.flip_h,
            flip_v: self.flip_v,
            shape_type,
            text_body,
            fill: self.fill,
            border,
            placeholder: self.placeholder,
            style_ref: self.style_ref,
            adjust_values,
            start_connection: self.start_connection,
            end_connection: self.end_connection,
            vertical_text: self.vertical_text,
            vertical_text_explicit: self.vertical_text_explicit,
            effects,
            ..Default::default()
        }
    }
}

fn read_archive_entry<R: Read + Seek>(
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

fn read_archive_bytes<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    name: &str,
) -> PptxResult<Vec<u8>> {
    let mut file = archive
        .by_name(name)
        .map_err(|_| PptxError::MissingFile(name.to_string()))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn rels_path_for(part_path: &str) -> String {
    let (dir, file) = part_path.rsplit_once('/').unwrap_or(("", part_path));
    if dir.is_empty() {
        format!("_rels/{file}.rels")
    } else {
        format!("{dir}/_rels/{file}.rels")
    }
}

fn resolve_relative_file_path(base_file: &str, target: &str) -> String {
    let base_dir = base_file.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
    resolve_rel_path(base_dir, target)
}

fn store_shape_level_defaults(shape: &mut Option<ShapeBuilder>, lvl: usize, pd: ParagraphDefaults) {
    if lvl >= 9 {
        return;
    }
    if let Some(shape) = shape.as_mut() {
        let list_style = shape.text_list_style.get_or_insert_with(ListStyle::default);
        list_style.levels[lvl] = Some(pd);
    }
}

#[derive(Default)]
struct ParagraphBuilder {
    runs: Vec<TextRun>,
    alignment: Alignment,
    rtl: bool,
    level: u32,
    indent: Option<f64>,
    margin_left: Option<f64>,
    bullet: Option<Bullet>,
    line_spacing: Option<SpacingValue>,
    space_before: Option<SpacingValue>,
    space_after: Option<SpacingValue>,
    // Bullet property accumulation (applied when buChar/buAutoNum is encountered)
    bu_font: Option<String>,
    bu_size_pct: Option<f64>,
    bu_color: Option<Color>,
    // Paragraph-level defRPr properties
    def_rpr_font_size: Option<f64>,
    def_rpr_letter_spacing: Option<f64>,
    def_rpr_baseline: Option<i32>,
    def_rpr_capitalization: Option<TextCapitalization>,
    def_rpr_underline: Option<UnderlineType>,
    def_rpr_strikethrough: Option<StrikethroughType>,
    def_rpr_bold: Option<bool>,
    def_rpr_italic: Option<bool>,
    def_rpr_color: Option<Color>,
    def_rpr_font_latin: Option<String>,
    def_rpr_font_ea: Option<String>,
    def_rpr_font_cs: Option<String>,
}

impl ParagraphBuilder {
    fn build(self) -> TextParagraph {
        let def_rpr = if self.def_rpr_font_size.is_some()
            || self.def_rpr_letter_spacing.is_some()
            || self.def_rpr_baseline.is_some()
            || self.def_rpr_capitalization.is_some()
            || self.def_rpr_underline.is_some()
            || self.def_rpr_strikethrough.is_some()
            || self.def_rpr_bold.is_some()
            || self.def_rpr_italic.is_some()
            || self.def_rpr_color.is_some()
            || self.def_rpr_font_latin.is_some()
            || self.def_rpr_font_ea.is_some()
            || self.def_rpr_font_cs.is_some()
        {
            Some(ParagraphDefRPr {
                font_size: self.def_rpr_font_size,
                letter_spacing: self.def_rpr_letter_spacing,
                baseline: self.def_rpr_baseline,
                capitalization: self.def_rpr_capitalization,
                underline: self.def_rpr_underline,
                strikethrough: self.def_rpr_strikethrough,
                bold: self.def_rpr_bold,
                italic: self.def_rpr_italic,
                color: self.def_rpr_color,
                font_latin: self.def_rpr_font_latin,
                font_ea: self.def_rpr_font_ea,
                font_cs: self.def_rpr_font_cs,
            })
        } else {
            None
        };
        TextParagraph {
            runs: self.runs,
            alignment: self.alignment,
            rtl: self.rtl,
            line_spacing: self.line_spacing,
            space_before: self.space_before,
            space_after: self.space_after,
            indent: self.indent,
            margin_left: self.margin_left,
            bullet: self.bullet,
            level: self.level,
            def_rpr,
        }
    }
}

#[derive(Default)]
struct RunBuilder {
    text: String,
    font_size: Option<f64>,
    bold: bool,
    italic: bool,
    underline: UnderlineType,
    strikethrough: StrikethroughType,
    capitalization: TextCapitalization,
    color: Color,
    font_latin: Option<String>,
    font_ea: Option<String>,
    font_cs: Option<String>,
    baseline: Option<i32>,
    letter_spacing: Option<f64>,
    highlight: Option<Color>,
    shadow: Option<TextShadow>,
    hyperlink: Option<String>,
    is_break: bool,
}

impl RunBuilder {
    fn build(self) -> TextRun {
        TextRun {
            text: self.text,
            style: TextStyle {
                font_size: self.font_size,
                bold: self.bold,
                italic: self.italic,
                underline: self.underline,
                strikethrough: self.strikethrough,
                capitalization: self.capitalization,
                color: self.color,
                baseline: self.baseline,
                letter_spacing: self.letter_spacing,
                highlight: self.highlight,
                shadow: self.shadow,
                ..Default::default()
            },
            font: FontStyle {
                latin: self.font_latin,
                east_asian: self.font_ea,
                complex_script: self.font_cs,
            },
            hyperlink: self.hyperlink,
            is_break: self.is_break,
        }
    }
}

// ── Table builder pattern ──

#[derive(Default)]
struct TableBuilder {
    col_widths: Vec<f64>,
    rows: Vec<TableRow>,
    band_row: bool,
    band_col: bool,
    first_row: bool,
    last_row: bool,
    first_col: bool,
    last_col: bool,
}

impl TableBuilder {
    fn build(self) -> TableData {
        TableData {
            rows: self.rows,
            col_widths: self.col_widths,
            band_row: self.band_row,
            band_col: self.band_col,
            first_row: self.first_row,
            last_row: self.last_row,
            first_col: self.first_col,
            last_col: self.last_col,
        }
    }
}

#[derive(Default)]
struct TableRowBuilder {
    height: f64,
    cells: Vec<TableCell>,
}

impl TableRowBuilder {
    fn build(self) -> TableRow {
        TableRow {
            height: self.height,
            cells: self.cells,
        }
    }
}

struct TableCellBuilder {
    text_body: Option<TextBody>,
    fill: Fill,
    border_left: Border,
    border_right: Border,
    border_top: Border,
    border_bottom: Border,
    col_span: u32,
    row_span: u32,
    v_merge: bool,
    margin_left: f64,
    margin_right: f64,
    margin_top: f64,
    margin_bottom: f64,
    vertical_align: VerticalAlign,
}

impl Default for TableCellBuilder {
    fn default() -> Self {
        let cell = TableCell::default();
        Self {
            text_body: cell.text_body,
            fill: cell.fill,
            border_left: cell.border_left,
            border_right: cell.border_right,
            border_top: cell.border_top,
            border_bottom: cell.border_bottom,
            col_span: cell.col_span,
            row_span: cell.row_span,
            v_merge: cell.v_merge,
            margin_left: cell.margin_left,
            margin_right: cell.margin_right,
            margin_top: cell.margin_top,
            margin_bottom: cell.margin_bottom,
            vertical_align: cell.vertical_align,
        }
    }
}

impl TableCellBuilder {
    fn build(self) -> TableCell {
        TableCell {
            text_body: self.text_body,
            fill: self.fill,
            border_left: self.border_left,
            border_right: self.border_right,
            border_top: self.border_top,
            border_bottom: self.border_bottom,
            col_span: self.col_span,
            row_span: self.row_span,
            v_merge: self.v_merge,
            margin_left: self.margin_left,
            margin_right: self.margin_right,
            margin_top: self.margin_top,
            margin_bottom: self.margin_bottom,
            vertical_align: self.vertical_align,
        }
    }
}

// ── Group shape context ──

struct GroupContext {
    shapes: Vec<Shape>,
    position: Position,
    size: Size,
    child_offset: Position,
    child_extent: Size,
}

/// Assign color to table cell fill or border based on context
fn assign_tc_color(
    color: Color,
    border_side: &Option<String>,
    cell: &mut Option<TableCellBuilder>,
) {
    let cell = match cell.as_mut() {
        Some(c) => c,
        None => return,
    };
    match border_side.as_deref() {
        Some("lnL") => {
            cell.border_left.color = color;
            if matches!(cell.border_left.style, BorderStyle::None) && cell.border_left.width > 0.0 {
                cell.border_left.style = BorderStyle::Solid;
            }
        }
        Some("lnR") => {
            cell.border_right.color = color;
            if matches!(cell.border_right.style, BorderStyle::None) && cell.border_right.width > 0.0
            {
                cell.border_right.style = BorderStyle::Solid;
            }
        }
        Some("lnT") => {
            cell.border_top.color = color;
            if matches!(cell.border_top.style, BorderStyle::None) && cell.border_top.width > 0.0 {
                cell.border_top.style = BorderStyle::Solid;
            }
        }
        Some("lnB") => {
            cell.border_bottom.color = color;
            if matches!(cell.border_bottom.style, BorderStyle::None)
                && cell.border_bottom.width > 0.0
            {
                cell.border_bottom.style = BorderStyle::Solid;
            }
        }
        None => {
            // Cell fill color (solidFill inside tcPr, not inside a border)
            cell.fill = Fill::Solid(SolidFill { color });
        }
        _ => {}
    }
}

/// Resolve a relative path from a base directory within the ZIP archive.
/// e.g., resolve_rel_path("ppt/slides", "../media/image1.png") -> "ppt/media/image1.png"
fn resolve_rel_path(base_dir: &str, target: &str) -> String {
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

/// Determine MIME type from file extension
fn mime_from_extension(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "tif" | "tiff" => "image/tiff",
        "svg" => "image/svg+xml",
        "emf" => "image/x-emf",
        "wmf" => "image/x-wmf",
        _ => "image/png",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::{Cursor, Write};

    use quick_xml::events::BytesStart;
    use zip::ZipArchive;
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    use super::*;

    #[test]
    fn assign_color_routes_to_runs_gradients_borders_and_fills() {
        let mut run = Some(RunBuilder::default());
        let mut shape = Some(ShapeBuilder::default());
        let mut grad_stops = Vec::new();

        assign_color(
            Color::rgb("112233"),
            &["sp".into(), "txBody".into(), "p".into(), "rPr".into()],
            false,
            false,
            true,
            false,
            0.0,
            &mut shape,
            &mut run,
            &mut grad_stops,
        );
        assert_eq!(
            run.as_ref().and_then(|rb| rb.color.to_css()).as_deref(),
            Some("#112233")
        );

        assign_color(
            Color::theme("accent1"),
            &["spPr".into(), "gradFill".into(), "gs".into()],
            false,
            false,
            false,
            true,
            0.25,
            &mut shape,
            &mut run,
            &mut grad_stops,
        );
        assert_eq!(grad_stops.len(), 1);
        assert!((grad_stops[0].position - 0.25).abs() < 1e-6);

        assign_color(
            Color::rgb("445566"),
            &["spPr".into(), "ln".into()],
            false,
            true,
            false,
            false,
            0.0,
            &mut shape,
            &mut run,
            &mut grad_stops,
        );
        let shape_ref = shape.as_ref().expect("shape builder");
        assert_eq!(shape_ref.border_color.to_css().as_deref(), Some("#445566"));
        assert_eq!(
            std::mem::discriminant(&shape_ref.border_style),
            std::mem::discriminant(&BorderStyle::Solid)
        );

        assign_color(
            Color::rgb("778899"),
            &["spPr".into(), "solidFill".into()],
            true,
            false,
            false,
            false,
            0.0,
            &mut shape,
            &mut run,
            &mut grad_stops,
        );
        assert!(matches!(
            shape.as_ref().expect("shape").fill,
            Fill::Solid(ref fill) if fill.color.to_css().as_deref() == Some("#778899")
        ));

        assign_color(
            Color::rgb("AABBCC"),
            &["spPr".into()],
            true,
            false,
            false,
            false,
            0.0,
            &mut shape,
            &mut run,
            &mut grad_stops,
        );
        assert!(matches!(
            shape.as_ref().expect("shape").fill,
            Fill::Solid(ref fill) if fill.color.to_css().as_deref() == Some("#AABBCC")
        ));
    }

    #[test]
    fn style_ref_and_line_end_helpers_cover_supported_variants() {
        let mut style_ref = Some(ShapeStyleRef::default());
        assign_style_ref_color("fillRef", "2", Color::rgb("112233"), &mut style_ref);
        assign_style_ref_color("lnRef", "3", Color::theme("accent2"), &mut style_ref);
        assign_style_ref_color("effectRef", "4", Color::rgb("445566"), &mut style_ref);
        assign_style_ref_color("fontRef", "minor", Color::theme("accent3"), &mut style_ref);
        ensure_style_ref("fillRef", "9", &mut style_ref);
        assign_style_ref_no_color("effectRef", "6", &mut style_ref);

        let style_ref = style_ref.expect("style ref");
        assert_eq!(style_ref.fill_ref.as_ref().map(|s| s.idx), Some(2));
        assert_eq!(style_ref.ln_ref.as_ref().map(|s| s.idx), Some(3));
        assert_eq!(style_ref.effect_ref.as_ref().map(|s| s.idx), Some(6));
        assert_eq!(
            style_ref.font_ref.as_ref().map(|s| s.idx.as_str()),
            Some("minor")
        );

        let arrow = parse_line_end(&bytes_start(
            "a:headEnd",
            &[("type", "arrow"), ("w", "sm"), ("len", "lg")],
        ))
        .expect("arrow line end");
        assert_eq!(
            std::mem::discriminant(&arrow.end_type),
            std::mem::discriminant(&LineEndType::Arrow)
        );
        assert_eq!(
            std::mem::discriminant(&arrow.width),
            std::mem::discriminant(&LineEndSize::Small)
        );
        assert_eq!(
            std::mem::discriminant(&arrow.length),
            std::mem::discriminant(&LineEndSize::Large)
        );
        assert!(parse_line_end(&bytes_start("a:tailEnd", &[("type", "none")])).is_none());
        assert!(parse_line_end(&bytes_start("a:tailEnd", &[("type", "weird")])).is_none());
    }

    #[test]
    fn guide_formula_and_body_parsers_cover_helper_branches() {
        let guides = HashMap::from([
            ("x".to_string(), 3.0),
            ("y".to_string(), 4.0),
            ("z".to_string(), 12.0),
        ]);
        assert_eq!(parse_guide_formula_value("val x", &guides), 3.0);
        assert_eq!(parse_guide_formula_value("+- 5 4 3", &guides), 6.0);
        assert_eq!(parse_guide_formula_value("*/ 6 4 3", &guides), 8.0);
        assert_eq!(parse_guide_formula_value("+/ 6 4 2", &guides), 5.0);
        assert_eq!(parse_guide_formula_value("pin 1 5 3", &guides), 3.0);
        assert_eq!(parse_guide_formula_value("min 7 3", &guides), 3.0);
        assert_eq!(parse_guide_formula_value("max 7 3", &guides), 7.0);
        assert_eq!(parse_guide_formula_value("?: 1 8 9", &guides), 8.0);
        assert_eq!(parse_guide_formula_value("?: 0 8 9", &guides), 9.0);
        assert_eq!(parse_guide_formula_value("abs -7", &guides), 7.0);
        assert_eq!(parse_guide_formula_value("sqrt 16", &guides), 4.0);
        assert!((parse_guide_formula_value("mod x y z", &guides) - 13.0).abs() < 1e-6);
        assert!((parse_guide_formula_value("sin 10 5400000", &guides) - 10.0).abs() < 1e-6);
        assert!((parse_guide_formula_value("cos 10 0", &guides) - 10.0).abs() < 1e-6);
        assert!(
            (parse_guide_formula_value("cat2 10 y z", &guides) - 10.0 * (12.0f64.atan2(4.0)).cos())
                .abs()
                < 1e-6
        );
        assert!(
            (parse_guide_formula_value("sat2 10 y z", &guides) - 10.0 * (12.0f64.atan2(4.0)).sin())
                .abs()
                < 1e-6
        );
        assert!(
            (parse_guide_formula_value("at2 x y", &guides)
                - 4.0f64.atan2(3.0).to_degrees() * 60_000.0)
                .abs()
                < 1e-6
        );
        assert!((parse_guide_formula_value("tan 10 2700000", &guides) - 10.0).abs() < 1e-6);
        assert_eq!(parse_guide_formula_value("unknown", &guides), 0.0);
        assert_eq!(resolve_custom_geom_value("42", &guides), 42.0);
        assert_eq!(resolve_custom_geom_value("x", &guides), 3.0);
        assert_eq!(resolve_custom_geom_value("missing", &guides), 0.0);
        assert!((ooxml_angle_to_radians(5_400_000.0) - std::f64::consts::FRAC_PI_2).abs() < 1e-6);

        let mut shape = Some(ShapeBuilder::default());
        parse_body_pr(
            &bytes_start(
                "a:bodyPr",
                &[
                    ("anchor", "ctr"),
                    ("anchorCtr", "1"),
                    ("rot", "5400000"),
                    ("lIns", "91440"),
                    ("tIns", "45720"),
                    ("rIns", "182880"),
                    ("bIns", "22860"),
                    ("wrap", "none"),
                    ("vert", "vert270"),
                ],
            ),
            &mut shape,
        );
        let shape = shape.expect("shape builder");
        assert_eq!(
            std::mem::discriminant(&shape.text_vertical_align),
            std::mem::discriminant(&VerticalAlign::Middle)
        );
        assert!(shape.text_anchor_center);
        assert!((shape.text_rotation_deg - 90.0).abs() < 1e-6);
        assert!(!shape.text_word_wrap);
        assert_eq!(shape.vertical_text.as_deref(), Some("vert270"));
        assert_eq!(shape.text_margins.left, 7.2);
        assert_eq!(shape.text_margins.top, 3.6);
        assert_eq!(shape.text_margins.right, 14.4);
        assert_eq!(shape.text_margins.bottom, 1.8);
        assert_eq!(
            parse_autofit_ratio(
                &bytes_start("a:normAutofit", &[("fontScale", "250000")]),
                "fontScale"
            ),
            Some(1.0)
        );
        assert_eq!(
            parse_autofit_ratio(
                &bytes_start("a:normAutofit", &[("fontScale", "-1000")]),
                "fontScale"
            ),
            Some(0.0)
        );
    }

    #[test]
    fn shape_paragraph_run_archive_and_table_helpers_cover_remaining_paths() {
        let mut shape = Some(ShapeBuilder::default());
        parse_shape_identity(
            &bytes_start("p:cNvPr", &[("id", "7"), ("name", "Connector")]),
            &mut shape,
        );
        parse_connector_ref(
            &bytes_start("a:stCxn", &[("id", "11"), ("idx", "2")]),
            &mut shape,
            true,
        );
        parse_connector_ref(
            &bytes_start("a:endCxn", &[("id", "12"), ("idx", "3")]),
            &mut shape,
            false,
        );
        let shape = shape.expect("shape");
        assert_eq!(shape.id, 7);
        assert_eq!(shape.name, "Connector");
        assert_eq!(
            shape.start_connection.as_ref().map(|c| c.shape_id),
            Some(11)
        );
        assert_eq!(shape.end_connection.as_ref().map(|c| c.site_idx), Some(3));

        let mut para = Some(ParagraphBuilder::default());
        parse_para_props(
            &bytes_start(
                "a:pPr",
                &[
                    ("algn", "ctr"),
                    ("rtl", "1"),
                    ("lvl", "2"),
                    ("indent", "12700"),
                    ("marL", "25400"),
                ],
            ),
            &mut para,
        );
        let mut run = Some(RunBuilder::default());
        parse_run_props(
            &bytes_start(
                "a:rPr",
                &[
                    ("sz", "2400"),
                    ("b", "1"),
                    ("i", "true"),
                    ("u", "dbl"),
                    ("strike", "sngStrike"),
                    ("cap", "all"),
                    ("baseline", "30000"),
                    ("spc", "200"),
                ],
            ),
            &mut run,
        );
        assert_eq!(
            hyperlink_rel_id(&bytes_start("a:hlinkClick", &[("r:id", "rIdHyper")])),
            Some("rIdHyper".to_string())
        );

        let mut archive = archive_with_entries(&[
            ("ppt/slides/slide1.xml", b"<slide/>".as_slice()),
            ("ppt/media/image.png", b"png".as_slice()),
        ]);
        assert_eq!(
            read_archive_entry(&mut archive, "ppt/slides/slide1.xml").unwrap(),
            "<slide/>"
        );
        assert_eq!(
            read_archive_bytes(&mut archive, "ppt/media/image.png").unwrap(),
            b"png"
        );
        assert!(read_archive_entry(&mut archive, "missing.xml").is_err());
        assert_eq!(
            rels_path_for("ppt/slides/slide1.xml"),
            "ppt/slides/_rels/slide1.xml.rels"
        );
        assert_eq!(
            resolve_relative_file_path("ppt/slides/slide1.xml", "../media/image1.png"),
            "ppt/media/image1.png"
        );
        assert_eq!(
            resolve_rel_path("ppt/slides", "../media/image1.png"),
            "ppt/media/image1.png"
        );
        assert_eq!(
            resolve_rel_path("ppt/slides", "media/image1.png"),
            "ppt/slides/media/image1.png"
        );
        assert_eq!(mime_from_extension("image.png"), "image/png");
        assert_eq!(mime_from_extension("image.jpg"), "image/jpeg");
        assert_eq!(mime_from_extension("image.gif"), "image/gif");
        assert_eq!(mime_from_extension("image.bmp"), "image/bmp");
        assert_eq!(mime_from_extension("image.tif"), "image/tiff");
        assert_eq!(mime_from_extension("image.svg"), "image/svg+xml");
        assert_eq!(mime_from_extension("image.emf"), "image/x-emf");
        assert_eq!(mime_from_extension("image.wmf"), "image/x-wmf");
        assert_eq!(mime_from_extension("image.bin"), "image/png");

        let mut list_shape = Some(ShapeBuilder::default());
        store_shape_level_defaults(&mut list_shape, 0, ParagraphDefaults::default());
        assert!(
            list_shape
                .as_ref()
                .and_then(|s| s.text_list_style.as_ref())
                .and_then(|ls| ls.levels[0].as_ref())
                .is_some()
        );

        let paragraph = para.expect("paragraph").build();
        let built_run = run.expect("run").build();
        assert_eq!(
            std::mem::discriminant(&paragraph.alignment),
            std::mem::discriminant(&Alignment::Center)
        );
        assert!(paragraph.rtl);
        assert_eq!(paragraph.level, 2);
        assert_eq!(paragraph.indent, Some(1.0));
        assert_eq!(paragraph.margin_left, Some(2.0));
        assert_eq!(built_run.style.font_size, Some(24.0));
        assert!(built_run.style.bold);
        assert!(built_run.style.italic);
        assert_eq!(
            std::mem::discriminant(&built_run.style.underline),
            std::mem::discriminant(&UnderlineType::Double)
        );
        assert_eq!(
            std::mem::discriminant(&built_run.style.strikethrough),
            std::mem::discriminant(&StrikethroughType::Single)
        );
        assert_eq!(
            std::mem::discriminant(&built_run.style.capitalization),
            std::mem::discriminant(&TextCapitalization::All)
        );
        assert_eq!(built_run.style.baseline, Some(30000));
        assert_eq!(built_run.style.letter_spacing, Some(2.0));

        let chart_shape = ShapeBuilder {
            is_chart: true,
            chart_rel_id: Some("rIdChart".to_string()),
            chart_direct_spec: Some(ChartSpec::default()),
            ..Default::default()
        }
        .build();
        assert_eq!(
            std::mem::discriminant(&chart_shape.shape_type),
            std::mem::discriminant(&ShapeType::Chart(ChartData::default()))
        );

        let picture_shape = ShapeBuilder {
            is_picture: true,
            image_rel_id: Some("rIdImage".to_string()),
            ..Default::default()
        }
        .build();
        assert_eq!(
            std::mem::discriminant(&picture_shape.shape_type),
            std::mem::discriminant(&ShapeType::Picture(PictureData::default()))
        );

        let connector_shape = ShapeBuilder {
            is_connector: true,
            ..Default::default()
        }
        .build();
        if let ShapeType::Custom(name) = &connector_shape.shape_type {
            assert_eq!(name, "line");
        } else {
            panic!("expected line custom shape");
        }

        let unsupported_shape = ShapeBuilder {
            unsupported_content: Some("SmartArt".to_string()),
            unresolved_type: Some(UnresolvedType::SmartArt),
            ..Default::default()
        }
        .build();
        assert_eq!(
            std::mem::discriminant(&unsupported_shape.shape_type),
            std::mem::discriminant(&ShapeType::Unsupported(UnsupportedData {
                label: String::new(),
                element_type: UnresolvedType::SmartArt,
                raw_xml: None,
            }))
        );

        let custom_geom_shape = ShapeBuilder {
            custom_geometry: Some(CustomGeometry {
                paths: Vec::new(),
                text_rect: None,
                adjust_handles: Vec::new(),
                connection_sites: Vec::new(),
            }),
            ..Default::default()
        }
        .build();
        assert_eq!(
            std::mem::discriminant(&custom_geom_shape.shape_type),
            std::mem::discriminant(&ShapeType::CustomGeom(CustomGeometry {
                paths: Vec::new(),
                text_rect: None,
                adjust_handles: Vec::new(),
                connection_sites: Vec::new(),
            }))
        );

        let mut cell = Some(default_table_cell_builder());
        cell.as_mut().unwrap().border_left.width = 1.0;
        cell.as_mut().unwrap().border_top.width = 1.0;
        assign_tc_color(Color::rgb("112233"), &Some("lnL".to_string()), &mut cell);
        assign_tc_color(Color::rgb("445566"), &Some("lnT".to_string()), &mut cell);
        assign_tc_color(Color::rgb("778899"), &None, &mut cell);
        let cell = cell.expect("cell").build();
        assert_eq!(cell.border_left.color.to_css().as_deref(), Some("#112233"));
        assert_eq!(cell.border_top.color.to_css().as_deref(), Some("#445566"));
        let fill = match &cell.fill {
            Fill::Solid(fill) => fill,
            other => panic!("expected solid fill, got {other:?}"),
        };
        assert_eq!(fill.color.to_css().as_deref(), Some("#778899"));

        let table = TableBuilder {
            rows: vec![
                TableRowBuilder {
                    height: 18.0,
                    cells: vec![default_table_cell_builder().build()],
                }
                .build(),
            ],
            col_widths: vec![120.0],
            band_row: true,
            ..Default::default()
        }
        .build();
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.col_widths, vec![120.0]);
        assert!(table.band_row);
    }

    #[test]
    fn parse_slide_covers_regular_shape_text_and_style_ref_branches() {
        let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="7" name="Styled Shape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="1800000" flipV="1">
            <a:off x="12700" y="25400"/>
            <a:ext cx="457200" cy="228600"/>
          </a:xfrm>
          <a:gradFill>
            <a:gsLst>
              <a:gs pos="0"><a:prstClr val="orange"/></a:gs>
              <a:gs pos="100000"><a:sysClr lastClr="112233"/></a:gs>
            </a:gsLst>
            <a:path path="circle"/>
          </a:gradFill>
          <a:ln w="12700" cap="flat" cmpd="tri" algn="in"></a:ln>
        </p:spPr>
        <p:style>
          <a:lnRef idx="1"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="2"><a:prstClr val="orange"/></a:fillRef>
          <a:effectRef idx="1"><a:sysClr val="windowText"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="accent2"/></a:fontRef>
        </p:style>
        <p:txBody>
          <a:bodyPr wrap="none"></a:bodyPr>
          <a:noAutofit></a:noAutofit>
          <a:lstStyle>
            <a:lvl1pPr algn="r">
              <a:lnSpc><a:spcPct val="80000"/></a:lnSpc>
              <a:spcBef><a:spcPts val="1200"/></a:spcBef>
              <a:spcAft><a:spcPct val="110000"/></a:spcAft>
              <a:defRPr sz="1600" spc="100" baseline="20000" cap="small" u="dash" strike="dblStrike" b="1" i="1">
                <a:schemeClr val="accent2"/>
              </a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
          <a:p>
            <a:pPr algn="ctr" rtl="1" lvl="1" indent="12700" marL="25400">
              <a:defRPr sz="2400" spc="200" baseline="30000" cap="all" u="dbl" strike="sngStrike" b="1" i="1"/>
            </a:pPr>
            <a:buClr><a:prstClr val="orange"/></a:buClr>
            <a:r>
              <a:rPr sz="1800">
                <a:hlinkClick r:id="rIdHyper"/>
              </a:rPr>
              <a:t>First</a:t>
            </a:r>
            <a:br/>
            <a:r><a:t>Second</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="8" name="Shrink Shape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="228600" cy="114300"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr"></a:bodyPr>
          <a:spAutoFit></a:spAutoFit>
          <a:p/>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

        let rels = HashMap::from([("rIdHyper".to_string(), "https://example.com".to_string())]);
        let mut archive = archive_with_entries(&[]);

        let slide = parse_slide(slide_xml, &rels, &mut archive).expect("slide should parse");
        assert_eq!(slide.shapes.len(), 2);

        let shape = &slide.shapes[0];
        assert_eq!(shape.id, 7);
        assert_eq!(shape.name, "Styled Shape");
        assert!((shape.rotation - 30.0).abs() < 1e-6);
        assert!(shape.flip_v);
        assert_eq!(
            std::mem::discriminant(&shape.border.cap),
            std::mem::discriminant(&LineCap::Flat)
        );
        assert_eq!(
            std::mem::discriminant(&shape.border.alignment),
            std::mem::discriminant(&LineAlignment::Inset)
        );
        assert_eq!(
            shape
                .style_ref
                .as_ref()
                .and_then(|style| style.effect_ref.as_ref())
                .map(|effect_ref| effect_ref.idx),
            Some(1)
        );
        let fill = match &shape.fill {
            Fill::Gradient(fill) => fill,
            other => panic!("expected gradient fill, got {other:?}"),
        };
        assert_eq!(fill.stops.len(), 2);
        assert_eq!(
            fill.stops[0].color.kind,
            ColorKind::Preset("orange".to_string())
        );
        assert_eq!(
            fill.stops[1].color.kind,
            ColorKind::Rgb("112233".to_string())
        );

        let text_body = shape.text_body.as_ref().expect("text body");
        assert!(!text_body.word_wrap);
        assert_eq!(
            std::mem::discriminant(&text_body.auto_fit),
            std::mem::discriminant(&AutoFit::NoAutoFit)
        );
        let list_style = text_body.list_style.as_ref().expect("list style");
        let lvl1 = list_style.levels[0].as_ref().expect("level 1 defaults");
        assert_eq!(
            lvl1.alignment
                .as_ref()
                .map(std::mem::discriminant::<Alignment>),
            Some(std::mem::discriminant(&Alignment::Right))
        );
        if let Some(SpacingValue::Percent(v)) = lvl1.line_spacing {
            assert!((v - 0.8).abs() < 1e-6);
        } else {
            panic!("expected percent line spacing");
        }
        if let Some(SpacingValue::Points(v)) = lvl1.space_before {
            assert!((v - 12.0).abs() < 1e-6);
        } else {
            panic!("expected points space before");
        }
        if let Some(SpacingValue::Percent(v)) = lvl1.space_after {
            assert!((v - 1.1).abs() < 1e-6);
        } else {
            panic!("expected percent space after");
        }

        let paragraph = &text_body.paragraphs[0];
        assert_eq!(
            std::mem::discriminant(&paragraph.alignment),
            std::mem::discriminant(&Alignment::Center)
        );
        assert!(paragraph.rtl);
        assert_eq!(paragraph.level, 1);
        assert_eq!(paragraph.runs.len(), 3);
        assert_eq!(
            paragraph.runs[0].hyperlink.as_deref(),
            Some("https://example.com")
        );
        assert!(paragraph.runs[1].is_break);
        let def_rpr = paragraph.def_rpr.as_ref().expect("paragraph defRPr");
        assert_eq!(def_rpr.font_size, Some(24.0));
        assert_eq!(def_rpr.letter_spacing, Some(2.0));
        assert_eq!(def_rpr.baseline, Some(30000));
        assert_eq!(def_rpr.bold, Some(true));
        assert_eq!(def_rpr.italic, Some(true));
        assert_eq!(
            def_rpr
                .capitalization
                .as_ref()
                .map(std::mem::discriminant::<TextCapitalization>),
            Some(std::mem::discriminant(&TextCapitalization::All))
        );
        assert_eq!(
            def_rpr
                .underline
                .as_ref()
                .map(std::mem::discriminant::<UnderlineType>),
            Some(std::mem::discriminant(&UnderlineType::Double))
        );
        assert_eq!(
            def_rpr
                .strikethrough
                .as_ref()
                .map(std::mem::discriminant::<StrikethroughType>),
            Some(std::mem::discriminant(&StrikethroughType::Single))
        );

        let shrink_shape = &slide.shapes[1];
        assert_eq!(
            shrink_shape
                .text_body
                .as_ref()
                .map(|tb| std::mem::discriminant(&tb.auto_fit)),
            Some(std::mem::discriminant(&AutoFit::Shrink))
        );
    }

    #[test]
    fn parse_slide_covers_ole_and_table_cell_variants() {
        let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="2" name="OLE"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="457200"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/presentationml/2006/oleObject">
            <p:oleObj progId="Excel.Sheet.12" name="Workbook"/>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="3" name="Table"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="1828800" cy="914400"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
            <a:tbl>
              <a:tblPr bandRow="1" bandCol="1" firstRow="1" lastRow="1" firstCol="1" lastCol="1"></a:tblPr>
              <a:tblGrid>
                <a:gridCol w="914400"/>
                <a:gridCol w="457200"/>
              </a:tblGrid>
              <a:tr h="457200">
                <a:tc gridSpan="2" rowSpan="2" vMerge="1">
                  <a:txBody>
                    <a:bodyPr/>
                    <a:lstStyle/>
                    <a:p>
                      <a:pPr algn="ctr" lvl="1" indent="91440" marL="45720">
                        <a:defRPr sz="2000" spc="100" baseline="10000" cap="small" u="dashLong" strike="dblStrike" b="1" i="1"/>
                      </a:pPr>
                      <a:buClr><a:schemeClr val="accent2"/></a:buClr>
                      <a:r>
                        <a:rPr sz="1800"><a:hlinkClick r:id="rIdHyper"/></a:rPr>
                        <a:t>Cell</a:t>
                      </a:r>
                      <a:br/>
                    </a:p>
                  </a:txBody>
                  <a:tcPr marL="91440" marR="137160" marT="45720" marB="22860" anchor="b">
                    <a:solidFill><a:srgbClr val="00FF00"/></a:solidFill>
                    <a:lnL w="12700"><a:prstDash val="sysDot"></a:prstDash><a:srgbClr val="FF0000"/></a:lnL>
                    <a:lnR w="12700"><a:prstDash val="lgDashDot"></a:prstDash><a:srgbClr val="0000FF"/></a:lnR>
                    <a:lnT w="12700"><a:prstDash val="lgDashDotDot"></a:prstDash><a:srgbClr val="123456"/></a:lnT>
                    <a:lnB w="12700"><a:prstDash val="sysDash"></a:prstDash><a:srgbClr val="654321"/></a:lnB>
                  </a:tcPr>
                </a:tc>
              </a:tr>
            </a:tbl>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

        let rels = HashMap::from([(
            "rIdHyper".to_string(),
            "https://example.com/table".to_string(),
        )]);
        let mut archive = archive_with_entries(&[]);

        let slide = parse_slide(slide_xml, &rels, &mut archive).expect("slide should parse");
        assert_eq!(slide.shapes.len(), 2);

        let ole = match &slide.shapes[0].shape_type {
            ShapeType::Unsupported(data) => data,
            other => panic!("expected unsupported OLE shape, got {other:?}"),
        };
        assert_eq!(ole.label, "OLE Object");
        assert!(
            ole.raw_xml
                .as_deref()
                .is_some_and(|raw| raw.contains("progId=\"Excel.Sheet.12\""))
        );

        let table = slide
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape_type {
                ShapeType::Table(table) => Some(table),
                _ => None,
            })
            .expect("table shape");
        assert!(table.band_row && table.band_col && table.first_row && table.last_row);
        assert!(table.first_col && table.last_col);
        assert_eq!(table.col_widths.len(), 2);

        let cell = &table.rows[0].cells[0];
        assert_eq!(cell.col_span, 2);
        assert_eq!(cell.row_span, 2);
        assert!(cell.v_merge);
        assert_eq!(
            std::mem::discriminant(&cell.vertical_align),
            std::mem::discriminant(&VerticalAlign::Bottom)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_left.style),
            std::mem::discriminant(&BorderStyle::Dotted)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_left.dash_style),
            std::mem::discriminant(&DashStyle::SystemDot)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_right.style),
            std::mem::discriminant(&BorderStyle::Dotted)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_right.dash_style),
            std::mem::discriminant(&DashStyle::LongDashDot)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_top.style),
            std::mem::discriminant(&BorderStyle::Dotted)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_top.dash_style),
            std::mem::discriminant(&DashStyle::LongDashDotDot)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_bottom.style),
            std::mem::discriminant(&BorderStyle::Dashed)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_bottom.dash_style),
            std::mem::discriminant(&DashStyle::SystemDash)
        );

        let paragraph = &cell.text_body.as_ref().expect("cell text body").paragraphs[0];
        assert_eq!(
            std::mem::discriminant(&paragraph.alignment),
            std::mem::discriminant(&Alignment::Center)
        );
        assert_eq!(paragraph.level, 1);
        assert_eq!(paragraph.runs.len(), 2);
        assert_eq!(
            paragraph.runs[0].hyperlink.as_deref(),
            Some("https://example.com/table")
        );
        assert!(paragraph.runs[1].is_break);
        let def_rpr = paragraph.def_rpr.as_ref().expect("cell paragraph defRPr");
        assert_eq!(def_rpr.font_size, Some(20.0));
        assert_eq!(def_rpr.letter_spacing, Some(1.0));
        assert_eq!(def_rpr.baseline, Some(10000));
        assert_eq!(def_rpr.bold, Some(true));
        assert_eq!(def_rpr.italic, Some(true));
        assert_eq!(
            def_rpr
                .capitalization
                .as_ref()
                .map(std::mem::discriminant::<TextCapitalization>),
            Some(std::mem::discriminant(&TextCapitalization::Small))
        );
        assert_eq!(
            def_rpr
                .underline
                .as_ref()
                .map(std::mem::discriminant::<UnderlineType>),
            Some(std::mem::discriminant(&UnderlineType::DashLong))
        );
        assert_eq!(
            def_rpr
                .strikethrough
                .as_ref()
                .map(std::mem::discriminant::<StrikethroughType>),
            Some(std::mem::discriminant(&StrikethroughType::Double))
        );
    }

    #[test]
    fn parse_slide_loads_chart_preview_and_picture_crop_variants() {
        let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="2" name="Chart"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="1828800" cy="914400"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
            <chart r:id="rIdChart"/>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:pic>
        <p:nvPicPr>
          <p:cNvPr id="3" name="Picture"/>
          <p:cNvPicPr/>
          <p:nvPr/>
        </p:nvPicPr>
        <p:blipFill>
          <a:blip r:embed="rIdImage"/>
          <a:srcRect l="10000" t="20000" r="30000" b="40000"/>
        </p:blipFill>
        <p:spPr>
          <a:xfrm rot="5400000" flipH="1" flipV="true"/>
          <a:prstGeom prst="rect"/>
        </p:spPr>
      </p:pic>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

        let slide_rels = HashMap::from([
            ("rIdChart".to_string(), "../charts/chart1.xml".to_string()),
            ("rIdImage".to_string(), "../media/image1.png".to_string()),
        ]);
        let chart_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart">
  <c:chart>
    <c:plotArea>
      <c:layout/>
      <c:pieChart>
        <c:varyColors val="1"/>
        <c:ser>
          <c:idx val="0"/>
          <c:order val="0"/>
          <c:tx><c:v>Series</c:v></c:tx>
          <c:cat>
            <c:strLit><c:ptCount val="1"/><c:pt idx="0"><c:v>Only</c:v></c:pt></c:strLit>
          </c:cat>
          <c:val>
            <c:numLit><c:ptCount val="1"/><c:pt idx="0"><c:v>42</c:v></c:pt></c:numLit>
          </c:val>
        </c:ser>
      </c:pieChart>
    </c:plotArea>
  </c:chart>
</c:chartSpace>"#;
        let chart_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rIdSkip" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" Target="../embeddings/ignored.bin"/>
  <Relationship Id="rIdPreview" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/chart-preview.png"/>
</Relationships>"#;
        let mut archive = archive_with_entries(&[
            ("ppt/charts/chart1.xml", chart_xml.as_bytes()),
            ("ppt/charts/_rels/chart1.xml.rels", chart_rels.as_bytes()),
            ("ppt/media/chart-preview.png", b"preview".as_slice()),
            ("ppt/media/image1.png", b"image-bytes".as_slice()),
        ]);

        let slide = parse_slide(slide_xml, &slide_rels, &mut archive).expect("slide should parse");
        assert_eq!(slide.shapes.len(), 2);

        let chart = slide
            .shapes
            .iter()
            .rev()
            .find_map(|shape| match &shape.shape_type {
                ShapeType::Chart(chart) => Some(chart),
                _ => None,
            })
            .expect("chart shape");
        assert_eq!(chart.rel_id, "rIdChart");
        assert!(
            chart.direct_spec.is_some(),
            "expected parsed direct chart spec"
        );
        assert_eq!(chart.preview_image.as_deref(), Some(b"preview".as_slice()));
        assert_eq!(chart.preview_mime.as_deref(), Some("image/png"));

        let picture = slide
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape_type {
                ShapeType::Picture(pic) => Some((shape, pic)),
                _ => None,
            })
            .expect("picture shape");
        assert!((picture.0.rotation - 90.0).abs() < 1e-6);
        assert!(picture.0.flip_h);
        assert!(picture.0.flip_v);
        assert_eq!(picture.1.data, b"image-bytes");
        let crop = picture.1.crop.as_ref().expect("picture crop");
        assert!((crop.left - 0.1).abs() < 1e-6);
        assert!((crop.top - 0.2).abs() < 1e-6);
        assert!((crop.right - 0.3).abs() < 1e-6);
        assert!((crop.bottom - 0.4).abs() < 1e-6);
    }

    #[test]
    fn parse_slide_handles_background_blips_custom_geometry_and_start_tag_connectors() {
        let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:o="urn:schemas-microsoft-com:office:office"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:blipFill><a:blip r:embed="rIdBg"></a:blip></a:blipFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""></p:cNvPr><p:cNvGrpSpPr></p:cNvGrpSpPr><p:nvPr></p:nvPr></p:nvGrpSpPr>
      <p:grpSpPr></p:grpSpPr>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="2" name="OLE"></p:cNvPr><p:cNvGraphicFramePr></p:cNvGraphicFramePr><p:nvPr></p:nvPr></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"></a:off><a:ext cx="914400" cy="457200"></a:ext></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/presentationml/2006/oleObject">
            <o:OLEObject ProgID="Excel.Sheet.12"><o:Link></o:Link></o:OLEObject>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="3" name="Custom Shape"></p:cNvPr><p:cNvSpPr></p:cNvSpPr><p:nvPr></p:nvPr></p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="5400000" flipH="1" flipV="true"><a:off x="12700" y="25400"></a:off><a:ext cx="914400" cy="457200"></a:ext></a:xfrm>
          <a:gradFill>
            <a:gsLst>
              <a:gs pos="0"><a:prstClr val="orange"></a:prstClr></a:gs>
              <a:gs pos="100000"><a:sysClr lastClr="ABCDEF"></a:sysClr></a:gs>
            </a:gsLst>
            <a:path path="shape"></a:path>
          </a:gradFill>
          <a:ln cap="flat" cmpd="tri" algn="in">
            <a:prstDash val="sysDashDot"/>
            <a:sysClr val="windowText"></a:sysClr>
          </a:ln>
          <a:effectLst>
            <a:outerShdw blurRad="12700" dist="25400" dir="5400000">
              <a:prstClr val="orange"></a:prstClr>
              <a:alpha val="75000"></a:alpha>
            </a:outerShdw>
            <a:glow rad="6350">
              <a:sysClr lastClr="123456"></a:sysClr>
              <a:alpha val="60000"></a:alpha>
            </a:glow>
          </a:effectLst>
          <a:custGeom>
            <a:avLst><a:gd name="adj1" fmla="val 50000"></a:gd></a:avLst>
            <a:gdLst><a:gd name="x1" fmla="val 100000"></a:gd></a:gdLst>
            <a:ahLst>
              <a:ahXY gdRefX="adj1" minX="0" maxX="100000" gdRefY="adj1" minY="0" maxY="100000">
                <a:pos x="50000" y="50000"></a:pos>
              </a:ahXY>
            </a:ahLst>
            <a:cxnLst><a:cxn ang="0"><a:pos x="0" y="0"></a:pos></a:cxn></a:cxnLst>
            <a:rect l="0" t="0" r="100000" b="100000"></a:rect>
            <a:pathLst>
              <a:path w="100000" h="100000" fill="darkenLess">
                <a:moveTo><a:pt x="0" y="0"></a:pt></a:moveTo>
                <a:lnTo><a:pt x="100000" y="0"></a:pt></a:lnTo>
                <a:arcTo wR="50000" hR="50000" stAng="0" swAng="5400000"></a:arcTo>
                <a:close></a:close>
              </a:path>
            </a:pathLst>
          </a:custGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="b" anchorCtr="1" rot="1800000" vert="vert" lIns="45720" tIns="91440" rIns="137160" bIns="182880" wrap="none"></a:bodyPr>
          <a:spAutoFit></a:spAutoFit>
          <a:p>
            <a:pPr algn="r" rtl="1" lvl="1" indent="12700" marL="25400">
              <a:lnSpc><a:spcPct val="120000"></a:spcPct></a:lnSpc>
              <a:spcBef><a:spcPts val="1200"></a:spcPts></a:spcBef>
              <a:spcAft><a:spcPct val="25000"></a:spcPct></a:spcAft>
              <a:buClr><a:srgbClr val="334455"></a:srgbClr></a:buClr>
              <a:defRPr sz="2000" spc="100" baseline="30000" cap="all" u="dbl" strike="sngStrike" b="1" i="1"></a:defRPr>
            </a:pPr>
            <a:r>
              <a:rPr sz="1800">
                <a:highlight><a:prstClr val="orange"></a:prstClr></a:highlight>
                <a:effectLst><a:outerShdw blurRad="12700" dist="25400" dir="2700000"><a:sysClr val="windowText"></a:sysClr></a:outerShdw></a:effectLst>
                <a:hlinkClick r:id="rIdHyper"></a:hlinkClick>
              </a:rPr>
              <a:t>Rich Text</a:t>
            </a:r>
            <a:br></a:br>
          </a:p>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="4" name="No AutoFit"></p:cNvPr><p:cNvSpPr></p:cNvSpPr><p:nvPr></p:nvPr></p:nvSpPr>
        <p:spPr><a:xfrm><a:off x="0" y="0"></a:off><a:ext cx="457200" cy="457200"></a:ext></a:xfrm><a:prstGeom prst="rect"><a:avLst></a:avLst></a:prstGeom></p:spPr>
        <p:txBody><a:bodyPr></a:bodyPr><a:noAutofit></a:noAutofit><a:p><a:r><a:t>None</a:t></a:r></a:p></p:txBody>
      </p:sp>
      <p:pic>
        <p:nvPicPr><p:cNvPr id="5" name="Picture"></p:cNvPr><p:cNvPicPr></p:cNvPicPr><p:nvPr></p:nvPr></p:nvPicPr>
        <p:blipFill><a:blip r:embed="rIdPic"></a:blip></p:blipFill>
        <p:spPr><a:xfrm><a:off x="0" y="0"></a:off><a:ext cx="457200" cy="457200"></a:ext></a:xfrm></p:spPr>
      </p:pic>
      <p:cxnSp>
        <p:nvCxnSpPr><p:cNvPr id="6" name="Connector"></p:cNvPr><p:cNvCxnSpPr></p:cNvCxnSpPr><p:nvPr></p:nvPr></p:nvCxnSpPr>
        <p:spPr><a:xfrm><a:off x="0" y="0"></a:off><a:ext cx="0" cy="914400"></a:ext></a:xfrm><a:ln><a:headEnd type="triangle" w="lg" len="sm"></a:headEnd><a:tailEnd type="oval" w="sm" len="lg"></a:tailEnd></a:ln></p:spPr>
        <p:stCxn id="10" idx="1"></p:stCxn>
        <p:endCxn id="11" idx="2"></p:endCxn>
      </p:cxnSp>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

        let rels = HashMap::from([
            ("rIdBg".to_string(), "../media/background.png".to_string()),
            ("rIdPic".to_string(), "../media/picture.png".to_string()),
            ("rIdHyper".to_string(), "https://example.com".to_string()),
        ]);
        let mut archive = archive_with_entries(&[
            ("ppt/media/background.png", b"bg-data"),
            ("ppt/media/picture.png", b"pic-data"),
        ]);

        let slide = parse_slide(slide_xml, &rels, &mut archive).expect("slide should parse");

        assert!(matches!(
            &slide.background,
            Some(Fill::Image(fill)) if fill.rel_id == "rIdBg" && fill.data == b"bg-data"
        ));
        assert_eq!(slide.shapes.len(), 5);

        assert!(slide.shapes.iter().any(|shape| matches!(
            &shape.shape_type,
            ShapeType::Unsupported(data)
                if data.raw_xml.as_deref().is_some_and(|raw| raw.contains("OLEObject"))
        )));

        let custom_shape = slide
            .shapes
            .iter()
            .find(|shape| matches!(shape.shape_type, ShapeType::CustomGeom(_)))
            .expect("custom geometry shape");
        assert!(custom_shape.flip_h);
        assert!(custom_shape.flip_v);
        assert_eq!(
            std::mem::discriminant(&custom_shape.border.cap),
            std::mem::discriminant(&LineCap::Flat)
        );
        assert_eq!(
            std::mem::discriminant(&custom_shape.border.compound),
            std::mem::discriminant(&CompoundLine::Triple)
        );
        assert_eq!(
            std::mem::discriminant(&custom_shape.border.alignment),
            std::mem::discriminant(&LineAlignment::Inset)
        );
        let custom_text = custom_shape.text_body.as_ref().expect("custom text body");
        assert_eq!(
            std::mem::discriminant(&custom_text.auto_fit),
            std::mem::discriminant(&AutoFit::Shrink)
        );
        assert_eq!(
            std::mem::discriminant(&custom_text.vertical_align),
            std::mem::discriminant(&VerticalAlign::Bottom)
        );
        assert_eq!(custom_shape.vertical_text.as_deref(), Some("vert"));
        let custom_paragraph = &custom_text.paragraphs[0];
        assert_eq!(
            custom_paragraph.runs[0].hyperlink.as_deref(),
            Some("https://example.com")
        );
        assert!(custom_paragraph.runs[1].is_break);
        assert!(custom_paragraph.runs[0].style.highlight.is_some());
        assert!(custom_paragraph.runs[0].style.shadow.is_some());

        let no_autofit = slide
            .shapes
            .iter()
            .find(|shape| shape.name == "No AutoFit")
            .expect("no autofit shape");
        assert_eq!(
            no_autofit
                .text_body
                .as_ref()
                .map(|body| std::mem::discriminant(&body.auto_fit)),
            Some(std::mem::discriminant(&AutoFit::NoAutoFit))
        );

        let picture = slide
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape_type {
                ShapeType::Picture(pic) => Some(pic),
                _ => None,
            })
            .expect("picture shape");
        assert_eq!(picture.rel_id, "rIdPic");
        assert_eq!(picture.data, b"pic-data");

        let connector = slide
            .shapes
            .iter()
            .find(|shape| matches!(shape.shape_type, ShapeType::Custom(ref name) if name == "line"))
            .expect("connector shape");
        assert_eq!(
            connector
                .start_connection
                .as_ref()
                .map(|conn| conn.shape_id),
            Some(10)
        );
        assert_eq!(
            connector.end_connection.as_ref().map(|conn| conn.site_idx),
            Some(2)
        );
    }

    #[test]
    fn parse_slide_covers_table_start_tags_norm_autofit_and_effect_color_variants() {
        let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="FF0000"></a:srgbClr></a:gs>
            <a:gs pos="100000"><a:schemeClr val="accent1"></a:schemeClr></a:gs>
          </a:gsLst>
          <a:path path="rect"></a:path>
        </a:gradFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""></p:cNvPr><p:cNvGrpSpPr></p:cNvGrpSpPr><p:nvPr></p:nvPr></p:nvGrpSpPr>
      <p:grpSpPr></p:grpSpPr>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="2" name="Table"></p:cNvPr><p:cNvGraphicFramePr></p:cNvGraphicFramePr><p:nvPr></p:nvPr></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"></a:off><a:ext cx="1828800" cy="914400"></a:ext></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
            <a:tbl>
              <a:tblPr bandRow="true" bandCol="true" firstRow="true" lastRow="true" firstCol="true" lastCol="true"></a:tblPr>
              <a:tblGrid><a:gridCol w="914400"></a:gridCol></a:tblGrid>
              <a:tr h="457200">
                <a:tc gridSpan="1" rowSpan="1" vMerge="true">
                  <a:txBody>
                    <a:bodyPr></a:bodyPr>
                    <a:lstStyle></a:lstStyle>
                    <a:p>
                      <a:pPr algn="ctr" lvl="1" indent="91440" marL="45720">
                        <a:defRPr sz="2000" spc="100" baseline="10000" cap="small" u="dashLong" strike="dblStrike" b="true" i="true"></a:defRPr>
                        <a:lnSpc><a:spcPct val="125000"></a:spcPct></a:lnSpc>
                        <a:spcBef><a:spcPts val="1200"></a:spcPts></a:spcBef>
                        <a:spcAft><a:spcPts val="600"></a:spcPts></a:spcAft>
                        <a:buClr><a:schemeClr val="accent2"></a:schemeClr></a:buClr>
                      </a:pPr>
                      <a:r>
                        <a:rPr sz="1800"><a:hlinkClick r:id="rIdHyper"/></a:rPr>
                        <a:t>Cell</a:t>
                      </a:r>
                      <a:br></a:br>
                    </a:p>
                  </a:txBody>
                  <a:tcPr marL="91440" marR="137160" marT="45720" marB="22860" anchor="b">
                    <a:solidFill><a:srgbClr val="00FF00"></a:srgbClr></a:solidFill>
                    <a:lnL w="12700"><a:prstDash val="solid"></a:prstDash><a:srgbClr val="FF0000"></a:srgbClr></a:lnL>
                    <a:lnR w="12700"><a:prstDash val="dash"></a:prstDash><a:srgbClr val="0000FF"></a:srgbClr></a:lnR>
                    <a:lnT w="12700"><a:prstDash val="dot"></a:prstDash><a:srgbClr val="123456"></a:srgbClr></a:lnT>
                    <a:lnB w="12700"><a:prstDash val="lgDash"></a:prstDash><a:srgbClr val="654321"></a:srgbClr></a:lnB>
                  </a:tcPr>
                </a:tc>
              </a:tr>
            </a:tbl>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="3" name="Norm Autofit Shape"></p:cNvPr><p:cNvSpPr></p:cNvSpPr><p:nvPr></p:nvPr></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"></a:off><a:ext cx="914400" cy="457200"></a:ext></a:xfrm>
          <a:gradFill>
            <a:gsLst>
              <a:gs pos="0"><a:prstClr val="orange"></a:prstClr></a:gs>
              <a:gs pos="100000"><a:sysClr lastClr="112233"></a:sysClr></a:gs>
            </a:gsLst>
            <a:path path="shape"></a:path>
          </a:gradFill>
          <a:effectLst>
            <a:outerShdw blurRad="12700" dist="25400" dir="5400000"><a:schemeClr val="accent1"></a:schemeClr></a:outerShdw>
            <a:glow rad="6350"><a:sysClr val="windowText"></a:sysClr></a:glow>
          </a:effectLst>
          <a:custGeom>
            <a:pathLst>
              <a:path w="100000" h="100000" fill="lighten"><a:moveTo><a:pt x="0" y="0"></a:pt></a:moveTo></a:path>
              <a:path w="100000" h="100000" fill="darken"><a:moveTo><a:pt x="0" y="0"></a:pt></a:moveTo></a:path>
              <a:path w="100000" h="100000" fill="lightenLess"><a:moveTo><a:pt x="0" y="0"></a:pt></a:moveTo></a:path>
            </a:pathLst>
          </a:custGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr wrap="none"></a:bodyPr>
          <a:normAutofit fontScale="70000" lnSpcReduction="15000"></a:normAutofit>
          <a:p><a:r><a:t>Shape</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

        let rels = HashMap::from([(
            "rIdHyper".to_string(),
            "https://example.com/table-start".to_string(),
        )]);
        let mut archive = archive_with_entries(&[]);

        let slide = parse_slide(slide_xml, &rels, &mut archive).expect("slide parses");

        let fill = match slide.background.as_ref().expect("gradient background") {
            Fill::Gradient(fill) => fill,
            other => panic!("expected gradient background, got {other:?}"),
        };
        assert_eq!(fill.stops.len(), 2);
        assert_eq!(
            std::mem::discriminant(&fill.gradient_type),
            std::mem::discriminant(&GradientType::Rectangular)
        );

        let table = slide
            .shapes
            .iter()
            .rev()
            .find_map(|shape| match &shape.shape_type {
                ShapeType::Table(table) => Some(table),
                _ => None,
            })
            .expect("table shape");
        assert!(table.band_row && table.band_col && table.first_row && table.last_row);
        assert!(table.first_col && table.last_col);
        let cell = &table.rows[0].cells[0];
        assert!(cell.v_merge);
        assert_eq!(
            std::mem::discriminant(&cell.vertical_align),
            std::mem::discriminant(&VerticalAlign::Bottom)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_left.style),
            std::mem::discriminant(&BorderStyle::Solid)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_left.dash_style),
            std::mem::discriminant(&DashStyle::Solid)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_right.style),
            std::mem::discriminant(&BorderStyle::Dashed)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_right.dash_style),
            std::mem::discriminant(&DashStyle::Dash)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_top.style),
            std::mem::discriminant(&BorderStyle::Dotted)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_top.dash_style),
            std::mem::discriminant(&DashStyle::Dot)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_bottom.style),
            std::mem::discriminant(&BorderStyle::Dashed)
        );
        assert_eq!(
            std::mem::discriminant(&cell.border_bottom.dash_style),
            std::mem::discriminant(&DashStyle::LongDash)
        );

        let paragraph = &cell.text_body.as_ref().expect("table text body").paragraphs[0];
        assert_eq!(
            paragraph.runs.len(),
            2,
            "expected one run plus one line break"
        );
        assert_eq!(
            paragraph.runs[0].hyperlink.as_deref(),
            Some("https://example.com/table-start")
        );
        assert!(paragraph.runs[1].is_break);
        let def_rpr = paragraph.def_rpr.as_ref().expect("table start-tag defRPr");
        assert_eq!(def_rpr.font_size, Some(20.0));
        assert_eq!(def_rpr.letter_spacing, Some(1.0));
        assert_eq!(def_rpr.baseline, Some(10000));
        assert_eq!(def_rpr.bold, Some(true));
        assert_eq!(def_rpr.italic, Some(true));
        assert_eq!(
            def_rpr
                .capitalization
                .as_ref()
                .map(std::mem::discriminant::<TextCapitalization>),
            Some(std::mem::discriminant(&TextCapitalization::Small))
        );
        assert_eq!(
            def_rpr
                .underline
                .as_ref()
                .map(std::mem::discriminant::<UnderlineType>),
            Some(std::mem::discriminant(&UnderlineType::DashLong))
        );
        assert_eq!(
            def_rpr
                .strikethrough
                .as_ref()
                .map(std::mem::discriminant::<StrikethroughType>),
            Some(std::mem::discriminant(&StrikethroughType::Double))
        );

        let shape = slide
            .shapes
            .iter()
            .find(|shape| shape.name == "Norm Autofit Shape")
            .expect("norm autofit shape");
        let auto_fit = shape
            .text_body
            .as_ref()
            .map(|body| &body.auto_fit)
            .expect("normal autofit body");
        if let AutoFit::Normal {
            font_scale: Some(v),
            line_spacing_reduction: Some(lsr),
        } = auto_fit
        {
            assert!((*v - 0.7).abs() < 1e-6);
            assert!((*lsr - 0.15).abs() < 1e-6);
        } else {
            panic!("expected normal autofit");
        }
        let custom_geom = slide
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape_type {
                ShapeType::CustomGeom(geom) => Some(geom),
                _ => None,
            })
            .expect("custom geometry");
        assert_eq!(custom_geom.paths.len(), 3);
        assert_eq!(
            std::mem::discriminant(&custom_geom.paths[0].fill),
            std::mem::discriminant(&PathFill::Lighten)
        );
        assert_eq!(
            std::mem::discriminant(&custom_geom.paths[1].fill),
            std::mem::discriminant(&PathFill::Darken)
        );
        assert_eq!(
            std::mem::discriminant(&custom_geom.paths[2].fill),
            std::mem::discriminant(&PathFill::LightenLess)
        );
    }

    #[test]
    fn parse_slide_covers_empty_variant_shape_and_table_branches() {
        let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="2" name="Table"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="1828800" cy="914400"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
            <a:tbl>
              <a:tblPr bandRow="1" bandCol="true" firstRow="1" lastRow="true" firstCol="1" lastCol="true"></a:tblPr>
              <a:tblGrid>
                <a:gridCol w="914400"/>
                <a:gridCol w="457200"/>
              </a:tblGrid>
              <a:tr h="457200">
                <a:tc gridSpan="2" rowSpan="1" vMerge="1">
                  <a:txBody>
                    <a:bodyPr/>
                    <a:lstStyle/>
                    <a:p>
                      <a:pPr algn="ctr" lvl="1" indent="91440" marL="45720"/>
                      <a:defRPr sz="2000" spc="100" baseline="10000" cap="small" u="dashLong" strike="dblStrike" b="1" i="true"/>
                      <a:r>
                        <a:rPr sz="1800"/>
                        <a:t>Cell One</a:t>
                      </a:r>
                      <a:r>
                        <a:rPr sz="1800"><a:hlinkClick r:id="rIdCell"/></a:rPr>
                        <a:t>Cell Two</a:t>
                      </a:r>
                      <a:br/>
                    </a:p>
                  </a:txBody>
                  <a:tcPr marL="91440" marR="137160" marT="45720" marB="22860" anchor="ctr">
                    <a:solidFill><a:srgbClr val="00FF00"/></a:solidFill>
                    <a:lnL w="12700"><a:prstDash val="solid"/><a:srgbClr val="FF0000"/></a:lnL>
                    <a:lnR w="12700"><a:prstDash val="dash"/><a:srgbClr val="0000FF"/></a:lnR>
                    <a:lnT w="12700"><a:prstDash val="dot"/><a:srgbClr val="123456"/></a:lnT>
                    <a:lnB w="12700"><a:prstDash val="sysDash"/><a:srgbClr val="654321"/></a:lnB>
                  </a:tcPr>
                </a:tc>
              </a:tr>
            </a:tbl>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Shape Empty Variants"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="5400000" flipH="1"><a:off x="12700" y="25400"/><a:ext cx="914400" cy="457200"/></a:xfrm>
          <a:prstGeom prst="rect"/>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr" wrap="none"/>
          <a:normAutofit fontScale="80000" lnSpcReduction="10000"/>
          <a:p>
            <a:pPr algn="r" lvl="1" indent="12700" marL="25400"/>
            <a:defRPr sz="2400" spc="200" baseline="30000" cap="all" u="dbl" strike="sngStrike" b="true" i="1"/>
            <a:r>
              <a:rPr sz="1800"/>
              <a:t>Shape One</a:t>
            </a:r>
            <a:r>
              <a:rPr sz="1800"><a:hlinkClick r:id="rIdShape"/></a:rPr>
              <a:t>Shape Two</a:t>
            </a:r>
            <a:br/>
          </a:p>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="4" name="No Autofit"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="457200" cy="228600"/></a:xfrm><a:prstGeom prst="rect"/></p:spPr>
        <p:txBody><a:bodyPr/><a:noAutofit/><a:p><a:r><a:t>No Autofit</a:t></a:r></a:p></p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="5" name="Shrink Autofit"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="457200" cy="228600"/></a:xfrm><a:prstGeom prst="rect"/></p:spPr>
        <p:txBody><a:bodyPr/><a:spAutoFit/><a:p><a:r><a:t>Shrink Autofit</a:t></a:r></a:p></p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

        let rels = HashMap::from([
            (
                "rIdCell".to_string(),
                "https://example.com/cell".to_string(),
            ),
            (
                "rIdShape".to_string(),
                "https://example.com/shape".to_string(),
            ),
        ]);
        let mut archive = archive_with_entries(&[]);

        let slide = parse_slide(slide_xml, &rels, &mut archive).expect("slide parses");
        assert_eq!(slide.shapes.len(), 4);

        let table = slide
            .shapes
            .iter()
            .rev()
            .find_map(|shape| match &shape.shape_type {
                ShapeType::Table(table) => Some(table),
                _ => None,
            })
            .expect("table shape");
        assert!(table.band_row && table.band_col && table.first_row && table.last_row);
        assert!(table.first_col && table.last_col);
        let cell = &table.rows[0].cells[0];
        assert_eq!(cell.col_span, 2);
        assert_eq!(
            cell.text_body.as_ref().expect("cell text body").paragraphs[0]
                .runs
                .len(),
            3
        );
        assert_eq!(
            cell.text_body.as_ref().expect("cell text body").paragraphs[0].runs[1]
                .hyperlink
                .as_deref(),
            Some("https://example.com/cell")
        );

        let shape = slide
            .shapes
            .iter()
            .find(|shape| shape.name == "Shape Empty Variants")
            .expect("shape with empty variants");
        let paragraph = &shape
            .text_body
            .as_ref()
            .expect("shape text body")
            .paragraphs[0];
        assert_eq!(paragraph.runs.len(), 3);
        assert_eq!(
            paragraph.runs[1].hyperlink.as_deref(),
            Some("https://example.com/shape")
        );
        let auto_fit = shape
            .text_body
            .as_ref()
            .map(|body| &body.auto_fit)
            .expect("shape empty variants body");
        if let AutoFit::Normal {
            font_scale: Some(v),
            line_spacing_reduction: Some(lsr),
        } = auto_fit
        {
            assert!((*v - 0.8).abs() < 1e-6);
            assert!((*lsr - 0.1).abs() < 1e-6);
        } else {
            panic!("expected normal autofit");
        }

        let no_autofit = slide
            .shapes
            .iter()
            .find(|shape| shape.name == "No Autofit")
            .expect("no autofit shape");
        assert_eq!(
            no_autofit
                .text_body
                .as_ref()
                .map(|body| std::mem::discriminant(&body.auto_fit)),
            Some(std::mem::discriminant(&AutoFit::NoAutoFit))
        );

        let shrink_autofit = slide
            .shapes
            .iter()
            .find(|shape| shape.name == "Shrink Autofit")
            .expect("shrink autofit shape");
        assert_eq!(
            shrink_autofit
                .text_body
                .as_ref()
                .map(|body| std::mem::discriminant(&body.auto_fit)),
            Some(std::mem::discriminant(&AutoFit::Shrink))
        );
    }

    #[test]
    fn helper_edge_cases_cover_absent_builders_and_defaults() {
        assert_eq!(rels_path_for("chart1.xml"), "_rels/chart1.xml.rels");
        assert_eq!(
            hyperlink_rel_id(&bytes_start("a:hlinkClick", &[("id", "rIdPlain")])),
            None
        );

        let mut missing_style_ref = None;
        assign_style_ref_color("lnRef", "1", Color::rgb("112233"), &mut missing_style_ref);
        ensure_style_ref("fillRef", "1", &mut missing_style_ref);
        assign_style_ref_no_color("effectRef", "2", &mut missing_style_ref);

        let mut style_ref = Some(ShapeStyleRef::default());
        ensure_style_ref("fontRef", "minor", &mut style_ref);
        assign_style_ref_color("unknownRef", "9", Color::rgb("445566"), &mut style_ref);
        assign_style_ref_no_color("unknownRef", "9", &mut style_ref);
        let font_ref = style_ref
            .as_ref()
            .and_then(|style| style.font_ref.as_ref())
            .expect("font ref");
        assert_eq!(font_ref.idx, "minor");
        assert!(font_ref.color.is_none());

        let default_cell = TableCellBuilder::default().build();
        assert_eq!(default_cell.margin_left, TableCell::default().margin_left);

        let mut missing_cell = None;
        assign_tc_color(Color::rgb("778899"), &None, &mut missing_cell);
        let mut cell = Some(TableCellBuilder::default());
        assign_tc_color(Color::rgb("AABBCC"), &Some("lnDiag".to_string()), &mut cell);
        assert_eq!(
            std::mem::discriminant(&cell.expect("cell").fill),
            std::mem::discriminant(&Fill::None)
        );

        let mut missing_shape = None;
        store_shape_level_defaults(&mut missing_shape, 9, ParagraphDefaults::default());
        let mut shape = Some(ShapeBuilder::default());
        store_shape_level_defaults(&mut shape, 10, ParagraphDefaults::default());
        assert!(
            shape
                .as_ref()
                .and_then(|shape| shape.text_list_style.as_ref())
                .is_none()
        );

        let mut archive = archive_with_entries(&[]);
        let malformed = "<p:sld xmlns:p=\"p\"><p:cSld><";
        assert!(parse_slide(malformed, &HashMap::new(), &mut archive).is_err());
    }

    fn bytes_start<'a>(name: &'a str, attrs: &[(&'a str, &'a str)]) -> BytesStart<'a> {
        let mut start = BytesStart::new(name);
        for (key, value) in attrs {
            start.push_attribute((*key, *value));
        }
        start
    }

    fn archive_with_entries(entries: &[(&str, &[u8])]) -> ZipArchive<Cursor<Vec<u8>>> {
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        let options = SimpleFileOptions::default();
        for (path, data) in entries {
            zip.start_file(path, options).expect("start file");
            zip.write_all(data).expect("write file");
        }
        let cursor = zip.finish().expect("finish zip");
        ZipArchive::new(cursor).expect("open archive")
    }

    fn default_table_cell_builder() -> TableCellBuilder {
        TableCellBuilder {
            text_body: None,
            fill: Fill::None,
            border_left: Border::default(),
            border_right: Border::default(),
            border_top: Border::default(),
            border_bottom: Border::default(),
            col_span: 0,
            row_span: 0,
            v_merge: false,
            margin_left: 7.2,
            margin_right: 7.2,
            margin_top: 3.6,
            margin_bottom: 3.6,
            vertical_align: VerticalAlign::Top,
        }
    }
}
