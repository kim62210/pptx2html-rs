use thiserror::Error;

/// Convenience alias for `Result<T, PptxError>`.
pub type PptxResult<T> = Result<T, PptxError>;

/// All errors that can occur during PPTX parsing and rendering.
#[derive(Error, Debug)]
pub enum PptxError {
    /// File I/O failure (read/write).
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    /// The input is not a valid ZIP archive (or is corrupt).
    #[error("ZIP archive error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Well-formedness or namespace error while streaming XML.
    #[error("XML parsing error: {0}")]
    Xml(#[from] quick_xml::Error),

    /// A required OPC part is missing from the package.
    #[error("Required file missing: {0}")]
    MissingFile(String),

    /// The file cannot be processed (e.g. password-protected or non-PPTX).
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    /// Error during HTML/CSS generation.
    #[error("Rendering error: {0}")]
    Render(String),
}
