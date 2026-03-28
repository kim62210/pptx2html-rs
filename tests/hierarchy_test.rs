//! Integration tests for hierarchy inheritance, style refs, and rendering

mod fixtures;

use pptx2html_rs::model::*;

fn parse_pptx(data: &[u8]) -> pptx2html_rs::model::Presentation {
    pptx2html_rs::parser::PptxParser::parse_bytes(data).expect("PPTX parsing failed")
}

fn render_html(data: &[u8]) -> String {
    let pres = parse_pptx(data);
    pptx2html_rs::renderer::HtmlRenderer::render(&pres).expect("HTML rendering failed")
}

// ── Background inheritance tests ──
// Note: Master/layout background parsing from <p:bg> is not yet implemented in parsers.
// These tests verify the inheritance resolution logic works via the model API.

#[test]
fn test_background_default_white() {
    // When no master/layout provides bg, slide gets default white background
    let pptx = fixtures::MinimalPptx::new("").build();
    let html = render_html(&pptx);
    assert!(html.contains("#FFFFFF"), "Default white background not found: {html}");
}

#[test]
fn test_background_slide_explicit() {
    // Slide with explicit background fill as a shape fill
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Bg"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="0" y="0"/><a:ext cx="9144000" cy="6858000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="FF1122"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("#FF1122"), "Explicit slide fill not found: {html}");
}

// ── Placeholder matching test ──

#[test]
fn test_title_placeholder_inherits_position() {
    let master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
  </p:spTree></p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
</p:sldMaster>"#;

    let layout_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
    <p:sp>
      <p:nvSpPr>
        <p:cNvPr id="2" name="Title"/>
        <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
        <p:nvPr><p:ph type="title"/></p:nvPr>
      </p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm>
      </p:spPr>
    </p:sp>
  </p:spTree></p:cSld>
</p:sldLayout>"#;

    // Slide with title placeholder but no position (should inherit from layout)
    let slide_body = r#"
    <p:sp>
      <p:nvSpPr>
        <p:cNvPr id="2" name="Title 1"/>
        <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
        <p:nvPr><p:ph type="title"/></p:nvPr>
      </p:nvSpPr>
      <p:spPr/>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:rPr sz="2400"/><a:t>Inherited Position</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide_body)
        .with_full_master(master_xml)
        .with_layout(layout_xml)
        .build();
    let pres = parse_pptx(&pptx);

    // Verify the shape exists and has text
    assert_eq!(pres.slides[0].shapes.len(), 1);
    let shape = &pres.slides[0].shapes[0];
    assert!(shape.placeholder.is_some());

    // Position should be inherited from layout (457200 EMU -> ~48px)
    let html = render_html(&pptx);
    assert!(html.contains("Inherited Position"), "Shape text not found: {html}");
    // 8229600 EMU = 864px width
    assert!(html.contains("864.0px"), "Layout width not inherited: {html}");
}

// ── Paragraph spacing tests ──

#[test]
fn test_line_spacing_percentage_rendering() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:pPr>
            <a:lnSpc><a:spcPct val="150000"/></a:lnSpc>
          </a:pPr>
          <a:r><a:rPr sz="1800"/><a:t>150% Line Spacing</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let para = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs[0];
    assert!(matches!(para.line_spacing, Some(SpacingValue::Percent(p)) if (p - 1.5).abs() < 0.01));

    let html = render_html(&pptx);
    assert!(html.contains("line-height: 1.50"), "150% line spacing not found: {html}");
}

#[test]
fn test_line_spacing_points_rendering() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:pPr>
            <a:lnSpc><a:spcPts val="2400"/></a:lnSpc>
          </a:pPr>
          <a:r><a:rPr sz="1800"/><a:t>24pt Line Spacing</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let para = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs[0];
    assert!(matches!(para.line_spacing, Some(SpacingValue::Points(p)) if (p - 24.0).abs() < 0.1));

    let html = render_html(&pptx);
    assert!(html.contains("line-height: 24.0pt"), "24pt line spacing not found: {html}");
}

