//! Integration tests — minimal PPTX generation → parsing → HTML rendering verification

mod fixtures;

use pptx2html_core::model::*;

fn parse_pptx(data: &[u8]) -> pptx2html_core::model::Presentation {
    pptx2html_core::parser::PptxParser::parse_bytes(data).expect("PPTX parsing failed")
}

fn render_html(data: &[u8]) -> String {
    let pres = parse_pptx(data);
    pptx2html_core::renderer::HtmlRenderer::render(&pres).expect("HTML rendering failed")
}

// ── Basic parsing tests ──

#[test]
fn test_empty_slide() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides.len(), 1);
    assert!(pres.slides[0].shapes.is_empty());
}

#[test]
fn test_slide_size() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let pres = parse_pptx(&pptx);
    // 9144000 EMU = 10 inches = 960px at 96dpi
    assert!((pres.slide_size.width.to_px() - 960.0).abs() < 0.1);
    assert!((pres.slide_size.height.to_px() - 720.0).abs() < 0.1);
}

// ── Theme color tests ──

#[test]
fn test_theme_parsing() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let pres = parse_pptx(&pptx);
    let theme = pres.primary_theme().expect("Theme not found");
    assert_eq!(theme.color_scheme.accent1, "4472C4");
    assert_eq!(theme.color_scheme.dk1, "000000");
    assert_eq!(theme.color_scheme.lt1, "FFFFFF");
}

#[test]
fn test_theme_color_in_text() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="TextBox 1"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:r>
            <a:rPr lang="en-US" sz="2400">
              <a:solidFill><a:schemeClr val="accent1"/></a:solidFill>
            </a:rPr>
            <a:t>Theme Color Text</a:t>
          </a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // accent1 = #4472C4
    assert!(html.contains("#4472C4"), "Theme accent1 color not found in HTML: {html}");
}

// ── ClrMap tests ──

#[test]
fn test_clr_map_parsing() {
    let pptx = fixtures::MinimalPptx::new("")
        .with_clr_map(r#"bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink""#)
        .build();
    let pres = parse_pptx(&pptx);
    assert!(!pres.clr_map.is_empty());
    assert_eq!(pres.clr_map.get("tx1"), Some(&"dk1".to_string()));
    assert_eq!(pres.clr_map.get("bg1"), Some(&"lt1".to_string()));
}

#[test]
fn test_clr_map_color_resolution() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:r>
            <a:rPr sz="1800">
              <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
            </a:rPr>
            <a:t>Mapped Color</a:t>
          </a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide)
        .with_clr_map(r#"bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink""#)
        .build();
    let html = render_html(&pptx);
    // tx1 → dk1 → "000000"
    assert!(html.contains("#000000"), "ClrMap tx1→dk1 color not found: {html}");
}

// ── SolidFill tests ──

#[test]
fn test_solid_fill_rgb() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="500000" y="500000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="FF5733"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    match &shape.fill {
        Fill::Solid(sf) => {
            assert_eq!(sf.color.kind, color::ColorKind::Rgb("FF5733".to_string()));
        }
        other => panic!("Expected SolidFill: {other:?}"),
    }
}

#[test]
fn test_solid_fill_theme() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="500000" y="500000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:schemeClr val="accent2"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // accent2 = ED7D31
    assert!(html.contains("#ED7D31"), "accent2 fill color not found: {html}");
}

// ── Color modifier tests ──

#[test]
fn test_color_modifier_tint() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill>
          <a:schemeClr val="accent1">
            <a:tint val="50000"/>
          </a:schemeClr>
        </a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    match &shape.fill {
        Fill::Solid(sf) => {
            assert_eq!(sf.color.modifiers.len(), 1);
            assert_eq!(sf.color.modifiers[0], color::ColorModifier::Tint(50000));
        }
        other => panic!("Expected SolidFill: {other:?}"),
    }
}

