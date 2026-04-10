mod fixtures;

use pptx2html_core::model::{
    Alignment, AutoFit, BorderStyle, Bullet, ClrMapOverride, ColorKind, CompoundLine, DashStyle,
    Fill, GradientType, LineAlignment, LineCap, LineJoin, PathFill, PlaceholderType, ShapeType,
    StrikethroughType, TextCapitalization, UnderlineType, VerticalAlign,
};
use pptx2html_core::parser::PptxParser;

fn parse_pptx(data: &[u8]) -> pptx2html_core::model::Presentation {
    PptxParser::parse_bytes(data).expect("PPTX parsing failed")
}

fn render_with_metadata(
    data: &[u8],
) -> pptx2html_core::error::PptxResult<pptx2html_core::ConversionResult> {
    pptx2html_core::convert_bytes_with_metadata(data)
}

#[test]
fn parses_theme_master_and_layout_branches_through_public_parser() {
    let theme_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="CoverageTheme">
  <a:themeElements>
    <a:clrScheme name="CoverageColors">
      <a:dk1><a:sysClr val="windowText" lastClr="111111"/></a:dk1>
      <a:lt1><a:srgbClr val="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="222222"/></a:dk2>
      <a:lt2><a:srgbClr val="F7F7F7"/></a:lt2>
      <a:accent1><a:srgbClr val="4472C4"/></a:accent1>
      <a:accent2><a:srgbClr val="ED7D31"/></a:accent2>
      <a:accent3><a:srgbClr val="A5A5A5"/></a:accent3>
      <a:accent4><a:srgbClr val="FFC000"/></a:accent4>
      <a:accent5><a:srgbClr val="5B9BD5"/></a:accent5>
      <a:accent6><a:srgbClr val="70AD47"/></a:accent6>
      <a:hlink><a:srgbClr val="0563C1"/></a:hlink>
      <a:folHlink><a:srgbClr val="954F72"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name="CoverageFonts">
      <a:majorFont>
        <a:latin typeface="Aptos"/>
        <a:ea typeface="Yu Gothic"/>
        <a:cs typeface="Noto Sans Devanagari"/>
      </a:majorFont>
      <a:minorFont>
        <a:latin typeface="Aptos Narrow"/>
        <a:ea typeface="Meiryo"/>
        <a:cs typeface="Noto Sans Arabic"/>
      </a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="CoverageFmt">
      <a:fillStyleLst>
        <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
        <a:solidFill><a:schemeClr val="accent2"/></a:solidFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="12700" cap="rnd" cmpd="dbl" algn="in">
          <a:solidFill><a:schemeClr val="accent3"/></a:solidFill>
          <a:prstDash val="lgDashDot"/>
          <a:miter lim="200000"/>
        </a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle>
          <a:effectLst>
            <a:outerShdw blurRad="12700" dist="25400" dir="5400000">
              <a:schemeClr val="accent4"/>
              <a:alpha val="50000"/>
            </a:outerShdw>
            <a:glow rad="6350">
              <a:srgbClr val="ABCDEF"/>
              <a:alpha val="60000"/>
            </a:glow>
          </a:effectLst>
        </a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:noFill/>
        <a:solidFill><a:schemeClr val="accent5"/></a:solidFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
</a:theme>"#;

    let master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
            <a:gs pos="100000"><a:schemeClr val="accent1"/></a:gs>
          </a:gsLst>
          <a:path path="rect"/>
          <a:lin ang="5400000"/>
        </a:gradFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Master Placeholder"/>
          <p:cNvSpPr/>
          <p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="12700" y="25400"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700" cap="rnd" cmpd="dbl" algn="in">
            <a:prstDash val="lgDashDot"/>
            <a:miter lim="200000"/>
            <a:headEnd type="triangle" w="lg" len="sm"/>
            <a:tailEnd type="stealth" w="sm" len="lg"/>
            <a:schemeClr val="accent3"/>
          </a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr" anchorCtr="1" rot="5400000" vert="vert270"
                    lIns="91440" tIns="45720" rIns="182880" bIns="22860" wrap="none"/>
          <a:lstStyle>
            <a:lvl1pPr algn="r">
              <a:spcBef><a:spcPts val="1200"/></a:spcBef>
              <a:defRPr sz="1800"><a:latin typeface="Calibri"/><a:srgbClr val="336699"/></a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
          <a:p/>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:txStyles>
    <p:titleStyle>
      <a:lvl1pPr algn="ctr" marL="457200" indent="-228600">
        <a:lnSpc><a:spcPct val="90000"/></a:lnSpc>
        <a:defRPr sz="2400" spc="200" baseline="30000" cap="all" u="dbl"
                  strike="sngStrike" b="1" i="1">
          <a:latin typeface="Aptos"/>
          <a:ea typeface="Yu Gothic"/>
          <a:cs typeface="Noto Sans Devanagari"/>
          <a:schemeClr val="accent2"/>
        </a:defRPr>
      </a:lvl1pPr>
    </p:titleStyle>
    <p:bodyStyle><a:lvl1pPr algn="just"/></p:bodyStyle>
    <p:otherStyle><a:lvl1pPr algn="r"/></p:otherStyle>
  </p:txStyles>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2"
            accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6"
            hlink="hlink" folHlink="folHlink"/>
</p:sldMaster>"#;

    let layout_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             type="title" showMasterSp="0">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="112233"/></a:gs>
            <a:gs pos="100000"><a:schemeClr val="accent2"/></a:gs>
          </a:gsLst>
          <a:path path="shape"/>
          <a:lin ang="2700000"/>
        </a:gradFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Layout Placeholder"/>
          <p:cNvSpPr/>
          <p:nvPr><p:ph type="title" idx="3"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="12700" y="38100"/><a:ext cx="254000" cy="381000"/></a:xfrm>
          <a:ln w="12700" cap="flat" cmpd="tri" algn="in">
            <a:prstDash val="sysDot"/>
            <a:bevel/>
            <a:headEnd type="arrow" w="sm" len="sm"/>
            <a:tailEnd type="diamond" w="lg" len="lg"/>
            <a:srgbClr val="445566"/>
          </a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="b" anchorCtr="1" rot="1800000" vert="vert"
                    lIns="45720" tIns="91440" rIns="137160" bIns="182880" wrap="none"/>
          <a:normAutofit fontScale="70000" lnSpcReduction="15000"/>
          <a:lstStyle>
            <a:lvl1pPr algn="just">
              <a:spcAft><a:spcPct val="150000"/></a:spcAft>
              <a:defRPr sz="2000"><a:latin typeface="Calibri"/><a:schemeClr val="accent4"/></a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
          <a:p/>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr><a:overrideClrMapping bg1="lt2" tx1="dk2" accent1="accent6"/></p:clrMapOvr>
</p:sldLayout>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_full_theme(theme_xml)
        .with_full_master(master_xml)
        .with_layout(layout_xml)
        .with_slide_layout_rel()
        .build();

    let presentation = parse_pptx(&pptx);

    let theme = &presentation.themes[0];
    assert_eq!(theme.font_scheme.major_latin, "Aptos");
    assert_eq!(theme.font_scheme.minor_latin, "Aptos Narrow");
    assert_eq!(
        theme.font_scheme.major_east_asian.as_deref(),
        Some("Yu Gothic")
    );
    assert_eq!(
        theme.font_scheme.minor_complex_script.as_deref(),
        Some("Noto Sans Arabic")
    );
    assert_eq!(theme.fmt_scheme.fill_style_lst.len(), 2);
    assert_eq!(theme.fmt_scheme.ln_style_lst.len(), 1);
    assert_eq!(theme.fmt_scheme.effect_style_lst.len(), 1);

    let master = &presentation.masters[0];
    assert!(matches!(
        &master.background,
        Some(Fill::Gradient(fill)) if fill.stops.len() == 2
    ));
    assert!(master.tx_styles.title_style.is_some());
    let master_shape = &master.shapes[0];
    assert!(matches!(
        master_shape
            .placeholder
            .as_ref()
            .and_then(|ph| ph.ph_type.as_ref()),
        Some(PlaceholderType::Body)
    ));
    assert!(matches!(master_shape.border.cap, LineCap::Round));
    assert!(matches!(master_shape.border.compound, CompoundLine::Double));
    assert!(matches!(
        master_shape.border.alignment,
        LineAlignment::Inset
    ));
    assert!(matches!(master_shape.border.join, LineJoin::Miter));
    assert!(matches!(
        master_shape.border.dash_style,
        DashStyle::LongDashDot
    ));
    assert_eq!(master_shape.vertical_text.as_deref(), Some("vert270"));
    let master_text = master_shape.text_body.as_ref().expect("master text body");
    assert!(matches!(master_text.vertical_align, VerticalAlign::Middle));
    assert!(master_text.anchor_center);

    let layout = &presentation.layouts[0];
    assert_eq!(layout.layout_type.as_deref(), Some("title"));
    assert!(!layout.show_master_sp);
    assert!(matches!(
        &layout.background,
        Some(Fill::Gradient(fill)) if fill.stops.len() == 2
    ));
    assert!(matches!(
        &layout.clr_map_ovr,
        Some(ClrMapOverride::Override(map))
            if map.get("bg1").map(String::as_str) == Some("lt2")
                && map.get("tx1").map(String::as_str) == Some("dk2")
                && map.get("accent1").map(String::as_str) == Some("accent6")
    ));
    let layout_shape = &layout.shapes[0];
    assert!(matches!(
        layout_shape
            .placeholder
            .as_ref()
            .and_then(|ph| ph.ph_type.as_ref()),
        Some(PlaceholderType::Title)
    ));
    let layout_text = layout_shape.text_body.as_ref().expect("layout text body");
    assert!(matches!(layout_text.vertical_align, VerticalAlign::Bottom));
    assert!(layout_text.anchor_center);
    assert!(matches!(
        layout_text.auto_fit,
        AutoFit::Normal {
            font_scale: Some(v),
            line_spacing_reduction: Some(lsr)
        } if (v - 0.7).abs() < 1e-6 && (lsr - 0.15).abs() < 1e-6
    ));
}

