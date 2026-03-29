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
    assert!(
        html.contains("#4472C4"),
        "Theme accent1 color not found in HTML: {html}"
    );
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
    assert!(
        html.contains("#000000"),
        "ClrMap tx1→dk1 color not found: {html}"
    );
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
    assert!(
        html.contains("#ED7D31"),
        "accent2 fill color not found: {html}"
    );
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
    assert!(
        html.contains("linear-gradient"),
        "Gradient not found in HTML: {html}"
    );
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
    assert_eq!(
        shape.border.color.kind,
        color::ColorKind::Rgb("FF0000".to_string())
    );
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
    assert!(
        html.contains("v-bottom"),
        "v-bottom class not found: {html}"
    );
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
    assert!(matches!(shape.fill, Fill::NoFill));
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
    assert!(
        html.contains("shape-svg"),
        "Expected SVG rendering for preset shapes"
    );
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
    assert_eq!(
        *adj.get("adj").unwrap() as i64,
        25000,
        "adj should be 25000"
    );
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
    assert!(
        !html.contains("<svg viewBox="),
        "Rect should not generate SVG viewBox"
    );
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
    let para = &pres.slides[0].shapes[0]
        .text_body
        .as_ref()
        .unwrap()
        .paragraphs[0];
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
    assert!(
        html.contains("writing-mode: vertical-rl"),
        "Should contain vertical writing mode"
    );
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
    let run = &pres.slides[0].shapes[0]
        .text_body
        .as_ref()
        .unwrap()
        .paragraphs[0]
        .runs[0];
    assert!(run.style.highlight.is_some(), "Should have highlight color");

    let html = render_html(&pptx);
    assert!(
        html.contains("background-color: #FFFF00"),
        "Should render highlight as background-color"
    );
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
    let run = &pres.slides[0].shapes[0]
        .text_body
        .as_ref()
        .unwrap()
        .paragraphs[0]
        .runs[0];
    assert!(run.style.shadow.is_some(), "Should have text shadow");
    let shadow = run.style.shadow.as_ref().unwrap();
    assert!(shadow.blur_rad > 0.0, "Blur radius should be positive");

    let html = render_html(&pptx);
    assert!(
        html.contains("text-shadow:"),
        "Should render text-shadow CSS"
    );
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
        assert!(
            (crop.right - 0.15).abs() < 0.01,
            "Right crop should be ~0.15"
        );
        assert!(
            (crop.bottom - 0.05).abs() < 0.01,
            "Bottom crop should be ~0.05"
        );
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
    assert!(
        html.contains("chart-placeholder"),
        "Should render chart placeholder"
    );
    assert!(html.contains("Chart"), "Should show Chart label");
}

// ── Shape effect tests (outerShdw / glow) ──

#[test]
fn test_shape_outer_shadow_parsing() {
    // outerShdw: blurRad=50800 EMU (4pt), dist=38100 EMU (3pt), dir=2700000 (45deg)
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        <a:effectLst>
          <a:outerShdw blurRad="50800" dist="38100" dir="2700000">
            <a:srgbClr val="000000"><a:alpha val="40000"/></a:srgbClr>
          </a:outerShdw>
        </a:effectLst>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    let shadow = shape.effects.outer_shadow.as_ref().expect("outer_shadow should be parsed");
    // blurRad: 50800 EMU / 12700 = 4pt
    assert!((shadow.blur_radius - 4.0).abs() < 0.1, "blur_radius should be ~4pt, got {}", shadow.blur_radius);
    // dist: 38100 EMU / 12700 = 3pt
    assert!((shadow.distance - 3.0).abs() < 0.1, "distance should be ~3pt, got {}", shadow.distance);
    // dir: 2700000 / 60000 = 45 deg
    assert!((shadow.direction - 45.0).abs() < 0.1, "direction should be 45deg, got {}", shadow.direction);
}

#[test]
fn test_shape_outer_shadow_css_rendering() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:effectLst>
          <a:outerShdw blurRad="50800" dist="38100" dir="2700000">
            <a:srgbClr val="000000"/>
          </a:outerShdw>
        </a:effectLst>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(
        html.contains("box-shadow:"),
        "HTML should contain box-shadow for outerShdw: {html}"
    );
}

