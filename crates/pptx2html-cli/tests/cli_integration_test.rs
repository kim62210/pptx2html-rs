use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use zip::ZipWriter;
use zip::write::SimpleFileOptions;

fn unique_temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("pptx2html-cli-{name}-{nanos}"))
}

fn write_temp_file(name: &str, bytes: &[u8]) -> PathBuf {
    let path = unique_temp_path(name).with_extension("pptx");
    fs::write(&path, bytes).expect("write temp pptx");
    path
}

#[test]
fn info_command_outputs_json_metadata() {
    let input = write_temp_file("info", include_bytes!("fixtures/single-slide.pptx"));

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--info")
        .output()
        .expect("run cli");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("\"slide_count\":1"));
    assert!(stdout.contains("\"width_px\":960.0"));

    fs::remove_file(input).ok();
}

#[test]
fn single_file_conversion_writes_requested_output() {
    let input = write_temp_file("single", include_bytes!("fixtures/single-slide.pptx"));
    let output_path = unique_temp_path("single-output").with_extension("html");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--slides")
        .arg("1")
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("run cli");

    assert!(output.status.success(), "{output:?}");
    let html = fs::read_to_string(&output_path).expect("read output html");
    assert!(html.contains("Slide One"));

    fs::remove_file(input).ok();
    fs::remove_file(output_path).ok();
}

#[test]
fn multi_file_conversion_writes_per_slide_outputs() {
    let input = write_temp_file("multi", include_bytes!("fixtures/two-slides.pptx"));
    let output_dir = unique_temp_path("multi-output");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--format")
        .arg("multi")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("run cli");

    assert!(output.status.success(), "{output:?}");
    let slide_one = fs::read_to_string(output_dir.join("slide-1.html")).expect("slide 1 html");
    let slide_two = fs::read_to_string(output_dir.join("slide-2.html")).expect("slide 2 html");
    assert!(slide_one.contains("Slide One"));
    assert!(slide_two.contains("Slide Two"));

    fs::remove_file(input).ok();
    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn invalid_slide_selection_returns_nonzero_exit_code() {
    let input = write_temp_file(
        "invalid-slides",
        include_bytes!("fixtures/single-slide.pptx"),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--slides")
        .arg("3-1")
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Invalid --slides value"));

    fs::remove_file(input).ok();
}

#[test]
fn missing_input_returns_nonzero_exit_code() {
    let missing = unique_temp_path("missing").with_extension("pptx");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&missing)
        .arg("--info")
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to read presentation"));
}

#[test]
fn single_file_conversion_reports_conversion_failures_for_missing_input() {
    let missing = unique_temp_path("missing-convert").with_extension("pptx");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&missing)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Conversion failed"));
}

#[test]
fn info_command_escapes_title_strings() {
    let input = write_temp_file("info-title", &build_titled_single_slide_pptx());

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--info")
        .output()
        .expect("run cli");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("\"title\":\"Quarterly \\\\\\\"Deck\\\\\\\" \\\\\\\\ Notes\""));

    fs::remove_file(input).ok();
}

#[test]
fn multi_file_conversion_reports_output_directory_creation_failure() {
    let input = write_temp_file(
        "multi-dir-fail",
        include_bytes!("fixtures/single-slide.pptx"),
    );
    let output_path = unique_temp_path("multi-dir-target");
    fs::write(&output_path, b"not-a-directory").expect("seed output path as file");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--format")
        .arg("multi")
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to create output directory"));

    fs::remove_file(input).ok();
    fs::remove_file(output_path).ok();
}