#[test]
fn parses_theme_and_master_start_tag_variants_through_public_parser() {
    let theme_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="StartTagTheme">
  <a:themeElements>
    <a:clrScheme name="StartTagColors">
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
    <a:fontScheme name="StartTagFonts">
      <a:majorFont><a:latin typeface="Aptos"/></a:majorFont>
      <a:minorFont><a:latin typeface="Aptos Narrow"/></a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="StartTagFmt">
      <a:fillStyleLst>
        <a:solidFill><a:srgbClr val="AA0000"></a:srgbClr></a:solidFill>
        <a:solidFill><a:schemeClr val="accent2"></a:schemeClr></a:solidFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="12700" cap="flat" cmpd="tri" algn="in">
          <a:solidFill><a:schemeClr val="accent3"></a:schemeClr></a:solidFill>
          <a:prstDash val="sysDashDotDot"/>
          <a:bevel/>
        </a:ln>
        <a:ln w="6350" cmpd="thickThin">
          <a:solidFill><a:srgbClr val="00FF00"></a:srgbClr></a:solidFill>
          <a:prstDash val="lgDashDotDot"/>
          <a:round/>
        </a:ln>
        <a:ln w="6350" cmpd="thinThick">
          <a:solidFill><a:srgbClr val="0000FF"></a:srgbClr></a:solidFill>
          <a:prstDash val="sysDash"/>
        </a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle>
          <a:effectLst>
            <a:outerShdw blurRad="12700" dist="25400" dir="5400000">
              <a:srgbClr val="112233"></a:srgbClr>
              <a:alpha val="40000"/>
            </a:outerShdw>
            <a:glow rad="6350">
              <a:schemeClr val="accent6"></a:schemeClr>
              <a:alpha val="50000"/>
            </a:glow>
          </a:effectLst>
        </a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:schemeClr val="accent5"></a:schemeClr></a:solidFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
</a:theme>"#;

    let master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="334455"></a:srgbClr></a:gs>
            <a:gs pos="100000"><a:schemeClr val="accent1"></a:schemeClr></a:gs>
          </a:gsLst>
          <a:path path="rect"></a:path>
          <a:lin ang="1800000"/>
        </a:gradFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="2" name="Master Normal"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="12700" y="25400"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700" cap="flat" cmpd="thinThick" algn="in">
            <a:prstDash val="sysDashDotDot"/>
            <a:bevel/>
            <a:schemeClr val="accent3"></a:schemeClr>
          </a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr" anchorCtr="true" rot="60000" vert="horz" wrap="square"></a:bodyPr>
          <a:normAutofit fontScale="65000" lnSpcReduction="12000"></a:normAutofit>
          <a:lstStyle>
            <a:lvl1pPr algn="ctr">
              <a:spcBef><a:spcPct val="25000"/></a:spcBef>
              <a:lnSpc><a:spcPts val="900"/></a:lnSpc>
              <a:defRPr sz="1800"><a:srgbClr val="112233"></a:srgbClr></a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="3" name="Master NoAuto"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="25400" y="38100"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700" cmpd="thickThin"><a:prstDash val="dash"/></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr></a:bodyPr>
          <a:noAutofit></a:noAutofit>
          <a:lstStyle>
            <a:lvl1pPr>
              <a:defRPr sz="1600"><a:schemeClr val="accent2"></a:schemeClr></a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="4" name="Master Shrink"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr><a:xfrm><a:off x="38100" y="50800"/><a:ext cx="381000" cy="254000"/></a:xfrm></p:spPr>
        <p:txBody>
          <a:bodyPr></a:bodyPr>
          <a:spAutoFit></a:spAutoFit>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="5" name="Implicit Solid"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="50800" y="63500"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700"></a:ln>
        </p:spPr>
      </p:sp>
    </p:spTree>
  </p:cSld>
    <p:txStyles>
    <p:titleStyle>
      <a:lvl1pPr algn="ctr">
        <a:spcBef><a:spcPct val="25000"/></a:spcBef>
        <a:defRPr sz="2000"><a:schemeClr val="accent2"></a:schemeClr></a:defRPr>
      </a:lvl1pPr>
    </p:titleStyle>
    <p:bodyStyle>
      <a:lvl1pPr>
        <a:lnSpc><a:spcPts val="1200"/></a:lnSpc>
      </a:lvl1pPr>
    </p:bodyStyle>
    <p:otherStyle>
      <a:lvl1pPr>
        <a:spcAft><a:spcPct val="50000"/></a:spcAft>
      </a:lvl1pPr>
    </p:otherStyle>
  </p:txStyles>
</p:sldMaster>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_full_theme(theme_xml)
        .with_full_master(master_xml)
        .build();

    let presentation = parse_pptx(&pptx);

    let theme = &presentation.themes[0];
    assert_eq!(theme.fmt_scheme.ln_style_lst.len(), 3);
    assert!(matches!(
        theme.fmt_scheme.ln_style_lst[0].compound,
        CompoundLine::Triple
    ));
    assert!(matches!(
        theme.fmt_scheme.ln_style_lst[0].dash_style,
        DashStyle::SystemDashDotDot
    ));
    assert!(matches!(
        theme.fmt_scheme.ln_style_lst[1].compound,
        CompoundLine::ThickThin
    ));
    assert!(matches!(
        theme.fmt_scheme.ln_style_lst[1].dash_style,
        DashStyle::LongDashDotDot
    ));
    assert!(matches!(
        theme.fmt_scheme.ln_style_lst[2].compound,
        CompoundLine::ThinThick
    ));
    assert!(matches!(
        theme.fmt_scheme.ln_style_lst[2].dash_style,
        DashStyle::SystemDash
    ));
    let effect = &theme.fmt_scheme.effect_style_lst[0];
    assert!(matches!(
        effect
            .outer_shadow
            .as_ref()
            .map(|shadow| shadow.color.kind.clone()),
        Some(ColorKind::Rgb(rgb)) if rgb == "112233"
    ));
    assert!(matches!(
        effect.glow.as_ref().map(|glow| glow.color.kind.clone()),
        Some(ColorKind::Theme(name)) if name == "accent6"
    ));

    let master = &presentation.masters[0];
    assert!(matches!(
        &master.background,
        Some(Fill::Gradient(fill)) if fill.stops.len() == 2
    ));
    let title_style = master.tx_styles.title_style.as_ref().expect("title style");
    assert!(matches!(
        title_style.levels[0]
            .as_ref()
            .and_then(|lvl| lvl.space_before.clone()),
        Some(pptx2html_core::model::SpacingValue::Percent(v)) if (v - 0.25).abs() < 1e-6
    ));
    let body_style = master.tx_styles.body_style.as_ref().expect("body style");
    assert!(matches!(
        body_style.levels[0]
            .as_ref()
            .and_then(|lvl| lvl.line_spacing.clone()),
        Some(pptx2html_core::model::SpacingValue::Points(v)) if (v - 12.0).abs() < 1e-6
    ));
    let other_style = master.tx_styles.other_style.as_ref().expect("other style");
    assert!(matches!(
        other_style.levels[0]
            .as_ref()
            .and_then(|lvl| lvl.space_after.clone()),
        Some(pptx2html_core::model::SpacingValue::Percent(v)) if (v - 0.5).abs() < 1e-6
    ));

    assert_eq!(master.shapes.len(), 4);
    let normal_shape = &master.shapes[0];
    assert!(matches!(
        normal_shape.border.compound,
        CompoundLine::ThinThick
    ));
    assert!(matches!(
        normal_shape.border.dash_style,
        DashStyle::SystemDashDotDot
    ));
    assert!(matches!(normal_shape.border.join, LineJoin::Bevel));
    assert!(matches!(
        normal_shape
            .text_body
            .as_ref()
            .expect("master normal text")
            .auto_fit,
        AutoFit::Normal {
            font_scale: Some(v),
            line_spacing_reduction: Some(lsr)
        } if (v - 0.65).abs() < 1e-6 && (lsr - 0.12).abs() < 1e-6
    ));
    let no_auto_shape = &master.shapes[1];
    assert!(matches!(
        no_auto_shape.border.compound,
        CompoundLine::ThickThin
    ));
    assert!(matches!(
        no_auto_shape
            .text_body
            .as_ref()
            .expect("master no-auto text")
            .auto_fit,
        AutoFit::NoAutoFit
    ));
    let shrink_shape = &master.shapes[2];
    assert!(matches!(
        shrink_shape
            .text_body
            .as_ref()
            .expect("master shrink text")
            .auto_fit,
        AutoFit::Shrink
    ));
    let implicit_line_shape = &master.shapes[3];
    assert!(matches!(
        implicit_line_shape.border.style,
        BorderStyle::Solid
    ));
}

