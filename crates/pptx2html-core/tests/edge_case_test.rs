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

// ── Custom Geometry (custGeom) tests ──

#[test]
fn test_custgeom_triangle_parsed() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="CustTriangle"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst>
            <a:path w="21600" h="21600">
              <a:moveTo><a:pt x="0" y="21600"/></a:moveTo>
              <a:lnTo><a:pt x="10800" y="0"/></a:lnTo>
              <a:lnTo><a:pt x="21600" y="21600"/></a:lnTo>
              <a:close/>
            </a:path>
          </a:pathLst>
        </a:custGeom>
        <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    assert_eq!(pres.slides[0].shapes.len(), 1);
    let shape = &pres.slides[0].shapes[0];

    match &shape.shape_type {
        ShapeType::CustomGeom(geom) => {
            assert_eq!(geom.paths.len(), 1);
            let path = &geom.paths[0];
            assert!((path.width - 21600.0).abs() < 0.01);
            assert!((path.height - 21600.0).abs() < 0.01);
            assert_eq!(path.commands.len(), 4); // moveTo + 2 lnTo + close
        }
        other => panic!("Expected CustomGeom, got {:?}", other),
    }
}

#[test]
fn test_custgeom_renders_svg_path() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="CustShape"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst>
            <a:path w="100" h="100">
              <a:moveTo><a:pt x="0" y="0"/></a:moveTo>
              <a:lnTo><a:pt x="100" y="0"/></a:lnTo>
              <a:lnTo><a:pt x="100" y="100"/></a:lnTo>
              <a:lnTo><a:pt x="0" y="100"/></a:lnTo>
              <a:close/>
            </a:path>
          </a:pathLst>
        </a:custGeom>
        <a:solidFill><a:srgbClr val="0000FF"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // SVG should be present with path element
    assert!(html.contains("<svg viewBox="), "Should contain SVG element");
    assert!(html.contains("<path d="), "Should contain SVG path");
    assert!(html.contains("shape-svg"), "Should have shape-svg class");
}

#[test]
fn test_custgeom_cubic_bezier() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="CustBez"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst>
            <a:path w="21600" h="21600">
              <a:moveTo><a:pt x="0" y="0"/></a:moveTo>
              <a:cubicBezTo>
                <a:pt x="7200" y="0"/>
                <a:pt x="14400" y="21600"/>
                <a:pt x="21600" y="21600"/>
              </a:cubicBezTo>
            </a:path>
          </a:pathLst>
        </a:custGeom>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];

    match &shape.shape_type {
        ShapeType::CustomGeom(geom) => {
            assert_eq!(geom.paths[0].commands.len(), 2); // moveTo + cubicBezTo
            match &geom.paths[0].commands[1] {
                PathCommand::CubicBezTo { x1, y1, x2, y2, x, y } => {
                    assert!((x1 - 7200.0).abs() < 0.01);
                    assert!((*y1).abs() < 0.01);
                    assert!((x2 - 14400.0).abs() < 0.01);
                    assert!((y2 - 21600.0).abs() < 0.01);
                    assert!((x - 21600.0).abs() < 0.01);
                    assert!((y - 21600.0).abs() < 0.01);
                }
                other => panic!("Expected CubicBezTo, got {:?}", other),
            }
        }
        other => panic!("Expected CustomGeom, got {:?}", other),
    }
}

#[test]
fn test_custgeom_quad_bezier() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="CustQuad"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst>
            <a:path w="100" h="100">
              <a:moveTo><a:pt x="0" y="100"/></a:moveTo>
              <a:quadBezTo>
                <a:pt x="50" y="0"/>
                <a:pt x="100" y="100"/>
              </a:quadBezTo>
            </a:path>
          </a:pathLst>
        </a:custGeom>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];

    match &shape.shape_type {
        ShapeType::CustomGeom(geom) => {
            assert_eq!(geom.paths[0].commands.len(), 2); // moveTo + quadBezTo
            match &geom.paths[0].commands[1] {
                PathCommand::QuadBezTo { x1, y1, x, y } => {
                    assert!((x1 - 50.0).abs() < 0.01);
                    assert!((*y1).abs() < 0.01);
                    assert!((x - 100.0).abs() < 0.01);
                    assert!((y - 100.0).abs() < 0.01);
                }
                other => panic!("Expected QuadBezTo, got {:?}", other),
            }
        }
        other => panic!("Expected CustomGeom, got {:?}", other),
    }
}

