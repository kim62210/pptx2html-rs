use std::path::Path;

use pyo3::prelude::*;

use pptx2html_core::ConversionOptions;

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

/// Convert PPTX to high-fidelity HTML
#[pymodule]
fn pptx2html(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_file, m)?)?;
    m.add_function(wrap_pyfunction!(convert_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(get_info, m)?)?;
    m.add_class::<PresentationInfo>()?;
    Ok(())
}
