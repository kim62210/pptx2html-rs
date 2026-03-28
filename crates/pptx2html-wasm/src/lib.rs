use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(_data: &[u8]) -> Result<String, JsError> {
    Err(JsError::new("not yet implemented"))
}