#[test]
fn test_custgeom_arc_to() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="CustArc"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst>
            <a:path w="21600" h="21600">
              <a:moveTo><a:pt x="0" y="10800"/></a:moveTo>
              <a:arcTo wR="5400" hR="5400" stAng="0" swAng="5400000"/>
            </a:path>
          </a:pathLst>
        </a:custGeom>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];

    match &shape.shape_type {
        ShapeType::CustomGeom(geom) => {
            assert_eq!(geom.paths[0].commands.len(), 2); // moveTo + arcTo
            match &geom.paths[0].commands[1] {
                PathCommand::ArcTo { wr, hr, start_angle, swing_angle } => {
                    assert!((wr - 5400.0).abs() < 0.01);
                    assert!((hr - 5400.0).abs() < 0.01);
                    assert!((*start_angle).abs() < 0.01);
                    assert!((swing_angle - 5400000.0).abs() < 0.01);
                }
                other => panic!("Expected ArcTo, got {:?}", other),
            }
        }
        other => panic!("Expected CustomGeom, got {:?}", other),
    }

    // Verify it renders as SVG with arc command
    let html = render_html(&pptx);
    assert!(html.contains("<path d="), "Should render SVG path with arc");
}

#[test]
fn test_custgeom_multiple_paths() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="MultiPath"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst>
            <a:path w="100" h="100">
              <a:moveTo><a:pt x="0" y="0"/></a:moveTo>
              <a:lnTo><a:pt x="100" y="100"/></a:lnTo>
            </a:path>
            <a:path w="100" h="100" fill="none">
              <a:moveTo><a:pt x="100" y="0"/></a:moveTo>
              <a:lnTo><a:pt x="0" y="100"/></a:lnTo>
            </a:path>
          </a:pathLst>
        </a:custGeom>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];

    match &shape.shape_type {
        ShapeType::CustomGeom(geom) => {
            assert_eq!(geom.paths.len(), 2);
            assert!(matches!(geom.paths[0].fill, PathFill::Norm));
            assert!(matches!(geom.paths[1].fill, PathFill::None));
        }
        other => panic!("Expected CustomGeom, got {:?}", other),
    }

    // Verify both paths render in SVG
    let html = render_html(&pptx);
    let path_count = html.matches("<path d=").count();
    assert!(path_count >= 2, "Expected 2+ SVG paths, got {}", path_count);
}

#[test]
fn test_custgeom_with_text_body() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="CustWithText"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst>
            <a:path w="100" h="100">
              <a:moveTo><a:pt x="0" y="0"/></a:moveTo>
              <a:lnTo><a:pt x="100" y="0"/></a:lnTo>
              <a:lnTo><a:pt x="50" y="100"/></a:lnTo>
              <a:close/>
            </a:path>
          </a:pathLst>
        </a:custGeom>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:t>Hello Custom</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("Hello Custom"), "Text should be rendered alongside custom geometry");
    assert!(html.contains("<path d="), "SVG path should be present");
}

#[test]
fn test_custgeom_empty_pathlist() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="EmptyCust"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="2000000"/></a:xfrm>
        <a:custGeom>
          <a:pathLst/>
        </a:custGeom>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    match &shape.shape_type {
        ShapeType::CustomGeom(geom) => {
            assert!(geom.paths.is_empty());
        }
        other => panic!("Expected CustomGeom with empty paths, got {:?}", other),
    }
    // Should not crash when rendering
    let _html = render_html(&pptx);
}

// ── Auto-fit (normAutofit / spAutoFit) ──

