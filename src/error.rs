use thiserror::Error;

pub type PptxResult<T> = Result<T, PptxError>;

#[derive(Error, Debug)]
pub enum PptxError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP archive error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML parsing error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("Required file missing: {0}")]
    MissingFile(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Rendering error: {0}")]
    Render(String),
}
