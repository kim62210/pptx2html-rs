//! Edge-case and robustness tests for pptx2html-core

mod fixtures;

use pptx2html_core::model::*;

fn parse_pptx(data: &[u8]) -> pptx2html_core::model::Presentation {
    pptx2html_core::parser::PptxParser::parse_bytes(data).expect("PPTX parsing failed")
}

fn render_html(data: &[u8]) -> String {
    let pres = parse_pptx(data);
    pptx2html_core::renderer::HtmlRenderer::render(&pres).expect("HTML rendering failed")
}

// ── Empty / minimal presentations ──

#[test]
fn test_empty_presentation_zero_shapes() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides.len(), 1);
    assert!(pres.slides[0].shapes.is_empty());
}

#[test]
fn test_empty_slide_renders_valid_html() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let html = render_html(&pptx);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<div class=\"slide\""));
    assert!(html.contains("</html>"));
}

#[test]
fn test_shape_with_no_text_body() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="AABBCC"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides[0].shapes.len(), 1);
    assert!(pres.slides[0].shapes[0].text_body.is_none());

    let html = render_html(&pptx);
    assert!(html.contains("#AABBCC"));
    // CSS contains ".text-body" class definition, but no actual <div class="text-body"> inside shape
    assert!(
        !html.contains("<div class=\"text-body"),
        "Shape without text body should not render text-body div"
    );
}

#[test]
fn test_shape_with_empty_text() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:t>   </a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("text-body"));
}

#[test]
fn test_very_long_text() {
    let long_text = "A".repeat(2000);
    let slide = format!(
        r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="LongText"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:t>{long_text}</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#
    );

    let pptx = fixtures::MinimalPptx::new(&slide).build();
    let html = render_html(&pptx);
    assert!(html.contains(&long_text));
}

// ── Unicode text tests ──

#[test]
fn test_unicode_cjk_text() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="CJK"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:t>한국어 日本語 中文</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("한국어"), "Korean text not found");
    assert!(html.contains("日本語"), "Japanese text not found");
    assert!(html.contains("中文"), "Chinese text not found");
}

#[test]
fn test_unicode_emoji_text() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Emoji"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:t>Hello World! 🚀🎉</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("Hello World!"), "Text not found");
}

// ── Unsupported content detection ──

#[test]
fn test_smartart_placeholder() {
    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="Diagram"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/diagram">
          <dgm:relIds xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram" r:dm="rId2"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides[0].shapes.len(), 1);
    assert!(
        matches!(&pres.slides[0].shapes[0].shape_type, ShapeType::Unsupported(data) if data.label == "SmartArt"),
        "Expected Unsupported(SmartArt)"
    );

    let html = render_html(&pptx);
    assert!(
        html.contains("[SmartArt]"),
        "SmartArt placeholder label not rendered"
    );
}

#[test]
fn test_ole_object_placeholder() {
    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="OLE"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/schemas/oleObject">
          <p:oleObj r:id="rId2"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides[0].shapes.len(), 1);
    assert!(
        matches!(&pres.slides[0].shapes[0].shape_type, ShapeType::Unsupported(data) if data.label == "OLE Object"),
        "Expected Unsupported(OLE Object)"
    );

    let html = render_html(&pptx);
    assert!(
        html.contains("[OLE Object]"),
        "OLE placeholder label not rendered"
    );
}

#[test]
fn test_math_equation_placeholder() {
    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="Math"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/officeDocument/2006/math">
          <m:oMath xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides[0].shapes.len(), 1);
    assert!(
        matches!(&pres.slides[0].shapes[0].shape_type, ShapeType::Unsupported(data) if data.label == "Math Equation"),
        "Expected Unsupported(Math Equation)"
    );

    let html = render_html(&pptx);
    assert!(
        html.contains("[Math Equation]"),
        "Math placeholder label not rendered"
    );
}

// ── Malformed input handling ──

#[test]
fn test_invalid_zip_returns_error() {
    let result = pptx2html_core::convert_bytes(b"this is not a zip file");
    assert!(result.is_err(), "Should fail on invalid ZIP");
}

#[test]
fn test_password_protected_detection() {
    // Build a fake "encrypted" package with EncryptedPackage entry
    use std::io::{Cursor, Write};
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    let buf = Vec::new();
    let cursor = Cursor::new(buf);
    let mut zip = ZipWriter::new(cursor);
    let opts = SimpleFileOptions::default();

    zip.start_file("EncryptedPackage", opts).expect("zip entry");
    zip.write_all(b"fake-encrypted-data").expect("write");
    zip.start_file("EncryptionInfo", opts).expect("zip entry");
    zip.write_all(b"fake-info").expect("write");
    let data = zip.finish().expect("finish").into_inner();

    let result = pptx2html_core::convert_bytes(&data);
    assert!(result.is_err(), "Should reject password-protected PPTX");
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("password-protected"),
        "Error should mention password-protected: {err_msg}"
    );
}

// ── Preset shape rendering (no panic) ──

