mod fixtures;

use std::fs;

use fixtures::MinimalPptx;
use pptx2html_core::{
    ConversionOptions, convert_bytes, convert_bytes_with_metadata, convert_bytes_with_options,
    convert_file, convert_file_with_metadata, convert_file_with_options, get_info,
    get_info_from_bytes,
};
use tempfile::tempdir;

fn basic_text_shape(text: &str) -> String {
    format!(
        r#"<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="TextBox"/>
    <p:cNvSpPr txBox="1"/>
    <p:nvPr/>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="0" y="0"/>
      <a:ext cx="914400" cy="457200"/>
    </a:xfrm>
    <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
  </p:spPr>
  <p:txBody>
    <a:bodyPr/>
    <a:lstStyle/>
    <a:p><a:r><a:t>{text}</a:t></a:r></a:p>
  </p:txBody>
</p:sp>"#
    )
}

#[test]
fn public_file_and_bytes_apis_delegate_consistently() {
    let bytes = MinimalPptx::new(&basic_text_shape("Public API"))
        .with_core_properties(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
                   xmlns:dc="http://purl.org/dc/elements/1.1/">
  <dc:title>API Deck</dc:title>
</cp:coreProperties>"#,
        )
        .build();
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("sample.pptx");
    fs::write(&path, &bytes).expect("write pptx");

    let opts = ConversionOptions {
        embed_images: false,
        include_hidden: false,
        slide_range: None,
        slide_indices: Some(vec![1]),
    };

    let file_html = convert_file(&path).expect("convert_file");
    let file_html_with_opts = convert_file_with_options(&path, &opts).expect("convert_file opts");
    let file_result = convert_file_with_metadata(&path).expect("convert_file metadata");
    let bytes_html = convert_bytes(&bytes).expect("convert_bytes");
    let bytes_html_with_opts =
        convert_bytes_with_options(&bytes, &opts).expect("convert_bytes opts");
    let bytes_result = convert_bytes_with_metadata(&bytes).expect("convert_bytes metadata");
    let info_from_file = get_info(&path).expect("get_info");
    let info_from_bytes = get_info_from_bytes(&bytes).expect("get_info_from_bytes");

    assert!(file_html.contains("Public API"));
    assert!(file_html_with_opts.contains("Public API"));
    assert!(file_result.html.contains("Public API"));
    assert!(bytes_html.contains("Public API"));
    assert!(bytes_html_with_opts.contains("Public API"));
    assert!(bytes_result.html.contains("Public API"));

    assert_eq!(file_result.slide_count, 1);
    assert_eq!(bytes_result.slide_count, 1);
    assert!(file_result.unresolved_elements.is_empty());
    assert!(bytes_result.unresolved_elements.is_empty());

    assert_eq!(info_from_file.slide_count, 1);
    assert_eq!(info_from_file.width_px, 960.0);
    assert_eq!(info_from_file.height_px, 720.0);
    assert_eq!(info_from_file.title.as_deref(), Some("API Deck"));
    assert_eq!(info_from_bytes.slide_count, info_from_file.slide_count);
    assert_eq!(info_from_bytes.width_px, info_from_file.width_px);
    assert_eq!(info_from_bytes.height_px, info_from_file.height_px);
    assert_eq!(info_from_bytes.title, info_from_file.title);
}

#[test]
fn should_include_slide_honors_hidden_indices_and_ranges() {
    let default_opts = ConversionOptions::default();
    assert!(default_opts.should_include_slide(1, false));
    assert!(!default_opts.should_include_slide(1, true));

    let indices_opts = ConversionOptions {
        include_hidden: true,
        slide_indices: Some(vec![1, 3]),
        ..Default::default()
    };
    assert!(indices_opts.should_include_slide(1, false));
    assert!(indices_opts.should_include_slide(3, true));
    assert!(!indices_opts.should_include_slide(2, false));

    let range_opts = ConversionOptions {
        slide_range: Some((2, 4)),
        ..Default::default()
    };
    assert!(!range_opts.should_include_slide(1, false));
    assert!(range_opts.should_include_slide(2, false));
    assert!(range_opts.should_include_slide(4, false));
    assert!(!range_opts.should_include_slide(5, false));
}
