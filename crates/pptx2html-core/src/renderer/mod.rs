//! HTML/CSS renderer
//! Presentation model -> self-contained HTML string generation

mod geometry;

use std::collections::HashMap;
use std::fmt::Write;

use base64::Engine;

use crate::ConversionOptions;
use crate::ConversionResult;
use crate::error::PptxResult;
use crate::model::presentation::{ClrMap, ColorScheme};
use crate::model::*;
use crate::resolver::inheritance;
use crate::resolver::placeholder;
use crate::resolver::style_ref;

use std::cell::RefCell;

/// Mutable state for collecting unresolved elements during rendering
struct UnresolvedCollector {
    elements: Vec<UnresolvedElement>,
    counter: usize,
    current_slide_index: usize,
    gradient_counter: usize,
    marker_counter: usize,
}

/// Rendering context -- propagates theme/ClrMap references and full presentation
struct RenderCtx<'a> {
    pres: &'a Presentation,
    scheme: Option<&'a ColorScheme>,
    clr_map: Option<&'a ClrMap>,
    embed_images: bool,
    collector: &'a RefCell<UnresolvedCollector>,
}

impl<'a> RenderCtx<'a> {
    fn resolve_color(&self, color: &Color) -> Option<ResolvedColor> {
        color.resolve(self.scheme, self.clr_map)
    }

    fn color_to_css(&self, color: &Color) -> Option<String> {
        self.resolve_color(color)
            .map(|c| c.to_css())
            .or_else(|| color.to_css())
    }

    fn next_gradient_id(&self) -> String {
        let mut coll = self.collector.borrow_mut();
        let id = coll.gradient_counter;
        coll.gradient_counter += 1;
        format!("grad{id}")
    }

    fn next_marker_id(&self, suffix: &str) -> String {
        let mut coll = self.collector.borrow_mut();
        let id = coll.marker_counter;
        coll.marker_counter += 1;
        format!("marker-{suffix}-{id}")
    }

    /// Create a slide-scoped context with resolved ClrMap and per-master theme
    fn for_slide(
        &self,
        slide_clr_map: Option<&'a ClrMap>,
        master_theme_idx: Option<usize>,
    ) -> RenderCtx<'a> {
        let scheme = master_theme_idx
            .and_then(|idx| self.pres.themes.get(idx))
            .map(|t| &t.color_scheme)
            .or(self.scheme);
        RenderCtx {
            pres: self.pres,
            scheme,
            clr_map: slide_clr_map.or(self.clr_map),
            embed_images: self.embed_images,
            collector: self.collector,
        }
    }
}

pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Render entire Presentation to HTML
    pub fn render(pres: &Presentation) -> PptxResult<String> {
        Self::render_with_options(pres, &ConversionOptions::default())
    }

    /// Render entire Presentation to HTML with conversion options
    pub fn render_with_options(
        pres: &Presentation,
        opts: &ConversionOptions,
    ) -> PptxResult<String> {
        Ok(Self::render_with_options_metadata(pres, opts)?.html)
    }

    /// Render entire Presentation to HTML with metadata about unresolved elements
    pub fn render_with_options_metadata(
        pres: &Presentation,
        opts: &ConversionOptions,
    ) -> PptxResult<ConversionResult> {
        let slide_w = pres.slide_size.width.to_px();
        let slide_h = pres.slide_size.height.to_px();

        let collector = RefCell::new(UnresolvedCollector {
            elements: Vec::new(),
            counter: 0,
            current_slide_index: 0,
            gradient_counter: 0,
            marker_counter: 0,
        });

        let ctx = RenderCtx {
            pres,
            scheme: pres.primary_theme().map(|t| &t.color_scheme),
            clr_map: if pres.clr_map.is_empty() {
                None
            } else {
                Some(&pres.clr_map)
            },
            embed_images: opts.embed_images,
            collector: &collector,
        };

        let mut html = String::with_capacity(4096);

        html.push_str("<!DOCTYPE html>\n<html lang=\"ko\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("<meta name=\"generator\" content=\"pptx2html-turbo\">\n");
        if let Some(ref title) = pres.title {
            let _ = writeln!(html, "<title>{}</title>", escape_html(title));
        } else {
            html.push_str("<title>Presentation</title>\n");
        }
        html.push_str("<style>\n");
        html.push_str(&Self::global_css(slide_w, slide_h));
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("<div class=\"pptx-container\">\n");

        let mut slide_count = 0;
        for (i, slide) in pres.slides.iter().enumerate() {
            let one_based = i + 1;
            if !opts.should_include_slide(one_based, slide.hidden) {
                continue;
            }
            collector.borrow_mut().current_slide_index = i;
            Self::render_slide(slide, one_based, slide_w, slide_h, &ctx, &mut html);
            slide_count += 1;
        }

        html.push_str("</div>\n</body>\n</html>");
        let coll = collector.into_inner();
        Ok(ConversionResult {
            html,
            unresolved_elements: coll.elements,
            slide_count,
        })
    }

    fn global_css(slide_w: f64, slide_h: f64) -> String {
        format!(
            r#"* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{ background: #f0f0f0; font-family: 'Calibri', 'Malgun Gothic', sans-serif; }}
.pptx-container {{ display: flex; flex-direction: column; align-items: center; gap: 20px; padding: 20px; }}
.slide {{
  position: relative;
  width: {slide_w:.1}px;
  height: {slide_h:.1}px;
  background: #fff;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0,0,0,0.15);
}}
.shape {{
  position: absolute;
  overflow: visible;
}}
.text-body {{
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow-wrap: break-word;
  word-wrap: break-word;
}}
.text-body.v-top {{ justify-content: flex-start; }}
.text-body.v-middle {{ justify-content: center; }}
.text-body.v-bottom {{ justify-content: flex-end; }}
.paragraph {{ margin: 0; }}
.run {{ white-space: pre-wrap; word-break: break-word; overflow-wrap: break-word; }}
img.shape-image {{ width: 100%; height: 100%; object-fit: cover; display: block; }}
.shape-svg {{ position: absolute; top: 0; left: 0; width: 100%; height: 100%; }}
.shape-svg + .text-body {{ position: relative; z-index: 1; }}
.chart-placeholder {{ display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; background: #f8f8f8; border: 1px dashed #ccc; color: #888; font-size: 14px; }}
.unresolved-element {{ display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; background: #f8f8f8; border: 1px dashed #ccc; color: #888; font-size: 14px; }}
"#
        )
    }

    fn render_slide(
        slide: &Slide,
        num: usize,
        _slide_w: f64,
        _slide_h: f64,
        ctx: &RenderCtx<'_>,
        html: &mut String,
    ) {
        // Look up layout and master for this slide
        let layout = slide.layout_idx.and_then(|idx| ctx.pres.layouts.get(idx));
        let master = layout
            .map(|l| l.master_idx)
            .and_then(|idx| ctx.pres.masters.get(idx));

        // Resolve ClrMap per slide (considering overrides) and per-master theme
        let slide_ctx = if let Some(m) = master {
            let resolved_cm = inheritance::resolve_clr_map(slide, layout, m);
            ctx.for_slide(
                if resolved_cm.is_empty() {
                    None
                } else {
                    Some(resolved_cm)
                },
                Some(m.theme_idx),
            )
        } else {
            ctx.for_slide(None, None)
        };

        // Resolve background via inheritance
        let bg = inheritance::resolve_background(slide, layout, master);
        let bg_style = Self::fill_to_css(&bg, &slide_ctx);
        let _ = writeln!(
            html,
            "<div class=\"slide\" data-slide=\"{num}\" style=\"{bg_style}\">"
        );

        // Render master shapes if show_master_sp is true.
        // Only non-placeholder master shapes (decorative elements) are rendered
        // directly. Placeholder shapes from the master are property-inheritance
        // sources only -- they must never appear as standalone HTML elements.
        let show_master = slide.show_master_sp && layout.is_none_or(|l| l.show_master_sp);
        if show_master && let Some(m) = master {
            for master_shape in &m.shapes {
                if master_shape.hidden {
                    continue;
                }
                // Skip ALL placeholder shapes -- they are property templates,
                // not renderable content.  Slide shapes inherit from them via
                // the layout/master cascade; rendering them here produces
                // duplicate shapes with template text (e.g. "Click to edit Master title style").
                if master_shape.placeholder.is_some() {
                    continue;
                }
                Self::render_shape_resolved(master_shape, None, None, &slide_ctx, html);
            }
        }

        // Render slide shapes with inheritance
        for shape in &slide.shapes {
            if shape.hidden {
                continue;
            }
            // Find matching placeholder in layout/master
            let layout_match = shape.placeholder.as_ref().and_then(|ph| {
                layout.and_then(|l| placeholder::find_matching_placeholder(ph, &l.shapes))
            });
            let master_match = shape.placeholder.as_ref().and_then(|ph| {
                master.and_then(|m| placeholder::find_matching_placeholder(ph, &m.shapes))
            });

            Self::render_shape_resolved(shape, layout_match, master_match, &slide_ctx, html);
        }

        html.push_str("</div>\n");
    }

    /// Render shape with resolved properties from inheritance cascade
    fn render_shape_resolved(
        shape: &Shape,
        layout_match: Option<&Shape>,
        master_match: Option<&Shape>,
        ctx: &RenderCtx<'_>,
        html: &mut String,
    ) {
        // Resolve position/size via inheritance
        let (pos, size) = inheritance::resolve_position(shape, layout_match, master_match);
        let x = pos.x.to_px();
        let y = pos.y.to_px();
        let w = size.width.to_px();
        let h = size.height.to_px();

        let mut style_buf = String::with_capacity(256);
        let _ = write!(
            style_buf,
            "left: {x:.1}px; top: {y:.1}px; width: {w:.1}px; height: {h:.1}px"
        );

        // Determine SVG preset name early so we know whether to skip CSS fill/border
        let svg_preset_name = match &shape.shape_type {
            ShapeType::Ellipse => Some("ellipse"),
            ShapeType::RoundedRectangle => Some("roundRect"),
            ShapeType::Triangle => Some("triangle"),
            ShapeType::Custom(name) => Some(name.as_str()),
            _ => None,
        };

        // For connector shapes with rotation, swap width/height in the CSS box
        // and handle flip via SVG path transform instead of CSS transform.
        // OOXML connectors use rotation to reorient the path (e.g., 270° to make
        // a horizontal connector into a vertical one) — this is a layout hint,
        // not a visual rotation.
        let is_connector = svg_preset_name.is_some_and(|pn| matches!(pn,
            "line" | "lineInv" | "straightConnector1"
                | "bentConnector2" | "bentConnector3" | "bentConnector4" | "bentConnector5"
                | "curvedConnector2" | "curvedConnector3" | "curvedConnector4" | "curvedConnector5"
        ));
        let connector_needs_swap = is_connector && (shape.rotation - 90.0).abs() < 1.0
            || is_connector && (shape.rotation - 270.0).abs() < 1.0;

        let (w, h) = if connector_needs_swap { (h, w) } else { (w, h) };
        if connector_needs_swap {
            // Rewrite CSS position with swapped dimensions, adjusting offset
            // so the center of the bounding box stays in the same place
            let dx = (size.width.to_px() - size.height.to_px()) / 2.0;
            let dy = (size.height.to_px() - size.width.to_px()) / 2.0;
            style_buf.clear();
            let _ = write!(
                style_buf,
                "left: {:.1}px; top: {:.1}px; width: {w:.1}px; height: {h:.1}px",
                x + dx, y + dy
            );
        }

        // Build transform: flip + rotation (skip for connectors with swap)
        if !connector_needs_swap && (shape.rotation != 0.0 || shape.flip_h || shape.flip_v) {
            let sx = if shape.flip_h { -1 } else { 1 };
            let sy = if shape.flip_v { -1 } else { 1 };
            if shape.flip_h || shape.flip_v {
                if shape.rotation != 0.0 {
                    let _ = write!(
                        style_buf,
                        "; transform: scale({sx},{sy}) rotate({:.1}deg)",
                        shape.rotation
                    );
                } else {
                    let _ = write!(style_buf, "; transform: scale({sx},{sy})");
                }
            } else {
                let _ = write!(style_buf, "; transform: rotate({:.1}deg)", shape.rotation);
            }
        }

        // Line shapes with zero width or height need a minimum CSS dimension
        // so the shape div is visible (otherwise browser collapses it)
        if let Some(pn) = svg_preset_name {
            let is_line = matches!(
                pn,
                "line" | "lineInv" | "straightConnector1"
                    | "bentConnector2" | "bentConnector3" | "bentConnector4" | "bentConnector5"
                    | "curvedConnector2" | "curvedConnector3" | "curvedConnector4" | "curvedConnector5"
            );
            if is_line {
                if w < 0.5 {
                    // Vertical line: give minimum width for stroke visibility
                    style_buf.clear();
                    let _ = write!(
                        style_buf,
                        "left: {:.1}px; top: {y:.1}px; width: 2px; height: {h:.1}px",
                        x - 1.0
                    );
                } else if h < 0.5 {
                    // Horizontal line: give minimum height for stroke visibility
                    style_buf.clear();
                    let _ = write!(
                        style_buf,
                        "left: {x:.1}px; top: {:.1}px; width: {w:.1}px; height: 2px",
                        y - 1.0
                    );
                }
            }
        }
        let uses_svg =
            svg_preset_name.is_some() || matches!(shape.shape_type, ShapeType::CustomGeom(_));

        // Resolve fill via inheritance (with style_ref fallback)
        let fmt_scheme = ctx.pres.primary_theme().map(|t| &t.fmt_scheme);
        let resolved_fill = inheritance::resolve_shape_fill_with_theme(
            shape,
            layout_match,
            master_match,
            fmt_scheme,
            ctx.scheme,
            ctx.clr_map,
        );
        // Only emit CSS background for non-SVG shapes; SVG shapes use the fill attribute
        // on the <path> element directly, so CSS background would leak outside the shape path
        if !uses_svg {
            Self::fill_to_css_buf(&resolved_fill, ctx, &mut style_buf);
        }

        // Resolve border via inheritance (with style_ref fallback)
        let resolved_border = inheritance::resolve_border_with_theme(
            shape,
            layout_match,
            master_match,
            fmt_scheme,
            ctx.scheme,
            ctx.clr_map,
        );

        // Only apply CSS outline for non-SVG shapes; SVG shapes use stroke instead.
        // Use outline instead of border to avoid box-sizing: border-box shrinking
        // the content area (text insets should not compete with border thickness).
        if resolved_border.width > 0.0 && !uses_svg {
            let border_color = ctx
                .color_to_css(&resolved_border.color)
                .unwrap_or_else(|| "#000".to_string());
            let border_style = match resolved_border.style {
                BorderStyle::Solid => "solid",
                BorderStyle::Dashed => "dashed",
                BorderStyle::Dotted => "dotted",
                BorderStyle::None => "none",
            };
            let _ = write!(
                style_buf,
                "; outline: {:.1}pt {border_style} {border_color}; outline-offset: {:.1}pt",
                resolved_border.width,
                -(resolved_border.width / 2.0)
            );
        }

        // Shape-level effects → CSS box-shadow
        // Use explicit effects if present; otherwise fall back to effectRef from theme
        {
            let resolved_effects = if shape.effects.outer_shadow.is_none()
                && shape.effects.glow.is_none()
            {
                if let (Some(sr), Some(fmt), Some(cs), Some(cm)) =
                    (&shape.style_ref, fmt_scheme, ctx.scheme, ctx.clr_map)
                    && let Some(effect_ref) = &sr.effect_ref
                {
                    style_ref::resolve_effect_ref(effect_ref, fmt, cs, cm)
                } else {
                    None
                }
            } else {
                None
            };
            let effective_effects = resolved_effects.as_ref().unwrap_or(&shape.effects);

            let mut shadows: Vec<String> = Vec::new();

            // outerShdw → box-shadow with offset
            if let Some(ref shadow) = effective_effects.outer_shadow {
                let angle_rad = shadow.direction.to_radians();
                let offset_x = shadow.distance * angle_rad.cos();
                let offset_y = shadow.distance * angle_rad.sin();
                let blur = shadow.blur_radius;
                let color = ctx
                    .color_to_css(&shadow.color)
                    .unwrap_or_else(|| "rgba(0,0,0,0.4)".to_string());
                shadows.push(format!(
                    "{offset_x:.1}pt {offset_y:.1}pt {blur:.1}pt {color}"
                ));
            }

            // glow → box-shadow with spread, no offset
            if let Some(ref glow) = effective_effects.glow {
                let spread = glow.radius;
                let color = ctx
                    .color_to_css(&glow.color)
                    .unwrap_or_else(|| "rgba(255,215,0,0.5)".to_string());
                shadows.push(format!("0 0 {spread:.1}pt {spread:.1}pt {color}"));
            }

            if !shadows.is_empty() {
                let _ = write!(style_buf, "; box-shadow: {}", shadows.join(", "));
            }
        }

        // Cropped images need overflow:hidden on the shape container
        if let ShapeType::Picture(pic) = &shape.shape_type
            && pic.crop.is_some()
        {
            style_buf.push_str("; overflow: hidden");
        }

        let _ = writeln!(html, "<div class=\"shape\" style=\"{style_buf}\">");

        // Table
        if let ShapeType::Table(ref table) = shape.shape_type {
            Self::render_table(table, ctx, html);
            html.push_str("</div>\n");
            return;
        }

        // Group
        if let ShapeType::Group(ref children, ref group_data) = shape.shape_type {
            Self::render_group(children, shape, group_data, ctx, html);
            html.push_str("</div>\n");
            return;
        }

        // Unsupported content (SmartArt, OLE, Math)
        if let ShapeType::Unsupported(ref data) = shape.shape_type {
            let mut coll = ctx.collector.borrow_mut();
            let placeholder_id =
                format!("unresolved-s{}-e{}", coll.current_slide_index, coll.counter);
            coll.counter += 1;

            let type_attr = match data.element_type {
                UnresolvedType::SmartArt => "smartart",
                UnresolvedType::OleObject => "ole",
                UnresolvedType::MathEquation => "math",
                UnresolvedType::CustomGeometry => "custom-geometry",
            };

            let escaped = escape_html(&data.label);
            let _ = writeln!(
                html,
                "<div class=\"unresolved-element\" id=\"{placeholder_id}\" \
                 data-type=\"{type_attr}\" data-slide=\"{}\">\
                 <span>[{escaped}]</span></div>",
                coll.current_slide_index
            );

            let pos_non_zero = pos.x.0 != 0 || pos.y.0 != 0;
            let size_non_zero = size.width.0 != 0 || size.height.0 != 0;
            let slide_idx = coll.current_slide_index;
            let elem = UnresolvedElement {
                slide_index: slide_idx,
                element_type: data.element_type.clone(),
                placeholder_id,
                position: if pos_non_zero { Some(pos) } else { None },
                size: if size_non_zero { Some(size) } else { None },
                raw_xml: data.raw_xml.clone(),
                data_model: None,
            };
            coll.elements.push(elem);

            drop(coll);
            html.push_str("</div>\n");
            return;
        }

        // Chart
        if let ShapeType::Chart(ref chart_data) = shape.shape_type {
            if let Some(ref img_data) = chart_data.preview_image {
                if !img_data.is_empty() {
                    let mime = chart_data.preview_mime.as_deref().unwrap_or("image/png");
                    let src = if ctx.embed_images {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(img_data);
                        format!("data:{mime};base64,{b64}")
                    } else {
                        format!("images/chart-{}.png", img_data.len() % 100000)
                    };
                    let _ = writeln!(
                        html,
                        "<img class=\"shape-image\" src=\"{src}\" alt=\"Chart\">"
                    );
                }
            } else {
                html.push_str(
                    "<div class=\"chart-placeholder\">\
                     <svg width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\">\
                     <rect x=\"3\" y=\"12\" width=\"4\" height=\"9\"/><rect x=\"10\" y=\"7\" width=\"4\" height=\"14\"/>\
                     <rect x=\"17\" y=\"3\" width=\"4\" height=\"18\"/></svg>\
                     <span style=\"margin-left:8px\">Chart</span></div>\n"
                );
            }
            html.push_str("</div>\n");
            return;
        }

        // SVG preset shape rendering
        if let Some(preset_name) = svg_preset_name {
            let empty_adj: HashMap<String, f64> = HashMap::new();
            let adj_values = shape.adjust_values.as_ref().unwrap_or(&empty_adj);
            // Connector/line shapes need a default visible stroke
            let is_line_shape = matches!(
                preset_name,
                "line"
                    | "lineInv"
                    | "straightConnector1"
                    | "bentConnector2"
                    | "bentConnector3"
                    | "bentConnector4"
                    | "bentConnector5"
                    | "curvedConnector2"
                    | "curvedConnector3"
                    | "curvedConnector4"
                    | "curvedConnector5"
            );
            // For line shapes with zero dimension, use a fixed viewBox and custom path
            let svg_w = if is_line_shape && w < 0.5 { 2.0 } else { w };
            let svg_h = if is_line_shape && h < 0.5 { 2.0 } else { h };
            // Generate path: for zero-dim lines, create centered line path directly
            let line_svg_override = if is_line_shape && (w < 0.5 || h < 0.5) {
                if w < 0.5 {
                    Some(format!("M1.0,0 L1.0,{svg_h:.1}"))
                } else {
                    Some(format!("M0,1.0 L{svg_w:.1},1.0"))
                }
            } else if connector_needs_swap {
                // Connectors with 90°/270° rotation need rotated path variants.
                // After dimension swap (w↔h), the original path direction is wrong.
                // Generate the correct path based on rotation + flip.
                let flip_h = shape.flip_h;
                let flip_v = shape.flip_v;
                match preset_name {
                    "line" | "lineInv" | "straightConnector1" => {
                        // Straight line: always diagonal or centered, rotation doesn't change path
                        None
                    }
                    "bentConnector2" => {
                        // Original: RIGHT→DOWN. After 270° rotation:
                        // +flipH → DOWN→RIGHT; +flipV → UP→LEFT; no flip → DOWN→LEFT
                        let path = if flip_h {
                            format!("M0,0 L0,{h:.1} L{w:.1},{h:.1}", w = svg_w, h = svg_h)
                        } else if flip_v {
                            format!("M{w:.1},{h:.1} L{w:.1},0 L0,0", w = svg_w, h = svg_h)
                        } else {
                            format!("M{w:.1},0 L{w:.1},{h:.1} L0,{h:.1}", w = svg_w, h = svg_h)
                        };
                        Some(path)
                    }
                    "bentConnector3" => {
                        // Original: RIGHT→DOWN→RIGHT with adj midpoint.
                        // After 270° rotation + flipH → DOWN→RIGHT→DOWN
                        let adj1 = adj_values.get("adj1").copied().unwrap_or(50000.0);
                        let mid = svg_h * adj1 / 100_000.0;
                        let path = if flip_h {
                            format!("M0,0 L0,{mid:.1} L{w:.1},{mid:.1} L{w:.1},{h:.1}",
                                w = svg_w, mid = mid, h = svg_h)
                        } else {
                            format!("M{w:.1},0 L{w:.1},{mid:.1} L0,{mid:.1} L0,{h:.1}",
                                w = svg_w, mid = mid, h = svg_h)
                        };
                        Some(path)
                    }
                    _ => None, // Other connectors: fall through to default path + transform
                }
            } else {
                None
            };
            let has_override = line_svg_override.is_some();
            let svg_path_opt = line_svg_override.or_else(|| geometry::preset_shape_svg(preset_name, svg_w, svg_h, adj_values));
            if let Some(svg_path) = svg_path_opt {
                // Convert border width from pt to px for SVG (viewBox is in px)
                let (stroke_color, stroke_width) = if resolved_border.width > 0.0 {
                    let c = ctx
                        .color_to_css(&resolved_border.color)
                        .unwrap_or_else(|| "#000".to_string());
                    (c, resolved_border.width * 4.0 / 3.0)
                } else if is_line_shape {
                    // Default 0.75pt stroke for connectors with no explicit line;
                    // still respect parsed color if available
                    let c = ctx
                        .color_to_css(&resolved_border.color)
                        .unwrap_or_else(|| "#000".to_string());
                    (c, 1.0) // 0.75pt = 1.0px
                } else {
                    ("none".to_string(), 0.0)
                };
                let dash_attr = dash_style_to_svg(&resolved_border.dash_style, stroke_width);
                let cap_attr = line_cap_to_svg(&resolved_border.cap);
                let join_attr = line_join_to_svg(&resolved_border.join);
                let _ = write!(
                    html,
                    "<svg viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\" class=\"shape-svg\" preserveAspectRatio=\"none\">"
                );
                // Build <defs> for gradient and/or marker definitions
                let grad_id = ctx.next_gradient_id();
                let mut defs_buf = String::new();
                let gradient_fill_ref =
                    svg_gradient_def(&resolved_fill, &grad_id, ctx, &mut defs_buf);
                // Emit marker defs for line endings (arrows) with unique IDs
                let mut marker_start_attr = String::new();
                let mut marker_end_attr = String::new();
                if resolved_border.head_end.is_some() || resolved_border.tail_end.is_some() {
                    if let Some(ref he) = resolved_border.head_end {
                        let mid = ctx.next_marker_id("head");
                        emit_marker_def(&mut defs_buf, &mid, he, &stroke_color, stroke_width, true);
                        marker_start_attr = format!(" marker-start=\"url(#{mid})\"");
                    }
                    if let Some(ref te) = resolved_border.tail_end {
                        let mid = ctx.next_marker_id("tail");
                        emit_marker_def(&mut defs_buf, &mid, te, &stroke_color, stroke_width, false);
                        marker_end_attr = format!(" marker-end=\"url(#{mid})\"");
                    }
                }
                if !defs_buf.is_empty() {
                    html.push_str("<defs>");
                    html.push_str(&defs_buf);
                    html.push_str("</defs>");
                }
                // Determine fill attribute: gradient url > solid color > none
                let fill_attr = if is_line_shape {
                    "none".to_string()
                } else if let Some(ref grad_ref) = gradient_fill_ref {
                    grad_ref.clone()
                } else {
                    ctx.color_to_css(&resolved_fill.color_ref())
                        .unwrap_or_else(|| "none".to_string())
                };
                // Shapes with holes (donut, frame, etc.) need evenodd fill rule
                let fill_rule_attr = if geometry::needs_evenodd_fill(preset_name) {
                    " fill-rule=\"evenodd\""
                } else {
                    ""
                };
                // For connectors with swapped dimensions where the path was NOT
                // directly generated (fallback case), apply flip via SVG transform
                let svg_transform = if connector_needs_swap
                    && !has_override
                    && (shape.flip_h || shape.flip_v)
                {
                    let sx = if shape.flip_h { -1.0 } else { 1.0 };
                    let sy = if shape.flip_v { -1.0 } else { 1.0 };
                    let tx = if shape.flip_h { svg_w } else { 0.0 };
                    let ty = if shape.flip_v { svg_h } else { 0.0 };
                    format!(" transform=\"translate({tx:.1},{ty:.1}) scale({sx},{sy})\"")
                } else {
                    String::new()
                };
                // non-scaling-stroke prevents stroke distortion when viewBox
                // and CSS dimensions have different aspect ratios.
                // Ensure minimum 1.5px for visibility at screen resolution.
                let (non_scaling, stroke_width) = if is_line_shape {
                    (" vector-effect=\"non-scaling-stroke\"", stroke_width.max(1.5))
                } else {
                    ("", stroke_width)
                };
                let _ = writeln!(
                    html,
                    "<path d=\"{svg_path}\" fill=\"{fill_attr}\"{fill_rule_attr} \
                     stroke=\"{stroke_color}\" stroke-width=\"{stroke_width:.1}\"\
                     {non_scaling}{dash_attr}{cap_attr}{join_attr}{marker_start_attr}{marker_end_attr}{svg_transform}/>\
                     </svg>"
                );
            }
        }

        // Custom geometry SVG rendering
        if let ShapeType::CustomGeom(ref geom) = shape.shape_type
            && let Some(svg_geom) = geometry::custom_geometry_svg(geom, w, h)
        {
            // Convert border width from pt to px for SVG (viewBox is in px)
            let (stroke_color, stroke_width) = if resolved_border.width > 0.0 {
                let c = ctx
                    .color_to_css(&resolved_border.color)
                    .unwrap_or_else(|| "#000".to_string());
                (c, resolved_border.width * 4.0 / 3.0)
            } else {
                ("none".to_string(), 0.0)
            };
            let dash_attr = dash_style_to_svg(&resolved_border.dash_style, stroke_width);
            let cap_attr = line_cap_to_svg(&resolved_border.cap);
            let join_attr = line_join_to_svg(&resolved_border.join);
            let _ = write!(
                html,
                "<svg viewBox=\"0 0 {w:.1} {h:.1}\" class=\"shape-svg\" preserveAspectRatio=\"none\">"
            );
            // Gradient fill support for custom geometry
            let grad_id = ctx.next_gradient_id();
            let mut defs_buf = String::new();
            let gradient_fill_ref = svg_gradient_def(&resolved_fill, &grad_id, ctx, &mut defs_buf);
            // Emit marker defs for custom geometry arrows
            let mut marker_start_attr = String::new();
            let mut marker_end_attr = String::new();
            if resolved_border.head_end.is_some() || resolved_border.tail_end.is_some() {
                if let Some(ref he) = resolved_border.head_end {
                    let mid = ctx.next_marker_id("head");
                    emit_marker_def(&mut defs_buf, &mid, he, &stroke_color, stroke_width, true);
                    marker_start_attr = format!(" marker-start=\"url(#{mid})\"");
                }
                if let Some(ref te) = resolved_border.tail_end {
                    let mid = ctx.next_marker_id("tail");
                    emit_marker_def(&mut defs_buf, &mid, te, &stroke_color, stroke_width, false);
                    marker_end_attr = format!(" marker-end=\"url(#{mid})\"");
                }
            }
            if !defs_buf.is_empty() {
                html.push_str("<defs>");
                html.push_str(&defs_buf);
                html.push_str("</defs>");
            }
            let default_fill = if let Some(ref grad_ref) = gradient_fill_ref {
                grad_ref.clone()
            } else {
                ctx.color_to_css(&resolved_fill.color_ref())
                    .unwrap_or_else(|| "none".to_string())
            };
            for path_svg in &svg_geom.paths {
                let fill = match path_svg.fill {
                    PathFill::None => "none".to_string(),
                    _ => default_fill.clone(),
                };
                let _ = write!(
                    html,
                    "<path d=\"{}\" fill=\"{fill}\" stroke=\"{stroke_color}\" stroke-width=\"{stroke_width:.1}\"\
                     {dash_attr}{cap_attr}{join_attr}{marker_start_attr}{marker_end_attr}/>",
                    path_svg.d
                );
            }
            html.push_str("</svg>\n");
        }

        // Image
        if let ShapeType::Picture(pic) = &shape.shape_type
            && !pic.data.is_empty()
        {
            let mime = if pic.content_type.is_empty() {
                "image/png"
            } else {
                &pic.content_type
            };
            let src = if ctx.embed_images {
                let b64 = base64::engine::general_purpose::STANDARD.encode(&pic.data);
                format!("data:{mime};base64,{b64}")
            } else {
                let ext = match mime {
                    "image/jpeg" => "jpg",
                    "image/gif" => "gif",
                    "image/svg+xml" => "svg",
                    "image/webp" => "webp",
                    _ => "png",
                };
                format!("images/image-{}.{ext}", pic.data.len() % 100000)
            };
            if let Some(ref crop) = pic.crop {
                // OOXML srcRect: l/t/r/b are fractions (0..1) to crop from each
                // edge of the SOURCE image.  The shape bounding box is the final
                // visible area.  We scale the <img> beyond 100% so the full source
                // fills more than the shape, then shift it so the crop region's
                // top-left aligns with the shape origin.  overflow:hidden on the
                // parent div clips the excess.
                let l = crop.left * 100.0; // left crop %
                let t = crop.top * 100.0; // top crop %
                let r = crop.right * 100.0; // right crop %
                let b = crop.bottom * 100.0; // bottom crop %
                let vis_w = 100.0 - l - r;
                let vis_h = 100.0 - t - b;
                if vis_w > 0.001 && vis_h > 0.001 {
                    let img_w_pct = 100.0 / vis_w * 100.0;
                    let img_h_pct = 100.0 / vis_h * 100.0;
                    // Use absolute px offsets for positioning (margin-%
                    // in CSS is always relative to container width, even
                    // for vertical — that gives wrong results).
                    let off_x_px = -(l / 100.0) * w * (img_w_pct / 100.0);
                    let off_y_px = -(t / 100.0) * h * (img_h_pct / 100.0);
                    let _ = writeln!(
                        html,
                        "<img class=\"shape-image\" src=\"{src}\" alt=\"\" style=\"\
                         object-fit: fill; \
                         width: {img_w_pct:.2}%; height: {img_h_pct:.2}%; \
                         margin-left: {off_x_px:.2}px; margin-top: {off_y_px:.2}px\">"
                    );
                } else {
                    // Degenerate crop — show the whole image
                    let _ = writeln!(html, "<img class=\"shape-image\" src=\"{src}\" alt=\"\">");
                }
            } else {
                let _ = writeln!(html, "<img class=\"shape-image\" src=\"{src}\" alt=\"\">");
            }
        }

        // Resolve text style source for this shape's placeholder type
        let text_style_ctx = Self::build_text_style_ctx(shape, ctx);

        // Resolve fontRef from <p:style> for font-family and color fallback
        let (font_ref_font, font_ref_color) = Self::resolve_font_ref_font(shape, ctx)
            .map(|(f, c)| (Some(f), c))
            .unwrap_or((None, None));

        // Text
        if let Some(ref text_body) = shape.text_body {
            let v_class = match text_body.vertical_align {
                VerticalAlign::Top => "v-top",
                VerticalAlign::Middle => "v-middle",
                VerticalAlign::Bottom => "v-bottom",
            };
            let mut tb_style = String::with_capacity(128);
            let _ = write!(
                tb_style,
                "padding: {:.1}pt {:.1}pt {:.1}pt {:.1}pt",
                text_body.margins.top,
                text_body.margins.right,
                text_body.margins.bottom,
                text_body.margins.left,
            );
            // Text wrapping control
            if !text_body.word_wrap {
                tb_style.push_str("; white-space: nowrap");
            }
            // Vertical text rendering
            let mut has_vert270 = false;
            if let Some(ref vert) = shape.vertical_text {
                match vert.as_str() {
                    "vert" | "wordArtVert" | "eaVert" => {
                        tb_style.push_str("; writing-mode: vertical-rl");
                    }
                    "vert270" => {
                        tb_style.push_str("; writing-mode: vertical-lr");
                        has_vert270 = true;
                    }
                    "mongolianVert" => {
                        tb_style.push_str("; writing-mode: vertical-lr");
                    }
                    _ => {}
                }
            }
            // Extract auto-fit scaling factors
            let (font_scale, ln_spc_reduction) = match &text_body.auto_fit {
                AutoFit::Normal {
                    font_scale,
                    line_spacing_reduction,
                } => (*font_scale, *line_spacing_reduction),
                _ => (None, None),
            };
            // Add overflow:hidden when text is auto-fitted with fontScale
            if font_scale.is_some() {
                tb_style.push_str("; overflow: hidden");
            }
            // Build combined transform for text-body: vert270 rotate + flip counter-scale
            // PowerPoint flips the shape geometry but keeps text left-to-right,
            // so we counter-flip the text container.
            if has_vert270 || shape.flip_h || shape.flip_v {
                let mut transforms = Vec::new();
                if shape.flip_h || shape.flip_v {
                    let tx = if shape.flip_h { -1 } else { 1 };
                    let ty = if shape.flip_v { -1 } else { 1 };
                    transforms.push(format!("scale({tx},{ty})"));
                }
                if has_vert270 {
                    transforms.push("rotate(180deg)".to_string());
                }
                let _ = write!(tb_style, "; transform: {}", transforms.join(" "));
            }
            let _ = writeln!(
                html,
                "<div class=\"text-body {v_class}\" style=\"{tb_style}\">"
            );
            // Track auto-number counters per level for this text body
            let mut auto_num_counters: [i32; 9] = [0; 9];
            for para in &text_body.paragraphs {
                Self::render_paragraph_with_defaults(
                    para,
                    ctx,
                    &mut auto_num_counters,
                    &text_style_ctx,
                    font_ref_font.as_deref(),
                    font_ref_color.as_ref(),
                    font_scale,
                    ln_spc_reduction,
                    html,
                );
            }
            html.push_str("</div>\n");
        }

        html.push_str("</div>\n");
    }

    /// Build text style context from placeholder type and master txStyles / defaultTextStyle
    fn build_text_style_ctx<'a>(shape: &Shape, ctx: &RenderCtx<'a>) -> TextStyleCtx<'a> {
        // Determine which txStyles list to use based on placeholder type
        let ph_type = shape
            .placeholder
            .as_ref()
            .and_then(|ph| ph.ph_type.as_ref());
        let source = placeholder::text_style_source(ph_type);

        // Find the master's txStyles list for this source
        let layout = shape.placeholder.as_ref().and({
            // We don't have layout_idx on shape, get from slide context
            None::<&SlideLayout>
        });
        let _ = layout; // Layout-level lstStyle is not yet tracked

        // txStyles from first master
        let master_list_style = ctx.pres.masters.first().and_then(|m| match source {
            placeholder::TextStyleSource::TitleStyle => m.tx_styles.title_style.as_ref(),
            placeholder::TextStyleSource::BodyStyle => m.tx_styles.body_style.as_ref(),
            placeholder::TextStyleSource::OtherStyle => m.tx_styles.other_style.as_ref(),
        });

        // defaultTextStyle from presentation
        let default_list_style = ctx.pres.default_text_style.as_ref();

        TextStyleCtx {
            master_list_style,
            default_list_style,
        }
    }

    /// Resolve fontRef from shape's <p:style> to a font-family name and optional color
    fn resolve_font_ref_font(
        shape: &Shape,
        ctx: &RenderCtx<'_>,
    ) -> Option<(String, Option<ResolvedColor>)> {
        let sr = shape.style_ref.as_ref()?;
        let font_ref = sr.font_ref.as_ref()?;
        let theme = ctx.pres.primary_theme()?;
        let font_scheme = &theme.font_scheme;
        let scheme = ctx.scheme?;
        let clr_map = ctx.clr_map?;
        style_ref::resolve_font_ref(font_ref, font_scheme, scheme, clr_map)
    }

    fn render_table(table: &TableData, ctx: &RenderCtx<'_>, html: &mut String) {
        let total_width: f64 = table.col_widths.iter().sum();
        html.push_str(
            "<table style=\"width:100%; height:100%; border-collapse:collapse; table-layout:fixed;\">\n<colgroup>\n",
        );
        for w in &table.col_widths {
            let pct = if total_width > 0.0 {
                w / total_width * 100.0
            } else {
                0.0
            };
            let _ = writeln!(html, "<col style=\"width:{pct:.1}%\"/>");
        }
        html.push_str("</colgroup>\n");

        for row in &table.rows {
            let _ = writeln!(html, "<tr style=\"height:{:.1}px;\">", row.height);
            for cell in &row.cells {
                // Skip cells that are continuation of a vertical merge
                if cell.v_merge {
                    continue;
                }

                let mut td_style = String::with_capacity(128);

                // Cell fill
                Self::fill_to_css_buf(&cell.fill, ctx, &mut td_style);

                // Cell borders
                if cell.border_left.width > 0.0 {
                    let color = ctx
                        .color_to_css(&cell.border_left.color)
                        .unwrap_or_else(|| "#000".to_string());
                    push_sep(&mut td_style);
                    let _ = write!(
                        td_style,
                        "border-left: {:.1}pt {} {}",
                        cell.border_left.width,
                        dash_style_to_css(&cell.border_left.dash_style),
                        color
                    );
                }
                if cell.border_right.width > 0.0 {
                    let color = ctx
                        .color_to_css(&cell.border_right.color)
                        .unwrap_or_else(|| "#000".to_string());
                    push_sep(&mut td_style);
                    let _ = write!(
                        td_style,
                        "border-right: {:.1}pt {} {}",
                        cell.border_right.width,
                        dash_style_to_css(&cell.border_right.dash_style),
                        color
                    );
                }
                if cell.border_top.width > 0.0 {
                    let color = ctx
                        .color_to_css(&cell.border_top.color)
                        .unwrap_or_else(|| "#000".to_string());
                    push_sep(&mut td_style);
                    let _ = write!(
                        td_style,
                        "border-top: {:.1}pt {} {}",
                        cell.border_top.width,
                        dash_style_to_css(&cell.border_top.dash_style),
                        color
                    );
                }
                if cell.border_bottom.width > 0.0 {
                    let color = ctx
                        .color_to_css(&cell.border_bottom.color)
                        .unwrap_or_else(|| "#000".to_string());
                    push_sep(&mut td_style);
                    let _ = write!(
                        td_style,
                        "border-bottom: {:.1}pt {} {}",
                        cell.border_bottom.width,
                        dash_style_to_css(&cell.border_bottom.dash_style),
                        color
                    );
                }

                // Cell margins and vertical alignment
                let va = match cell.vertical_align {
                    VerticalAlign::Top => "top",
                    VerticalAlign::Middle => "middle",
                    VerticalAlign::Bottom => "bottom",
                };
                push_sep(&mut td_style);
                let _ = write!(
                    td_style,
                    "padding: {:.1}pt {:.1}pt {:.1}pt {:.1}pt; vertical-align: {}",
                    cell.margin_top, cell.margin_right, cell.margin_bottom, cell.margin_left, va
                );

                let _ = write!(html, "<td");
                if cell.col_span > 1 {
                    let _ = write!(html, " colspan=\"{}\"", cell.col_span);
                }
                if cell.row_span > 1 {
                    let _ = write!(html, " rowspan=\"{}\"", cell.row_span);
                }
                let _ = writeln!(html, " style=\"{td_style}\">");
                if let Some(ref tb) = cell.text_body {
                    let mut auto_num_counters: [i32; 9] = [0; 9];
                    for para in &tb.paragraphs {
                        Self::render_paragraph(para, ctx, &mut auto_num_counters, html);
                    }
                }
                html.push_str("</td>\n");
            }
            html.push_str("</tr>\n");
        }
        html.push_str("</table>\n");
    }

    fn render_group(
        children: &[Shape],
        parent: &Shape,
        group_data: &GroupData,
        ctx: &RenderCtx<'_>,
        html: &mut String,
    ) {
        // Group coordinate transform:
        // Child coords are in child coordinate space (chOff/chExt).
        // We need to map them to the group's actual bounding box.
        let (parent_pos, parent_size) =
            crate::resolver::inheritance::resolve_position(parent, None, None);
        let ch_off_x = group_data.child_offset.x.to_px();
        let ch_off_y = group_data.child_offset.y.to_px();
        let ch_ext_w = group_data.child_extent.width.to_px();
        let ch_ext_h = group_data.child_extent.height.to_px();
        let grp_w = parent_size.width.to_px();
        let grp_h = parent_size.height.to_px();

        for child in children {
            if child.hidden {
                continue;
            }
            // Transform child position from child coordinate space to group-relative pixels
            let child_x = child.position.x.to_px();
            let child_y = child.position.y.to_px();
            let child_w = child.size.width.to_px();
            let child_h = child.size.height.to_px();

            let (rel_x, rel_y, rel_w, rel_h) = if ch_ext_w > 0.0 && ch_ext_h > 0.0 {
                let scale_x = grp_w / ch_ext_w;
                let scale_y = grp_h / ch_ext_h;
                (
                    (child_x - ch_off_x) * scale_x,
                    (child_y - ch_off_y) * scale_y,
                    child_w * scale_x,
                    child_h * scale_y,
                )
            } else {
                // Fallback: use child coords relative to parent position
                (
                    child_x - parent_pos.x.to_px(),
                    child_y - parent_pos.y.to_px(),
                    child_w,
                    child_h,
                )
            };

            // Create a modified child shape with group-relative coordinates
            let mut child_clone = child.clone();
            child_clone.position = Position {
                x: Emu((rel_x / 96.0 * 914400.0) as i64),
                y: Emu((rel_y / 96.0 * 914400.0) as i64),
            };
            child_clone.size = Size {
                width: Emu((rel_w / 96.0 * 914400.0) as i64),
                height: Emu((rel_h / 96.0 * 914400.0) as i64),
            };
            Self::render_shape_resolved(&child_clone, None, None, ctx, html);
        }
    }

    fn render_paragraph(
        para: &TextParagraph,
        ctx: &RenderCtx<'_>,
        auto_num_counters: &mut [i32; 9],
        html: &mut String,
    ) {
        Self::render_paragraph_with_defaults(
            para,
            ctx,
            auto_num_counters,
            &TextStyleCtx::default(),
            None,
            None,
            None,
            None,
            html,
        );
    }

    /// Render paragraph with inherited text style defaults from txStyles / defaultTextStyle
    #[allow(clippy::too_many_arguments)]
    fn render_paragraph_with_defaults(
        para: &TextParagraph,
        ctx: &RenderCtx<'_>,
        auto_num_counters: &mut [i32; 9],
        text_ctx: &TextStyleCtx<'_>,
        font_ref_font: Option<&str>,
        font_ref_color: Option<&ResolvedColor>,
        font_scale: Option<f64>,
        ln_spc_reduction: Option<f64>,
        html: &mut String,
    ) {
        let level = (para.level as usize).min(8);

        // Look up inherited paragraph defaults for this level
        let inherited = text_ctx.get_level_defaults(level);

        let align = para.alignment.to_css();
        let mut para_style = String::with_capacity(128);
        let _ = write!(para_style, "text-align: {align}");

        // Line spacing (explicit > inherited), with optional reduction from normAutofit
        let line_spacing = para
            .line_spacing
            .as_ref()
            .or_else(|| inherited.and_then(|d| d.line_spacing.as_ref()));
        let reduction_factor = ln_spc_reduction.map(|r| 1.0 - r).unwrap_or(1.0);
        if let Some(ls) = line_spacing {
            match ls {
                SpacingValue::Percent(p) => {
                    let effective = p * reduction_factor;
                    let _ = write!(para_style, "; line-height: {effective:.2}");
                }
                SpacingValue::Points(pt) => {
                    let effective = pt * reduction_factor;
                    let _ = write!(para_style, "; line-height: {effective:.1}pt");
                }
            }
        } else if ln_spc_reduction.is_some() {
            // Apply reduction to default line-height (1.0 = 100%)
            let effective = reduction_factor;
            let _ = write!(para_style, "; line-height: {effective:.2}");
        }
        // Space before (explicit > inherited)
        let space_before = para
            .space_before
            .as_ref()
            .or_else(|| inherited.and_then(|d| d.space_before.as_ref()));
        if let Some(sb) = space_before {
            match sb {
                SpacingValue::Percent(p) => {
                    let _ = write!(para_style, "; margin-top: {p:.1}em");
                }
                SpacingValue::Points(pt) => {
                    let _ = write!(para_style, "; margin-top: {pt:.1}pt");
                }
            }
        }
        // Space after (explicit > inherited)
        let space_after = para
            .space_after
            .as_ref()
            .or_else(|| inherited.and_then(|d| d.space_after.as_ref()));
        if let Some(sa) = space_after {
            match sa {
                SpacingValue::Percent(p) => {
                    let _ = write!(para_style, "; margin-bottom: {p:.1}em");
                }
                SpacingValue::Points(pt) => {
                    let _ = write!(para_style, "; margin-bottom: {pt:.1}pt");
                }
            }
        }

        // Level-based indentation via margin_left and indent (explicit > inherited)
        let margin_left = para
            .margin_left
            .or_else(|| inherited.and_then(|d| d.margin_left));
        let indent = para.indent.or_else(|| inherited.and_then(|d| d.indent));

        if let Some(ml) = margin_left {
            let _ = write!(para_style, "; padding-left: {ml:.1}pt");
        } else if para.level > 0 {
            // Fallback: ~36pt (0.5in) per level when no explicit margin
            let margin = para.level as f64 * 36.0;
            let _ = write!(para_style, "; padding-left: {margin:.1}pt");
        }
        if let Some(ind) = indent {
            let _ = write!(para_style, "; text-indent: {ind:.1}pt");
        }

        let _ = write!(html, "<p class=\"paragraph\" style=\"{para_style}\">");

        // Skip bullet for empty paragraphs (no visible text content)
        let has_visible_text = para
            .runs
            .iter()
            .any(|r| !r.is_break && !r.text.trim().is_empty());

        // Bullet rendering (explicit > inherited)
        let bullet = if has_visible_text {
            para.bullet
                .as_ref()
                .or_else(|| inherited.and_then(|d| d.bullet.as_ref()))
        } else {
            None
        };
        if let Some(bullet) = bullet {
            match bullet {
                Bullet::Char(bc) => {
                    // Reset counters at deeper levels when a char bullet is encountered
                    for counter in auto_num_counters.iter_mut().skip(level) {
                        *counter = 0;
                    }
                    let mut bullet_style = String::new();
                    if let Some(ref font) = bc.font {
                        let _ = write!(bullet_style, "font-family: '{}'; ", escape_html(font));
                    }
                    if let Some(ref color) = bc.color
                        && let Some(css) = ctx.color_to_css(color)
                    {
                        let _ = write!(bullet_style, "color: {}; ", css);
                    }
                    if let Some(size_pct) = bc.size_pct {
                        if size_pct < 0.0 {
                            // Absolute points (stored as negative)
                            let pts = -size_pct;
                            let _ = write!(bullet_style, "font-size: {pts:.1}pt; ");
                        } else if (size_pct - 1.0).abs() > 0.01 {
                            // Percentage of text size (only if not 100%)
                            let pct = size_pct * 100.0;
                            let _ = write!(bullet_style, "font-size: {pct:.0}%; ");
                        }
                    }
                    let _ = write!(
                        html,
                        "<span class=\"bullet\" style=\"{bullet_style}\">{} </span>",
                        escape_html(&bc.char)
                    );
                }
                Bullet::AutoNum(an) => {
                    // Increment counter for this level
                    let start = an.start_at.unwrap_or(1);
                    auto_num_counters[level] += 1;
                    // Reset deeper level counters
                    for counter in auto_num_counters.iter_mut().skip(level + 1) {
                        *counter = 0;
                    }
                    let counter_val = start + auto_num_counters[level] - 1;

                    let label = format_auto_num(&an.num_type, counter_val);
                    let mut bullet_style = String::new();
                    if let Some(ref font) = an.font {
                        let _ = write!(bullet_style, "font-family: '{}'; ", escape_html(font));
                    }
                    if let Some(ref color) = an.color
                        && let Some(css) = ctx.color_to_css(color)
                    {
                        let _ = write!(bullet_style, "color: {}; ", css);
                    }
                    if let Some(size_pct) = an.size_pct {
                        if size_pct < 0.0 {
                            let pts = -size_pct;
                            let _ = write!(bullet_style, "font-size: {pts:.1}pt; ");
                        } else if (size_pct - 1.0).abs() > 0.01 {
                            let pct = size_pct * 100.0;
                            let _ = write!(bullet_style, "font-size: {pct:.0}%; ");
                        }
                    }
                    let _ = write!(
                        html,
                        "<span class=\"bullet\" style=\"{bullet_style}\">{} </span>",
                        escape_html(&label)
                    );
                }
                Bullet::None => {
                    // Reset counters when bullet is explicitly suppressed
                    for counter in auto_num_counters.iter_mut().skip(level) {
                        *counter = 0;
                    }
                }
            }
        } else {
            // No bullet specified — reset counters at this level
            for counter in auto_num_counters.iter_mut().skip(level) {
                *counter = 0;
            }
        }

        // Get inherited run defaults for this level
        let run_defaults = inherited.and_then(|d| d.def_run_props.as_ref());

        for run in &para.runs {
            Self::render_run_with_defaults(
                run,
                ctx,
                para.def_rpr.as_ref(),
                run_defaults,
                font_ref_font,
                font_ref_color,
                font_scale,
                html,
            );
        }

        if para.runs.is_empty() {
            html.push_str("&nbsp;");
        }

        html.push_str("</p>\n");
    }

    /// Render run with inherited defaults from txStyles/defaultTextStyle
    fn render_run_with_defaults(
        run: &TextRun,
        ctx: &RenderCtx<'_>,
        para_def_rpr: Option<&ParagraphDefRPr>,
        run_defaults: Option<&RunDefaults>,
        font_ref_font: Option<&str>,
        font_ref_color: Option<&ResolvedColor>,
        font_scale: Option<f64>,
        html: &mut String,
    ) {
        // Line break (early return)
        if run.is_break {
            html.push_str("<br/>");
            return;
        }

        let mut run_style = String::with_capacity(128);

        // Font family: explicit > para defRPr > inherited defRPr > fontRef > theme
        let font = run.font.east_asian.as_deref().or(run.font.latin.as_deref());

        let font_scheme = ctx.pres.primary_theme().map(|t| &t.font_scheme);

        // Resolve font through typeface -> theme -> inherited -> fontRef chain,
        // skipping empty strings and unresolved theme references ("+mj-*"/"+mn-*").
        fn resolve_font_name<'a>(
            name: &'a str,
            font_scheme: Option<&'a FontScheme>,
        ) -> Option<&'a str> {
            if name.starts_with('+') {
                font_scheme.and_then(|fs| fs.resolve_typeface(name))
            } else if name.is_empty() {
                None
            } else {
                Some(name)
            }
        }

        let resolved_font: Option<&str> = font
            .and_then(|f| resolve_font_name(f, font_scheme))
            .or_else(|| {
                para_def_rpr.and_then(|pd| {
                    let f = pd.font_ea.as_deref().or(pd.font_latin.as_deref())?;
                    resolve_font_name(f, font_scheme)
                })
            })
            .or_else(|| {
                run_defaults.and_then(|rd| {
                    let inherited = rd.font_ea.as_deref().or(rd.font_latin.as_deref())?;
                    resolve_font_name(inherited, font_scheme)
                })
            })
            .or_else(|| font_ref_font.and_then(|f| resolve_font_name(f, font_scheme)));

        if let Some(f) = resolved_font {
            let _ = write!(run_style, "font-family: '{f}'");
        }

        // Font size: explicit > para defRPr > inherited, scaled by fontScale from normAutofit
        let font_size = run
            .style
            .font_size
            .or_else(|| para_def_rpr.and_then(|pd| pd.font_size))
            .or_else(|| run_defaults.and_then(|rd| rd.font_size));
        if let Some(sz) = font_size {
            let effective_sz = sz * font_scale.unwrap_or(1.0);
            push_sep(&mut run_style);
            let _ = write!(run_style, "font-size: {effective_sz:.1}pt");
        }

        // Bold: explicit > para defRPr > inherited
        let bold = if run.style.bold {
            true
        } else if let Some(b) = para_def_rpr.and_then(|pd| pd.bold) {
            b
        } else {
            run_defaults.and_then(|rd| rd.bold).unwrap_or(false)
        };
        if bold {
            push_sep(&mut run_style);
            run_style.push_str("font-weight: bold");
        }

        // Italic: explicit > para defRPr > inherited
        let italic = if run.style.italic {
            true
        } else if let Some(i) = para_def_rpr.and_then(|pd| pd.italic) {
            i
        } else {
            run_defaults.and_then(|rd| rd.italic).unwrap_or(false)
        };
        if italic {
            push_sep(&mut run_style);
            run_style.push_str("font-style: italic");
        }

        if let Some(ul_css) = run.style.underline.to_css() {
            push_sep(&mut run_style);
            run_style.push_str(&ul_css);
        }
        if let Some(st_css) = run.style.strikethrough.to_css() {
            push_sep(&mut run_style);
            run_style.push_str(st_css);
        }

        // Color -- explicit > para defRPr > inherited > fontRef > none
        // Use or_else chaining so that a None at any level falls through to the next
        let color_css = if !run.style.color.is_none() {
            ctx.color_to_css(&run.style.color)
        } else {
            para_def_rpr
                .and_then(|pd| pd.color.as_ref())
                .and_then(|c| ctx.color_to_css(c))
                .or_else(|| {
                    run_defaults
                        .and_then(|rd| rd.color.as_ref())
                        .and_then(|c| ctx.color_to_css(c))
                })
                .or_else(|| font_ref_color.as_ref().map(|c| c.to_css()))
        };
        if let Some(css_color) = color_css {
            push_sep(&mut run_style);
            let _ = write!(run_style, "color: {css_color}");
        }

        // Superscript/subscript -- use actual OOXML baseline percentage
        // baseline is in thousandths of percent (e.g., 30000 = 30%)
        if let Some(baseline) = run.style.baseline {
            if baseline != 0 {
                let pct = baseline as f64 / 1000.0;
                let abs_pct = pct.abs();
                // Scale font size proportionally: larger offset = smaller font
                let scale = (1.0 - abs_pct / 100.0).max(0.5);
                push_sep(&mut run_style);
                let _ = write!(
                    run_style,
                    "vertical-align: {pct:.1}%; font-size: {scale:.2}em"
                );
            }
        }

        // Letter spacing
        if let Some(spacing) = run.style.letter_spacing {
            push_sep(&mut run_style);
            let _ = write!(run_style, "letter-spacing: {spacing:.2}pt");
        }

        // Text highlight
        if let Some(ref highlight) = run.style.highlight
            && let Some(hl_css) = ctx.color_to_css(highlight)
        {
            push_sep(&mut run_style);
            let _ = write!(run_style, "background-color: {hl_css}");
        }

        // Text shadow
        if let Some(ref shadow) = run.style.shadow {
            let angle_rad = shadow.dir.to_radians();
            let dx = shadow.dist * angle_rad.cos();
            let dy = shadow.dist * angle_rad.sin();
            let shadow_color = ctx
                .color_to_css(&shadow.color)
                .unwrap_or_else(|| "rgba(0,0,0,0.5)".to_string());
            push_sep(&mut run_style);
            let _ = write!(
                run_style,
                "text-shadow: {dx:.1}pt {dy:.1}pt {blur:.1}pt {shadow_color}",
                blur = shadow.blur_rad,
            );
        }

        let text = escape_html(&run.text);

        if let Some(ref href) = run.hyperlink {
            let _ = write!(
                html,
                "<a class=\"run\" href=\"{}\" style=\"{run_style}\">{text}</a>",
                escape_html(href)
            );
        } else {
            let _ = write!(
                html,
                "<span class=\"run\" style=\"{run_style}\">{text}</span>"
            );
        }
    }

    /// Append fill CSS directly into an existing style buffer (avoids intermediate String)
    fn fill_to_css_buf(fill: &Fill, ctx: &RenderCtx<'_>, buf: &mut String) {
        match fill {
            Fill::None | Fill::NoFill => {}
            Fill::Solid(sf) => {
                if let Some(color_css) = ctx.color_to_css(&sf.color) {
                    push_sep(buf);
                    let _ = write!(buf, "background-color: {color_css}");
                }
            }
            Fill::Gradient(gf) => {
                let mut has_stops = false;
                let mut stops_buf = String::with_capacity(64);
                for s in &gf.stops {
                    if let Some(c) = ctx.color_to_css(&s.color) {
                        if has_stops {
                            stops_buf.push_str(", ");
                        }
                        let _ = write!(stops_buf, "{c} {:.0}%", s.position * 100.0);
                        has_stops = true;
                    }
                }
                if has_stops {
                    push_sep(buf);
                    let _ = write!(
                        buf,
                        "background: linear-gradient({:.0}deg, {stops_buf})",
                        gf.angle
                    );
                }
            }
            Fill::Image(img_fill) => {
                if !img_fill.data.is_empty() {
                    let mime = if img_fill.content_type.is_empty() {
                        "image/png"
                    } else {
                        &img_fill.content_type
                    };
                    push_sep(buf);
                    if ctx.embed_images {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&img_fill.data);
                        let _ = write!(
                            buf,
                            "background-image: url(data:{mime};base64,{b64}); \
                             background-size: cover; background-position: center; \
                             background-repeat: no-repeat"
                        );
                    } else {
                        let ext = match mime {
                            "image/jpeg" => "jpg",
                            "image/gif" => "gif",
                            "image/svg+xml" => "svg",
                            "image/webp" => "webp",
                            _ => "png",
                        };
                        let _ = write!(
                            buf,
                            "background-image: url(images/bg-{}.{ext}); \
                             background-size: cover; background-position: center; \
                             background-repeat: no-repeat",
                            img_fill.data.len() % 100000
                        );
                    }
                }
            }
        }
    }

    /// Convert Fill to CSS (theme-aware)
    fn fill_to_css(fill: &Fill, ctx: &RenderCtx<'_>) -> String {
        match fill {
            Fill::None | Fill::NoFill => String::new(),
            Fill::Solid(sf) => {
                if let Some(color_css) = ctx.color_to_css(&sf.color) {
                    format!("background-color: {color_css}")
                } else {
                    String::new()
                }
            }
            Fill::Gradient(gf) => {
                let stops: Vec<String> = gf
                    .stops
                    .iter()
                    .filter_map(|s| {
                        ctx.color_to_css(&s.color)
                            .map(|c| format!("{c} {:.0}%", s.position * 100.0))
                    })
                    .collect();
                if stops.is_empty() {
                    String::new()
                } else {
                    format!(
                        "background: linear-gradient({:.0}deg, {})",
                        gf.angle,
                        stops.join(", ")
                    )
                }
            }
            Fill::Image(img_fill) => {
                if !img_fill.data.is_empty() {
                    let mime = if img_fill.content_type.is_empty() {
                        "image/png"
                    } else {
                        &img_fill.content_type
                    };
                    let url = if ctx.embed_images {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&img_fill.data);
                        format!("data:{mime};base64,{b64}")
                    } else {
                        let ext = match mime {
                            "image/jpeg" => "jpg",
                            "image/gif" => "gif",
                            "image/svg+xml" => "svg",
                            "image/webp" => "webp",
                            _ => "png",
                        };
                        format!("images/bg-{}.{ext}", img_fill.data.len() % 100000)
                    };
                    format!(
                        "background-image: url({url}); background-size: cover; background-position: center; background-repeat: no-repeat"
                    )
                } else {
                    String::new()
                }
            }
        }
    }
}

