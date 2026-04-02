use std::path::Path;

use pyo3::prelude::*;

use pptx2html_core::ConversionOptions;
use pptx2html_core::model::slide::UnresolvedType;

/// Convert a PPTX file to HTML string
#[pyfunction]
fn convert_file(path: &str) -> PyResult<String> {
    pptx2html_core::convert_file(Path::new(path))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// Convert PPTX bytes to HTML string
#[pyfunction]
fn convert_bytes(data: &[u8]) -> PyResult<String> {
    pptx2html_core::convert_bytes(data)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// Convert a PPTX file to HTML with options
///
/// Args:
///     path: Path to the PPTX file
///     embed_images: Embed images as base64 data URIs (default: True)
///     include_hidden: Include hidden slides (default: False)
///     slides: List of 1-based slide indices to include (default: all)
#[pyfunction]
#[pyo3(signature = (path, *, embed_images=true, include_hidden=false, slides=None))]
fn convert(
    path: &str,
    embed_images: bool,
    include_hidden: bool,
    slides: Option<Vec<usize>>,
) -> PyResult<String> {
    let opts = ConversionOptions {
        embed_images,
        include_hidden,
        slide_range: None,
        slide_indices: slides,
    };
    pptx2html_core::convert_file_with_options(Path::new(path), &opts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// Convert a PPTX file to HTML with metadata about unresolved elements
///
/// Args:
///     path: Path to the PPTX file
///     embed_images: Embed images as base64 data URIs (default: True)
///     include_hidden: Include hidden slides (default: False)
///     slides: List of 1-based slide indices to include (default: all)
///
/// Returns:
///     ConversionResult with html, unresolved_elements, and slide_count
#[pyfunction]
#[pyo3(signature = (path, *, embed_images=true, include_hidden=false, slides=None))]
fn convert_with_metadata(
    path: &str,
    embed_images: bool,
    include_hidden: bool,
    slides: Option<Vec<usize>>,
) -> PyResult<PyConversionResult> {
    let opts = ConversionOptions {
        embed_images,
        include_hidden,
        slide_range: None,
        slide_indices: slides,
    };
    let result = pptx2html_core::convert_file_with_options_metadata(Path::new(path), &opts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let elements: Vec<PyUnresolvedElement> = result
        .unresolved_elements
        .into_iter()
        .map(|e| PyUnresolvedElement {
            slide_index: e.slide_index,
            element_type: match e.element_type {
                UnresolvedType::SmartArt => "smartart".to_string(),
                UnresolvedType::OleObject => "ole".to_string(),
                UnresolvedType::MathEquation => "math".to_string(),
                UnresolvedType::CustomGeometry => "custom-geometry".to_string(),
            },
            placeholder_id: e.placeholder_id,
            raw_xml: e.raw_xml,
            data_model: e.data_model,
        })
        .collect();

    Ok(PyConversionResult {
        html: result.html,
        unresolved_elements: elements,
        slide_count: result.slide_count,
    })
}

/// Convert PPTX bytes to HTML with metadata about unresolved elements
///
/// Args:
///     data: PPTX file bytes
///     embed_images: Embed images as base64 data URIs (default: True)
///     include_hidden: Include hidden slides (default: False)
///     slides: List of 1-based slide indices to include (default: all)
///
/// Returns:
///     ConversionResult with html, unresolved_elements, and slide_count
#[pyfunction]
#[pyo3(signature = (data, *, embed_images=true, include_hidden=false, slides=None))]
fn convert_bytes_with_metadata(
    data: &[u8],
    embed_images: bool,
    include_hidden: bool,
    slides: Option<Vec<usize>>,
) -> PyResult<PyConversionResult> {
    let opts = ConversionOptions {
        embed_images,
        include_hidden,
        slide_range: None,
        slide_indices: slides,
    };
    let result = pptx2html_core::convert_bytes_with_options_metadata(data, &opts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let elements: Vec<PyUnresolvedElement> = result
        .unresolved_elements
        .into_iter()
        .map(|e| PyUnresolvedElement {
            slide_index: e.slide_index,
            element_type: match e.element_type {
                UnresolvedType::SmartArt => "smartart".to_string(),
                UnresolvedType::OleObject => "ole".to_string(),
                UnresolvedType::MathEquation => "math".to_string(),
                UnresolvedType::CustomGeometry => "custom-geometry".to_string(),
            },
            placeholder_id: e.placeholder_id,
            raw_xml: e.raw_xml,
            data_model: e.data_model,
        })
        .collect();

    Ok(PyConversionResult {
        html: result.html,
        unresolved_elements: elements,
        slide_count: result.slide_count,
    })
}

/// Get presentation metadata (slide count, size, title)
#[pyfunction]
fn get_info(path: &str) -> PyResult<PresentationInfo> {
    let info = pptx2html_core::get_info(Path::new(path))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PresentationInfo {
        slide_count: info.slide_count,
        width_px: info.width_px,
        height_px: info.height_px,
        title: info.title,
    })
}

/// Presentation metadata
#[pyclass]
#[derive(Debug, Clone)]
struct PresentationInfo {
    #[pyo3(get)]
    slide_count: usize,
    #[pyo3(get)]
    width_px: f64,
    #[pyo3(get)]
    height_px: f64,
    #[pyo3(get)]
    title: Option<String>,
}

#[pymethods]
impl PresentationInfo {
    fn __repr__(&self) -> String {
        format!(
            "PresentationInfo(slide_count={}, width_px={:.1}, height_px={:.1}, title={:?})",
            self.slide_count, self.width_px, self.height_px, self.title
        )
    }
}

/// Result of PPTX conversion with metadata
#[pyclass]
#[derive(Debug, Clone)]
struct PyConversionResult {
    #[pyo3(get)]
    html: String,
    #[pyo3(get)]
    unresolved_elements: Vec<PyUnresolvedElement>,
    #[pyo3(get)]
    slide_count: usize,
}

#[pymethods]
impl PyConversionResult {
    fn __repr__(&self) -> String {
        format!(
            "ConversionResult(slide_count={}, unresolved_elements={})",
            self.slide_count,
            self.unresolved_elements.len()
        )
    }
}

/// Metadata about an unresolved element
#[pyclass]
#[derive(Debug, Clone)]
struct PyUnresolvedElement {
    #[pyo3(get)]
    slide_index: usize,
    #[pyo3(get)]
    element_type: String,
    #[pyo3(get)]
    placeholder_id: String,
    #[pyo3(get)]
    raw_xml: Option<String>,
    #[pyo3(get)]
    data_model: Option<String>,
}

#[pymethods]
impl PyUnresolvedElement {
    fn __repr__(&self) -> String {
        format!(
            "UnresolvedElement(slide={}, type='{}', id='{}')",
            self.slide_index, self.element_type, self.placeholder_id
        )
    }
}

/// Convert PPTX to high-fidelity HTML
#[pymodule]
fn pptx2html(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_file, m)?)?;
    m.add_function(wrap_pyfunction!(convert_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(convert_with_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(convert_bytes_with_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(get_info, m)?)?;
    m.add_class::<PresentationInfo>()?;
    m.add_class::<PyConversionResult>()?;
    m.add_class::<PyUnresolvedElement>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{Cursor, Write};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    use super::{convert, convert_bytes_with_metadata, convert_with_metadata, get_info};

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn convert_uses_one_based_slide_indices() {
        let path = write_temp_pptx(build_two_slide_pptx());

        let html = convert(path.to_str().unwrap(), true, false, Some(vec![1]))
            .expect("convert should succeed");

        assert!(html.contains("Slide One"), "expected first slide text in HTML");
        assert!(
            !html.contains("Slide Two"),
            "expected second slide to be filtered out"
        );

        fs::remove_file(path).expect("remove temp pptx");
    }

    #[test]
    fn get_info_reports_slide_count_for_generated_fixture() {
        let path = write_temp_pptx(build_two_slide_pptx());

        let info = get_info(path.to_str().unwrap()).expect("get_info should succeed");

        assert_eq!(info.slide_count, 2);
        assert_eq!(info.title, None);

        fs::remove_file(path).expect("remove temp pptx");
    }

    #[test]
    fn convert_with_metadata_reports_no_unresolved_elements_for_basic_fixture() {
        let path = write_temp_pptx(build_two_slide_pptx());

        let result = convert_with_metadata(path.to_str().unwrap(), true, false, Some(vec![1]))
            .expect("convert_with_metadata should succeed");

        assert_eq!(result.slide_count, 1);
        assert!(result.html.contains("Slide One"));
        assert_eq!(result.unresolved_elements.len(), 0);

        fs::remove_file(path).expect("remove temp pptx");
    }

    #[test]
    fn convert_bytes_with_metadata_respects_one_based_slide_filtering() {
        let bytes = build_two_slide_pptx();

        let result = convert_bytes_with_metadata(&bytes, true, false, Some(vec![2]))
            .expect("convert_bytes_with_metadata should succeed");

        assert_eq!(result.slide_count, 1);
        assert!(result.html.contains("Slide Two"));
        assert!(!result.html.contains("Slide One"));
        assert_eq!(result.unresolved_elements.len(), 0);
    }

    fn write_temp_pptx(bytes: Vec<u8>) -> PathBuf {
        let unique = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("pptx2html-py-{unique}.pptx"));
        fs::write(&path, bytes).expect("write temp pptx");
        path
    }

    fn build_two_slide_pptx() -> Vec<u8> {
        let cursor = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(cursor);
        let options = SimpleFileOptions::default();

        zip.start_file("[Content_Types].xml", options).unwrap();
        zip.write_all(content_types().as_bytes()).unwrap();

        zip.start_file("_rels/.rels", options).unwrap();
        zip.write_all(root_rels().as_bytes()).unwrap();

        zip.start_file("ppt/presentation.xml", options).unwrap();
        zip.write_all(presentation_xml().as_bytes()).unwrap();

        zip.start_file("ppt/_rels/presentation.xml.rels", options)
            .unwrap();
        zip.write_all(presentation_rels().as_bytes()).unwrap();

        zip.start_file("ppt/slides/slide1.xml", options).unwrap();
        zip.write_all(slide_xml("Slide One").as_bytes()).unwrap();

        zip.start_file("ppt/slides/slide2.xml", options).unwrap();
        zip.write_all(slide_xml("Slide Two").as_bytes()).unwrap();

        zip.start_file("ppt/slides/_rels/slide1.xml.rels", options)
            .unwrap();
        zip.write_all(empty_relationships().as_bytes()).unwrap();

        zip.start_file("ppt/slides/_rels/slide2.xml.rels", options)
            .unwrap();
        zip.write_all(empty_relationships().as_bytes()).unwrap();

        zip.start_file("ppt/theme/theme1.xml", options).unwrap();
        zip.write_all(theme_xml().as_bytes()).unwrap();

        zip.finish().unwrap().into_inner()
    }

    fn content_types() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/slides/slide2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
</Types>"#
    }

    fn root_rels() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#
    }

    fn presentation_xml() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst/>
  <p:sldIdLst>
    <p:sldId id="256" r:id="rId1"/>
    <p:sldId id="257" r:id="rId2"/>
  </p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#
    }

    fn presentation_rels() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide2.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
</Relationships>"#
    }

    fn slide_xml(text: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="TextBox 1"/>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="914400" cy="457200"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" sz="1800"/>
              <a:t>{text}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"#
        )
    }

    fn empty_relationships() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#
    }

    fn theme_xml() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="TestTheme">
  <a:themeElements>
    <a:clrScheme name="TestColors">
      <a:dk1><a:srgbClr val="000000"/></a:dk1>
      <a:lt1><a:srgbClr val="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="1F1F1F"/></a:dk2>
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
    <a:fontScheme name="TestFonts">
      <a:majorFont><a:latin typeface="Calibri"/></a:majorFont>
      <a:minorFont><a:latin typeface="Calibri"/></a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="TestFmt"/>
  </a:themeElements>
</a:theme>"#
    }
}