#[test]
fn parses_start_tag_layout_defaults_and_background_relations_through_public_parser() {
    let presentation_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst><p:sldMasterId r:id="rId1"/></p:sldMasterIdLst>
  <p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000" foo="ignored"/>
    <p:defaultTextStyle>
    <a:lvl1pPr algn="ctr">
      <a:spcAft><a:spcPts val="600"/></a:spcAft>
      <a:defRPr sz="2000">
        <a:srgbClr val="112233"></a:srgbClr>
      </a:defRPr>
    </a:lvl1pPr>
    <a:lvl2pPr algn="just">
      <a:lnSpc><a:spcPct val="150000"/></a:lnSpc>
      <a:defRPr sz="1800">
        <a:schemeClr val="accent2"></a:schemeClr>
      </a:defRPr>
    </a:lvl2pPr>
    <a:lvl3pPr algn="r">
      <a:spcBef><a:spcPct val="25000"/></a:spcBef>
    </a:lvl3pPr>
    <a:lvl4pPr algn="l">
      <a:lnSpc><a:spcPts val="900"/></a:lnSpc>
    </a:lvl4pPr>
    <a:lvl5pPr algn="just">
      <a:spcAft><a:spcPct val="50000"/></a:spcAft>
    </a:lvl5pPr>
  </p:defaultTextStyle>
</p:presentation>"#;

    let layout_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             type="title" showMasterSp="false">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0"><a:srgbClr val="FF0000"></a:srgbClr></a:gs>
            <a:gs pos="100000"><a:schemeClr val="accent2"></a:schemeClr></a:gs>
          </a:gsLst>
          <a:path path="rect"></a:path>
          <a:lin ang="1800000"/>
        </a:gradFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="StartTagNormal"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="12700" y="25400"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700" cap="flat" cmpd="thickThin" algn="in">
            <a:prstDash val="sysDashDotDot"/>
            <a:round/>
            <a:schemeClr val="accent3"/>
          </a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr" anchorCtr="true" rot="60000" vert="horz"
                    lIns="91440" tIns="45720" rIns="182880" bIns="22860" wrap="square"></a:bodyPr>
          <a:normAutofit fontScale="65000" lnSpcReduction="12000"></a:normAutofit>
          <a:lstStyle>
            <a:lvl1pPr algn="ctr">
              <a:defRPr sz="1800">
                <a:srgbClr val="112233"></a:srgbClr>
              </a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
          <a:p/>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="StartTagNoAuto"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="25400" y="38100"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700" cmpd="thinThick">
            <a:prstDash val="dash"/>
          </a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr></a:bodyPr>
          <a:noAutofit></a:noAutofit>
          <a:lstStyle>
            <a:lvl1pPr>
              <a:defRPr sz="1600">
                <a:schemeClr val="accent2"></a:schemeClr>
              </a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
          <a:p/>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="4" name="StartTagShrink"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="38100" y="50800"/><a:ext cx="381000" cy="254000"/></a:xfrm>
        </p:spPr>
        <p:txBody>
          <a:bodyPr></a:bodyPr>
          <a:spAutoFit></a:spAutoFit>
          <a:p/>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="5" name="ImplicitSolidLine"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="50800" y="63500"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700"></a:ln>
        </p:spPr>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sldLayout>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_presentation_xml(presentation_xml)
        .with_clr_map(
            r#"bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink""#,
        )
        .with_layout(layout_xml)
        .build();

    let presentation = parse_pptx(&pptx);

    let defaults = presentation
        .default_text_style
        .as_ref()
        .expect("default text style should be parsed");
    let lvl1 = defaults.levels[0].as_ref().expect("level 1 defaults");
    assert!(matches!(
        lvl1.space_after,
        Some(pptx2html_core::model::SpacingValue::Points(v)) if (v - 6.0).abs() < 1e-6
    ));
    assert_eq!(
        lvl1.def_run_props
            .as_ref()
            .and_then(|rd| rd.color.as_ref())
            .and_then(|color| color.to_css())
            .as_deref(),
        Some("#112233")
    );
    let lvl2 = defaults.levels[1].as_ref().expect("level 2 defaults");
    assert!(matches!(
        lvl2.line_spacing,
        Some(pptx2html_core::model::SpacingValue::Percent(v)) if (v - 1.5).abs() < 1e-6
    ));
    assert_eq!(
        lvl2.def_run_props
            .as_ref()
            .and_then(|rd| rd.color.as_ref())
            .and_then(|color| color.to_css())
            .as_deref(),
        Some("#ED7D31")
    );
    let lvl3 = defaults.levels[2].as_ref().expect("level 3 defaults");
    assert!(matches!(
        lvl3.space_before,
        Some(pptx2html_core::model::SpacingValue::Percent(v)) if (v - 0.25).abs() < 1e-6
    ));
    let lvl4 = defaults.levels[3].as_ref().expect("level 4 defaults");
    assert!(matches!(
        lvl4.line_spacing,
        Some(pptx2html_core::model::SpacingValue::Points(v)) if (v - 9.0).abs() < 1e-6
    ));
    let lvl5 = defaults.levels[4].as_ref().expect("level 5 defaults");
    assert!(matches!(
        lvl5.space_after,
        Some(pptx2html_core::model::SpacingValue::Percent(v)) if (v - 0.5).abs() < 1e-6
    ));

    let layout = &presentation.layouts[0];
    assert_eq!(layout.layout_type.as_deref(), Some("title"));
    assert!(!layout.show_master_sp);
    assert!(matches!(
        &layout.background,
        Some(Fill::Gradient(fill)) if fill.stops.len() == 2
    ));

    assert_eq!(layout.shapes.len(), 4);
    let normal_shape = &layout.shapes[0];
    assert!(matches!(normal_shape.border.cap, LineCap::Flat));
    assert!(matches!(
        normal_shape.border.compound,
        CompoundLine::ThickThin
    ));
    assert!(matches!(normal_shape.border.join, LineJoin::Round));
    assert!(matches!(
        normal_shape.border.dash_style,
        DashStyle::SystemDashDotDot
    ));
    assert_eq!(
        normal_shape.border.color.to_css().as_deref(),
        Some("#A5A5A5")
    );
    assert_eq!(normal_shape.vertical_text, None);
    let normal_text = normal_shape
        .text_body
        .as_ref()
        .expect("normal shape text body");
    assert!(matches!(normal_text.vertical_align, VerticalAlign::Middle));
    assert!(normal_text.anchor_center);
    assert!(normal_text.word_wrap);
    assert!(matches!(
        normal_text.auto_fit,
        AutoFit::Normal {
            font_scale: Some(v),
            line_spacing_reduction: Some(lsr)
        } if (v - 0.65).abs() < 1e-6 && (lsr - 0.12).abs() < 1e-6
    ));
    assert_eq!(
        normal_text
            .list_style
            .as_ref()
            .and_then(|style| style.levels[0].as_ref())
            .and_then(|level| level.def_run_props.as_ref())
            .and_then(|rd| rd.color.as_ref())
            .and_then(|color| color.to_css())
            .as_deref(),
        Some("#112233")
    );

    let no_auto_shape = &layout.shapes[1];
    assert!(matches!(
        no_auto_shape.border.compound,
        CompoundLine::ThinThick
    ));
    let no_auto_text = no_auto_shape.text_body.as_ref().expect("no-auto text body");
    assert!(matches!(no_auto_text.auto_fit, AutoFit::NoAutoFit));
    assert_eq!(
        no_auto_text
            .list_style
            .as_ref()
            .and_then(|style| style.levels[0].as_ref())
            .and_then(|level| level.def_run_props.as_ref())
            .and_then(|rd| rd.color.as_ref())
            .and_then(|color| color.to_css())
            .as_deref(),
        Some("#ED7D31")
    );

    let shrink_shape = &layout.shapes[2];
    assert!(matches!(
        shrink_shape
            .text_body
            .as_ref()
            .expect("shrink text body")
            .auto_fit,
        AutoFit::Shrink
    ));
    let implicit_line_shape = &layout.shapes[3];
    assert!(matches!(
        implicit_line_shape.border.style,
        BorderStyle::Solid
    ));
}