#[test]
fn test_all_basic_preset_shapes_no_panic() {
    let presets = [
        "rect",
        "roundRect",
        "ellipse",
        "triangle",
        "rtTriangle",
        "diamond",
        "parallelogram",
        "trapezoid",
        "pentagon",
        "hexagon",
        "octagon",
    ];

    for prst in &presets {
        let slide = format!(
            r#"
            <p:sp>
              <p:nvSpPr><p:cNvPr id="2" name="Shape"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
              <p:spPr>
                <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
                <a:prstGeom prst="{prst}"/>
              </p:spPr>
            </p:sp>"#
        );
        let pptx = fixtures::MinimalPptx::new(&slide).build();
        let html = render_html(&pptx);
        assert!(
            html.contains("class=\"slide\""),
            "Preset shape '{prst}' failed to render"
        );
    }
}

#[test]
fn test_arrow_preset_shapes_no_panic() {
    let presets = [
        "rightArrow",
        "leftArrow",
        "upArrow",
        "downArrow",
        "leftRightArrow",
        "upDownArrow",
        "chevron",
    ];

    for prst in &presets {
        let slide = format!(
            r#"
            <p:sp>
              <p:nvSpPr><p:cNvPr id="2" name="Arrow"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
              <p:spPr>
                <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
                <a:prstGeom prst="{prst}"/>
              </p:spPr>
            </p:sp>"#
        );
        let pptx = fixtures::MinimalPptx::new(&slide).build();
        let html = render_html(&pptx);
        assert!(
            html.contains("class=\"slide\""),
            "Arrow shape '{prst}' failed to render"
        );
    }
}

// ── Missing theme graceful fallback ──

#[test]
fn test_theme_color_fallback_without_theme() {
    // schemeClr with no theme available should not panic
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:r>
            <a:rPr sz="1800"><a:solidFill><a:schemeClr val="unknownSchemeColor"/></a:solidFill></a:rPr>
            <a:t>Fallback Color</a:t>
          </a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    // Should not panic even with unknown scheme color
    let html = render_html(&pptx);
    assert!(html.contains("Fallback Color"));
}

#[test]
fn test_invalid_color_value_graceful() {
    // An invalid srgbClr value should not panic
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="ZZZZZZ"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    // Should not panic, just skip the invalid color
    let html = render_html(&pptx);
    assert!(html.contains("class=\"slide\""));
}

// ── ConversionOptions edge cases ──

#[test]
fn test_conversion_options_empty_slide_range() {
    use pptx2html_core::ConversionOptions;

    let opts = ConversionOptions {
        slide_range: Some((5, 10)),
        ..Default::default()
    };
    // Slide 1 with range (5, 10) should be excluded
    assert!(!opts.should_include_slide(1, false));
    assert!(opts.should_include_slide(5, false));
    assert!(opts.should_include_slide(10, false));
    assert!(!opts.should_include_slide(11, false));
}

#[test]
fn test_conversion_options_hidden_slide() {
    use pptx2html_core::ConversionOptions;

    let opts_no_hidden = ConversionOptions::default();
    assert!(!opts_no_hidden.should_include_slide(1, true));

    let opts_with_hidden = ConversionOptions {
        include_hidden: true,
        ..Default::default()
    };
    assert!(opts_with_hidden.should_include_slide(1, true));
}

// ── Group shape nesting ──

#[test]
fn test_nested_group_shapes() {
    // A group containing a shape — should not panic
    let slide = r#"
    <p:grpSp>
      <p:nvGrpSpPr><p:cNvPr id="10" name="Group"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="100000" y="100000"/>
          <a:ext cx="5000000" cy="3000000"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="5000000" cy="3000000"/>
        </a:xfrm>
      </p:grpSpPr>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="11" name="InnerShape"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="500000" y="500000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"/>
          <a:solidFill><a:srgbClr val="AABB00"/></a:solidFill>
        </p:spPr>
      </p:sp>
    </p:grpSp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides[0].shapes.len(), 1);
    assert!(
        matches!(&pres.slides[0].shapes[0].shape_type, ShapeType::Group(children, _) if children.len() == 1),
        "Expected group with one child"
    );

    let html = render_html(&pptx);
    assert!(html.contains("#AABB00"), "Inner shape fill not rendered");
}

// ── get_info / get_info_from_bytes ──

#[test]
fn test_get_info_from_bytes() {
    let pptx = fixtures::MinimalPptx::new("").build();
    let info = pptx2html_core::get_info_from_bytes(&pptx).expect("get_info_from_bytes failed");
    assert_eq!(info.slide_count, 1);
    assert!((info.width_px - 960.0).abs() < 0.1);
    assert!((info.height_px - 720.0).abs() < 0.1);
}

// ── Metadata sideband tests ──