#[test]
fn multi_file_conversion_reports_slide_write_failures() {
    let input = write_temp_file(
        "multi-write-fail",
        include_bytes!("fixtures/single-slide.pptx"),
    );
    let output_dir = unique_temp_path("multi-write-target");
    fs::create_dir_all(output_dir.join("slide-1.html")).expect("seed slide html path as dir");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--format")
        .arg("multi")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to write"));

    fs::remove_file(input).ok();
    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn single_file_conversion_reports_output_write_failures() {
    let input = write_temp_file(
        "single-write-fail",
        include_bytes!("fixtures/single-slide.pptx"),
    );
    let output_dir = unique_temp_path("single-output-dir");
    fs::create_dir_all(&output_dir).expect("create output dir");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to write output file"));

    fs::remove_file(input).ok();
    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn multi_file_conversion_reports_info_failures_for_missing_input() {
    let missing = unique_temp_path("missing-multi").with_extension("pptx");
    let output_dir = unique_temp_path("missing-multi-output");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&missing)
        .arg("--format")
        .arg("multi")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to read presentation"));

    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn multi_file_conversion_reports_slide_conversion_failures() {
    let input = write_temp_file("missing-slide", &build_missing_slide_pptx());
    let output_dir = unique_temp_path("missing-slide-output");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--format")
        .arg("multi")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(
        stderr.contains("Failed to read presentation")
            || stderr.contains("Failed to convert slide 1"),
        "unexpected stderr: {stderr}"
    );

    fs::remove_file(input).ok();
    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn multi_file_conversion_reports_external_asset_write_failures() {
    let input = write_temp_file("multi-assets-fail", &build_background_image_pptx());
    let output_dir = unique_temp_path("multi-assets-output");
    fs::create_dir_all(&output_dir).expect("create output dir");
    fs::write(output_dir.join("images"), b"blocking-file").expect("create blocking file");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--format")
        .arg("multi")
        .arg("--no-embed")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to write external assets"));

    fs::remove_file(input).ok();
    fs::remove_file(output_dir.join("images")).ok();
    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn single_file_conversion_reports_external_asset_write_failures() {
    let input = write_temp_file("single-assets-fail", &build_background_image_pptx());
    let output_dir = unique_temp_path("single-assets-output");
    fs::create_dir_all(&output_dir).expect("create output dir");
    fs::write(output_dir.join("images"), b"blocking-file").expect("create blocking file");
    let output_path = output_dir.join("deck.html");

    let output = Command::new(env!("CARGO_BIN_EXE_pptx2html"))
        .arg(&input)
        .arg("--no-embed")
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("run cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("Failed to write external assets"));

    fs::remove_file(input).ok();
    fs::remove_file(output_dir.join("images")).ok();
    fs::remove_file(output_path).ok();
    fs::remove_dir_all(output_dir).ok();
}

fn build_titled_single_slide_pptx() -> Vec<u8> {
    let cursor = std::io::Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default();

    zip.start_file("[Content_Types].xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
</Types>"#,
    )
    .unwrap();

    zip.start_file("_rels/.rels", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/presentation.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst/>
  <p:sldIdLst><p:sldId id="256" r:id="rId1"/></p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#,
    )
    .unwrap();

    zip.start_file("ppt/_rels/presentation.xml.rels", options)
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/slides/slide1.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="2" name="Title"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
        <p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="457200"/></a:xfrm></p:spPr>
        <p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>Titled Slide</a:t></a:r></a:p></p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"#,
    )
    .unwrap();

    zip.start_file("ppt/slides/_rels/slide1.xml.rels", options)
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#,
    )
    .unwrap();

    zip.start_file("ppt/theme/theme1.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="TestTheme">
  <a:themeElements>
    <a:clrScheme name="TestColors">
      <a:dk1><a:srgbClr val="000000"/></a:dk1>
      <a:lt1><a:srgbClr val="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="000000"/></a:dk2>
      <a:lt2><a:srgbClr val="FFFFFF"/></a:lt2>
      <a:accent1><a:srgbClr val="4472C4"/></a:accent1>
      <a:accent2><a:srgbClr val="ED7D31"/></a:accent2>
      <a:accent3><a:srgbClr val="A5A5A5"/></a:accent3>
      <a:accent4><a:srgbClr val="FFC000"/></a:accent4>
      <a:accent5><a:srgbClr val="5B9BD5"/></a:accent5>
      <a:accent6><a:srgbClr val="70AD47"/></a:accent6>
      <a:hlink><a:srgbClr val="0563C1"/></a:hlink>
      <a:folHlink><a:srgbClr val="954F72"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name="TestFonts">
      <a:majorFont><a:latin typeface="Calibri"/></a:majorFont>
      <a:minorFont><a:latin typeface="Calibri"/></a:minorFont>
    </a:fontScheme>
  </a:themeElements>
</a:theme>"#,
    )
    .unwrap();

    zip.start_file("docProps/core.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
                   xmlns:dc="http://purl.org/dc/elements/1.1/">
  <dc:title>Quarterly \"Deck\" \\ Notes</dc:title>
</cp:coreProperties>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}

fn build_missing_slide_pptx() -> Vec<u8> {
    let cursor = std::io::Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default();

    zip.start_file("[Content_Types].xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/slides/slide2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
</Types>"#,
    )
    .unwrap();

    zip.start_file("_rels/.rels", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/presentation.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldIdLst>
    <p:sldId id="256" r:id="rId1"/>
    <p:sldId id="257" r:id="rId2"/>
  </p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
</p:presentation>"#,
    )
    .unwrap();

    zip.start_file("ppt/_rels/presentation.xml.rels", options)
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide2.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/slides/slide1.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sld>"#,
    )
    .unwrap();

    zip.start_file("ppt/slides/slide2.xml", options).unwrap();
    zip.write_all(b"<not-xml").unwrap();

    zip.finish().unwrap().into_inner()
}

fn build_background_image_pptx() -> Vec<u8> {
    let cursor = std::io::Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default();

    zip.start_file("[Content_Types].xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="png" ContentType="image/png"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
</Types>"#,
    )
    .unwrap();

    zip.start_file("_rels/.rels", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/presentation.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst><p:sldMasterId r:id="rId1"/></p:sldMasterIdLst>
  <p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
</p:presentation>"#,
    )
    .unwrap();

    zip.start_file("ppt/_rels/presentation.xml.rels", options)
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/slides/slide1.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
</p:sld>"#,
    )
    .unwrap();

    zip.start_file("ppt/slides/_rels/slide1.xml.rels", options)
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/image1.png"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/slideMasters/slideMaster1.xml", options)
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/></p:spTree></p:cSld>
</p:sldMaster>"#,
    )
    .unwrap();

    zip.start_file("ppt/slideMasters/_rels/slideMaster1.xml.rels", options)
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
</Relationships>"#,
    )
    .unwrap();

    zip.start_file("ppt/media/image1.png", options).unwrap();
    zip.write_all(&[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xE2, 0x21, 0xBC, 0x33, 0x00, 0x00, 0x00,
        0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ])
    .unwrap();

    zip.start_file("ppt/theme/theme1.xml", options).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="TestTheme">
  <a:themeElements>
    <a:clrScheme name="TestColors">
      <a:dk1><a:srgbClr val="000000"/></a:dk1>
      <a:lt1><a:srgbClr val="FFFFFF"/></a:lt1>
    </a:clrScheme>
    <a:fontScheme name="TestFonts">
      <a:majorFont><a:latin typeface="Calibri"/></a:majorFont>
      <a:minorFont><a:latin typeface="Calibri"/></a:minorFont>
    </a:fontScheme>
  </a:themeElements>
</a:theme>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}
