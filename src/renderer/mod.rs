//! HTML/CSS renderer
//! Presentation model → self-contained HTML string generation

use base64::Engine;

use crate::error::PptxResult;
use crate::model::presentation::{ClrMap, ColorScheme};
use crate::model::*;

/// Extract the f64 value from a SpacingValue
fn spacing_to_f64(sv: &SpacingValue) -> f64 {
    match sv {
        SpacingValue::Percent(v) => *v,
        SpacingValue::Points(v) => *v,
    }
}

/// Rendering context — propagates theme/ClrMap references
struct RenderCtx<'a> {
    scheme: Option<&'a ColorScheme>,
    clr_map: Option<&'a ClrMap>,
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
}

pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Render entire Presentation to HTML
    pub fn render(pres: &Presentation) -> PptxResult<String> {
        let slide_w = pres.slide_size.width.to_px();
        let slide_h = pres.slide_size.height.to_px();

        let ctx = RenderCtx {
            scheme: pres.primary_theme().map(|t| &t.color_scheme),
            clr_map: if pres.clr_map.is_empty() { None } else { Some(&pres.clr_map) },
        };

        let mut html = String::with_capacity(4096);

        html.push_str("<!DOCTYPE html>\n<html lang=\"ko\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<meta name=\"generator\" content=\"pptx2html-rs\">\n");
        if let Some(ref title) = pres.title {
            html.push_str(&format!("<title>{}</title>\n", escape_html(title)));
        } else {
            html.push_str("<title>Presentation</title>\n");
        }
        html.push_str("<style>\n");
        html.push_str(&Self::global_css(slide_w, slide_h));
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("<div class=\"pptx-container\">\n");

        for (i, slide) in pres.slides.iter().enumerate() {
            if slide.hidden {
                continue;
            }
            html.push_str(&Self::render_slide(slide, i + 1, slide_w, slide_h, &ctx));
        }

        html.push_str("</div>\n</body>\n</html>");
        Ok(html)
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
  overflow: hidden;
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
"#
        )
    }

    fn render_slide(
        slide: &Slide,
        num: usize,
        _slide_w: f64,
        _slide_h: f64,
        ctx: &RenderCtx<'_>,
    ) -> String {
        let mut html = String::new();

        let default_bg = Fill::None;
        let bg = slide.background.as_ref().unwrap_or(&default_bg);
        let bg_style = Self::fill_to_css(bg, ctx);
        html.push_str(&format!(
            "<div class=\"slide\" data-slide=\"{num}\" style=\"{bg_style}\">\n"
        ));

        for shape in &slide.shapes {
            if shape.hidden {
                continue;
            }
            html.push_str(&Self::render_shape(shape, ctx));
        }

        html.push_str("</div>\n");
        html
    }

    fn render_shape(shape: &Shape, ctx: &RenderCtx<'_>) -> String {
        let x = shape.position.x.to_px();
        let y = shape.position.y.to_px();
        let w = shape.size.width.to_px();
        let h = shape.size.height.to_px();

        let mut styles = vec![
            format!("left: {x:.1}px"),
            format!("top: {y:.1}px"),
            format!("width: {w:.1}px"),
            format!("height: {h:.1}px"),
        ];

        if shape.rotation != 0.0 {
            styles.push(format!("transform: rotate({:.1}deg)", shape.rotation));
        }

        let fill_css = Self::fill_to_css(&shape.fill, ctx);
        if !fill_css.is_empty() {
            styles.push(fill_css);
        }

        if shape.border.width > 0.0 {
            let border_color = ctx.color_to_css(&shape.border.color)
                .unwrap_or_else(|| "#000".to_string());
            let border_style = match shape.border.style {
                BorderStyle::Solid => "solid",
                BorderStyle::Dashed => "dashed",
                BorderStyle::Dotted => "dotted",
                BorderStyle::None => "none",
            };
            styles.push(format!(
                "border: {:.1}px {border_style} {border_color}",
                shape.border.width
            ));
        }

        match &shape.shape_type {
            ShapeType::Ellipse => styles.push("border-radius: 50%".to_string()),
            ShapeType::RoundedRectangle => styles.push("border-radius: 8px".to_string()),
            _ => {}
        }

        let style_str = styles.join("; ");
        let mut html = format!("<div class=\"shape\" style=\"{style_str}\">\n");

        // Image
        if let ShapeType::Picture(pic) = &shape.shape_type {
            if !pic.data.is_empty() {
                let b64 = base64::engine::general_purpose::STANDARD.encode(&pic.data);
                let mime = if pic.content_type.is_empty() {
                    "image/png"
                } else {
                    &pic.content_type
                };
                html.push_str(&format!(
                    "<img class=\"shape-image\" src=\"data:{mime};base64,{b64}\" alt=\"\">\n"
                ));
            }
        }

        // Text
        if let Some(ref text_body) = shape.text_body {
            let v_class = match text_body.vertical_align {
                VerticalAlign::Top => "v-top",
                VerticalAlign::Middle => "v-middle",
                VerticalAlign::Bottom => "v-bottom",
            };
            let margin_style = format!(
                "padding: {:.1}pt {:.1}pt {:.1}pt {:.1}pt",
                text_body.margins.top,
                text_body.margins.right,
                text_body.margins.bottom,
                text_body.margins.left,
            );
            html.push_str(&format!(
                "<div class=\"text-body {v_class}\" style=\"{margin_style}\">\n"
            ));
            for para in &text_body.paragraphs {
                html.push_str(&Self::render_paragraph(para, ctx));
            }
            html.push_str("</div>\n");
        }

        html.push_str("</div>\n");
        html
    }

    fn render_paragraph(para: &TextParagraph, ctx: &RenderCtx<'_>) -> String {
        let align = para.alignment.to_css();
        let mut style_parts = vec![format!("text-align: {align}")];

        if let Some(ref ls) = para.line_spacing {
            let v = spacing_to_f64(ls);
            style_parts.push(format!("line-height: {v:.2}"));
        }
        if let Some(ref sb) = para.space_before {
            let v = spacing_to_f64(sb);
            style_parts.push(format!("margin-top: {v:.1}pt"));
        }
        if let Some(ref sa) = para.space_after {
            let v = spacing_to_f64(sa);
            style_parts.push(format!("margin-bottom: {v:.1}pt"));
        }
        if let Some(indent) = para.indent {
            style_parts.push(format!("text-indent: {indent:.1}pt"));
        }

        let style = style_parts.join("; ");
        let mut html = format!("<p class=\"paragraph\" style=\"{style}\">");

        // Bullet
        if let Some(ref bullet) = para.bullet {
            match bullet {
                Bullet::Char(ch) => {
                    html.push_str(&format!("<span class=\"bullet\">{} </span>", escape_html(ch)));
                }
                Bullet::AutoNum(_) => {
                    // Auto numbering to be implemented later (requires counter)
                }
                Bullet::None => {}
            }
        }

        for run in &para.runs {
            html.push_str(&Self::render_run(run, ctx));
        }

        if para.runs.is_empty() {
            html.push_str("&nbsp;");
        }

        html.push_str("</p>\n");
        html
    }

    fn render_run(run: &TextRun, ctx: &RenderCtx<'_>) -> String {
        let mut styles = Vec::new();

        let font = run
            .font
            .east_asian
            .as_deref()
            .or(run.font.latin.as_deref());
        if let Some(f) = font {
            styles.push(format!("font-family: '{f}'"));
        }

        if let Some(sz) = run.style.font_size {
            styles.push(format!("font-size: {sz:.1}pt"));
        }

        if run.style.bold {
            styles.push("font-weight: bold".to_string());
        }
        if run.style.italic {
            styles.push("font-style: italic".to_string());
        }
        if run.style.underline {
            styles.push("text-decoration: underline".to_string());
        }
        if run.style.strikethrough {
            styles.push("text-decoration: line-through".to_string());
        }

        // Color — theme-aware resolution
        if let Some(css_color) = ctx.color_to_css(&run.style.color) {
            styles.push(format!("color: {css_color}"));
        }

        // Superscript/subscript
        if let Some(baseline) = run.style.baseline {
            if baseline > 0 {
                styles.push("vertical-align: super".to_string());
                styles.push("font-size: 0.6em".to_string());
            } else if baseline < 0 {
                styles.push("vertical-align: sub".to_string());
                styles.push("font-size: 0.6em".to_string());
            }
        }

        // Letter spacing
        if let Some(spacing) = run.style.letter_spacing {
            styles.push(format!("letter-spacing: {spacing:.2}pt"));
        }

        let style_str = styles.join("; ");
        let text = escape_html(&run.text);

        if let Some(ref href) = run.hyperlink {
            format!(
                "<a class=\"run\" href=\"{}\" style=\"{style_str}\">{text}</a>",
                escape_html(href)
            )
        } else {
            format!("<span class=\"run\" style=\"{style_str}\">{text}</span>")
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
            Fill::Image(_) => String::new(),
        }
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
