use pyo3::prelude::*;

#[pymodule]
fn pptx2html(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
