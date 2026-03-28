use wasm_bindgen::prelude::*;

/// Convert PPTX bytes to an HTML string
#[wasm_bindgen]
pub fn convert(data: &[u8]) -> Result<String, JsError> {
    pptx2html_core::convert_bytes(data).map_err(|e| JsError::new(&e.to_string()))
}

/// Convert PPTX bytes to HTML with slide filtering
#[wasm_bindgen]
pub fn convert_slides(data: &[u8], slides: &[usize]) -> Result<String, JsError> {
    let opts = pptx2html_core::ConversionOptions {
        slide_indices: Some(slides.to_vec()),
        ..Default::default()
    };
    pptx2html_core::convert_bytes_with_options(data, &opts)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Get the number of slides in a PPTX file
#[wasm_bindgen]
pub fn get_slide_count(data: &[u8]) -> Result<usize, JsError> {
    let info =
        pptx2html_core::get_info_from_bytes(data).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(info.slide_count)
}

/// Get presentation metadata as JSON string
#[wasm_bindgen]
pub fn get_info(data: &[u8]) -> Result<String, JsError> {
    let info =
        pptx2html_core::get_info_from_bytes(data).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(format!(
        r#"{{"slide_count":{},"width_px":{:.1},"height_px":{:.1},"title":{}}}"#,
        info.slide_count,
        info.width_px,
        info.height_px,
        match &info.title {
            Some(t) => format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\"")),
            None => "null".to_string(),
        }
    ))
}