#[test]
fn parses_layout_background_image_start_tags_through_public_parser() {
    let layout_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             type="body">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:blipFill>
          <a:blip r:embed="rIdBg"></a:blip>
        </a:blipFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sldLayout>"#;

    let layout_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
  <Relationship Id="rIdBg" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/layout-bg.png"/>
</Relationships>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_clr_map(
            r#"bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink""#,
        )
        .with_layout(layout_xml)
        .with_layout_rels(layout_rels)
        .with_extra_file("ppt/media/layout-bg.png", b"layout-background")
        .build();

    let presentation = parse_pptx(&pptx);
    let layout = &presentation.layouts[0];
    assert_eq!(layout.layout_type.as_deref(), Some("body"));
    assert!(matches!(
        &layout.background,
        Some(Fill::Image(image))
            if image.content_type == "image/png" && image.data == b"layout-background"
    ));
}

#[test]
fn parses_master_solid_background_and_dashdot_start_variants_through_public_parser() {
    let master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill><a:srgbClr val="334455"></a:srgbClr></a:solidFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="2" name="DashDot"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="12700" y="25400"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700"><a:prstDash val="dashDot"/></a:ln>
        </p:spPr>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sldMaster>"#;

    let pptx = fixtures::MinimalPptx::new("")
        .with_full_master(master_xml)
        .build();

    let presentation = parse_pptx(&pptx);
    let master = &presentation.masters[0];
    assert!(matches!(
        &master.background,
        Some(Fill::Solid(fill)) if matches!(fill.color.kind, ColorKind::Rgb(ref rgb) if rgb == "334455")
    ));
    let shape = &master.shapes[0];
    assert!(matches!(shape.border.style, BorderStyle::Solid));
    assert!(matches!(shape.border.dash_style, DashStyle::DashDot));

    let empty_master_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill><a:schemeClr val="accent2"/></a:solidFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sldMaster>"#;

    let empty_pptx = fixtures::MinimalPptx::new("")
        .with_full_master(empty_master_xml)
        .build();
    let empty_presentation = parse_pptx(&empty_pptx);
    assert!(matches!(
        &empty_presentation.masters[0].background,
        Some(Fill::Solid(fill))
            if matches!(fill.color.kind, ColorKind::Theme(ref name) if name == "accent2")
    ));
}

#[test]
fn parse_slide_master_directly_covers_solid_background_and_dashdot_branches() {
    use std::collections::HashMap;
    use std::io::{Cursor, Write};

    use pptx2html_core::parser::master_parser::parse_slide_master;
    use zip::write::SimpleFileOptions;
    use zip::{ZipArchive, ZipWriter};

    fn empty_archive() -> ZipArchive<Cursor<Vec<u8>>> {
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        let options = SimpleFileOptions::default();
        zip.start_file("ppt/empty.txt", options)
            .expect("start archive file");
        zip.write_all(b"").expect("write empty archive payload");
        ZipArchive::new(Cursor::new(
            zip.finish().expect("finish archive").into_inner(),
        ))
        .expect("open archive")
    }

    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill><a:schemeClr val="accent2"/></a:solidFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="2" name="DashDot"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="12700" y="25400"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700" cap="flat" cmpd="tri" algn="in">
            <a:prstDash val="dashDot"/>
            <a:bevel/>
          </a:ln>
        </p:spPr>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="3" name="ImplicitSolid"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="25400" y="38100"/><a:ext cx="381000" cy="254000"/></a:xfrm>
          <a:ln w="12700"></a:ln>
        </p:spPr>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sldMaster>"#;

    let mut archive = empty_archive();
    let master = parse_slide_master(xml, &HashMap::new(), &mut archive).expect("master parses");

    assert!(matches!(
        &master.background,
        Some(Fill::Solid(fill))
            if matches!(fill.color.kind, ColorKind::Theme(ref name) if name == "accent2")
    ));
    assert!(matches!(master.shapes[0].border.cap, LineCap::Flat));
    assert!(matches!(
        master.shapes[0].border.compound,
        CompoundLine::Triple
    ));
    assert!(matches!(master.shapes[0].border.style, BorderStyle::Solid));
    assert!(matches!(
        master.shapes[0].border.dash_style,
        DashStyle::DashDot
    ));
    assert!(matches!(master.shapes[0].border.join, LineJoin::Bevel));
    assert!(matches!(master.shapes[1].border.style, BorderStyle::Solid));
}

