//! # pptx2html-core
//!
//! Pure Rust library for converting PPTX presentations to self-contained HTML.
//! Built on the ECMA-376 (OOXML) open standard — no Microsoft dependencies.
//!
//! ## Quick Start
//!
//! ```no_run
//! use std::path::Path;
//!
//! // Convert a file
//! let html = pptx2html_core::convert_file(Path::new("input.pptx")).unwrap();
//! std::fs::write("output.html", &html).unwrap();
//!
//! // Convert from bytes
//! let pptx_data = std::fs::read("input.pptx").unwrap();
//! let html = pptx2html_core::convert_bytes(&pptx_data).unwrap();
//! ```
//!
//! ## Features
//!
//! - High-fidelity layout preservation (absolute positioning in EMU coordinates)
//! - Theme color resolution with 12 color modifiers (tint, shade, lumMod, etc.)
//! - Slide master / layout inheritance chain
//! - 30 preset shape SVG rendering
//! - Table, group shape, and connector support
//! - Image embedding (base64) or external references
//! - Text styling: bold, italic, underline, bullets, vertical text, shadows
//! - Graceful fallback for unsupported content (SmartArt, OLE, Math)

pub mod error;
pub mod model;
pub mod parser;
pub mod renderer;
pub mod resolver;

use std::path::Path;

use error::PptxResult;
use model::UnresolvedElement;
use parser::PptxParser;
use renderer::HtmlRenderer;

/// Options controlling how PPTX content is converted to HTML.
///
/// Use [`Default::default()`] for sensible defaults (embed images, exclude hidden slides).
///
/// ```
/// use pptx2html_core::ConversionOptions;
///
/// let opts = ConversionOptions {
///     embed_images: false,                 // external image refs
///     slide_indices: Some(vec![1, 3, 5]),  // specific slides only
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    /// Embed images as base64 data URIs (default: true).
    /// When false, outputs `<img src="images/{name}">` references.
    pub embed_images: bool,
    /// Include hidden slides in output (default: false).
    pub include_hidden: bool,
    /// Render only slides in this 1-based inclusive range (e.g. `(2, 5)` = slides 2..=5).
    pub slide_range: Option<(usize, usize)>,
    /// Render only slides at these 1-based indices (e.g. `vec![1, 3, 5]`).
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
    /// Check whether a slide at the given 1-based index should be included.
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

/// Result of PPTX conversion with metadata about unresolved elements.
///
/// Use this with `convert_*_with_metadata()` functions to get both the HTML
/// and structured information about elements that could not be fully rendered.
#[derive(Debug, Clone)]
pub struct ConversionResult {
    /// Generated HTML string.
    pub html: String,
    /// Metadata about elements that were rendered as placeholders.
    pub unresolved_elements: Vec<UnresolvedElement>,
    /// Number of slides processed.
    pub slide_count: usize,
}

/// Convert a PPTX file at `path` to a self-contained HTML string.
///
/// Images are embedded as base64 data URIs by default.
/// Returns [`PptxError`](error::PptxError) on I/O, ZIP, or XML errors.
pub fn convert_file(path: &Path) -> PptxResult<String> {
    Ok(convert_file_with_metadata(path)?.html)
}

/// Convert PPTX byte data to a self-contained HTML string.
///
/// Useful when the PPTX is already loaded in memory (e.g. from a network request).
pub fn convert_bytes(data: &[u8]) -> PptxResult<String> {
    Ok(convert_bytes_with_metadata(data)?.html)
}

/// Convert a PPTX file to HTML with custom [`ConversionOptions`].
pub fn convert_file_with_options(path: &Path, opts: &ConversionOptions) -> PptxResult<String> {
    Ok(convert_file_with_options_metadata(path, opts)?.html)
}

/// Convert PPTX byte data to HTML with custom [`ConversionOptions`].
pub fn convert_bytes_with_options(data: &[u8], opts: &ConversionOptions) -> PptxResult<String> {
    Ok(convert_bytes_with_options_metadata(data, opts)?.html)
}

/// Convert a PPTX file to HTML with metadata about unresolved elements.
pub fn convert_file_with_metadata(path: &Path) -> PptxResult<ConversionResult> {
    convert_file_with_options_metadata(path, &ConversionOptions::default())
}

/// Convert PPTX byte data to HTML with metadata about unresolved elements.
pub fn convert_bytes_with_metadata(data: &[u8]) -> PptxResult<ConversionResult> {
    convert_bytes_with_options_metadata(data, &ConversionOptions::default())
}

/// Convert a PPTX file to HTML with options and metadata about unresolved elements.
pub fn convert_file_with_options_metadata(
    path: &Path,
    opts: &ConversionOptions,
) -> PptxResult<ConversionResult> {
    let presentation = PptxParser::parse_file(path)?;
    HtmlRenderer::render_with_options_metadata(&presentation, opts)
}

/// Convert PPTX byte data to HTML with options and metadata about unresolved elements.
pub fn convert_bytes_with_options_metadata(
    data: &[u8],
    opts: &ConversionOptions,
) -> PptxResult<ConversionResult> {
    let presentation = PptxParser::parse_bytes(data)?;
    HtmlRenderer::render_with_options_metadata(&presentation, opts)
}

/// Lightweight presentation metadata (no rendering performed).
#[derive(Debug, Clone)]
pub struct PresentationInfo {
    /// Number of slides in the presentation.
    pub slide_count: usize,
    /// Slide width in CSS pixels (96 DPI).
    pub width_px: f64,
    /// Slide height in CSS pixels (96 DPI).
    pub height_px: f64,
    /// Presentation title from core properties, if present.
    pub title: Option<String>,
}

/// Extract metadata from a PPTX file without rendering.
pub fn get_info(path: &Path) -> PptxResult<PresentationInfo> {
    let presentation = PptxParser::parse_file(path)?;
    Ok(PresentationInfo {
        slide_count: presentation.slides.len(),
        width_px: presentation.slide_size.width.to_px(),
        height_px: presentation.slide_size.height.to_px(),
        title: presentation.title.clone(),
    })
}

/// Extract metadata from in-memory PPTX byte data without rendering.
pub fn get_info_from_bytes(data: &[u8]) -> PptxResult<PresentationInfo> {
    let presentation = PptxParser::parse_bytes(data)?;
    Ok(PresentationInfo {
        slide_count: presentation.slides.len(),
        width_px: presentation.slide_size.width.to_px(),
        height_px: presentation.slide_size.height.to_px(),
        title: presentation.title.clone(),
    })
}