/// Append a "; " separator to the style buffer if it's non-empty
#[inline]
fn push_sep(buf: &mut String) {
    if !buf.is_empty() {
        buf.push_str("; ");
    }
}

/// Format auto-numbered bullet label based on OOXML numbering type
fn format_auto_num(num_type: &str, val: i32) -> String {
    match num_type {
        "arabicPeriod" => format!("{val}."),
        "arabicParenR" => format!("{val})"),
        "arabicParenBoth" => format!("({val})"),
        "arabicPlain" => format!("{val}"),
        "alphaLcPeriod" => format!("{}.", to_alpha_lc(val)),
        "alphaLcParenR" => format!("{})", to_alpha_lc(val)),
        "alphaLcParenBoth" => format!("({})", to_alpha_lc(val)),
        "alphaUcPeriod" => format!("{}.", to_alpha_uc(val)),
        "alphaUcParenR" => format!("{})", to_alpha_uc(val)),
        "alphaUcParenBoth" => format!("({})", to_alpha_uc(val)),
        "romanLcPeriod" => format!("{}.", to_roman_lc(val)),
        "romanLcParenR" => format!("{})", to_roman_lc(val)),
        "romanLcParenBoth" => format!("({})", to_roman_lc(val)),
        "romanUcPeriod" => format!("{}.", to_roman_uc(val)),
        "romanUcParenR" => format!("{})", to_roman_uc(val)),
        "romanUcParenBoth" => format!("({})", to_roman_uc(val)),
        _ => format!("{val}."),
    }
}