#[test]
fn test_space_before_after_rendering() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:pPr>
            <a:spcBef><a:spcPts val="1200"/></a:spcBef>
            <a:spcAft><a:spcPts val="600"/></a:spcAft>
          </a:pPr>
          <a:r><a:rPr sz="1800"/><a:t>Spaced Paragraph</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let para = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs[0];
    assert!(matches!(para.space_before, Some(SpacingValue::Points(p)) if (p - 12.0).abs() < 0.1));
    assert!(matches!(para.space_after, Some(SpacingValue::Points(p)) if (p - 6.0).abs() < 0.1));

    let html = render_html(&pptx);
    assert!(html.contains("margin-top: 12.0pt"), "Space before not found: {html}");
    assert!(html.contains("margin-bottom: 6.0pt"), "Space after not found: {html}");
}

// ── Master shapes test ──

#[test]
fn test_show_master_sp_default_true() {
    let master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
    <p:sp>
      <p:nvSpPr>
        <p:cNvPr id="10" name="Footer"/>
        <p:cNvSpPr/>
        <p:nvPr><p:ph type="ftr"/></p:nvPr>
      </p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="6000000"/><a:ext cx="3000000" cy="500000"/></a:xfrm>
      </p:spPr>
    </p:sp>
  </p:spTree></p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
</p:sldMaster>"#;

    let layout_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
  </p:spTree></p:cSld>
</p:sldLayout>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_full_master(master_xml)
        .with_layout(layout_xml)
        .build();
    let pres = parse_pptx(&pptx);

    // Master should have a footer placeholder shape
    assert!(!pres.masters.is_empty(), "No masters parsed");
    assert!(!pres.masters[0].shapes.is_empty(), "Master has no shapes");

    // Default show_master_sp is true, so master shapes appear in HTML
    let html = render_html(&pptx);
    // The master shape div should be rendered (footer placeholder with position)
    assert!(html.contains("class=\"shape\""), "Master shape not rendered: {html}");
}

// ── defaultTextStyle test ──

#[test]
fn test_default_text_style_parsed() {
    let pres_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst><p:sldMasterId r:id="rId1"/></p:sldMasterIdLst>
  <p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:defaultTextStyle>
    <a:lvl1pPr algn="l">
      <a:defRPr sz="1800" b="0"/>
    </a:lvl1pPr>
    <a:lvl2pPr algn="l">
      <a:defRPr sz="1600"/>
    </a:lvl2pPr>
  </p:defaultTextStyle>
</p:presentation>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_presentation_xml(pres_xml)
        .build();
    let pres = parse_pptx(&pptx);

    let dts = pres.default_text_style.as_ref().expect("defaultTextStyle not parsed");
    let lvl1 = dts.levels[0].as_ref().expect("Level 1 not parsed");
    assert!(matches!(lvl1.alignment, Some(Alignment::Left)));
    let run_defaults = lvl1.def_run_props.as_ref().expect("defRPr not parsed");
    assert_eq!(run_defaults.font_size, Some(18.0));
    assert_eq!(run_defaults.bold, Some(false));

    let lvl2 = dts.levels[1].as_ref().expect("Level 2 not parsed");
    let rd2 = lvl2.def_run_props.as_ref().expect("Level 2 defRPr not parsed");
    assert_eq!(rd2.font_size, Some(16.0));
}

// ── Shape style reference parsing test ──

#[test]
fn test_shape_style_ref_parsing() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:style>
        <a:lnRef idx="2"><a:schemeClr val="accent1"/></a:lnRef>
        <a:fillRef idx="1"><a:schemeClr val="accent1"/></a:fillRef>
        <a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef>
        <a:fontRef idx="minor"><a:schemeClr val="dk1"/></a:fontRef>
      </p:style>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:rPr sz="1800"/><a:t>Style Ref Test</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];

    let style_ref = shape.style_ref.as_ref().expect("style_ref not parsed");
    let fill_ref = style_ref.fill_ref.as_ref().expect("fillRef not parsed");
    assert_eq!(fill_ref.idx, 1);
    assert!(matches!(fill_ref.color.kind, color::ColorKind::Theme(ref n) if n == "accent1"));

    let ln_ref = style_ref.ln_ref.as_ref().expect("lnRef not parsed");
    assert_eq!(ln_ref.idx, 2);

    let effect_ref = style_ref.effect_ref.as_ref().expect("effectRef not parsed");
    assert_eq!(effect_ref.idx, 0);

    let font_ref = style_ref.font_ref.as_ref().expect("fontRef not parsed");
    assert_eq!(font_ref.idx, "minor");
    assert!(matches!(font_ref.color.kind, color::ColorKind::Theme(ref n) if n == "dk1"));
}