#[test]
fn test_color_modifier_lum_mod_off() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill>
          <a:schemeClr val="accent1">
            <a:lumMod val="75000"/>
            <a:lumOff val="25000"/>
          </a:schemeClr>
        </a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    match &shape.fill {
        Fill::Solid(sf) => {
            assert_eq!(sf.color.modifiers.len(), 2);
            assert_eq!(sf.color.modifiers[0], color::ColorModifier::LumMod(75000));
            assert_eq!(sf.color.modifiers[1], color::ColorModifier::LumOff(25000));
        }
        other => panic!("Expected SolidFill: {other:?}"),
    }
}

// ── GradientFill tests ──

#[test]
fn test_gradient_fill() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
            <a:gs pos="50000"><a:srgbClr val="00FF00"/></a:gs>
            <a:gs pos="100000"><a:srgbClr val="0000FF"/></a:gs>
          </a:gsLst>
          <a:lin ang="5400000" scaled="1"/>
        </a:gradFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    match &shape.fill {
        Fill::Gradient(gf) => {
            assert_eq!(gf.stops.len(), 3);
            assert!((gf.stops[0].position - 0.0).abs() < 0.01);
            assert!((gf.stops[1].position - 0.5).abs() < 0.01);
            assert!((gf.stops[2].position - 1.0).abs() < 0.01);
            assert!((gf.angle - 90.0).abs() < 0.1); // 5400000/60000 = 90
        }
        other => panic!("Expected GradientFill: {other:?}"),
    }
}

#[test]
fn test_gradient_fill_html() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
            <a:gs pos="100000"><a:srgbClr val="0000FF"/></a:gs>
          </a:gsLst>
          <a:lin ang="5400000"/>
        </a:gradFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("linear-gradient"), "Gradient not found in HTML: {html}");
    assert!(html.contains("#FF0000"), "Start color not found");
    assert!(html.contains("#0000FF"), "End color not found");
}

// ── Border tests ──

#[test]
fn test_border_parsing() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:ln w="25400">
          <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
          <a:prstDash val="dash"/>
        </a:ln>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    // 25400 EMU = 2pt
    assert!((shape.border.width - 2.0).abs() < 0.1);
    assert!(matches!(shape.border.style, BorderStyle::Dashed));
    assert_eq!(shape.border.color.kind, color::ColorKind::Rgb("FF0000".to_string()));
}

#[test]
fn test_border_html() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:ln w="12700">
          <a:solidFill><a:srgbClr val="0000FF"/></a:solidFill>
        </a:ln>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("border:"), "border not found: {html}");
    assert!(html.contains("#0000FF"), "border color not found: {html}");
}

// ── bodyPr tests ──

#[test]
fn test_body_pr_vertical_align() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr anchor="ctr" lIns="91440" tIns="45720" rIns="91440" bIns="45720"/>
        <a:p><a:r><a:rPr sz="1800"/><a:t>Centered</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    let tb = shape.text_body.as_ref().expect("TextBody not found");
    assert!(matches!(tb.vertical_align, VerticalAlign::Middle));
    // 91440 EMU = 7.2pt
    assert!((tb.margins.left - 7.2).abs() < 0.1);
    assert!((tb.margins.top - 3.6).abs() < 0.1);
}

#[test]
fn test_body_pr_html_rendering() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr anchor="b"/>
        <a:p><a:r><a:rPr sz="1800"/><a:t>Bottom</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("v-bottom"), "v-bottom class not found: {html}");
}

// ── NoFill tests ──

#[test]
fn test_no_fill() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:noFill/>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert!(matches!(shape.fill, Fill::None));
}

// ── E2E HTML rendering tests ──