#[test]
fn test_shape_glow_parsing() {
    // glow: rad=63500 EMU (5pt)
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:effectLst>
          <a:glow rad="63500">
            <a:srgbClr val="FFC000"><a:alpha val="60000"/></a:srgbClr>
          </a:glow>
        </a:effectLst>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    let glow = shape.effects.glow.as_ref().expect("glow should be parsed");
    // rad: 63500 EMU / 12700 = 5pt
    assert!((glow.radius - 5.0).abs() < 0.1, "glow radius should be ~5pt, got {}", glow.radius);
}

#[test]
fn test_shape_glow_css_rendering() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:effectLst>
          <a:glow rad="63500">
            <a:srgbClr val="FFC000"/>
          </a:glow>
        </a:effectLst>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(
        html.contains("box-shadow:"),
        "HTML should contain box-shadow for glow: {html}"
    );
    // Glow uses spread with zero offset
    assert!(
        html.contains("0 0"),
        "Glow box-shadow should have zero offsets: {html}"
    );
}

#[test]
fn test_shape_combined_shadow_and_glow() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:effectLst>
          <a:outerShdw blurRad="50800" dist="38100" dir="2700000">
            <a:srgbClr val="000000"/>
          </a:outerShdw>
          <a:glow rad="63500">
            <a:srgbClr val="FFC000"/>
          </a:glow>
        </a:effectLst>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert!(shape.effects.outer_shadow.is_some(), "outer_shadow should be parsed");
    assert!(shape.effects.glow.is_some(), "glow should be parsed");

    let html = render_html(&pptx);
    // Combined box-shadow should have comma-separated values
    assert!(
        html.contains("box-shadow:"),
        "HTML should contain box-shadow: {html}"
    );
}

#[test]
fn test_shape_no_effects_no_box_shadow() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // Extract the shape div's style attribute (not global CSS which has slide box-shadow)
    let shape_div_start = html.find("<div class=\"shape\"").expect("shape div should exist");
    let shape_section = &html[shape_div_start..shape_div_start + 300.min(html.len() - shape_div_start)];
    assert!(
        !shape_section.contains("box-shadow"),
        "No effects means no box-shadow on shape div: {shape_section}"
    );
}

#[test]
fn test_shape_shadow_with_scheme_color() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1500000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:effectLst>
          <a:outerShdw blurRad="50800" dist="38100" dir="5400000">
            <a:schemeClr val="dk1"/>
          </a:outerShdw>
        </a:effectLst>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    let shadow = shape.effects.outer_shadow.as_ref().expect("should parse scheme color shadow");
    // dir: 5400000 / 60000 = 90 deg (straight down)
    assert!((shadow.direction - 90.0).abs() < 0.1, "direction should be 90deg");
}

// ── Month 4: CSS global classes test ──

#[test]
fn test_global_css_contains_svg_styles() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let html = render_html(&pptx);
    assert!(
        html.contains(".shape-svg"),
        "CSS should contain .shape-svg class"
    );
    assert!(
        html.contains(".chart-placeholder"),
        "CSS should contain .chart-placeholder class"
    );
}

// ── Connector line color tests ──

#[test]
fn test_connector_border_color_srgb() {
    // Connector with inline srgbClr in <a:ln> — must parse border color
    let slide = r#"
    <p:cxnSp>
      <p:nvCxnSpPr><p:cNvPr id="2" name="Connector"/><p:cNvCxnSpPr/><p:nvPr/></p:nvCxnSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="0"/></a:xfrm>
        <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
        <a:ln w="9525">
          <a:solidFill><a:srgbClr val="C00000"/></a:solidFill>
        </a:ln>
      </p:spPr>
    </p:cxnSp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert!(
        shape.border.width > 0.0,
        "border width should be > 0, got {}",
        shape.border.width
    );
    assert_eq!(
        shape.border.color.kind,
        color::ColorKind::Rgb("C00000".to_string()),
        "border color should be C00000, got {:?}",
        shape.border.color
    );
}