#[test]
fn parses_tables_and_unresolved_graphic_frames_and_renders_markers() {
    let slide = r#"
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="2" name="SmartArt"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="457200"/></p:xfrm>
        <a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/diagram">
          <dgm:relIds r:dm="1"/>
        </a:graphicData></a:graphic>
      </p:graphicFrame>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="3" name="Math"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="457200"/></p:xfrm>
        <a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/officeDocument/2006/math">
          <m:oMath/>
        </a:graphicData></a:graphic>
      </p:graphicFrame>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="4" name="Table"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="1828800" cy="914400"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
            <a:tbl>
              <a:tblPr bandRow="1" bandCol="1" firstRow="1" lastRow="1" firstCol="1" lastCol="1"></a:tblPr>
              <a:tblGrid>
                <a:gridCol w="914400"/>
                <a:gridCol w="457200"></a:gridCol>
              </a:tblGrid>
              <a:tr h="457200">
                <a:tc gridSpan="2" rowSpan="2" vMerge="1">
                  <a:txBody>
                    <a:bodyPr/>
                    <a:lstStyle/>
                    <a:p>
                      <a:pPr algn="ctr" lvl="1" indent="91440" marL="45720"/>
                      <a:defRPr sz="2000" cap="small" b="1"/>
                      <a:buClr><a:schemeClr val="accent2"/></a:buClr>
                      <a:r>
                        <a:rPr sz="1800" u="sng" strike="dblStrike" cap="all" baseline="20000" spc="100">
                          <a:hlinkClick r:id="rIdLink"/>
                        </a:rPr>
                        <a:t>Cell</a:t>
                      </a:r>
                    </a:p>
                  </a:txBody>
                  <a:tcPr marL="91440" marR="137160" marT="45720" marB="22860" anchor="ctr">
                    <a:solidFill><a:srgbClr val="00FF00"/></a:solidFill>
                    <a:lnL w="12700"><a:prstDash val="dash"/><a:srgbClr val="FF0000"/></a:lnL>
                    <a:lnR w="12700"><a:schemeClr val="accent3"/></a:lnR>
                    <a:lnT w="12700"><a:solidFill><a:srgbClr val="0000FF"/></a:solidFill></a:lnT>
                    <a:lnB w="12700"><a:noFill/></a:lnB>
                  </a:tcPr>
                </a:tc>
              </a:tr>
            </a:tbl>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:cxnSp>
        <p:nvCxnSpPr>
          <p:cNvPr id="5" name="Connector"/>
          <p:cNvCxnSpPr/>
          <p:nvPr/>
        </p:nvCxnSpPr>
        <p:spPr>
          <a:xfrm rot="5400000"><a:off x="0" y="0"/><a:ext cx="0" cy="914400"/></a:xfrm>
          <a:ln w="12700">
            <a:headEnd type="triangle" w="lg" len="sm"/>
            <a:tailEnd type="oval" w="sm" len="lg"/>
            <a:schemeClr val="accent1"/>
          </a:ln>
        </p:spPr>
        <p:stCxn id="10" idx="1"/>
        <p:endCxn id="11" idx="2"/>
      </p:cxnSp>
    "#;

    let slide_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rIdLink" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com" TargetMode="External"/>
</Relationships>"#;

    let pptx = fixtures::MinimalPptx::new(slide)
        .with_slide_rels(slide_rels)
        .build();

    let presentation = parse_pptx(&pptx);
    let slide = &presentation.slides[0];
    assert_eq!(slide.shapes.len(), 4);
    assert!(slide.shapes.iter().any(|shape| matches!(
        &shape.shape_type,
        ShapeType::Unsupported(data)
            if data.raw_xml.as_deref().is_some_and(|raw| raw.contains("relIds"))
    )));
    let table = slide
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape_type {
            ShapeType::Table(table) => Some(table),
            _ => None,
        })
        .expect("table shape");
    assert!(table.band_row && table.band_col && table.first_row && table.last_row);
    assert_eq!(table.col_widths.len(), 2);
    let cell = &table.rows[0].cells[0];
    assert_eq!(cell.col_span, 2);
    assert_eq!(cell.row_span, 2);
    assert!(cell.v_merge);
    assert!(matches!(cell.vertical_align, VerticalAlign::Middle));
    let para = &cell.text_body.as_ref().expect("cell text").paragraphs[0];
    assert!(matches!(para.alignment, Alignment::Center));
    assert_eq!(para.level, 1);
    assert_eq!(
        para.runs[0].hyperlink.as_deref(),
        Some("https://example.com")
    );

    let result = render_with_metadata(&pptx).expect("render with metadata");
    assert!(
        result.html.contains("marker-"),
        "expected SVG marker defs in HTML"
    );
    assert!(
        result.html.contains("unresolved-element"),
        "expected unresolved placeholders in HTML"
    );
    assert!(
        result.unresolved_elements.iter().any(|elem| elem
            .raw_xml
            .as_deref()
            .is_some_and(|raw| raw.contains("oMath"))),
        "expected raw XML to survive into unresolved metadata"
    );
}

#[test]
fn parses_group_and_dash_empty_variants_through_public_parser() {
    let slide = r#"
      <p:grpSp>
        <p:nvGrpSpPr><p:cNvPr id="10" name="Group"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
        <p:grpSpPr>
          <a:xfrm>
            <a:off x="100000" y="100000"/>
            <a:ext cx="5000000" cy="3000000"/>
            <a:chOff><a:off x="0" y="0"/></a:chOff>
            <a:chExt><a:ext cx="5000000" cy="3000000"/></a:chExt>
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
      </p:grpSp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="20" name="SolidDash"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="1000000" cy="500000"/></a:xfrm>
          <a:prstGeom prst="rect"/>
          <a:noFill/>
          <a:ln w="12700"><a:prstDash val="solid"/><a:sysClr lastClr="111111"/></a:ln>
        </p:spPr>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="21" name="DotDash"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="1000000" cy="500000"/></a:xfrm>
          <a:prstGeom prst="rect"/>
          <a:ln w="12700"><a:prstDash val="dot"/><a:sysClr val="windowText"/></a:ln>
        </p:spPr>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="22" name="LongDash"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="1000000" cy="500000"/></a:xfrm>
          <a:prstGeom prst="rect"/>
          <a:ln w="12700"><a:prstDash val="lgDash"/><a:srgbClr val="223344"/></a:ln>
        </p:spPr>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="23" name="SystemDot"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="1000000" cy="500000"/></a:xfrm>
          <a:prstGeom prst="rect"/>
          <a:ln w="12700"><a:prstDash val="sysDot"/><a:srgbClr val="445566"/></a:ln>
        </p:spPr>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="24" name="SystemDashDotDot"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="5400000" flipH="true" flipV="1"/>
          <a:prstGeom prst="rect"/>
          <a:ln w="12700"><a:prstDash val="sysDashDotDot"/><a:srgbClr val="778899"/></a:ln>
        </p:spPr>
      </p:sp>
    "#;

    let pptx = fixtures::MinimalPptx::new(slide).build();
    let pres = parse_pptx(&pptx);
    let shapes = &pres.slides[0].shapes;

    let group = shapes
        .iter()
        .find(|shape| matches!(shape.shape_type, ShapeType::Group(_, _)))
        .expect("group shape");
    assert!(matches!(&group.shape_type, ShapeType::Group(children, _) if children.len() == 1));

    let solid_dash = shapes
        .iter()
        .find(|shape| shape.name == "SolidDash")
        .expect("solid dash shape");
    assert!(matches!(solid_dash.fill, Fill::NoFill));
    assert!(matches!(solid_dash.border.style, BorderStyle::Solid));
    assert!(matches!(solid_dash.border.dash_style, DashStyle::Solid));

    let dot_dash = shapes
        .iter()
        .find(|shape| shape.name == "DotDash")
        .expect("dot dash shape");
    assert!(matches!(dot_dash.border.style, BorderStyle::Dotted));
    assert!(matches!(dot_dash.border.dash_style, DashStyle::Dot));

    let long_dash = shapes
        .iter()
        .find(|shape| shape.name == "LongDash")
        .expect("long dash shape");
    assert!(matches!(long_dash.border.style, BorderStyle::Dashed));
    assert!(matches!(long_dash.border.dash_style, DashStyle::LongDash));

    let system_dot = shapes
        .iter()
        .find(|shape| shape.name == "SystemDot")
        .expect("system dot shape");
    assert!(matches!(system_dot.border.style, BorderStyle::Dotted));
    assert!(matches!(system_dot.border.dash_style, DashStyle::SystemDot));

    let system_dash_dot_dot = shapes
        .iter()
        .find(|shape| shape.name == "SystemDashDotDot")
        .expect("system dash dot dot shape");
    assert!(system_dash_dot_dot.flip_h);
    assert!(system_dash_dot_dot.flip_v);
    assert!(matches!(
        system_dash_dot_dot.border.style,
        BorderStyle::Dotted
    ));
    assert!(matches!(
        system_dash_dot_dot.border.dash_style,
        DashStyle::SystemDashDotDot
    ));
}