/// Convert number to lowercase alphabetic (1=a, 2=b, ..., 26=z, 27=aa, ...)
fn to_alpha_lc(mut val: i32) -> String {
    if val <= 0 {
        return "a".to_string();
    }
    let mut result = String::new();
    while val > 0 {
        val -= 1;
        result.insert(0, (b'a' + (val % 26) as u8) as char);
        val /= 26;
    }
    result
}

/// Convert number to uppercase alphabetic
fn to_alpha_uc(val: i32) -> String {
    to_alpha_lc(val).to_uppercase()
}

/// Convert number to lowercase Roman numerals
fn to_roman_lc(val: i32) -> String {
    to_roman_uc(val).to_lowercase()
}

/// Convert number to uppercase Roman numerals
fn to_roman_uc(mut val: i32) -> String {
    if val <= 0 || val > 3999 {
        return val.to_string();
    }
    const NUMERALS: &[(i32, &str)] = &[
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut result = String::new();
    for &(value, symbol) in NUMERALS {
        while val >= value {
            result.push_str(symbol);
            val -= value;
        }
    }
    result
}

/// Context for resolving inherited text styles from txStyles/defaultTextStyle
#[derive(Default)]
struct TextStyleCtx<'a> {
    /// txStyles list matching placeholder type (titleStyle/bodyStyle/otherStyle)
    master_list_style: Option<&'a ListStyle>,
    /// presentation defaultTextStyle
    default_list_style: Option<&'a ListStyle>,
}