#[test]
fn test_full_html_structure() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Title"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="500000" y="300000"/><a:ext cx="8000000" cy="1200000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:schemeClr val="accent1"/></a:solidFill>
      </p:spPr>
      <p:txBody>
        <a:bodyPr anchor="ctr"/>
        <a:p>
          <a:pPr algn="ctr"/>
          <a:r>
            <a:rPr sz="3600" b="1">
              <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
              <a:latin typeface="Arial"/>
            </a:rPr>
            <a:t>Hello World</a:t>
          </a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);

    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("pptx-container"));
    assert!(html.contains("Hello World"));
    assert!(html.contains("font-weight: bold"));
    assert!(html.contains("text-align: center"));
    assert!(html.contains("#FFFFFF"));
    assert!(html.contains("#4472C4")); // accent1 fill
    assert!(html.contains("v-middle"));
    assert!(html.contains("Arial"));
}

// ── Composite test: multiple shapes + various attributes ──

#[test]
fn test_multiple_shapes() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Shape1"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="ellipse"/>
        <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
      </p:spPr>
    </p:sp>
    <p:sp>
      <p:nvSpPr><p:cNvPr id="3" name="Shape2"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="3000000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="roundRect"/>
        <a:solidFill><a:srgbClr val="00FF00"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides[0].shapes.len(), 2);

    let html = render_html(&pptx);
    assert!(html.contains("#FF0000"));
    assert!(html.contains("#00FF00"));
    // Ellipse and roundRect are now rendered as SVG paths
    assert!(html.contains("shape-svg"), "Expected SVG rendering for preset shapes");
    assert!(html.contains("<path d="), "Expected SVG path element");
}

// ── Month 4: Preset Shape SVG tests ──

#[test]
fn test_preset_shape_svg_diamond() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Diamond"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="diamond"/>
        <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("shape-svg"), "Diamond should be SVG rendered");
    assert!(html.contains("<path d="), "Should contain SVG path");
    assert!(html.contains("#4472C4"), "Should contain fill color");
}

#[test]
fn test_preset_shape_svg_right_arrow() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Arrow"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rightArrow"/>
        <a:solidFill><a:srgbClr val="FF5733"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert!(
        matches!(&pres.slides[0].shapes[0].shape_type, ShapeType::Custom(name) if name == "rightArrow"),
        "Should be Custom(rightArrow)"
    );
    let html = render_html(&pptx);
    assert!(html.contains("shape-svg"));
}

#[test]
fn test_preset_shape_with_adjust_values() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="RRect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="roundRect">
          <a:avLst>
            <a:gd name="adj" fmla="val 25000"/>
          </a:avLst>
        </a:prstGeom>
        <a:solidFill><a:srgbClr val="00FF00"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert!(shape.adjust_values.is_some(), "Should have adjust values");
    let adj = shape.adjust_values.as_ref().unwrap();
    assert_eq!(*adj.get("adj").unwrap() as i64, 25000, "adj should be 25000");
}

#[test]
fn test_preset_shape_star5() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Star"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="1000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="star5"/>
        <a:solidFill><a:srgbClr val="FFD700"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("shape-svg"));
    assert!(html.contains("#FFD700"));
}

#[test]
fn test_rect_shape_no_svg() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="0000FF"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // rect is ShapeType::Rectangle (rendered without SVG path)
    // Note: .shape-svg class exists in CSS, so we check for actual SVG element usage
    assert!(!html.contains("<svg viewBox="), "Rect should not generate SVG viewBox");
    assert!(html.contains("#0000FF"));
}

// ── Month 4: Text break (<a:br>) test ──

#[test]
fn test_text_break_renders_as_br() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="TextBox"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:r><a:t>Line 1</a:t></a:r>
          <a:br/>
          <a:r><a:t>Line 2</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let para = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs[0];
    assert_eq!(para.runs.len(), 3, "Should have 3 runs (text, break, text)");
    assert!(para.runs[1].is_break, "Second run should be a break");

    let html = render_html(&pptx);
    assert!(html.contains("<br/>"), "Should render line break");
    assert!(html.contains("Line 1"), "Should contain first line text");
    assert!(html.contains("Line 2"), "Should contain second line text");
}

// ── Month 4: Vertical text test ──