#[test]
fn parses_empty_event_color_and_dash_matrix_through_public_parser() {
    let slide = r#"
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="30" name="Table"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="1828800" cy="914400"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
            <a:tbl>
              <a:tblPr bandRow="1" bandCol="true" firstRow="1" lastRow="true" firstCol="1" lastCol="true"></a:tblPr>
              <a:tblGrid>
                <a:gridCol w="914400"/>
                <a:gridCol w="457200"/>
              </a:tblGrid>
              <a:tr h="457200">
                <a:tc gridSpan="2" rowSpan="1" vMerge="1">
                  <a:txBody>
                    <a:bodyPr/>
                    <a:lstStyle/>
                    <a:p>
                      <a:pPr algn="ctr" lvl="1" indent="91440" marL="45720"/>
                      <a:defRPr sz="2000" spc="100" baseline="10000" cap="small" u="dashLong" strike="dblStrike" b="true" i="true"/>
                      <a:buClr><a:prstClr val="orange"/></a:buClr>
                      <a:r><a:rPr sz="1800"/><a:t>Cell One</a:t></a:r>
                      <a:r><a:rPr sz="1800"><a:hlinkClick r:id="rIdCell"/></a:rPr><a:t>Cell Two</a:t></a:r>
                      <a:br/>
                    </a:p>
                  </a:txBody>
                  <a:tcPr marL="91440" marR="137160" marT="45720" marB="22860" anchor="ctr">
                    <a:solidFill><a:srgbClr val="00FF00"/></a:solidFill>
                    <a:lnL w="12700"><a:prstDash val="solid"/><a:srgbClr val="FF0000"/></a:lnL>
                    <a:lnR w="12700"><a:prstDash val="dashDot"/><a:srgbClr val="0000FF"/></a:lnR>
                    <a:lnT w="12700"><a:prstDash val="lgDash"/><a:srgbClr val="123456"/></a:lnT>
                    <a:lnB w="12700"><a:prstDash val="sysDashDotDot"/><a:srgbClr val="654321"/></a:lnB>
                  </a:tcPr>
                </a:tc>
              </a:tr>
            </a:tbl>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="31" name="Shape Matrix"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="5400000" flipH="true"/>
          <a:gradFill>
            <a:gsLst>
              <a:gs pos="0"><a:prstClr val="orange"/></a:gs>
              <a:gs pos="100000"><a:sysClr lastClr="112233"/></a:gs>
            </a:gsLst>
            <a:lin ang="5400000"/>
            <a:path path="shape"/>
          </a:gradFill>
          <a:ln w="12700" cap="rnd" cmpd="dbl" algn="in">
            <a:prstDash val="sysDashDot"/>
            <a:bevel/>
            <a:headEnd type="triangle" w="lg" len="sm"/>
            <a:tailEnd type="diamond" w="sm" len="lg"/>
            <a:sysClr val="windowText"/>
          </a:ln>
          <a:effectLst>
            <a:outerShdw blurRad="12700" dist="25400" dir="5400000"><a:schemeClr val="accent1"/></a:outerShdw>
            <a:glow rad="6350"><a:sysClr lastClr="123456"/></a:glow>
          </a:effectLst>
          <a:custGeom>
            <a:avLst><a:gd name="adj1" fmla="val 50000"/></a:avLst>
            <a:gdLst><a:gd name="x1" fmla="val 100000"/></a:gdLst>
            <a:ahLst>
              <a:ahXY gdRefX="adj1" minX="0" maxX="100000" gdRefY="adj1" minY="0" maxY="100000"><a:pos x="50000" y="50000"/></a:ahXY>
              <a:ahPolar gdRefR="adj1" minR="0" maxR="100000" gdRefAng="adj1" minAng="0" maxAng="100000"><a:pos x="50000" y="50000"/></a:ahPolar>
            </a:ahLst>
            <a:cxnLst><a:cxn ang="0"><a:pos x="0" y="0"/></a:cxn></a:cxnLst>
            <a:rect l="0" t="0" r="100000" b="100000"/>
            <a:pathLst>
              <a:path w="100000" h="100000" fill="lighten"><a:moveTo><a:pt x="0" y="0"/></a:moveTo></a:path>
              <a:path w="100000" h="100000" fill="darken"><a:moveTo><a:pt x="0" y="0"/></a:moveTo></a:path>
              <a:path w="100000" h="100000" fill="lightenLess"><a:moveTo><a:pt x="0" y="0"/></a:moveTo></a:path>
            </a:pathLst>
          </a:custGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="1"/>
          <a:fillRef idx="2"/>
          <a:effectRef idx="3"/>
          <a:fontRef idx="minor"/>
        </p:style>
        <p:txBody>
          <a:bodyPr anchor="ctr" wrap="none"/>
          <a:normAutofit fontScale="80000" lnSpcReduction="10000"/>
          <a:p>
            <a:pPr algn="r" lvl="1" indent="12700" marL="25400"/>
            <a:defRPr sz="2400" spc="200" baseline="30000" cap="all" u="dbl" strike="sngStrike" b="true" i="1"/>
            <a:buClr><a:sysClr lastClr="445566"/></a:buClr>
            <a:r><a:rPr sz="1800"/><a:t>Shape One</a:t></a:r>
            <a:r><a:rPr sz="1800"><a:hlinkClick r:id="rIdShape"/></a:rPr><a:t>Shape Two</a:t></a:r>
            <a:br/>
          </a:p>
        </p:txBody>
      </p:sp>
    "#;

    let pptx = fixtures::MinimalPptx::new(slide)
        .with_slide_rels(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rIdCell" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com/cell" TargetMode="External"/>
  <Relationship Id="rIdShape" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com/shape" TargetMode="External"/>
</Relationships>"#,
        )
        .build();

    let presentation = parse_pptx(&pptx);
    let shapes = &presentation.slides[0].shapes;

    let table = shapes
        .iter()
        .find_map(|shape| match &shape.shape_type {
            ShapeType::Table(table) => Some(table),
            _ => None,
        })
        .expect("table shape");
    let cell = &table.rows[0].cells[0];
    assert!(table.band_row && table.band_col && table.first_row && table.last_row);
    assert!(table.first_col && table.last_col);
    assert_eq!(cell.col_span, 2);
    assert!(matches!(cell.border_left.style, BorderStyle::Solid));
    assert!(matches!(cell.border_left.dash_style, DashStyle::Solid));
    assert!(cell.border_right.width > 0.0);
    assert!(cell.border_top.width > 0.0);
    assert!(cell.border_bottom.width > 0.0);
    let cell_para = &cell.text_body.as_ref().expect("cell text").paragraphs[0];
    assert_eq!(cell_para.runs.len(), 3);
    assert_eq!(
        cell_para.runs[1].hyperlink.as_deref(),
        Some("https://example.com/cell")
    );
    let cell_def = cell_para.def_rpr.as_ref().expect("cell defRPr");
    assert_eq!(cell_def.font_size, Some(20.0));
    assert_eq!(cell_def.letter_spacing, Some(1.0));
    assert_eq!(cell_def.baseline, Some(10000));
    assert_eq!(cell_def.bold, Some(true));
    assert_eq!(cell_def.italic, Some(true));
    assert!(matches!(
        cell_def.capitalization,
        Some(TextCapitalization::Small)
    ));
    assert!(matches!(cell_def.underline, Some(UnderlineType::DashLong)));
    assert!(matches!(
        cell_def.strikethrough,
        Some(StrikethroughType::Double)
    ));

    let shape = shapes
        .iter()
        .find(|shape| shape.name == "Shape Matrix")
        .expect("shape matrix");
    assert!(matches!(
        shape.fill,
        Fill::Gradient(ref fill)
            if fill.stops.len() == 2
                && matches!(fill.gradient_type, GradientType::Shape)
                && matches!(fill.stops[0].color.kind, ColorKind::Preset(_))
                && matches!(fill.stops[1].color.kind, ColorKind::Rgb(_))
    ));
    assert!(matches!(shape.border.cap, LineCap::Round));
    assert!(matches!(shape.border.compound, CompoundLine::Double));
    assert!(matches!(shape.border.alignment, LineAlignment::Inset));
    assert!(matches!(shape.border.join, LineJoin::Bevel));
    assert!(matches!(shape.border.dash_style, DashStyle::SystemDashDot));
    assert!(shape.style_ref.as_ref().is_some());
    let text_body = shape.text_body.as_ref().expect("shape text body");
    assert!(matches!(
        text_body.auto_fit,
        AutoFit::Normal {
            font_scale: Some(v),
            line_spacing_reduction: Some(lsr)
        } if (v - 0.8).abs() < 1e-6 && (lsr - 0.1).abs() < 1e-6
    ));
    let para = &text_body.paragraphs[0];
    assert_eq!(para.runs.len(), 3);
    assert_eq!(
        para.runs[1].hyperlink.as_deref(),
        Some("https://example.com/shape")
    );
    let def = para.def_rpr.as_ref().expect("shape defRPr");
    assert_eq!(def.font_size, Some(24.0));
    assert!(matches!(def.capitalization, Some(TextCapitalization::All)));
    assert!(matches!(def.underline, Some(UnderlineType::Double)));
    assert!(matches!(def.strikethrough, Some(StrikethroughType::Single)));

    let custom_geom = match &shape.shape_type {
        ShapeType::CustomGeom(geom) => geom,
        other => panic!("expected custom geometry, got {other:?}"),
    };
    assert_eq!(custom_geom.adjust_handles.len(), 2);
    assert_eq!(custom_geom.connection_sites.len(), 1);
    assert_eq!(custom_geom.paths.len(), 3);
    assert!(matches!(custom_geom.paths[0].fill, PathFill::Lighten));
    assert!(matches!(custom_geom.paths[1].fill, PathFill::Darken));
    assert!(matches!(custom_geom.paths[2].fill, PathFill::LightenLess));

    let rendered = render_with_metadata(&pptx).expect("render with metadata");
    assert!(
        !rendered.html.is_empty(),
        "rendered HTML should still be produced for the fixture"
    );
}