#[test]
fn test_connector_with_style_and_inline_color() {
    // Connector with both inline <a:ln> color AND <p:style> (real-world pattern)
    let slide = r#"
    <p:cxnSp>
      <p:nvCxnSpPr><p:cNvPr id="2" name="Connector"/><p:cNvCxnSpPr/><p:nvPr/></p:nvCxnSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="0"/></a:xfrm>
        <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
        <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
          <a:solidFill><a:srgbClr val="C00000"/></a:solidFill>
          <a:prstDash val="dash"/>
          <a:round/>
          <a:headEnd type="none" w="med" len="med"/>
          <a:tailEnd type="none" w="med" len="med"/>
        </a:ln>
      </p:spPr>
      <p:style>
        <a:lnRef idx="0"><a:scrgbClr r="0" g="0" b="0"/></a:lnRef>
        <a:fillRef idx="0"><a:scrgbClr r="0" g="0" b="0"/></a:fillRef>
        <a:effectRef idx="0"><a:scrgbClr r="0" g="0" b="0"/></a:effectRef>
        <a:fontRef idx="minor"><a:schemeClr val="tx1"/></a:fontRef>
      </p:style>
    </p:cxnSp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert!(
        shape.border.width > 0.0,
        "border width should be > 0, got {}",
        shape.border.width
    );
    assert_eq!(
        shape.border.color.kind,
        color::ColorKind::Rgb("C00000".to_string()),
        "border color should be C00000 with style element present, got {:?}",
        shape.border.color
    );
}

#[test]
fn test_connector_border_color_srgb_html() {
    // Connector rendered as SVG should use the parsed stroke color, not black
    let slide = r#"
    <p:cxnSp>
      <p:nvCxnSpPr><p:cNvPr id="2" name="Connector"/><p:cNvCxnSpPr/><p:nvPr/></p:nvCxnSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="0"/></a:xfrm>
        <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
        <a:ln w="9525">
          <a:solidFill><a:srgbClr val="C00000"/></a:solidFill>
        </a:ln>
      </p:spPr>
    </p:cxnSp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(
        html.contains("#C00000"),
        "HTML should contain connector stroke color #C00000, got: {html}"
    );
}

// ── noFill inside <a:ln> suppresses border ──

#[test]
fn test_ln_nofill_suppresses_border() {
    // Shape with <a:ln><a:noFill/></a:ln> must have zero-width border
    // even though <a:ln> is present (regression: was defaulting to 1pt black)
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Donut"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="200000" cy="200000"/></a:xfrm>
        <a:prstGeom prst="donut"><a:avLst/></a:prstGeom>
        <a:solidFill><a:srgbClr val="C00000"/></a:solidFill>
        <a:ln><a:noFill/></a:ln>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    assert_eq!(
        shape.border.width, 0.0,
        "noFill inside <a:ln> should yield border width 0, got {}",
        shape.border.width
    );
    assert!(
        shape.border.no_fill,
        "noFill inside <a:ln> should set no_fill flag"
    );
}

#[test]
fn test_ln_nofill_no_black_stroke_in_svg() {
    // SVG output for shape with <a:ln><a:noFill/></a:ln> must NOT have #000 stroke
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Donut"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="200000" cy="200000"/></a:xfrm>
        <a:prstGeom prst="donut"><a:avLst/></a:prstGeom>
        <a:solidFill><a:srgbClr val="C00000"/></a:solidFill>
        <a:ln><a:noFill/></a:ln>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(
        !html.contains("stroke=\"#000\""),
        "noFill inside <a:ln> must not produce #000 stroke: {html}"
    );
}

// ── Background gradient fill tests ──

#[test]
fn test_background_gradient_fill_parsing() {
    // Slide with background gradient fill using self-closing color tags (Empty events)
    let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
            <a:gs pos="50000"><a:srgbClr val="00FF00"/></a:gs>
            <a:gs pos="100000"><a:srgbClr val="0000FF"/></a:gs>
          </a:gsLst>
          <a:lin ang="5400000" scaled="0"/>
        </a:gradFill>
        <a:effectLst/>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

    let pptx = fixtures::MinimalPptx::new("").with_raw_slide(slide_xml).build();
    let pres = parse_pptx(&pptx);
    let slide = &pres.slides[0];
    match &slide.background {
        Some(Fill::Gradient(gf)) => {
            assert_eq!(gf.stops.len(), 3, "Expected 3 gradient stops");
            assert!((gf.stops[0].position - 0.0).abs() < 0.01);
            assert!((gf.stops[1].position - 0.5).abs() < 0.01);
            assert!((gf.stops[2].position - 1.0).abs() < 0.01);
            assert!((gf.angle - 90.0).abs() < 0.1);
        }
        other => panic!("Expected Fill::Gradient for background, got: {other:?}"),
    }
}

#[test]
fn test_background_gradient_fill_html() {
    let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
            <a:gs pos="100000"><a:srgbClr val="0000FF"/></a:gs>
          </a:gsLst>
          <a:lin ang="5400000"/>
        </a:gradFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

    let pptx = fixtures::MinimalPptx::new("").with_raw_slide(slide_xml).build();
    let html = render_html(&pptx);
    assert!(
        html.contains("linear-gradient"),
        "Background gradient should produce linear-gradient CSS: {html}"
    );
    assert!(
        html.contains("#FF0000"),
        "Start color not found in gradient CSS"
    );
    assert!(
        html.contains("#0000FF"),
        "End color not found in gradient CSS"
    );
}