// ── Style ref fill resolution in rendering ──

#[test]
fn test_style_ref_fill_rendered() {
    // Theme with fmtScheme fillStyleLst
    let theme_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="TestTheme">
  <a:themeElements>
    <a:clrScheme name="Office">
      <a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
      <a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
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
    <a:fontScheme name="Office">
      <a:majorFont><a:latin typeface="Calibri Light"/></a:majorFont>
      <a:minorFont><a:latin typeface="Calibri"/></a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <a:fillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:solidFill><a:srgbClr val="AABBCC"/></a:solidFill>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="9525"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln>
        <a:ln w="19050"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle><a:effectLst/></a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
</a:theme>"#;

    let master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
  </p:spTree></p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
</p:sldMaster>"#;

    let layout_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
  </p:spTree></p:cSld>
</p:sldLayout>"#;

    // Shape with fillRef idx=1 and accent1 color, no explicit fill
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="StyleRef"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:style>
        <a:lnRef idx="0"><a:schemeClr val="accent1"/></a:lnRef>
        <a:fillRef idx="1"><a:schemeClr val="accent1"/></a:fillRef>
        <a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef>
        <a:fontRef idx="minor"><a:schemeClr val="dk1"/></a:fontRef>
      </p:style>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:rPr sz="1800"/><a:t>Filled via StyleRef</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide)
        .with_full_theme(theme_xml)
        .with_full_master(master_xml)
        .with_layout(layout_xml)
        .build();
    let html = render_html(&pptx);
    // fillRef idx=1 with accent1 -> phClr placeholder replaced by accent1 -> #4472C4
    assert!(html.contains("#4472C4"), "StyleRef fill color not rendered: {html}");
}

// ── Style ref with explicit fill override test ──

#[test]
fn test_explicit_fill_overrides_style_ref() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="DD0000"/></a:solidFill>
      </p:spPr>
      <p:style>
        <a:lnRef idx="0"><a:schemeClr val="accent1"/></a:lnRef>
        <a:fillRef idx="1"><a:schemeClr val="accent1"/></a:fillRef>
        <a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef>
        <a:fontRef idx="minor"><a:schemeClr val="dk1"/></a:fontRef>
      </p:style>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    // Explicit fill should take priority over style ref
    assert!(html.contains("#DD0000"), "Explicit fill should override style ref: {html}");
}

// ── Theme FmtScheme parsing test ──

#[test]
fn test_fmt_scheme_parsed() {
    let theme_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Test">
  <a:themeElements>
    <a:clrScheme name="Office">
      <a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
      <a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
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
    <a:fontScheme name="Office">
      <a:majorFont><a:latin typeface="Calibri Light"/></a:majorFont>
      <a:minorFont><a:latin typeface="Calibri"/></a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <a:fillStyleLst>
        <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
        <a:solidFill><a:srgbClr val="00FF00"/></a:solidFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="12700"><a:solidFill><a:srgbClr val="0000FF"/></a:solidFill></a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle><a:effectLst/></a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:srgbClr val="CCCCCC"/></a:solidFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
</a:theme>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_full_theme(theme_xml)
        .build();
    let pres = parse_pptx(&pptx);
    let theme = pres.primary_theme().expect("Theme not found");

    assert_eq!(theme.fmt_scheme.fill_style_lst.len(), 2);
    assert_eq!(theme.fmt_scheme.ln_style_lst.len(), 1);
    assert_eq!(theme.fmt_scheme.bg_fill_style_lst.len(), 1);

    // Check line style width (12700 EMU = 1pt)
    assert!((theme.fmt_scheme.ln_style_lst[0].width - 1.0).abs() < 0.1);
}

// ── Slide with background percentage space ──

#[test]
fn test_space_before_percentage() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:pPr>
            <a:spcBef><a:spcPct val="50000"/></a:spcBef>
          </a:pPr>
          <a:r><a:rPr sz="1800"/><a:t>50% Space Before</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let para = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs[0];
    assert!(matches!(para.space_before, Some(SpacingValue::Percent(p)) if (p - 0.5).abs() < 0.01));

    let html = render_html(&pptx);
    assert!(html.contains("margin-top: 0.5em"), "50% space before not found: {html}");
}