impl<'a> TextStyleCtx<'a> {
    /// Get paragraph defaults for a given level (0-based).
    /// Priority: master txStyles > defaultTextStyle
    fn get_level_defaults(&self, level: usize) -> Option<&'a ParagraphDefaults> {
        if level >= 9 {
            return None;
        }
        // txStyles takes priority
        if let Some(ls) = self.master_list_style
            && let Some(ref pd) = ls.levels[level]
        {
            return Some(pd);
        }
        // Fallback to defaultTextStyle
        if let Some(ls) = self.default_list_style
            && let Some(ref pd) = ls.levels[level]
        {
            return Some(pd);
        }
        None
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Convert DashStyle to SVG stroke-dasharray attribute string (including leading space)
fn dash_style_to_svg(style: &DashStyle, stroke_width: f64) -> String {
    let sw = if stroke_width > 0.0 {
        stroke_width
    } else {
        1.0
    };
    match style {
        DashStyle::Solid => String::new(),
        DashStyle::Dash => format!(" stroke-dasharray=\"{:.1} {:.1}\"", 8.0 * sw, 4.0 * sw),
        DashStyle::Dot => format!(" stroke-dasharray=\"{:.1} {:.1}\"", 2.0 * sw, 2.0 * sw),
        DashStyle::DashDot => format!(
            " stroke-dasharray=\"{:.1} {:.1} {:.1} {:.1}\"",
            8.0 * sw,
            4.0 * sw,
            2.0 * sw,
            4.0 * sw
        ),
        DashStyle::LongDash => format!(" stroke-dasharray=\"{:.1} {:.1}\"", 12.0 * sw, 4.0 * sw),
        DashStyle::LongDashDot => format!(
            " stroke-dasharray=\"{:.1} {:.1} {:.1} {:.1}\"",
            12.0 * sw,
            4.0 * sw,
            2.0 * sw,
            4.0 * sw
        ),
        DashStyle::LongDashDotDot => format!(
            " stroke-dasharray=\"{:.1} {:.1} {:.1} {:.1} {:.1} {:.1}\"",
            12.0 * sw,
            4.0 * sw,
            2.0 * sw,
            4.0 * sw,
            2.0 * sw,
            4.0 * sw
        ),
        DashStyle::SystemDash => format!(" stroke-dasharray=\"{:.1} {:.1}\"", 6.0 * sw, 3.0 * sw),
        DashStyle::SystemDot => format!(" stroke-dasharray=\"{:.1} {:.1}\"", 1.0 * sw, 2.0 * sw),
        DashStyle::SystemDashDot => format!(
            " stroke-dasharray=\"{:.1} {:.1} {:.1} {:.1}\"",
            3.0 * sw,
            1.0 * sw,
            1.0 * sw,
            1.0 * sw
        ),
        DashStyle::SystemDashDotDot => format!(
            " stroke-dasharray=\"{:.1} {:.1} {:.1} {:.1} {:.1} {:.1}\"",
            3.0 * sw,
            1.0 * sw,
            1.0 * sw,
            1.0 * sw,
            1.0 * sw,
            1.0 * sw
        ),
    }
}

/// Convert DashStyle to CSS border-style keyword
fn dash_style_to_css(style: &DashStyle) -> &'static str {
    match style {
        DashStyle::Solid => "solid",
        DashStyle::Dash | DashStyle::LongDash | DashStyle::SystemDash => "dashed",
        DashStyle::Dot | DashStyle::SystemDot => "dotted",
        DashStyle::DashDot
        | DashStyle::LongDashDot
        | DashStyle::LongDashDotDot
        | DashStyle::SystemDashDot
        | DashStyle::SystemDashDotDot => "dashed",
    }
}