#[test]
fn test_norm_autofit_font_scale_parsed() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr>
          <a:normAutofit fontScale="62500" lnSpcReduction="20000"/>
        </a:bodyPr>
        <a:p><a:r><a:rPr sz="2400"/><a:t>Scaled text</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];
    let tb = shape.text_body.as_ref().expect("text_body");

    match &tb.auto_fit {
        AutoFit::Normal {
            font_scale,
            line_spacing_reduction,
        } => {
            let fs = font_scale.expect("font_scale should be Some");
            assert!((fs - 0.625).abs() < 0.001, "font_scale: expected ~0.625, got {fs}");
            let lr = line_spacing_reduction.expect("line_spacing_reduction should be Some");
            assert!((lr - 0.2).abs() < 0.001, "line_spacing_reduction: expected ~0.2, got {lr}");
        }
        other => panic!("Expected AutoFit::Normal, got {:?}", other),
    }
}

#[test]
fn test_norm_autofit_without_attributes() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr>
          <a:normAutofit/>
        </a:bodyPr>
        <a:p><a:r><a:rPr sz="1800"/><a:t>Text</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let tb = pres.slides[0].shapes[0]
        .text_body
        .as_ref()
        .expect("text_body");

    match &tb.auto_fit {
        AutoFit::Normal {
            font_scale,
            line_spacing_reduction,
        } => {
            assert!(font_scale.is_none(), "font_scale should be None");
            assert!(
                line_spacing_reduction.is_none(),
                "line_spacing_reduction should be None"
            );
        }
        other => panic!("Expected AutoFit::Normal, got {:?}", other),
    }
}

#[test]
fn test_sp_auto_fit_parsed_as_shrink() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr><a:spAutoFit/></a:bodyPr>
        <a:p><a:r><a:rPr sz="1800"/><a:t>Shrink</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let tb = pres.slides[0].shapes[0]
        .text_body
        .as_ref()
        .expect("text_body");
    assert!(matches!(tb.auto_fit, AutoFit::Shrink));
}

#[test]
fn test_norm_autofit_renders_scaled_font_size() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr>
          <a:normAutofit fontScale="50000"/>
        </a:bodyPr>
        <a:p><a:r><a:rPr sz="2000"/><a:t>Half size</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // Original 20pt * 0.5 = 10pt
    assert!(
        html.contains("font-size: 10.0pt"),
        "Expected scaled font-size 10.0pt in HTML: {html}"
    );
}

#[test]
fn test_norm_autofit_renders_overflow_hidden() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr>
          <a:normAutofit fontScale="80000"/>
        </a:bodyPr>
        <a:p><a:r><a:rPr sz="1800"/><a:t>Clipped</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // Check that the text-body div's inline style contains overflow:hidden
    let tb_start = html.find("class=\"text-body").expect("text-body div");
    let tb_chunk = &html[tb_start..tb_start + 300.min(html.len() - tb_start)];
    assert!(
        tb_chunk.contains("overflow: hidden"),
        "Expected overflow:hidden on text-body when fontScale is present, got: {tb_chunk}"
    );
}

#[test]
fn test_norm_autofit_line_spacing_reduction() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr>
          <a:normAutofit lnSpcReduction="20000"/>
        </a:bodyPr>
        <a:p>
          <a:pPr><a:lnSpc><a:spcPct val="150000"/></a:lnSpc></a:pPr>
          <a:r><a:rPr sz="1800"/><a:t>Reduced spacing</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // line-height 1.5 * (1.0 - 0.2) = 1.2
    assert!(
        html.contains("line-height: 1.20"),
        "Expected reduced line-height 1.20 in HTML: {html}"
    );
}

#[test]
fn test_norm_autofit_no_font_scale_no_overflow() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr>
          <a:normAutofit/>
        </a:bodyPr>
        <a:p><a:r><a:rPr sz="1800"/><a:t>No scale</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // No fontScale → text-body div should not have overflow:hidden in its style
    // Extract the text-body div's style to check (global CSS has overflow:hidden on .slide, so check inline)
    let tb_start = html.find("class=\"text-body").expect("text-body div");
    let tb_chunk = &html[tb_start..tb_start + 300.min(html.len() - tb_start)];
    assert!(
        !tb_chunk.contains("overflow: hidden"),
        "text-body should not have overflow:hidden when no fontScale, got: {tb_chunk}"
    );
    assert!(
        html.contains("font-size: 18.0pt"),
        "Font size should remain 18.0pt without fontScale"
    );
}
