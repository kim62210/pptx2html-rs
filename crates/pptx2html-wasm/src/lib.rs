use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Backward-compatible API (v0.5)
// ---------------------------------------------------------------------------

/// Convert PPTX bytes to an HTML string.
#[wasm_bindgen]
pub fn convert(data: &[u8]) -> Result<String, JsError> {
    pptx2html_core::convert_bytes(data).map_err(|e| JsError::new(&e.to_string()))
}

/// Convert PPTX bytes to HTML with slide filtering (0-based indices).
#[wasm_bindgen]
pub fn convert_slides(data: &[u8], slides: &[usize]) -> Result<String, JsError> {
    let opts = pptx2html_core::ConversionOptions {
        slide_indices: Some(slides.to_vec()),
        ..Default::default()
    };
    pptx2html_core::convert_bytes_with_options(data, &opts)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Get the number of slides in a PPTX file.
#[wasm_bindgen]
pub fn get_slide_count(data: &[u8]) -> Result<usize, JsError> {
    let info =
        pptx2html_core::get_info_from_bytes(data).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(info.slide_count)
}

/// Get presentation metadata as JSON string.
///
/// Kept for backward compatibility. Prefer `get_presentation_info()` for typed access.
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

// ---------------------------------------------------------------------------
// Enhanced API (v0.6) — typed return values, full ConversionOptions support
// ---------------------------------------------------------------------------

/// Presentation metadata returned as a structured JS object.
#[wasm_bindgen]
pub struct PresentationInfo {
    slide_count: usize,
    width_px: f64,
    height_px: f64,
    title: Option<String>,
}

#[wasm_bindgen]
impl PresentationInfo {
    #[wasm_bindgen(getter, js_name = "slideCount")]
    pub fn slide_count(&self) -> usize {
        self.slide_count
    }

    #[wasm_bindgen(getter, js_name = "widthPx")]
    pub fn width_px(&self) -> f64 {
        self.width_px
    }

    #[wasm_bindgen(getter, js_name = "heightPx")]
    pub fn height_px(&self) -> f64 {
        self.height_px
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }
}

/// Get presentation metadata as a typed object.
///
/// ```js
/// const info = get_presentation_info(data);
/// console.log(info.slideCount, info.widthPx, info.heightPx, info.title);
/// ```
#[wasm_bindgen]
pub fn get_presentation_info(data: &[u8]) -> Result<PresentationInfo, JsError> {
    let info =
        pptx2html_core::get_info_from_bytes(data).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(PresentationInfo {
        slide_count: info.slide_count,
        width_px: info.width_px,
        height_px: info.height_px,
        title: info.title,
    })
}

/// Conversion result with HTML and metadata about unresolved elements.
#[wasm_bindgen]
pub struct ConversionResult {
    html: String,
    unresolved_json: String,
    slide_count: usize,
}

#[wasm_bindgen]
impl ConversionResult {
    /// The generated HTML string.
    #[wasm_bindgen(getter)]
    pub fn html(&self) -> String {
        self.html.clone()
    }

    /// JSON array of unresolved elements (SmartArt, OLE, Math).
    ///
    /// Each element: `{ slideIndex, elementType, placeholderId, rawXml?, dataModel? }`
    #[wasm_bindgen(getter, js_name = "unresolvedElements")]
    pub fn unresolved_elements(&self) -> String {
        self.unresolved_json.clone()
    }

    /// Number of slides processed.
    #[wasm_bindgen(getter, js_name = "slideCount")]
    pub fn slide_count(&self) -> usize {
        self.slide_count
    }
}

/// Convert PPTX with explicit options.
///
/// ```js
/// const html = convert_with_options(data, {
///   embedImages: false,
///   includeHidden: true,
///   slideIndices: [1, 3, 5],
/// });
/// ```
///
/// `slide_indices` uses 1-based indexing. Pass an empty array to include all slides.
#[wasm_bindgen]
pub fn convert_with_options(
    data: &[u8],
    embed_images: bool,
    include_hidden: bool,
    slide_indices: &[usize],
) -> Result<String, JsError> {
    let opts = pptx2html_core::ConversionOptions {
        embed_images,
        include_hidden,
        slide_indices: if slide_indices.is_empty() {
            None
        } else {
            Some(slide_indices.to_vec())
        },
        ..Default::default()
    };
    pptx2html_core::convert_bytes_with_options(data, &opts)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Convert PPTX and return both HTML and metadata about unresolved elements.
///
/// ```js
/// const result = convert_with_metadata(data);
/// document.body.innerHTML = result.html;
/// const unresolved = JSON.parse(result.unresolvedElements);
/// ```
#[wasm_bindgen]
pub fn convert_with_metadata(data: &[u8]) -> Result<ConversionResult, JsError> {
    let result = pptx2html_core::convert_bytes_with_metadata(data)
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(ConversionResult {
        html: result.html,
        unresolved_json: serialize_unresolved(&result.unresolved_elements),
        slide_count: result.slide_count,
    })
}

/// Convert PPTX with options and return both HTML and metadata.
#[wasm_bindgen]
pub fn convert_with_options_metadata(
    data: &[u8],
    embed_images: bool,
    include_hidden: bool,
    slide_indices: &[usize],
) -> Result<ConversionResult, JsError> {
    let opts = pptx2html_core::ConversionOptions {
        embed_images,
        include_hidden,
        slide_indices: if slide_indices.is_empty() {
            None
        } else {
            Some(slide_indices.to_vec())
        },
        ..Default::default()
    };
    let result = pptx2html_core::convert_bytes_with_options_metadata(data, &opts)
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(ConversionResult {
        html: result.html,
        unresolved_json: serialize_unresolved(&result.unresolved_elements),
        slide_count: result.slide_count,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn serialize_unresolved(elements: &[pptx2html_core::model::UnresolvedElement]) -> String {
    let mut json = String::from("[");
    for (i, elem) in elements.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        let element_type = match elem.element_type {
            pptx2html_core::model::UnresolvedType::SmartArt => "SmartArt",
            pptx2html_core::model::UnresolvedType::OleObject => "OleObject",
            pptx2html_core::model::UnresolvedType::MathEquation => "MathEquation",
            pptx2html_core::model::UnresolvedType::CustomGeometry => "CustomGeometry",
        };
        let raw_xml = match &elem.raw_xml {
            Some(xml) => format!(
                "\"{}\"",
                xml.replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
            ),
            None => "null".to_string(),
        };
        let data_model = match &elem.data_model {
            Some(dm) => format!(
                "\"{}\"",
                dm.replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
            ),
            None => "null".to_string(),
        };
        json.push_str(&format!(
            r#"{{"slideIndex":{},"elementType":"{}","placeholderId":"{}","rawXml":{},"dataModel":{}}}"#,
            elem.slide_index,
            element_type,
            elem.placeholder_id.replace('\\', "\\\\").replace('"', "\\\""),
            raw_xml,
            data_model,
        ));
    }
    json.push(']');
    json
}
