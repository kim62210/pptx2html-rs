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

    /// Create a slide-scoped context with resolved ClrMap
    fn for_slide(&self, slide_clr_map: Option<&'a ClrMap>) -> RenderCtx<'a> {
        RenderCtx {
            pres: self.pres,
            scheme: self.scheme,
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
        html.push_str("<meta name=\"generator\" content=\"pptx2html-rs\">\n");
        if let Some(ref title) = pres.title {
            let _ = write!(html, "<title>{}</title>\n", escape_html(title));
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
}}
.text-body.v-top {{ justify-content: flex-start; }}
.text-body.v-middle {{ justify-content: center; }}
.text-body.v-bottom {{ justify-content: flex-end; }}
.paragraph {{ margin: 0; }}
.run {{ white-space: pre-wrap; }}
img.shape-image {{ width: 100%; height: 100%; object-fit: cover; }}
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

        // Resolve ClrMap per slide (considering overrides)
        let slide_ctx = if let Some(m) = master {
            let resolved_cm = inheritance::resolve_clr_map(slide, layout, m);
            ctx.for_slide(if resolved_cm.is_empty() {
                None
            } else {
                Some(resolved_cm)
            })
        } else {
            ctx.for_slide(None)
        };

        // Resolve background via inheritance
        let bg = inheritance::resolve_background(slide, layout, master);
        let bg_style = Self::fill_to_css(&bg, &slide_ctx);
        let _ = write!(
            html,
            "<div class=\"slide\" data-slide=\"{num}\" style=\"{bg_style}\">\n"
        );

        // Render master shapes if show_master_sp is true
        let show_master = slide.show_master_sp && layout.is_none_or(|l| l.show_master_sp);
        if show_master && let Some(m) = master {
            for master_shape in &m.shapes {
                if master_shape.hidden {
                    continue;
                }
                if let Some(ref ph) = master_shape.placeholder {
                    // Skip if the slide already defines this placeholder
                    let has_slide_match =
                        placeholder::find_matching_placeholder(ph, &slide.shapes).is_some();
                    if has_slide_match {
                        continue;
                    }
                    // Per OOXML: master placeholder shapes only appear if the
                    // layout also carries a matching placeholder.  When the
                    // layout omits the placeholder the master shape is hidden.
                    if let Some(l) = layout {
                        let has_layout_match =
                            placeholder::find_matching_placeholder(ph, &l.shapes).is_some();
                        if !has_layout_match {
                            continue;
                        }
                    }
                }
                Self::render_shape_resolved(
                    master_shape,
                    None,
                    None,
                    &slide_ctx,
                    html,
                );
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

            Self::render_shape_resolved(
                shape,
                layout_match,
                master_match,
                &slide_ctx,
                html,
            );
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
        let _ = write!(style_buf, "left: {x:.1}px; top: {y:.1}px; width: {w:.1}px; height: {h:.1}px");

        if shape.rotation != 0.0 {
            let _ = write!(style_buf, "; transform: rotate({:.1}deg)", shape.rotation);
        }

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
        Self::fill_to_css_buf(&resolved_fill, ctx, &mut style_buf);

        // Resolve border via inheritance (with style_ref fallback)
        let resolved_border = inheritance::resolve_border_with_theme(
            shape,
            layout_match,
            master_match,
            fmt_scheme,
            ctx.scheme,
            ctx.clr_map,
        );
        if resolved_border.width > 0.0 {
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
                "; border: {:.1}px {border_style} {border_color}",
                resolved_border.width
            );
        }

        // Shape-level effects → CSS box-shadow
        {
            let mut shadows: Vec<String> = Vec::new();

            // outerShdw → box-shadow with offset
            if let Some(ref shadow) = shape.effects.outer_shadow {
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
            if let Some(ref glow) = shape.effects.glow {
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

        // Determine SVG preset name for the shape (if applicable)
        let svg_preset_name = match &shape.shape_type {
            ShapeType::Ellipse => Some("ellipse"),
            ShapeType::RoundedRectangle => Some("roundRect"),
            ShapeType::Triangle => Some("triangle"),
            ShapeType::Custom(name) => Some(name.as_str()),
            _ => None,
        };

        // Only apply CSS border-radius for non-SVG shapes; SVG handles geometry
        if svg_preset_name.is_none() {
            match &shape.shape_type {
                ShapeType::Ellipse => style_buf.push_str("; border-radius: 50%"),
                ShapeType::RoundedRectangle => style_buf.push_str("; border-radius: 8px"),
                _ => {}
            }
        }

        let _ = write!(html, "<div class=\"shape\" style=\"{style_buf}\">\n");

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
            let placeholder_id = format!(
                "unresolved-s{}-e{}",
                coll.current_slide_index, coll.counter
            );
            coll.counter += 1;

            let type_attr = match data.element_type {
                UnresolvedType::SmartArt => "smartart",
                UnresolvedType::OleObject => "ole",
                UnresolvedType::MathEquation => "math",
                UnresolvedType::CustomGeometry => "custom-geometry",
            };

            let escaped = escape_html(&data.label);
            let _ = write!(
                html,
                "<div class=\"unresolved-element\" id=\"{placeholder_id}\" \
                 data-type=\"{type_attr}\" data-slide=\"{}\">\
                 <span>[{escaped}]</span></div>\n",
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
                    let _ = write!(
                        html,
                        "<img class=\"shape-image\" src=\"{src}\" alt=\"Chart\">\n"
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
            if let Some(svg_path) = geometry::preset_shape_svg(preset_name, w, h, adj_values) {
                let fill_color = ctx
                    .color_to_css(&shape.fill.color_ref())
                    .unwrap_or_else(|| "none".to_string());
                let (stroke_color, stroke_width) = if resolved_border.width > 0.0 {
                    let c = ctx
                        .color_to_css(&resolved_border.color)
                        .unwrap_or_else(|| "#000".to_string());
                    (c, resolved_border.width)
                } else {
                    ("none".to_string(), 0.0)
                };
                let _ = write!(
                    html,
                    "<svg viewBox=\"0 0 {w:.1} {h:.1}\" class=\"shape-svg\" preserveAspectRatio=\"none\">\
                     <path d=\"{svg_path}\" fill=\"{fill_color}\" stroke=\"{stroke_color}\" stroke-width=\"{stroke_width:.1}\"/>\
                     </svg>\n"
                );
            }
        }

        // Custom geometry SVG rendering
        if let ShapeType::CustomGeom(ref geom) = shape.shape_type {
            if let Some(svg_geom) = geometry::custom_geometry_svg(geom, w, h) {
                let default_fill = ctx
                    .color_to_css(&resolved_fill.color_ref())
                    .unwrap_or_else(|| "none".to_string());
                let (stroke_color, stroke_width) = if resolved_border.width > 0.0 {
                    let c = ctx
                        .color_to_css(&resolved_border.color)
                        .unwrap_or_else(|| "#000".to_string());
                    (c, resolved_border.width)
                } else {
                    ("none".to_string(), 0.0)
                };
                let _ = write!(
                    html,
                    "<svg viewBox=\"0 0 {w:.1} {h:.1}\" class=\"shape-svg\" preserveAspectRatio=\"none\">"
                );
                for path_svg in &svg_geom.paths {
                    let fill = match path_svg.fill {
                        PathFill::None => "none".to_string(),
                        _ => default_fill.clone(),
                    };
                    let _ = write!(
                        html,
                        "<path d=\"{}\" fill=\"{fill}\" stroke=\"{stroke_color}\" stroke-width=\"{stroke_width:.1}\"/>",
                        path_svg.d
                    );
                }
                html.push_str("</svg>\n");
            }
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
                // CSS object-view-box or clip approach for cropping
                let left_pct = crop.left * 100.0;
                let top_pct = crop.top * 100.0;
                let right_pct = crop.right * 100.0;
                let bottom_pct = crop.bottom * 100.0;
                let scale_x = 100.0 / (100.0 - left_pct - right_pct) * 100.0;
                let scale_y = 100.0 / (100.0 - top_pct - bottom_pct) * 100.0;
                let offset_x = -left_pct / (100.0 - left_pct - right_pct) * 100.0;
                let offset_y = -top_pct / (100.0 - top_pct - bottom_pct) * 100.0;
                let _ = write!(
                    html,
                    "<img class=\"shape-image\" src=\"{src}\" alt=\"\" style=\"\
                     clip-path: inset({top_pct:.1}% {right_pct:.1}% {bottom_pct:.1}% {left_pct:.1}%); \
                     width: {scale_x:.1}%; height: {scale_y:.1}%; \
                     margin-left: {offset_x:.1}%; margin-top: {offset_y:.1}%\">\n"
                );
            } else {
                let _ = write!(
                    html,
                    "<img class=\"shape-image\" src=\"{src}\" alt=\"\">\n"
                );
            }
        }

        // Resolve text style source for this shape's placeholder type
        let text_style_ctx = Self::build_text_style_ctx(shape, ctx);

        // Resolve fontRef from <p:style> for font-family fallback
        let font_ref_font = Self::resolve_font_ref_font(shape, ctx);

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
            // Vertical text rendering
            if let Some(ref vert) = shape.vertical_text {
                match vert.as_str() {
                    "vert" | "wordArtVert" | "eaVert" => {
                        tb_style.push_str("; writing-mode: vertical-rl");
                    }
                    "vert270" => {
                        tb_style.push_str("; writing-mode: vertical-lr; transform: rotate(180deg)");
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
            let _ = write!(
                html,
                "<div class=\"text-body {v_class}\" style=\"{tb_style}\">\n"
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

    /// Resolve fontRef from shape's <p:style> to a font-family name
    fn resolve_font_ref_font(shape: &Shape, ctx: &RenderCtx<'_>) -> Option<String> {
        let sr = shape.style_ref.as_ref()?;
        let font_ref = sr.font_ref.as_ref()?;
        let theme = ctx.pres.primary_theme()?;
        let font_scheme = &theme.font_scheme;
        let scheme = ctx.scheme?;
        let clr_map = ctx.clr_map?;
        let (font_name, _color) =
            style_ref::resolve_font_ref(font_ref, font_scheme, scheme, clr_map)?;
        Some(font_name)
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
            let _ = write!(html, "<col style=\"width:{pct:.1}%\"/>\n");
        }
        html.push_str("</colgroup>\n");

        for row in &table.rows {
            let _ = write!(html, "<tr style=\"height:{:.1}px;\">\n", row.height);
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
                        "border-left: {:.1}px solid {}",
                        cell.border_left.width, color
                    );
                }
                if cell.border_right.width > 0.0 {
                    let color = ctx
                        .color_to_css(&cell.border_right.color)
                        .unwrap_or_else(|| "#000".to_string());
                    push_sep(&mut td_style);
                    let _ = write!(
                        td_style,
                        "border-right: {:.1}px solid {}",
                        cell.border_right.width, color
                    );
                }
                if cell.border_top.width > 0.0 {
                    let color = ctx
                        .color_to_css(&cell.border_top.color)
                        .unwrap_or_else(|| "#000".to_string());
                    push_sep(&mut td_style);
                    let _ = write!(
                        td_style,
                        "border-top: {:.1}px solid {}",
                        cell.border_top.width, color
                    );
                }
                if cell.border_bottom.width > 0.0 {
                    let color = ctx
                        .color_to_css(&cell.border_bottom.color)
                        .unwrap_or_else(|| "#000".to_string());
                    push_sep(&mut td_style);
                    let _ = write!(
                        td_style,
                        "border-bottom: {:.1}px solid {}",
                        cell.border_bottom.width, color
                    );
                }

                push_sep(&mut td_style);
                td_style.push_str("padding: 4px; vertical-align: top");

                let _ = write!(html, "<td");
                if cell.col_span > 1 {
                    let _ = write!(html, " colspan=\"{}\"", cell.col_span);
                }
                if cell.row_span > 1 {
                    let _ = write!(html, " rowspan=\"{}\"", cell.row_span);
                }
                let _ = write!(html, " style=\"{td_style}\">\n");
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
            html,
        );
    }

    /// Render paragraph with inherited text style defaults from txStyles / defaultTextStyle
    fn render_paragraph_with_defaults(
        para: &TextParagraph,
        ctx: &RenderCtx<'_>,
        auto_num_counters: &mut [i32; 9],
        text_ctx: &TextStyleCtx<'_>,
        font_ref_font: Option<&str>,
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

        // Bullet rendering (explicit > inherited)
        let bullet = para
            .bullet
            .as_ref()
            .or_else(|| inherited.and_then(|d| d.bullet.as_ref()));
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
                run_defaults,
                font_ref_font,
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
        run_defaults: Option<&RunDefaults>,
        font_ref_font: Option<&str>,
        font_scale: Option<f64>,
        html: &mut String,
    ) {
        // Line break (early return)
        if run.is_break {
            html.push_str("<br/>");
            return;
        }

        let mut run_style = String::with_capacity(128);

        // Font family: explicit > inherited defRPr > fontRef > theme
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
                run_defaults.and_then(|rd| {
                    let inherited = rd.font_ea.as_deref().or(rd.font_latin.as_deref())?;
                    resolve_font_name(inherited, font_scheme)
                })
            })
            .or_else(|| {
                font_ref_font.and_then(|f| resolve_font_name(f, font_scheme))
            });

        if let Some(f) = resolved_font {
            let _ = write!(run_style, "font-family: '{f}'");
        }

        // Font size: explicit > inherited, scaled by fontScale from normAutofit
        let font_size = run
            .style
            .font_size
            .or_else(|| run_defaults.and_then(|rd| rd.font_size));
        if let Some(sz) = font_size {
            let effective_sz = sz * font_scale.unwrap_or(1.0);
            push_sep(&mut run_style);
            let _ = write!(run_style, "font-size: {effective_sz:.1}pt");
        }

        // Bold: explicit > inherited
        let bold = if run.style.bold {
            true
        } else {
            run_defaults.and_then(|rd| rd.bold).unwrap_or(false)
        };
        if bold {
            push_sep(&mut run_style);
            run_style.push_str("font-weight: bold");
        }

        // Italic: explicit > inherited
        let italic = if run.style.italic {
            true
        } else {
            run_defaults.and_then(|rd| rd.italic).unwrap_or(false)
        };
        if italic {
            push_sep(&mut run_style);
            run_style.push_str("font-style: italic");
        }

        if run.style.underline {
            push_sep(&mut run_style);
            run_style.push_str("text-decoration: underline");
        }
        if run.style.strikethrough {
            push_sep(&mut run_style);
            run_style.push_str("text-decoration: line-through");
        }

        // Color -- explicit > inherited > theme-aware resolution
        let color_css = if !run.style.color.is_none() {
            ctx.color_to_css(&run.style.color)
        } else if let Some(rd) = run_defaults {
            rd.color.as_ref().and_then(|c| ctx.color_to_css(c))
        } else {
            None
        };
        if let Some(css_color) = color_css {
            push_sep(&mut run_style);
            let _ = write!(run_style, "color: {css_color}");
        }

        // Superscript/subscript
        if let Some(baseline) = run.style.baseline {
            if baseline > 0 {
                push_sep(&mut run_style);
                run_style.push_str("vertical-align: super; font-size: 0.6em");
            } else if baseline < 0 {
                push_sep(&mut run_style);
                run_style.push_str("vertical-align: sub; font-size: 0.6em");
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
            let _ = write!(html, "<span class=\"run\" style=\"{run_style}\">{text}</span>");
        }
    }

    /// Append fill CSS directly into an existing style buffer (avoids intermediate String)
    fn fill_to_css_buf(fill: &Fill, ctx: &RenderCtx<'_>, buf: &mut String) {
        match fill {
            Fill::None => {}
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
                    let _ = write!(buf, "background: linear-gradient({:.0}deg, {stops_buf})", gf.angle);
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
                        let b64 =
                            base64::engine::general_purpose::STANDARD.encode(&img_fill.data);
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
            Fill::None => String::new(),
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