// ── Shape with no text body still renders ──

#[test]
fn test_shape_no_text_body() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="FFAA00"/></a:solidFill>
      </p:spPr>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];

    // Shape should have fill but no text body
    assert!(matches!(shape.fill, Fill::Solid(_)));
    assert!(shape.text_body.is_none(), "text_body should be None for shape without txBody");

    let html = render_html(&pptx);
    assert!(html.contains("#FFAA00"), "Shape fill not rendered: {html}");
}

// ── Layout type parsed test ──

#[test]
fn test_layout_type_parsed() {
    let master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
  </p:spTree></p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
</p:sldMaster>"#;

    let layout_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             type="title">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
  </p:spTree></p:cSld>
</p:sldLayout>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_full_master(master_xml)
        .with_layout(layout_xml)
        .build();
    let pres = parse_pptx(&pptx);
    assert!(!pres.layouts.is_empty(), "No layouts parsed");
    assert_eq!(pres.layouts[0].layout_type.as_deref(), Some("title"));
}

// ── Multiple paragraphs with different styles ──

#[test]
fn test_multiple_paragraphs_spacing() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="3000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:pPr algn="l">
            <a:lnSpc><a:spcPct val="120000"/></a:lnSpc>
          </a:pPr>
          <a:r><a:rPr sz="1800"/><a:t>First paragraph</a:t></a:r>
        </a:p>
        <a:p>
          <a:pPr algn="ctr">
            <a:lnSpc><a:spcPts val="1800"/></a:lnSpc>
            <a:spcBef><a:spcPts val="600"/></a:spcBef>
          </a:pPr>
          <a:r><a:rPr sz="2400" b="1"/><a:t>Second paragraph</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let paras = &pres.slides[0].shapes[0].text_body.as_ref().unwrap().paragraphs;
    assert_eq!(paras.len(), 2);

    // First paragraph: left aligned, 120% line spacing
    assert!(matches!(paras[0].alignment, Alignment::Left));
    assert!(matches!(paras[0].line_spacing, Some(SpacingValue::Percent(p)) if (p - 1.2).abs() < 0.01));

    // Second paragraph: center aligned, 18pt line spacing, 6pt space before
    assert!(matches!(paras[1].alignment, Alignment::Center));
    assert!(matches!(paras[1].line_spacing, Some(SpacingValue::Points(p)) if (p - 18.0).abs() < 0.1));
    assert!(matches!(paras[1].space_before, Some(SpacingValue::Points(p)) if (p - 6.0).abs() < 0.1));

    let html = render_html(&pptx);
    assert!(html.contains("First paragraph"), "First paragraph not found");
    assert!(html.contains("Second paragraph"), "Second paragraph not found");
    assert!(html.contains("text-align: center"), "Center alignment not rendered");
    assert!(html.contains("line-height: 18.0pt"), "18pt line-height not rendered");
}

// ── Verify style ref with no color child ──

#[test]
fn test_style_ref_no_color_child() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="3000000" cy="1000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:style>
        <a:lnRef idx="0"/>
        <a:fillRef idx="0"/>
        <a:effectRef idx="0"/>
        <a:fontRef idx="minor"/>
      </p:style>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shape = &pres.slides[0].shapes[0];

    let sr = shape.style_ref.as_ref().expect("style_ref not parsed");
    assert_eq!(sr.fill_ref.as_ref().unwrap().idx, 0);
    assert_eq!(sr.ln_ref.as_ref().unwrap().idx, 0);
    assert_eq!(sr.effect_ref.as_ref().unwrap().idx, 0);
    assert_eq!(sr.font_ref.as_ref().unwrap().idx, "minor");
}

// ── Bullet rendering test ──

#[test]
fn test_bullet_char_rendered() {
    let slide = r#"
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Box"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="100000" y="100000"/><a:ext cx="5000000" cy="2000000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p>
          <a:pPr>
            <a:buChar char="&#x2022;"/>
          </a:pPr>
          <a:r><a:rPr sz="1800"/><a:t>Bullet item</a:t></a:r>
        </a:p>
      </p:txBody>
    </p:sp>"#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let html = render_html(&pptx);
    assert!(html.contains("class=\"bullet\""), "Bullet span not found: {html}");
    assert!(html.contains("Bullet item"), "Bullet text not found: {html}");
}
