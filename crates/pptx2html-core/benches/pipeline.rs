use std::io::{Cursor, Write};

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use pptx2html_core::parser::PptxParser;
use pptx2html_core::renderer::HtmlRenderer;

/// Build a minimal PPTX with N slides, each containing a text shape
fn build_test_pptx(slide_count: usize) -> Vec<u8> {
    let buf = Vec::new();
    let cursor = Cursor::new(buf);
    let mut zip = ZipWriter::new(cursor);
    let opts = SimpleFileOptions::default();

    // Build slide ID list and relationships
    let mut slide_id_list = String::new();
    let mut pres_rels = String::new();
    let mut content_overrides = String::new();

    for i in 1..=slide_count {
        let id = 255 + i;
        let rid = format!("rId{}", i + 2); // rId1=master, rId2=theme, rId3+=slides
        slide_id_list.push_str(&format!(
            "<p:sldId id=\"{id}\" r:id=\"{rid}\"/>"
        ));
        pres_rels.push_str(&format!(
            "<Relationship Id=\"{rid}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{i}.xml\"/>"
        ));
        content_overrides.push_str(&format!(
            "<Override PartName=\"/ppt/slides/slide{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>"
        ));
    }

    // [Content_Types].xml
    let content_types = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  {content_overrides}
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
</Types>"#
    );
    zip.start_file("[Content_Types].xml", opts).unwrap();
    zip.write_all(content_types.as_bytes()).unwrap();

    // _rels/.rels
    zip.start_file("_rels/.rels", opts).unwrap();
    zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#).unwrap();

    // ppt/presentation.xml
    let pres_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst><p:sldMasterId r:id="rId1"/></p:sldMasterIdLst>
  <p:sldIdLst>{slide_id_list}</p:sldIdLst>
  <p:sldSz cx="12192000" cy="6858000"/>
</p:presentation>"#
    );
    zip.start_file("ppt/presentation.xml", opts).unwrap();
    zip.write_all(pres_xml.as_bytes()).unwrap();

    // ppt/_rels/presentation.xml.rels
    let pres_rels_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
  {pres_rels}
</Relationships>"#
    );
    zip.start_file("ppt/_rels/presentation.xml.rels", opts).unwrap();
    zip.write_all(pres_rels_xml.as_bytes()).unwrap();

    // Slide XML template with multiple shapes
    for i in 1..=slide_count {
        let slide_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Title"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm>
        <a:prstGeom prst="rect"/>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:rPr lang="en-US" sz="2400" b="1"/><a:t>Slide {i} Title</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>
    <p:sp>
      <p:nvSpPr><p:cNvPr id="3" name="Content"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="457200" y="1600200"/><a:ext cx="8229600" cy="4525963"/></a:xfrm>
        <a:prstGeom prst="rect"/>
        <a:solidFill><a:srgbClr val="F0F0F0"/></a:solidFill>
      </p:spPr>
      <p:txBody>
        <a:bodyPr/>
        <a:p><a:r><a:rPr lang="en-US" sz="1800"/><a:t>Content text for slide {i}. This paragraph has multiple lines of text to simulate real content.</a:t></a:r></a:p>
        <a:p><a:r><a:rPr lang="en-US" sz="1400" i="1"/><a:t>Additional detail paragraph with italic text styling.</a:t></a:r></a:p>
      </p:txBody>
    </p:sp>
    <p:sp>
      <p:nvSpPr><p:cNvPr id="4" name="Shape"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr>
        <a:xfrm><a:off x="8000000" y="300000"/><a:ext cx="1000000" cy="800000"/></a:xfrm>
        <a:prstGeom prst="ellipse"/>
        <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        <a:ln w="12700"><a:solidFill><a:srgbClr val="2F5597"/></a:solidFill></a:ln>
      </p:spPr>
    </p:sp>
  </p:spTree></p:cSld>
</p:sld>"#
        );
        let slide_path = format!("ppt/slides/slide{i}.xml");
        zip.start_file(&slide_path, opts).unwrap();
        zip.write_all(slide_xml.as_bytes()).unwrap();

        // Slide rels
        let rels_path = format!("ppt/slides/_rels/slide{i}.xml.rels");
        zip.start_file(&rels_path, opts).unwrap();
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>"#).unwrap();
    }

    // Theme
    zip.start_file("ppt/theme/theme1.xml", opts).unwrap();
    zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Bench">
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
  </a:themeElements>
</a:theme>"#).unwrap();

    zip.finish().unwrap().into_inner()
}

fn bench_parse_simple(c: &mut Criterion) {
    let data = build_test_pptx(5);
    c.bench_function("parse_5_slides", |b| {
        b.iter(|| {
            let _pres = PptxParser::parse_bytes(black_box(&data)).unwrap();
        });
    });
}

fn bench_parse_medium(c: &mut Criterion) {
    let data = build_test_pptx(20);
    c.bench_function("parse_20_slides", |b| {
        b.iter(|| {
            let _pres = PptxParser::parse_bytes(black_box(&data)).unwrap();
        });
    });
}

fn bench_render_simple(c: &mut Criterion) {
    let data = build_test_pptx(5);
    let pres = PptxParser::parse_bytes(&data).unwrap();
    c.bench_function("render_5_slides", |b| {
        b.iter(|| {
            let _html = HtmlRenderer::render(black_box(&pres)).unwrap();
        });
    });
}

fn bench_render_medium(c: &mut Criterion) {
    let data = build_test_pptx(20);
    let pres = PptxParser::parse_bytes(&data).unwrap();
    c.bench_function("render_20_slides", |b| {
        b.iter(|| {
            let _html = HtmlRenderer::render(black_box(&pres)).unwrap();
        });
    });
}

fn bench_full_pipeline(c: &mut Criterion) {
    let data = build_test_pptx(20);
    c.bench_function("full_pipeline_20_slides", |b| {
        b.iter(|| {
            let _html = pptx2html_core::convert_bytes(black_box(&data)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_parse_simple,
    bench_parse_medium,
    bench_render_simple,
    bench_render_medium,
    bench_full_pipeline,
);
criterion_main!(benches);
