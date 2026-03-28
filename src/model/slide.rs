use super::geometry::{Position, Size};
use super::style::{Alignment, Border, Fill, FontStyle, TextStyle};

/// Slide
#[derive(Debug, Clone, Default)]
pub struct Slide {
    pub shapes: Vec<Shape>,
    pub background: Fill,
    pub hidden: bool,
}

/// Shape type
#[derive(Debug, Clone, Default)]
pub enum ShapeType {
    #[default]
    Rectangle,
    RoundedRectangle,
    Ellipse,
    Triangle,
    Arrow,
    Line,
    TextBox,
    Picture(PictureData),
    Table(TableData),
    Group(Vec<Shape>),
    Custom(String), // preset shape name
}

/// Shape
#[derive(Debug, Clone, Default)]
pub struct Shape {
    pub id: u32,
    pub name: String,
    pub shape_type: ShapeType,
    pub position: Position,
    pub size: Size,
    pub rotation: f64, // in degrees
    pub fill: Fill,
    pub border: Border,
    pub text_body: Option<TextBody>,
    pub hidden: bool,
}

/// Text body
#[derive(Debug, Clone, Default)]
pub struct TextBody {
    pub paragraphs: Vec<TextParagraph>,
    pub vertical_align: VerticalAlign,
    pub word_wrap: bool,
    pub auto_fit: AutoFit,
    pub margins: TextMargins,
}

/// Text paragraph
#[derive(Debug, Clone, Default)]
pub struct TextParagraph {
    pub runs: Vec<TextRun>,
    pub alignment: Alignment,
    pub line_spacing: Option<f64>,
    pub space_before: Option<f64>,
    pub space_after: Option<f64>,
    pub indent: Option<f64>,
    pub bullet: Option<Bullet>,
}

/// Text run (text segment with uniform style)
#[derive(Debug, Clone, Default)]
pub struct TextRun {
    pub text: String,
    pub style: TextStyle,
    pub font: FontStyle,
    pub hyperlink: Option<String>,
}

/// Picture data
#[derive(Debug, Clone, Default)]
pub struct PictureData {
    pub rel_id: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

/// Table data
#[derive(Debug, Clone, Default)]
pub struct TableData {
    pub rows: Vec<TableRow>,
    pub col_widths: Vec<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct TableRow {
    pub height: f64,
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, Default)]
pub struct TableCell {
    pub text_body: Option<TextBody>,
    pub fill: Fill,
    pub border_top: Border,
    pub border_bottom: Border,
    pub border_left: Border,
    pub border_right: Border,
    pub col_span: u32,
    pub row_span: u32,
}

/// Vertical alignment
#[derive(Debug, Clone, Default)]
pub enum VerticalAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

impl VerticalAlign {
    pub fn from_ooxml(val: &str) -> Self {
        match val {
            "ctr" => Self::Middle,
            "b" => Self::Bottom,
            _ => Self::Top,
        }
    }
}

/// Text auto-fit
#[derive(Debug, Clone, Default)]
pub enum AutoFit {
    #[default]
    None,
    Normal,
    Shrink,
}

/// Text internal margins
#[derive(Debug, Clone)]
pub struct TextMargins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Default for TextMargins {
    fn default() -> Self {
        Self {
            top: 3.6,  // OOXML default 45720 EMU ~ 3.6pt
            bottom: 3.6,
            left: 7.2,  // 91440 EMU ~ 7.2pt
            right: 7.2,
        }
    }
}

/// Bullet
#[derive(Debug, Clone)]
pub enum Bullet {
    Char(String),
    AutoNum(String),
    None,
}
