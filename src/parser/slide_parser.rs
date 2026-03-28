use std::collections::HashMap;
use std::io::{Read, Seek};

use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

use crate::error::{PptxError, PptxResult};
use crate::model::*;
use super::xml_utils;

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
    let mut current_gs_pos: f64 = 0.0;

    // Paragraph spacing nesting state
    let mut in_ln_spc = false;
    let mut in_spc_bef = false;
    let mut in_spc_aft = false;

    // Shape style reference (<p:style>) state
    let mut in_p_style = false;
    let mut p_style_builder: Option<ShapeStyleRef> = None;
    let mut p_style_current_ref: Option<String> = None;
    let mut p_style_idx: Option<String> = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.push(local.clone());

                match local.as_str() {
                    // Shape start
                    "sp" | "pic" | "cxnSp" => {
                        current_shape = Some(ShapeBuilder::default());
                        if local == "pic" {
                            if let Some(sb) = &mut current_shape {
                                sb.is_picture = true;
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
                    // Line/border
                    "ln" if in_sp_pr => {
                        in_ln = true;
                        if let Some(sb) = &mut current_shape {
                            if let Some(w) = xml_utils::attr_str(e, "w") {
                                sb.border_width = Emu::from_str(&w).to_pt();
                            }
                        }
                    }
                    // Text body
                    "txBody" => {
                        if let Some(sb) = &mut current_shape {
                            sb.has_text_body = true;
                        }
                    }
                    // bodyPr — text area properties
                    "bodyPr" if current_shape.is_some() => {
                        parse_body_pr(e, &mut current_shape);
                    }
                    // Paragraph
                    "p" if current_shape.is_some() => {
                        current_paragraph = Some(ParagraphBuilder::default());
                    }
                    // Paragraph properties
                    "pPr" if current_paragraph.is_some() => {
                        parse_para_props(e, &mut current_paragraph);
                    }
                    // Paragraph spacing containers
                    "lnSpc" if current_paragraph.is_some() => {
                        in_ln_spc = true;
                    }
                    "spcBef" if current_paragraph.is_some() => {
                        in_spc_bef = true;
                    }
                    "spcAft" if current_paragraph.is_some() => {
                        in_spc_aft = true;
                    }
                    // Text run
                    "r" if current_paragraph.is_some() => {
                        current_run = Some(RunBuilder::default());
                    }
                    // Run properties
                    "rPr" if current_run.is_some() => {
                        in_r_pr = true;
                        parse_run_props(e, &mut current_run);
                    }
                    // Text content
                    "t" if current_run.is_some() => {
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
                    // Fill — solidFill (Start variant)
                    "solidFill" => {
                        // solidFill has child color elements
                    }
                    // Gradient fill
                    "gradFill" if in_sp_pr && !in_ln => {
                        in_grad_fill = true;
                        grad_stops.clear();
                        grad_angle = 0.0;
                    }
                    // Gradient stop
                    "gs" if in_grad_fill => {
                        current_gs_pos = xml_utils::attr_str(e, "pos")
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|v| v / 100_000.0)
                            .unwrap_or(0.0);
                    }
                    // Color element (Start — may have child modifiers)
                    "srgbClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            current_color = Some(Color::rgb(val));
                        }
                    }
                    "schemeClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            current_color = Some(Color::theme(val));
                        }
                    }
                    "prstClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            current_color = Some(Color::preset(val));
                        }
                    }
                    "sysClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            current_color = Some(Color::system(val));
                        } else if let Some(val) = xml_utils::attr_str(e, "lastClr") {
                            current_color = Some(Color::rgb(val));
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();

                match local.as_str() {
                    // Shape position/size
                    "off" => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.position.x = Emu::from_str(&xml_utils::attr_str(e, "x").unwrap_or_default());
                            sb.position.y = Emu::from_str(&xml_utils::attr_str(e, "y").unwrap_or_default());
                        }
                    }
                    "ext" => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.size.width = Emu::from_str(&xml_utils::attr_str(e, "cx").unwrap_or_default());
                            sb.size.height = Emu::from_str(&xml_utils::attr_str(e, "cy").unwrap_or_default());
                        }
                    }
                    // Preset geometry
                    "prstGeom" => {
                        if let Some(sb) = current_shape.as_mut() {
                            if let Some(prst) = xml_utils::attr_str(e, "prst") {
                                sb.preset_geometry = Some(prst);
                            }
                        }
                    }
                    // bodyPr (Empty variant)
                    "bodyPr" if current_shape.is_some() => {
                        parse_body_pr(e, &mut current_shape);
                    }
                    // Paragraph properties (Empty variant)
                    "pPr" if current_paragraph.is_some() => {
                        parse_para_props(e, &mut current_paragraph);
                    }
                    // Run properties (Empty variant)
                    "rPr" if current_run.is_some() => {
                        parse_run_props(e, &mut current_run);
                    }
                    // noFill
                    "noFill" => {
                        if in_ln {
                            if let Some(sb) = &mut current_shape {
                                sb.border_style = BorderStyle::None;
                            }
                        } else if in_sp_pr {
                            if let Some(sb) = &mut current_shape {
                                sb.fill = Fill::None;
                            }
                        }
                    }
                    // Line dash style
                    "prstDash" if in_ln => {
                        if let Some(sb) = &mut current_shape {
                            if let Some(val) = xml_utils::attr_str(e, "val") {
                                sb.border_style = match val.as_str() {
                                    "solid" => BorderStyle::Solid,
                                    "dash" | "lgDash" | "sysDash" => BorderStyle::Dashed,
                                    "dot" | "sysDot" | "lgDashDot" | "lgDashDotDot" => BorderStyle::Dotted,
                                    _ => BorderStyle::Solid,
                                };
                            }
                        }
                    }
                    // Gradient direction
                    "lin" if in_grad_fill => {
                        if let Some(ang) = xml_utils::attr_str(e, "ang") {
                            // OOXML angle: in 1/60000 degree units
                            grad_angle = ang.parse::<f64>().unwrap_or(0.0) / 60_000.0;
                        }
                    }
                    // Style ref elements (Empty variant -- self-closing with color child)
                    "lnRef" | "fillRef" | "effectRef" | "fontRef" if in_p_style => {
                        // Self-closing style ref with no child color
                        if let Some(idx_val) = xml_utils::attr_str(e, "idx") {
                            assign_style_ref_no_color(
                                &local,
                                &idx_val,
                                &mut p_style_builder,
                            );
                        }
                    }
                    // Color element (Empty — simple color without modifiers)
                    "srgbClr" => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            let color = Color::rgb(val);
                            if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr, in_ln, in_r_pr, in_grad_fill,
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
                            if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr, in_ln, in_r_pr, in_grad_fill,
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
                            if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr, in_ln, in_r_pr, in_grad_fill,
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
                            if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr, in_ln, in_r_pr, in_grad_fill,
                                    current_gs_pos,
                                    &mut current_shape,
                                    &mut current_run,
                                    &mut grad_stops,
                                );
                            }
                        }
                    }
                    // Color modifiers (Empty tags)
                    "tint" | "shade" | "alpha" | "lumMod" | "lumOff"
                    | "satMod" | "satOff" | "hueMod" | "hueOff"
                    | "comp" | "inv" | "gray" => {
                        if let Some(ref mut color) = current_color {
                            let val = xml_utils::attr_str(e, "val")
                                .and_then(|v| v.parse::<i32>().ok());
                            if let Some(modifier) = ColorModifier::from_ooxml(&local, val) {
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
                    // Image reference
                    "blip" => {
                        if let Some(sb) = current_shape.as_mut() {
                            for attr in e.attributes().flatten() {
                                let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                                if key.ends_with("embed") {
                                    sb.image_rel_id = Some(
                                        String::from_utf8_lossy(&attr.value).to_string(),
                                    );
                                }
                            }
                        }
                    }
                    // Font
                    "latin" => {
                        if let Some(rb) = current_run.as_mut() {
                            if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                                rb.font_latin = Some(typeface);
                            }
                        }
                    }
                    "ea" => {
                        if let Some(rb) = current_run.as_mut() {
                            if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                                rb.font_ea = Some(typeface);
                            }
                        }
                    }
                    // Spacing percentage (inside lnSpc/spcBef/spcAft)
                    "spcPct" if current_paragraph.is_some() => {
                        if let Some(val_str) = xml_utils::attr_str(e, "val") {
                            if let Ok(val) = val_str.parse::<f64>() {
                                let spacing = SpacingValue::Percent(val / 100_000.0);
                                if let Some(pb) = current_paragraph.as_mut() {
                                    if in_ln_spc {
                                        pb.line_spacing = Some(spacing);
                                    } else if in_spc_bef {
                                        pb.space_before = Some(spacing);
                                    } else if in_spc_aft {
                                        pb.space_after = Some(spacing);
                                    }
                                }
                            }
                        }
                    }
                    // Spacing points (inside lnSpc/spcBef/spcAft)
                    "spcPts" if current_paragraph.is_some() => {
                        if let Some(val_str) = xml_utils::attr_str(e, "val") {
                            if let Ok(val) = val_str.parse::<f64>() {
                                let spacing = SpacingValue::Points(val / 100.0);
                                if let Some(pb) = current_paragraph.as_mut() {
                                    if in_ln_spc {
                                        pb.line_spacing = Some(spacing);
                                    } else if in_spc_bef {
                                        pb.space_before = Some(spacing);
                                    } else if in_spc_aft {
                                        pb.space_after = Some(spacing);
                                    }
                                }
                            }
                        }
                    }
                    // Bullet size (percentage and points)
                    "buSzPct" if current_paragraph.is_some() => {
                        // Parsed for completeness; bullet sizing stored if needed
                    }
                    "buSzPts" if current_paragraph.is_some() => {
                        // Parsed for completeness; bullet sizing stored if needed
                    }
                    // Bullet
                    "buNone" => {
                        if let Some(pb) = current_paragraph.as_mut() {
                            pb.bullet = Some(Bullet::None);
                        }
                    }
                    "buChar" => {
                        if let Some(pb) = current_paragraph.as_mut() {
                            if let Some(ch) = xml_utils::attr_str(e, "char") {
                                pb.bullet = Some(Bullet::Char(ch));
                            }
                        }
                    }
                    "buAutoNum" => {
                        if let Some(pb) = current_paragraph.as_mut() {
                            let num_type = xml_utils::attr_str(e, "type")
                                .unwrap_or_else(|| "arabicPeriod".to_string());
                            pb.bullet = Some(Bullet::AutoNum(num_type));
                        }
                    }
                    _ => {}
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

                match local.as_str() {
                    "t" => in_text = false,
                    "rPr" => in_r_pr = false,
                    // End of paragraph spacing containers
                    "lnSpc" => in_ln_spc = false,
                    "spcBef" => in_spc_bef = false,
                    "spcAft" => in_spc_aft = false,
                    "r" => {
                        if let (Some(pb), Some(rb)) =
                            (&mut current_paragraph, current_run.take())
                        {
                            pb.runs.push(rb.build());
                        }
                    }
                    "p" if current_shape.is_some() => {
                        if let (Some(sb), Some(pb)) =
                            (&mut current_shape, current_paragraph.take())
                        {
                            sb.paragraphs.push(pb.build());
                        }
                    }
                    // End of color element — assign to target after applying modifiers
                    "srgbClr" | "schemeClr" | "prstClr" | "sysClr" => {
                        if let Some(color) = current_color.take() {
                            if in_p_style && p_style_current_ref.is_some() {
                                assign_style_ref_color(
                                    p_style_current_ref.as_deref().unwrap_or(""),
                                    p_style_idx.as_deref().unwrap_or("0"),
                                    color,
                                    &mut p_style_builder,
                                );
                            } else {
                                assign_color(
                                    color,
                                    &depth,
                                    in_sp_pr, in_ln, in_r_pr, in_grad_fill,
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
                                ensure_style_ref(
                                    &local,
                                    &idx_val,
                                    &mut p_style_builder,
                                );
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
                    // End of solidFill — assign fill based on context
                    "solidFill" => {
                        // Color already assigned in assign_color
                    }
                    // End of gradient stop
                    "gs" => {
                        // Color added to grad_stops in assign_color
                    }
                    // End of gradient fill
                    "gradFill" if in_grad_fill => {
                        in_grad_fill = false;
                        if let Some(sb) = &mut current_shape {
                            sb.fill = Fill::Gradient(GradientFill {
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
                        if let Some(sb) = current_shape.take() {
                            let mut shape = sb.build();
                            // Load image data
                            if let ShapeType::Picture(pic) = &mut shape.shape_type {
                                if let Some(target) = rels.get(&pic.rel_id) {
                                    let img_path = format!("ppt/slides/{target}");
                                    let alt_path = format!("ppt/{target}");
                                    let path = if archive.by_name(&img_path).is_ok() {
                                        img_path
                                    } else {
                                        alt_path
                                    };
                                    if let Ok(mut entry) = archive.by_name(&path) {
                                        let mut buf = Vec::new();
                                        let _ = entry.read_to_end(&mut buf);
                                        pic.data = buf;
                                    }
                                }
                            }
                            slide.shapes.push(shape);
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

    // Run text color (solidFill in rPr, or directly inside rPr)
    if in_r_pr || depth_contains(depth, "rPr") {
        if let Some(rb) = run.as_mut() {
            rb.color = color;
        }
        return;
    }

    // Other cases inside spPr (no solidFill ancestor)
    if in_sp_pr {
        if let Some(sb) = shape.as_mut() {
            sb.fill = Fill::Solid(SolidFill { color });
        }
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
fn ensure_style_ref(
    ref_kind: &str,
    idx_str: &str,
    builder: &mut Option<ShapeStyleRef>,
) {
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
fn assign_style_ref_no_color(
    ref_kind: &str,
    idx_str: &str,
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

/// Parse bodyPr attributes
fn parse_body_pr(
    e: &quick_xml::events::BytesStart<'_>,
    shape: &mut Option<ShapeBuilder>,
) {
    if let Some(sb) = shape.as_mut() {
        // Vertical alignment
        if let Some(anchor) = xml_utils::attr_str(e, "anchor") {
            sb.text_vertical_align = VerticalAlign::from_ooxml(&anchor);
        }
        // Inner margins (EMU → pt)
        if let Some(v) = xml_utils::attr_str(e, "lIns") {
            sb.text_margins.left = Emu::from_str(&v).to_pt();
        }
        if let Some(v) = xml_utils::attr_str(e, "tIns") {
            sb.text_margins.top = Emu::from_str(&v).to_pt();
        }
        if let Some(v) = xml_utils::attr_str(e, "rIns") {
            sb.text_margins.right = Emu::from_str(&v).to_pt();
        }
        if let Some(v) = xml_utils::attr_str(e, "bIns") {
            sb.text_margins.bottom = Emu::from_str(&v).to_pt();
        }
        // Word wrap
        if let Some(wrap) = xml_utils::attr_str(e, "wrap") {
            sb.text_word_wrap = wrap != "none";
        }
    }
}

/// Parse pPr (paragraph properties)
fn parse_para_props(
    e: &quick_xml::events::BytesStart<'_>,
    para: &mut Option<ParagraphBuilder>,
) {
    if let Some(pb) = para.as_mut() {
        if let Some(algn) = xml_utils::attr_str(e, "algn") {
            pb.alignment = Alignment::from_ooxml(&algn);
        }
        if let Some(lvl) = xml_utils::attr_str(e, "lvl") {
            pb.level = lvl.parse::<u32>().unwrap_or(0);
        }
        if let Some(indent) = xml_utils::attr_str(e, "indent") {
            pb.indent = Some(Emu::from_str(&indent).to_pt());
        }
        if let Some(mar_l) = xml_utils::attr_str(e, "marL") {
            pb.margin_left = Some(Emu::from_str(&mar_l).to_pt());
        }
    }
}

/// Parse rPr (run properties)
fn parse_run_props(
    e: &quick_xml::events::BytesStart<'_>,
    run: &mut Option<RunBuilder>,
) {
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
            rb.underline = u != "none";
        }
        if let Some(strike) = xml_utils::attr_str(e, "strike") {
            rb.strikethrough = strike != "noStrike";
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

// ── Builder pattern ──

#[derive(Default)]
struct ShapeBuilder {
    position: Position,
    size: Size,
    paragraphs: Vec<TextParagraph>,
    has_text_body: bool,
    is_picture: bool,
    image_rel_id: Option<String>,
    preset_geometry: Option<String>,
    // Fill/Border
    fill: Fill,
    border_width: f64,
    border_color: Color,
    border_style: BorderStyle,
    // bodyPr
    text_vertical_align: VerticalAlign,
    text_margins: TextMargins,
    text_word_wrap: bool,
    text_auto_fit: AutoFit,
    // Placeholder and style reference (parsed as None for now)
    placeholder: Option<PlaceholderInfo>,
    style_ref: Option<ShapeStyleRef>,
}

impl ShapeBuilder {
    fn build(self) -> Shape {
        let shape_type = if self.is_picture {
            ShapeType::Picture(PictureData {
                rel_id: self.image_rel_id.unwrap_or_default(),
                ..Default::default()
            })
        } else if let Some(ref prst) = self.preset_geometry {
            match prst.as_str() {
                "rect" => ShapeType::Rectangle,
                "roundRect" => ShapeType::RoundedRectangle,
                "ellipse" => ShapeType::Ellipse,
                "triangle" | "rtTriangle" => ShapeType::Triangle,
                other => ShapeType::Custom(other.to_string()),
            }
        } else {
            ShapeType::TextBox
        };

        let text_body = if self.has_text_body {
            Some(TextBody {
                paragraphs: self.paragraphs,
                vertical_align: self.text_vertical_align,
                word_wrap: self.text_word_wrap,
                auto_fit: self.text_auto_fit,
                margins: self.text_margins,
            })
        } else {
            None
        };

        let border = Border {
            width: self.border_width,
            color: self.border_color,
            style: if self.border_width > 0.0 && matches!(self.border_style, BorderStyle::None) {
                BorderStyle::Solid
            } else {
                self.border_style
            },
        };

        Shape {
            position: self.position,
            size: self.size,
            shape_type,
            text_body,
            fill: self.fill,
            border,
            placeholder: self.placeholder,
            style_ref: self.style_ref,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct ParagraphBuilder {
    runs: Vec<TextRun>,
    alignment: Alignment,
    level: u32,
    indent: Option<f64>,
    margin_left: Option<f64>,
    bullet: Option<Bullet>,
    line_spacing: Option<SpacingValue>,
    space_before: Option<SpacingValue>,
    space_after: Option<SpacingValue>,
}

impl ParagraphBuilder {
    fn build(self) -> TextParagraph {
        TextParagraph {
            runs: self.runs,
            alignment: self.alignment,
            line_spacing: self.line_spacing,
            space_before: self.space_before,
            space_after: self.space_after,
            indent: self.indent,
            bullet: self.bullet,
            level: self.level,
        }
    }
}

#[derive(Default)]
struct RunBuilder {
    text: String,
    font_size: Option<f64>,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    color: Color,
    font_latin: Option<String>,
    font_ea: Option<String>,
    baseline: Option<i32>,
    letter_spacing: Option<f64>,
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
                color: self.color,
                baseline: self.baseline,
                letter_spacing: self.letter_spacing,
                ..Default::default()
            },
            font: FontStyle {
                latin: self.font_latin,
                east_asian: self.font_ea,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
