mod fixtures;

use pptx2html_core::model::{
    Alignment, AutoFit, Bullet, ClrMapOverride, CompoundLine, DashStyle, Fill, LineAlignment,
    LineCap, LineJoin, PlaceholderType, ShapeType, VerticalAlign,
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
        connector.start_connection.as_ref().map(|connection| connection.shape_id),
        Some(10)
    );
    assert_eq!(
        connector.end_connection.as_ref().map(|connection| connection.site_idx),
        Some(2)
    );
}
