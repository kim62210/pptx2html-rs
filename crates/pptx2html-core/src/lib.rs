//! # pptx2html-rs
//!
//! Library for converting PPTX files to HTML while preserving original layout/styles.
//! Pure Rust implementation based on the ECMA-376 (OOXML) open standard.

pub mod error;
pub mod model;
pub mod parser;
pub mod renderer;
pub mod resolver;

use std::path::Path;

use error::PptxResult;
use parser::PptxParser;
use renderer::HtmlRenderer;

/// Convert a PPTX file to an HTML string
pub fn convert_file(path: &Path) -> PptxResult<String> {
    let presentation = PptxParser::parse_file(path)?;
    let html = HtmlRenderer::render(&presentation)?;
    Ok(html)
}

/// Convert PPTX byte data to an HTML string
pub fn convert_bytes(data: &[u8]) -> PptxResult<String> {
    let presentation = PptxParser::parse_bytes(data)?;
    let html = HtmlRenderer::render(&presentation)?;
    Ok(html)
}