#[test]
fn test_vertical_text_rendering() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="VertText"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="1000000" cy="3000000"/></a:xfrm>
      </p:spPr>
      <p:txBody>
        <a:bodyPr vert="vert"/>
        <a:p><a:r><a:t>Vertical</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert_eq!(shape.vertical_text.as_deref(), Some("vert"));

    let html = render_html(&pptx);
    assert!(html.contains("writing-mode: vertical-rl"), "Should contain vertical writing mode");
}

#[test]
fn test_vertical_text_270() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Vert270"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="1000000" cy="3000000"/></a:xfrm>
      </p:spPr>
      <p:txBody>
        <a:bodyPr vert="vert270"/>
        <a:p><a:r><a:t>Rotated</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("writing-mode: vertical-lr"));
}

// ── Month 4: Text highlight test ──

#[test]
fn test_text_highlight() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Highlight"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:r>
            <a:rPr>
              <a:highlight><a:srgbClr val="FFFF00"/></a:highlight>
            </a:rPr>
            <a:t>Highlighted text</a:t>
          </a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let run = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs[0].runs[0];
    assert!(run.style.highlight.is_some(), "Should have highlight color");

    let html = render_html(&pptx);
    assert!(html.contains("background-color: #FFFF00"), "Should render highlight as background-color");
}

// ── Month 4: Text shadow test ──

#[test]
fn test_text_shadow() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Shadow"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:r>
            <a:rPr>
              <a:effectLst>
                <a:outerShdw blurRad="38100" dist="25400" dir="2700000">
                  <a:srgbClr val="000000"/>
                </a:outerShdw>
              </a:effectLst>
            </a:rPr>
            <a:t>Shadow text</a:t>
          </a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let run = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs[0].runs[0];
    assert!(run.style.shadow.is_some(), "Should have text shadow");
    let shadow = run.style.shadow.as_ref().unwrap();
    assert!(shadow.blur_rad > 0.0, "Blur radius should be positive");

    let html = render_html(&pptx);
    assert!(html.contains("text-shadow:"), "Should render text-shadow CSS");
}

// ── Month 4: Image crop test ──

#[test]
fn test_image_crop_parsing() {
    let slide = r#"
    <p:pic>
      <p:nvPicPr><p:cNvPr id="2" name="Pic"/><p:cNvPicPr/><p:nvPr/></p:nvPicPr>
      <p:blipFill>
        <a:blip r:embed="rId1"/>
        <a:srcRect l="10000" t="20000" r="15000" b="5000"/>
      </p:blipFill>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1500000"/></a:xfrm>
      </p:spPr>
    </p:pic>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    if let ShapeType::Picture(pic) = &shape.shape_type {
        assert!(pic.crop.is_some(), "Should have crop rect");
        let crop = pic.crop.as_ref().unwrap();
        assert!((crop.left - 0.1).abs() < 0.01, "Left crop should be ~0.1");
        assert!((crop.top - 0.2).abs() < 0.01, "Top crop should be ~0.2");
        assert!((crop.right - 0.15).abs() < 0.01, "Right crop should be ~0.15");
        assert!((crop.bottom - 0.05).abs() < 0.01, "Bottom crop should be ~0.05");
    } else {
        panic!("Expected Picture shape type");
    }
}

// ── Month 4: Chart detection test ──

#[test]
fn test_chart_detection() {
    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="Chart"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
          <c:chart r:id="rId2" xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert!(
        matches!(&shape.shape_type, ShapeType::Chart(_)),
        "Should detect chart in graphicFrame"
    );
}

#[test]
fn test_chart_renders_placeholder() {
    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="Chart"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
          <c:chart r:id="rId2" xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("chart-placeholder"), "Should render chart placeholder");
    assert!(html.contains("Chart"), "Should show Chart label");
}

// ── Month 4: CSS global classes test ──

#[test]
fn test_global_css_contains_svg_styles() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let html = render_html(&pptx);
    assert!(html.contains(".shape-svg"), "CSS should contain .shape-svg class");
    assert!(html.contains(".chart-placeholder"), "CSS should contain .chart-placeholder class");
}