#[test]
fn test_smartart_metadata() {
    use pptx2html_core::model::slide::UnresolvedType;

    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="Diagram"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="200000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/diagram">
          <dgm:relIds xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram" r:dm="rId1" r:lo="rId2"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let result = pptx2html_core::convert_bytes_with_metadata(&pptx).expect("conversion failed");

    // HTML contains structured placeholder
    assert!(
        result.html.contains("unresolved-element"),
        "HTML should contain unresolved-element class"
    );
    assert!(
        result.html.contains("data-type=\"smartart\""),
        "HTML should contain data-type=smartart attribute"
    );
    assert!(
        result.html.contains("data-slide=\"0\""),
        "HTML should contain data-slide attribute"
    );
    assert!(
        result.html.contains("id=\"unresolved-s0-e0\""),
        "HTML should contain structured placeholder ID"
    );

    // Metadata
    assert_eq!(result.unresolved_elements.len(), 1);
    let elem = &result.unresolved_elements[0];
    assert_eq!(elem.element_type, UnresolvedType::SmartArt);
    assert_eq!(elem.slide_index, 0);
    assert!(elem.placeholder_id.starts_with("unresolved-"));

    // Raw XML captured
    assert!(
        elem.raw_xml.is_some(),
        "SmartArt raw_xml should be captured"
    );
    let raw = elem.raw_xml.as_ref().expect("raw_xml");
    assert!(
        raw.contains("relIds") || raw.contains("dgm"),
        "Raw XML should contain diagram-related content: {raw}"
    );
}

#[test]
fn test_math_metadata() {
    use pptx2html_core::model::slide::UnresolvedType;

    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="Math"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/officeDocument/2006/math">
          <m:oMath xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let result = pptx2html_core::convert_bytes_with_metadata(&pptx).expect("conversion failed");

    assert_eq!(result.unresolved_elements.len(), 1);
    let elem = &result.unresolved_elements[0];
    assert_eq!(elem.element_type, UnresolvedType::MathEquation);
    assert!(result.html.contains("data-type=\"math\""));
}

#[test]
fn test_ole_metadata() {
    use pptx2html_core::model::slide::UnresolvedType;

    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="OLE"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/schemas/oleObject">
          <p:oleObj r:id="rId2" progId="Excel.Sheet.12"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let result = pptx2html_core::convert_bytes_with_metadata(&pptx).expect("conversion failed");

    assert_eq!(result.unresolved_elements.len(), 1);
    let elem = &result.unresolved_elements[0];
    assert_eq!(elem.element_type, UnresolvedType::OleObject);
    assert!(result.html.contains("data-type=\"ole\""));

    // Raw XML should capture OLE attributes
    assert!(elem.raw_xml.is_some());
    let raw = elem.raw_xml.as_ref().expect("raw_xml");
    assert!(
        raw.contains("oleObj") || raw.contains("progId"),
        "OLE raw XML should contain object attributes: {raw}"
    );
}

#[test]
fn test_no_unresolved_for_normal_shapes() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:t>Normal text</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let result = pptx2html_core::convert_bytes_with_metadata(&pptx).expect("conversion failed");

    assert!(
        result.unresolved_elements.is_empty(),
        "Normal shapes should produce no unresolved elements"
    );
    assert_eq!(result.slide_count, 1);
    assert!(result.html.contains("Normal text"));
}

#[test]
fn test_backward_compat_convert_bytes() {
    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="Diagram"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="200000"/><a:ext cx="5000000" cy="3000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/diagram">
          <dgm:relIds xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram" r:dm="rId1"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();

    // Existing API still returns String, not ConversionResult
    let html: String = pptx2html_core::convert_bytes(&pptx).expect("conversion failed");
    assert!(html.contains("[SmartArt]"), "Backward compat: label should still appear");
    assert!(html.contains("<!DOCTYPE html>"), "Should be complete HTML");
}

#[test]
fn test_multiple_unresolved_elements_unique_ids() {
    let slide = r#"
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="2" name="SmartArt1"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="100000"/><a:ext cx="4000000" cy="2000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/diagram">
          <dgm:relIds xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram" r:dm="rId1"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>
    <p:graphicFrame>
      <p:nvGraphicFramePr><p:cNvPr id="3" name="Math1"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
      <p:xfrm><a:off x="100000" y="3000000"/><a:ext cx="4000000" cy="2000000"/></p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/officeDocument/2006/math">
          <m:oMath xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let result = pptx2html_core::convert_bytes_with_metadata(&pptx).expect("conversion failed");

    assert_eq!(result.unresolved_elements.len(), 2);

    // Each element should have a unique placeholder_id
    let id0 = &result.unresolved_elements[0].placeholder_id;
    let id1 = &result.unresolved_elements[1].placeholder_id;
    assert_ne!(id0, id1, "Placeholder IDs must be unique");

    // IDs should be present in HTML
    assert!(result.html.contains(id0.as_str()));
    assert!(result.html.contains(id1.as_str()));
}

#[test]
fn test_conversion_result_slide_count() {
    use pptx2html_core::ConversionOptions;

    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();

    // Default options
    let result = pptx2html_core::convert_bytes_with_metadata(&pptx).expect("conversion failed");
    assert_eq!(result.slide_count, 1);

    // With options
    let opts = ConversionOptions::default();
    let result = pptx2html_core::convert_bytes_with_options_metadata(&pptx, &opts)
        .expect("conversion failed");
    assert_eq!(result.slide_count, 1);
}