#[test]
fn test_background_gradient_with_scheme_colors() {
    // Background gradient using schemeClr (self-closing Empty event)
    let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:schemeClr val="accent1"/></a:gs>
            <a:gs pos="100000"><a:schemeClr val="accent2"/></a:gs>
          </a:gsLst>
          <a:lin ang="0"/>
        </a:gradFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

    let pptx = fixtures::MinimalPptx::new("").with_raw_slide(slide_xml).build();
    let pres = parse_pptx(&pptx);
    let slide = &pres.slides[0];
    match &slide.background {
        Some(Fill::Gradient(gf)) => {
            assert_eq!(gf.stops.len(), 2, "Expected 2 gradient stops");
        }
        other => panic!("Expected Fill::Gradient for scheme color background, got: {other:?}"),
    }
}

#[test]
fn test_background_image_fill() {
    // Build a PPTX with a background image using blipFill
    use std::io::{Cursor, Write};
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    let slide_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:blipFill>
          <a:blip r:embed="rId2"/>
          <a:stretch><a:fillRect/></a:stretch>
        </a:blipFill>
        <a:effectLst/>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sld>"#;

    let slide_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/image1.png"/>
</Relationships>"#;

    // Create a minimal 1x1 PNG
    let png_data: Vec<u8> = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, // 8-bit RGB
        0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, // IDAT chunk
        0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x01, 0xE2, 0x21, 0xBC, 0x33,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, // IEND chunk
        0xAE, 0x42, 0x60, 0x82,
    ];

    let buf = Vec::new();
    let cursor = Cursor::new(buf);
    let mut zip = ZipWriter::new(cursor);
    let opts = SimpleFileOptions::default();

    let content_types = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="png" ContentType="image/png"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
</Types>"#;

    zip.start_file("[Content_Types].xml", opts).unwrap();
    zip.write_all(content_types.as_bytes()).unwrap();

    zip.start_file("_rels/.rels", opts).unwrap();
    zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#).unwrap();

    zip.start_file("ppt/presentation.xml", opts).unwrap();
    zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst><p:sldMasterId r:id="rId1"/></p:sldMasterIdLst>
  <p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
</p:presentation>"#).unwrap();

    zip.start_file("ppt/_rels/presentation.xml.rels", opts).unwrap();
    zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
</Relationships>"#).unwrap();

    zip.start_file("ppt/slides/slide1.xml", opts).unwrap();
    zip.write_all(slide_xml.as_bytes()).unwrap();

    zip.start_file("ppt/slides/_rels/slide1.xml.rels", opts).unwrap();
    zip.write_all(slide_rels.as_bytes()).unwrap();

    zip.start_file("ppt/media/image1.png", opts).unwrap();
    zip.write_all(&png_data).unwrap();

    zip.start_file("ppt/theme/theme1.xml", opts).unwrap();
    zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="T">
  <a:themeElements>
    <a:clrScheme name="O">
      <a:dk1><a:srgbClr val="000000"/></a:dk1>
      <a:lt1><a:srgbClr val="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="44546A"/></a:dk2>
      <a:lt2><a:srgbClr val="E7E6E6"/></a:lt2>
      <a:accent1><a:srgbClr val="4472C4"/></a:accent1>
      <a:accent2><a:srgbClr val="ED7D31"/></a:accent2>
      <a:accent3><a:srgbClr val="A5A5A5"/></a:accent3>
      <a:accent4><a:srgbClr val="FFC000"/></a:accent4>
      <a:accent5><a:srgbClr val="5B9BD5"/></a:accent5>
      <a:accent6><a:srgbClr val="70AD47"/></a:accent6>
      <a:hlink><a:srgbClr val="0563C1"/></a:hlink>
      <a:folHlink><a:srgbClr val="954F72"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name="O"><a:majorFont><a:latin typeface="Calibri"/></a:majorFont><a:minorFont><a:latin typeface="Calibri"/></a:minorFont></a:fontScheme>
  </a:themeElements>
