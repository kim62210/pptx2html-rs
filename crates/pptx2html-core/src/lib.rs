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

/// Conversion options for controlling output behavior
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    /// Embed images as base64 data URIs (default: true).
    /// When false, outputs `<img src="images/{name}">` references.
    pub embed_images: bool,
    /// Include hidden slides in output (default: false)
    pub include_hidden: bool,
    /// Render only slides in this 1-based inclusive range (e.g. (2, 5) = slides 2..=5)
    pub slide_range: Option<(usize, usize)>,
    /// Render only slides at these 1-based indices (e.g. vec![1, 3, 5])
    pub slide_indices: Option<Vec<usize>>,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            embed_images: true,
            include_hidden: false,
            slide_range: None,
            slide_indices: None,
        }
    }
}

impl ConversionOptions {
    /// Check whether a slide at the given 1-based index should be included
    pub fn should_include_slide(&self, one_based_index: usize, hidden: bool) -> bool {
        if hidden && !self.include_hidden {
            return false;
        }
        if let Some(ref indices) = self.slide_indices {
            return indices.contains(&one_based_index);
        }
        if let Some((start, end)) = self.slide_range {
            return one_based_index >= start && one_based_index <= end;
        }
        true
    }
}

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

/// Convert a PPTX file to HTML with options
pub fn convert_file_with_options(path: &Path, opts: &ConversionOptions) -> PptxResult<String> {
    let presentation = PptxParser::parse_file(path)?;
    let html = HtmlRenderer::render_with_options(&presentation, opts)?;
    Ok(html)
}

/// Convert PPTX byte data to HTML with options
pub fn convert_bytes_with_options(data: &[u8], opts: &ConversionOptions) -> PptxResult<String> {
    let presentation = PptxParser::parse_bytes(data)?;
    let html = HtmlRenderer::render_with_options(&presentation, opts)?;
    Ok(html)
}

/// Presentation metadata
#[derive(Debug, Clone)]
pub struct PresentationInfo {
    pub slide_count: usize,
    pub width_px: f64,
    pub height_px: f64,
    pub title: Option<String>,
}

/// Get presentation metadata from a file
pub fn get_info(path: &Path) -> PptxResult<PresentationInfo> {
    let presentation = PptxParser::parse_file(path)?;
    Ok(PresentationInfo {
        slide_count: presentation.slides.len(),
        width_px: presentation.slide_size.width.to_px(),
        height_px: presentation.slide_size.height.to_px(),
        title: presentation.title.clone(),
    })
}

/// Get presentation metadata from bytes
pub fn get_info_from_bytes(data: &[u8]) -> PptxResult<PresentationInfo> {
    let presentation = PptxParser::parse_bytes(data)?;
    Ok(PresentationInfo {
        slide_count: presentation.slides.len(),
        width_px: presentation.slide_size.width.to_px(),
        height_px: presentation.slide_size.height.to_px(),
        title: presentation.title.clone(),
    })
}