/// Convert LineCap to SVG stroke-linecap attribute string (including leading space).
/// Returns empty string for Flat (SVG default "butt").
fn line_cap_to_svg(cap: &LineCap) -> &'static str {
    match cap {
        LineCap::Flat => "",
        LineCap::Square => " stroke-linecap=\"square\"",
        LineCap::Round => " stroke-linecap=\"round\"",
    }
}

/// Convert LineJoin to SVG stroke-linejoin attribute string (including leading space).
/// Returns empty string for Miter (SVG default).
fn line_join_to_svg(join: &LineJoin) -> &'static str {
    match join {
        LineJoin::Miter => "",
        LineJoin::Bevel => " stroke-linejoin=\"bevel\"",
        LineJoin::Round => " stroke-linejoin=\"round\"",
    }
}

/// Emit an SVG `<linearGradient>` definition and return the fill attribute string.
/// Returns `None` if the fill is not a gradient or has no resolvable stops.
fn svg_gradient_def(
    fill: &Fill,
    grad_id: &str,
    ctx: &RenderCtx<'_>,
    html: &mut String,
) -> Option<String> {
    if let Fill::Gradient(gf) = fill {
        let stops: Vec<(f64, String)> = gf
            .stops
            .iter()
            .filter_map(|s| ctx.color_to_css(&s.color).map(|c| (s.position, c)))
            .collect();
        if stops.is_empty() {
            return None;
        }
        // Convert OOXML angle (clockwise from top) to SVG linear-gradient coordinates.
        // SVG linearGradient uses x1,y1 -> x2,y2 as the gradient vector.
        let angle_rad = (gf.angle - 90.0_f64).to_radians();
        let x1 = 50.0 - 50.0 * angle_rad.cos();
        let y1 = 50.0 - 50.0 * angle_rad.sin();
        let x2 = 50.0 + 50.0 * angle_rad.cos();
        let y2 = 50.0 + 50.0 * angle_rad.sin();
        let _ = write!(
            html,
            "<linearGradient id=\"{grad_id}\" \
             x1=\"{x1:.1}%\" y1=\"{y1:.1}%\" x2=\"{x2:.1}%\" y2=\"{y2:.1}%\">"
        );
        for (pos, color) in &stops {
            let _ = write!(
                html,
                "<stop offset=\"{:.0}%\" stop-color=\"{color}\"/>",
                pos * 100.0
            );
        }
        html.push_str("</linearGradient>");
        return Some(format!("url(#{grad_id})"));
    }
    None
}