</a:theme>"#).unwrap();

    let data = zip.finish().unwrap().into_inner();

    // Verify parsing produces Fill::Image
    let pres = parse_pptx(&data);
    let slide = &pres.slides[0];
    match &slide.background {
        Some(Fill::Image(img)) => {
            assert!(!img.data.is_empty(), "Image data should not be empty");
            assert_eq!(img.content_type, "image/png");
        }
        other => panic!("Expected Fill::Image for background, got: {other:?}"),
    }

    // Verify HTML rendering produces background-image
    let html = render_html(&data);
    assert!(
        html.contains("background-image"),
        "Background image should produce background-image CSS"
    );
    assert!(
        html.contains("data:image/png;base64,"),
        "Background image should be embedded as base64"
    );
}

// ── Gradient fill with color modifiers (Start+End events) ──

#[test]
fn test_gradient_fill_with_modifiers() {
    // Real-world gradient: colors have child modifiers (tint, satMod),
    // making them Start+End events instead of Empty events.
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="roundRect"/>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0">
              <a:srgbClr val="FF0000">
                <a:tint val="50000"/>
              </a:srgbClr>
            </a:gs>
            <a:gs pos="100000">
              <a:srgbClr val="0000FF">
                <a:shade val="80000"/>
              </a:srgbClr>
            </a:gs>
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
            assert_eq!(gf.stops.len(), 2, "Expected 2 gradient stops, got {}", gf.stops.len());
            assert!((gf.stops[0].position - 0.0).abs() < 0.01);
            assert!((gf.stops[1].position - 1.0).abs() < 0.01);
            assert!((gf.angle - 90.0).abs() < 0.1);
        }
        other => panic!("Expected Fill::Gradient with modifiers, got: {other:?}"),
    }
}

#[test]
fn test_gradient_fill_with_modifiers_html() {
    // Gradient with color modifiers should produce SVG linearGradient for SVG shapes
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="roundRect"/>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0">
              <a:srgbClr val="FF0000">
                <a:tint val="50000"/>
              </a:srgbClr>
            </a:gs>
            <a:gs pos="100000">
              <a:srgbClr val="0000FF">
                <a:shade val="80000"/>
              </a:srgbClr>
            </a:gs>
          </a:gsLst>
          <a:lin ang="5400000"/>
        </a:gradFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // SVG shapes (roundRect) use SVG <linearGradient> + url(#gradN)
    assert!(
        html.contains("linearGradient"),
        "SVG gradient def not found in HTML: {html}"
    );
    assert!(
        html.contains("url(#grad"),
        "SVG gradient url reference not found in HTML: {html}"
    );
}

#[test]
fn test_gradient_fill_rect_css() {
    // Non-SVG rect shapes should use CSS linear-gradient
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0">
              <a:srgbClr val="FF0000">
                <a:tint val="50000"/>
              </a:srgbClr>
            </a:gs>
            <a:gs pos="100000">
              <a:srgbClr val="0000FF">
                <a:shade val="80000"/>
              </a:srgbClr>
            </a:gs>
          </a:gsLst>
          <a:lin ang="5400000"/>
        </a:gradFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(
        html.contains("linear-gradient"),
        "CSS gradient not found in HTML for rect shape: {html}"
    );
}

#[test]
fn test_gradient_fill_scheme_colors_with_modifiers() {
    // Real-world pattern: schemeClr with tint/satMod modifiers in shape gradient
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="roundRect"/>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0">
              <a:schemeClr val="accent1">
                <a:tint val="66000"/>
                <a:satMod val="160000"/>
              </a:schemeClr>
            </a:gs>
            <a:gs pos="50000">
              <a:schemeClr val="accent1">
                <a:tint val="44500"/>
                <a:satMod val="160000"/>
              </a:schemeClr>
            </a:gs>
            <a:gs pos="100000">
              <a:schemeClr val="accent1">
                <a:tint val="23500"/>
                <a:satMod val="160000"/>
              </a:schemeClr>
            </a:gs>
          </a:gsLst>
          <a:lin ang="5400000" scaled="0"/>
        </a:gradFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    match &shape.fill {
        Fill::Gradient(gf) => {
            assert_eq!(gf.stops.len(), 3, "Expected 3 gradient stops, got {}", gf.stops.len());
            assert!((gf.stops[0].position - 0.0).abs() < 0.01);
            assert!((gf.stops[1].position - 0.5).abs() < 0.01);
            assert!((gf.stops[2].position - 1.0).abs() < 0.01);
        }
        other => panic!("Expected Fill::Gradient with scheme colors, got: {other:?}"),
    }
}