#[test]
fn parses_empty_event_bullet_font_highlight_and_style_ref_matrix_through_public_parser() {
    let slide = r#"
      <p:bg>
        <p:bgPr>
          <a:solidFill><a:prstClr val="orange"/></a:solidFill>
        </p:bgPr>
      </p:bg>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="40" name="Style Ref Matrix"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="457200"/></a:xfrm>
          <a:prstGeom prst="rect"/>
        </p:spPr>
        <p:style>
          <a:lnRef idx="1"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="2"><a:prstClr val="orange"/></a:fillRef>
          <a:effectRef idx="3"><a:sysClr lastClr="112233"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="accent2"/></a:fontRef>
        </p:style>
        <p:txBody>
          <a:bodyPr/>
          <a:p>
            <a:pPr algn="ctr">
              <a:lnSpc><a:spcPct val="110000"/></a:lnSpc>
              <a:spcAft><a:spcPts val="1400"/></a:spcAft>
              <a:buChar char="•"/>
              <a:buClr><a:schemeClr val="accent4"/></a:buClr>
              <a:buFont typeface="Wingdings"/>
              <a:buSzPct val="125000"/>
              <a:defRPr sz="2000">
                <a:latin typeface="Aptos"/>
                <a:ea typeface="Yu Gothic"/>
                <a:cs typeface="Noto Sans Arabic"/>
              </a:defRPr>
            </a:pPr>
            <a:r>
              <a:rPr sz="1800">
                <a:hlinkClick r:id="rIdStart"></a:hlinkClick>
                <a:highlight><a:schemeClr val="accent6"/></a:highlight>
                <a:latin typeface="Calibri"/>
                <a:ea typeface="Meiryo"/>
                <a:cs typeface="Noto Sans Devanagari"/>
                <a:sysClr lastClr="223344"/>
              </a:rPr>
              <a:t>Styled Run</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="41" name="Cell Matrix"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="1828800" cy="914400"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
            <a:tbl>
              <a:tblPr bandRow="1"/>
              <a:tblGrid><a:gridCol w="914400"/></a:tblGrid>
              <a:tr h="457200">
                <a:tc>
                  <a:txBody>
                    <a:bodyPr/>
                    <a:lstStyle/>
                    <a:p>
                      <a:pPr algn="l">
                        <a:lnSpc><a:spcPts val="900"/></a:lnSpc>
                        <a:spcBef><a:spcPct val="25000"/></a:spcBef>
                        <a:spcAft><a:spcPts val="1200"/></a:spcAft>
                        <a:buChar char="◦"/>
                        <a:buClr><a:prstClr val="orange"/></a:buClr>
                        <a:buFont typeface="Symbol"/>
                        <a:buSzPts val="1600"/>
                        <a:defRPr sz="1900">
                          <a:latin typeface="Cell Latin"/>
                          <a:ea typeface="Cell EA"/>
                          <a:cs typeface="Cell CS"/>
                        </a:defRPr>
                      </a:pPr>
                      <a:r>
                        <a:rPr sz="1700">
                          <a:highlight><a:srgbClr val="FFFF00"/></a:highlight>
                          <a:latin typeface="CellRun Latin"/>
                          <a:ea typeface="CellRun EA"/>
                          <a:cs typeface="CellRun CS"/>
                          <a:prstClr val="orange"/>
                        </a:rPr>
                        <a:t>Cell Run</a:t>
                      </a:r>
                    </a:p>
                  </a:txBody>
                  <a:tcPr anchor="ctr">
                    <a:solidFill><a:schemeClr val="accent5"/></a:solidFill>
                    <a:lnL w="12700"><a:sysClr val="windowText"/></a:lnL>
                    <a:lnR w="12700"><a:prstClr val="orange"/></a:lnR>
                  </a:tcPr>
                </a:tc>
              </a:tr>
            </a:tbl>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
    "#;

    let pptx = fixtures::MinimalPptx::new(slide)
        .with_slide_rels(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rIdStart" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com/start" TargetMode="External"/>
</Relationships>"#,
        )
        .build();

    let presentation = parse_pptx(&pptx);
    let slide = &presentation.slides[0];
    assert!(matches!(
        &slide.background,
        Some(Fill::Solid(fill))
            if matches!(fill.color.kind, ColorKind::Preset(ref name) if name == "orange")
    ));

    let shape = slide
        .shapes
        .iter()
        .find(|shape| shape.name == "Style Ref Matrix")
        .expect("style ref shape");
    let style_ref = shape.style_ref.as_ref().expect("shape style ref");
    assert!(matches!(
        style_ref
            .ln_ref
            .as_ref()
            .map(|style| style.color.kind.clone()),
        Some(ColorKind::Theme(name)) if name == "accent1"
    ));
    assert!(matches!(
        style_ref
            .fill_ref
            .as_ref()
            .map(|style| style.color.kind.clone()),
        Some(ColorKind::Preset(name)) if name == "orange"
    ));
    assert!(matches!(
        style_ref
            .effect_ref
            .as_ref()
            .map(|style| style.color.kind.clone()),
        Some(ColorKind::Rgb(rgb)) if rgb == "112233"
    ));
    assert!(matches!(
        style_ref
            .font_ref
            .as_ref()
            .map(|style| style.color.kind.clone()),
        Some(ColorKind::Theme(name)) if name == "accent2"
    ));

    let para = &shape
        .text_body
        .as_ref()
        .expect("shape text body")
        .paragraphs[0];
    assert!(matches!(
        para.line_spacing,
        Some(pptx2html_core::model::SpacingValue::Percent(v)) if (v - 1.1).abs() < 1e-6
    ));
    assert!(matches!(
        para.space_after,
        Some(pptx2html_core::model::SpacingValue::Points(v)) if (v - 14.0).abs() < 1e-6
    ));
    assert!(matches!(
        &para.bullet,
        Some(Bullet::Char(bullet))
            if bullet.char == "•"
    ));
    let shape_def = para.def_rpr.as_ref().expect("shape defRPr");
    assert_eq!(shape_def.font_latin.as_deref(), Some("Aptos"));
    assert_eq!(shape_def.font_ea.as_deref(), Some("Yu Gothic"));
    assert_eq!(shape_def.font_cs.as_deref(), Some("Noto Sans Arabic"));
    let run = &para.runs[0];
    assert_eq!(run.hyperlink.as_deref(), Some("https://example.com/start"));
    assert_eq!(run.font.latin.as_deref(), Some("Calibri"));
    assert_eq!(run.font.east_asian.as_deref(), Some("Meiryo"));
    assert_eq!(
        run.font.complex_script.as_deref(),
        Some("Noto Sans Devanagari")
    );
    assert_eq!(run.style.color.to_css().as_deref(), Some("#223344"));
    assert_eq!(
        run.style
            .highlight
            .as_ref()
            .and_then(|c| c.to_css())
            .as_deref(),
        Some("#70AD47")
    );

    let table = slide
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape_type {
            ShapeType::Table(table) => Some(table),
            _ => None,
        })
        .expect("table shape");
    let cell = &table.rows[0].cells[0];
    assert!(matches!(
        &cell.fill,
        Fill::Solid(fill) if fill.color.to_css().as_deref() == Some("#5B9BD5")
    ));
    assert_eq!(cell.border_left.color.to_css().as_deref(), Some("#000000"));
    assert_eq!(cell.border_right.color.to_css().as_deref(), Some("#FFA500"));
    let cell_para = &cell.text_body.as_ref().expect("cell text body").paragraphs[0];
    assert!(matches!(
        cell_para.line_spacing,
        Some(pptx2html_core::model::SpacingValue::Points(v)) if (v - 9.0).abs() < 1e-6
    ));
    assert!(matches!(
        cell_para.space_before,
        Some(pptx2html_core::model::SpacingValue::Percent(v)) if (v - 0.25).abs() < 1e-6
    ));
    assert!(matches!(
        cell_para.space_after,
        Some(pptx2html_core::model::SpacingValue::Points(v)) if (v - 12.0).abs() < 1e-6
    ));
    assert!(matches!(
        &cell_para.bullet,
        Some(Bullet::Char(bullet))
            if bullet.char == "◦"
    ));
    let cell_def = cell_para.def_rpr.as_ref().expect("cell defRPr");
    assert_eq!(cell_def.font_latin.as_deref(), Some("Cell Latin"));
    assert_eq!(cell_def.font_ea.as_deref(), Some("Cell EA"));
    assert_eq!(cell_def.font_cs.as_deref(), Some("Cell CS"));
    let cell_run = &cell_para.runs[0];
    assert_eq!(cell_run.font.latin.as_deref(), Some("CellRun Latin"));
    assert_eq!(cell_run.font.east_asian.as_deref(), Some("CellRun EA"));
    assert_eq!(cell_run.font.complex_script.as_deref(), Some("CellRun CS"));
    assert_eq!(cell_run.style.color.to_css().as_deref(), Some("#FFA500"));
    assert_eq!(
        cell_run
            .style
            .highlight
            .as_ref()
            .and_then(|c| c.to_css())
            .as_deref(),
        Some("#FFFF00")
    );
}

