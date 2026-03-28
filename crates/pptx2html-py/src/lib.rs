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
