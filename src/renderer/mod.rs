//! HTML/CSS renderer
//! Presentation model -> self-contained HTML string generation

use base64::Engine;

use crate::error::PptxResult;
use crate::model::presentation::{ClrMap, ColorScheme};
use crate::model::*;
use crate::resolver::inheritance;
use crate::resolver::placeholder;

/// Rendering context -- propagates theme/ClrMap references and full presentation
struct RenderCtx<'a> {
    pres: &'a Presentation,
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

    /// Create a slide-scoped context with resolved ClrMap
    fn for_slide(&self, slide_clr_map: Option<&'a ClrMap>) -> RenderCtx<'a> {
        RenderCtx {
            pres: self.pres,
            scheme: self.scheme,
            clr_map: slide_clr_map.or(self.clr_map),
        }
    }
}

pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Render entire Presentation to HTML
    pub fn render(pres: &Presentation) -> PptxResult<String> {
        let slide_w = pres.slide_size.width.to_px();
        let slide_h = pres.slide_size.height.to_px();

        let ctx = RenderCtx {
            pres,
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

        // Look up layout and master for this slide
        let layout = slide
            .layout_idx
            .and_then(|idx| ctx.pres.layouts.get(idx));
        let master = layout
            .map(|l| l.master_idx)
            .and_then(|idx| ctx.pres.masters.get(idx));

        // Resolve ClrMap per slide (considering overrides)
        let slide_ctx = if let Some(m) = master {
            let resolved_cm = inheritance::resolve_clr_map(slide, layout, m);
            ctx.for_slide(if resolved_cm.is_empty() { None } else { Some(resolved_cm) })
        } else {
            ctx.for_slide(None)
        };

        // Resolve background via inheritance
        let bg = inheritance::resolve_background(slide, layout, master);
        let bg_style = Self::fill_to_css(&bg, &slide_ctx);
        html.push_str(&format!(
            "<div class=\"slide\" data-slide=\"{num}\" style=\"{bg_style}\">\n"
        ));

        // Render master shapes if show_master_sp is true
        let show_master = slide.show_master_sp
            && layout.map_or(true, |l| l.show_master_sp);
        if show_master {
            if let Some(m) = master {
                for master_shape in &m.shapes {
                    if master_shape.hidden {
                        continue;
                    }
                    // Only render master shapes that don't have a matching slide shape
                    if let Some(ref ph) = master_shape.placeholder {
                        let has_slide_match =
                            placeholder::find_matching_placeholder(ph, &slide.shapes).is_some();
                        if has_slide_match {
                            continue;
                        }
                    }
                    html.push_str(&Self::render_shape_resolved(master_shape, None, None, &slide_ctx));
                }
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

            html.push_str(&Self::render_shape_resolved(shape, layout_match, master_match, &slide_ctx));
        }

        html.push_str("</div>\n");
        html
    }

    /// Render shape with resolved properties from inheritance cascade
    fn render_shape_resolved(
        shape: &Shape,
        layout_match: Option<&Shape>,
        master_match: Option<&Shape>,
        ctx: &RenderCtx<'_>,
    ) -> String {
        // Resolve position/size via inheritance
        let (pos, size) = inheritance::resolve_position(shape, layout_match, master_match);
        let x = pos.x.to_px();
        let y = pos.y.to_px();
        let w = size.width.to_px();
        let h = size.height.to_px();

        let mut styles = vec![
            format!("left: {x:.1}px"),
            format!("top: {y:.1}px"),
            format!("width: {w:.1}px"),
            format!("height: {h:.1}px"),
        ];

        if shape.rotation != 0.0 {
            styles.push(format!("transform: rotate({:.1}deg)", shape.rotation));
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
        let fill_css = Self::fill_to_css(&resolved_fill, ctx);
        if !fill_css.is_empty() {
            styles.push(fill_css);
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
        if resolved_border.width > 0.0 {
            let border_color = ctx.color_to_css(&resolved_border.color)
                .unwrap_or_else(|| "#000".to_string());
            let border_style = match resolved_border.style {
                BorderStyle::Solid => "solid",
                BorderStyle::Dashed => "dashed",
                BorderStyle::Dotted => "dotted",
                BorderStyle::None => "none",
            };
            styles.push(format!(
                "border: {:.1}px {border_style} {border_color}",
                resolved_border.width
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
            // Track auto-number counters per level for this text body
            let mut auto_num_counters: [i32; 9] = [0; 9];
            for para in &text_body.paragraphs {
                html.push_str(&Self::render_paragraph(para, ctx, &mut auto_num_counters));
            }
            html.push_str("</div>\n");
        }

        html.push_str("</div>\n");
        html
    }

    fn render_paragraph(
        para: &TextParagraph,
        ctx: &RenderCtx<'_>,
        auto_num_counters: &mut [i32; 9],
    ) -> String {
        let align = para.alignment.to_css();
        let mut style_parts = vec![format!("text-align: {align}")];

        // Line spacing
        if let Some(ref ls) = para.line_spacing {
            match ls {
                SpacingValue::Percent(p) => {
                    style_parts.push(format!("line-height: {p:.2}"));
                }
                SpacingValue::Points(pt) => {
                    style_parts.push(format!("line-height: {pt:.1}pt"));
                }
            }
        }
        // Space before
        if let Some(ref sb) = para.space_before {
            match sb {
                SpacingValue::Percent(p) => {
                    style_parts.push(format!("margin-top: {p:.1}em"));
                }
                SpacingValue::Points(pt) => {
                    style_parts.push(format!("margin-top: {pt:.1}pt"));
                }
            }
        }
        // Space after
        if let Some(ref sa) = para.space_after {
            match sa {
                SpacingValue::Percent(p) => {
                    style_parts.push(format!("margin-bottom: {p:.1}em"));
                }
                SpacingValue::Points(pt) => {
                    style_parts.push(format!("margin-bottom: {pt:.1}pt"));
                }
            }
        }

        // Level-based indentation via margin_left and indent
        if let Some(ml) = para.margin_left {
            style_parts.push(format!("padding-left: {ml:.1}pt"));
        } else if para.level > 0 {
            // Fallback: ~36pt (0.5in) per level when no explicit margin
            let margin = para.level as f64 * 36.0;
            style_parts.push(format!("padding-left: {margin:.1}pt"));
        }
        if let Some(indent) = para.indent {
            style_parts.push(format!("text-indent: {indent:.1}pt"));
        }

        let style = style_parts.join("; ");
        let mut html = format!("<p class=\"paragraph\" style=\"{style}\">");

        // Bullet rendering
        let level = (para.level as usize).min(8);
        if let Some(ref bullet) = para.bullet {
            match bullet {
                Bullet::Char(bc) => {
                    // Reset counters at deeper levels when a char bullet is encountered
                    for counter in auto_num_counters.iter_mut().skip(level) {
                        *counter = 0;
                    }
                    let mut bullet_style = String::new();
                    if let Some(ref font) = bc.font {
                        bullet_style.push_str(&format!("font-family: '{}'; ", escape_html(font)));
                    }
                    if let Some(ref color) = bc.color {
                        if let Some(css) = ctx.color_to_css(color) {
                            bullet_style.push_str(&format!("color: {}; ", css));
                        }
                    }
                    if let Some(size_pct) = bc.size_pct {
                        if size_pct < 0.0 {
                            // Absolute points (stored as negative)
                            let pts = -size_pct;
                            bullet_style.push_str(&format!("font-size: {pts:.1}pt; "));
                        } else if (size_pct - 1.0).abs() > 0.01 {
                            // Percentage of text size (only if not 100%)
                            let pct = size_pct * 100.0;
                            bullet_style.push_str(&format!("font-size: {pct:.0}%; "));
                        }
                    }
                    html.push_str(&format!(
                        "<span class=\"bullet\" style=\"{bullet_style}\">{} </span>",
                        escape_html(&bc.char)
                    ));
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
                        bullet_style.push_str(&format!("font-family: '{}'; ", escape_html(font)));
                    }
                    if let Some(ref color) = an.color {
                        if let Some(css) = ctx.color_to_css(color) {
                            bullet_style.push_str(&format!("color: {}; ", css));
                        }
                    }
                    if let Some(size_pct) = an.size_pct {
                        if size_pct < 0.0 {
                            let pts = -size_pct;
                            bullet_style.push_str(&format!("font-size: {pts:.1}pt; "));
                        } else if (size_pct - 1.0).abs() > 0.01 {
                            let pct = size_pct * 100.0;
                            bullet_style.push_str(&format!("font-size: {pct:.0}%; "));
                        }
                    }
                    html.push_str(&format!(
                        "<span class=\"bullet\" style=\"{bullet_style}\">{} </span>",
                        escape_html(&label)
                    ));
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

        // Color -- theme-aware resolution
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
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
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

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