/// Emit an SVG <marker> definition for a line ending (arrowhead)
fn emit_marker_def(
    html: &mut String,
    marker_id: &str,
    line_end: &LineEnd,
    color: &str,
    stroke_width: f64,
    is_start: bool,
) {
    let w_mult = line_end.width.multiplier();
    let l_mult = line_end.length.multiplier();
    let marker_w = w_mult * stroke_width;
    let marker_h = l_mult * stroke_width;
    let half_w = marker_w / 2.0;

    let (path, fill_attr) = match line_end.end_type {
        LineEndType::Arrow => (
            format!("M0,0 L{marker_h:.1},{half_w:.1} L0,{marker_w:.1}"),
            "none".to_string(),
        ),
        LineEndType::Triangle => (
            format!("M0,0 L{marker_h:.1},{half_w:.1} L0,{marker_w:.1} Z"),
            color.to_string(),
        ),
        LineEndType::Stealth => (
            format!(
                "M0,0 L{marker_h:.1},{half_w:.1} L0,{marker_w:.1} L{back:.1},{half_w:.1} Z",
                back = marker_h * 0.35,
            ),
            color.to_string(),
        ),
        LineEndType::Diamond => {
            let mid_h = marker_h / 2.0;
            (
                format!(
                    "M0,{half_w:.1} L{mid_h:.1},0 L{marker_h:.1},{half_w:.1} L{mid_h:.1},{marker_w:.1} Z",
                ),
                color.to_string(),
            )
        }
        LineEndType::Oval => {
            let rx = marker_h / 2.0;
            let ry = half_w;
            let cx = rx;
            let cy = ry;
            (
                format!(
                    "M{start:.1},{cy:.1} A{rx:.1},{ry:.1} 0 1,1 {end:.1},{cy:.1} A{rx:.1},{ry:.1} 0 1,1 {start:.1},{cy:.1} Z",
                    start = cx - rx,
                    end = cx + rx,
                ),
                color.to_string(),
            )
        }
        LineEndType::None => return,
    };

    // For marker-start the reference point is at the base (refX=0) so the
    // marker sits at the start of the line.  For marker-end the reference
    // point is at the tip (refX=marker_h).  orient="auto-start-reverse"
    // handles directional flipping automatically.
    let ref_x = if is_start { 0.0 } else { marker_h };

    let _ = write!(
        html,
        "<marker id=\"{marker_id}\" viewBox=\"0 0 {marker_h:.1} {marker_w:.1}\" \
         refX=\"{ref_x:.1}\" refY=\"{half_w:.1}\" \
         markerWidth=\"{marker_h:.1}\" markerHeight=\"{marker_w:.1}\" \
         markerUnits=\"userSpaceOnUse\" \
         orient=\"auto-start-reverse\">\
         <path d=\"{path}\" fill=\"{fill_attr}\" stroke=\"{color}\" stroke-width=\"0.5\"/>\
         </marker>"
    );
}