#[test]
fn parses_shape_text_autofit_connector_and_ole_branches() {
    let slide = r#"
      <p:graphicFrame>
        <p:nvGraphicFramePr><p:cNvPr id="5" name="OLE"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr>
        <p:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="457200"/></p:xfrm>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/presentationml/2006/oleObject">
            <oleObject progId="Excel.Sheet">
              <oleData sheet="Budget"/>
            </oleObject>
          </a:graphicData>
        </a:graphic>
      </p:graphicFrame>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="6" name="Styled Shape"/>
          <p:cNvSpPr/>
          <p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="5400000" flipH="1" flipV="true">
            <a:off x="12700" y="25400"/>
            <a:ext cx="381000" cy="254000"/>
          </a:xfrm>
          <a:ln w="12700" cap="flat" cmpd="tri" algn="in">
            <a:prstDash val="sysDash"/>
            <a:solidFill><a:srgbClr val="AA00AA"/></a:solidFill>
          </a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr anchor="ctr" anchorCtr="1" rot="5400000" vert="vert270"
                    lIns="91440" tIns="45720" rIns="182880" bIns="22860" wrap="none"/>
          <a:noAutofit/>
          <a:lstStyle>
            <a:lvl1pPr algn="r">
              <a:lnSpc><a:spcPct val="90000"/></a:lnSpc>
              <a:spcBef><a:spcPts val="600"/></a:spcBef>
              <a:spcAft><a:spcPts val="800"/></a:spcAft>
              <a:defRPr sz="1800" spc="200" baseline="30000" cap="small" u="dbl"
                        strike="sngStrike" b="1" i="1">
                <a:latin typeface="Calibri"/>
                <a:ea typeface="Yu Gothic"/>
                <a:cs typeface="Noto Sans Arabic"/>
                <a:schemeClr val="accent2"/>
              </a:defRPr>
            </a:lvl1pPr>
          </a:lstStyle>
          <a:p>
            <a:pPr algn="just" lvl="1" indent="91440" marL="45720">
              <a:lnSpc><a:spcPct val="110000"/></a:lnSpc>
              <a:spcBef><a:spcPts val="1200"/></a:spcBef>
              <a:spcAft><a:spcPts val="2400"/></a:spcAft>
              <a:buChar char="•"/>
              <a:buClr><a:srgbClr val="00FF00"/></a:buClr>
            </a:pPr>
            <a:defRPr sz="2000" cap="all" b="1" i="1" u="sng" strike="dblStrike"
                      baseline="20000" spc="100">
              <a:latin typeface="Calibri"/>
              <a:ea typeface="Meiryo"/>
              <a:cs typeface="Noto Sans Devanagari"/>
              <a:srgbClr val="123456"/>
            </a:defRPr>
            <a:r>
              <a:rPr sz="1600" cap="all" b="1" i="1" u="sng" strike="dblStrike"
                     baseline="10000" spc="50">
                <a:hlinkClick r:id="rIdLink"/>
                <a:srgbClr val="654321"/>
              </a:rPr>
              <a:t>Styled</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="7" name="Shrink Shape"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="25400" y="38100"/><a:ext cx="254000" cy="127000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr wrap="none"/>
          <a:spAutoFit/>
          <a:p><a:r><a:t>Shrink</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
      <p:cxnSp>
        <p:nvCxnSpPr>
          <p:cNvPr id="8" name="Connector"/>
          <p:cNvCxnSpPr/>
          <p:nvPr/>
        </p:nvCxnSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="0"/></a:xfrm>
          <a:ln w="12700" cap="rnd" cmpd="dbl" algn="in">
            <a:prstDash val="dashDot"/>
            <a:headEnd type="triangle" w="lg" len="sm"/>
            <a:tailEnd type="oval" w="sm" len="lg"/>
            <a:schemeClr val="accent1"/>
          </a:ln>
        </p:spPr>
        <p:stCxn id="10" idx="1"/>
        <p:endCxn id="11" idx="2"/>
      </p:cxnSp>
    "#;

    let slide_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rIdLink" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com/styled" TargetMode="External"/>
</Relationships>"#;

    let pptx = fixtures::MinimalPptx::new(slide)
        .with_slide_rels(slide_rels)
        .build();

    let presentation = parse_pptx(&pptx);
    let slide = &presentation.slides[0];

    let ole = slide
        .shapes
        .iter()
        .find(|shape| matches!(&shape.shape_type, ShapeType::Unsupported(_)))
        .expect("ole placeholder shape");
    assert!(matches!(
        &ole.shape_type,
        ShapeType::Unsupported(data)
            if data.raw_xml.as_deref().is_some_and(|raw| raw.contains("oleData"))
    ));

    let styled = slide
        .shapes
        .iter()
        .find(|shape| shape.name == "Styled Shape")
        .expect("styled shape");
    assert!(styled.flip_h && styled.flip_v);
    assert!(matches!(styled.border.cap, LineCap::Flat));
    assert!(matches!(styled.border.compound, CompoundLine::Triple));
    assert!(matches!(styled.border.alignment, LineAlignment::Inset));
    let text_body = styled.text_body.as_ref().expect("styled text body");
    assert!(matches!(text_body.vertical_align, VerticalAlign::Middle));
    assert!(matches!(text_body.auto_fit, AutoFit::NoAutoFit));
    assert_eq!(styled.vertical_text.as_deref(), Some("vert270"));
    let defaults = text_body
        .list_style
        .as_ref()
        .and_then(|ls| ls.levels[0].as_ref())
        .and_then(|pd| pd.def_run_props.as_ref())
        .expect("shape list-style defaults");
    assert_eq!(defaults.font_size, Some(18.0));
    assert_eq!(defaults.letter_spacing, Some(2.0));
    assert_eq!(defaults.baseline, Some(30000));
    assert_eq!(defaults.font_latin.as_deref(), Some("Calibri"));
    let para = &text_body.paragraphs[0];
    assert!(matches!(para.alignment, Alignment::Justify));
    assert_eq!(para.level, 1);
    assert!(matches!(
        &para.bullet,
        Some(Bullet::Char(bullet)) if bullet.char == "•"
    ));
    let run = &para.runs[0];
    assert_eq!(run.hyperlink.as_deref(), Some("https://example.com/styled"));
    assert_eq!(run.style.color.to_css().as_deref(), Some("#654321"));

    let shrink = slide
        .shapes
        .iter()
        .find(|shape| shape.name == "Shrink Shape")
        .expect("shrink shape");
    assert!(matches!(
        shrink.text_body.as_ref().map(|body| &body.auto_fit),
        Some(AutoFit::Shrink)
    ));

    let connector = slide
        .shapes
        .iter()
        .find(|shape| shape.name == "Connector")
        .expect("connector");
    assert!(matches!(&connector.shape_type, ShapeType::Custom(name) if name == "line"));
    assert_eq!(
        connector
            .start_connection
            .as_ref()
            .map(|connection| connection.shape_id),
        Some(10)
    );
    assert_eq!(
        connector
            .end_connection
            .as_ref()
            .map(|connection| connection.site_idx),
        Some(2)
    );
}
