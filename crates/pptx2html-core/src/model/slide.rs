use std::collections::HashMap;

use super::color::Color;
use super::geometry::{CustomGeometry, Position, Size};
use super::hierarchy::{ClrMapOverride, ListStyle, PlaceholderInfo, ShapeStyleRef, SpacingValue};
use super::style::{
    Alignment, Border, Fill, FontStyle, ShapeEffects, StrikethroughType, TextStyle,
    UnderlineType,
};

/// Slide
#[derive(Debug, Clone)]
pub struct Slide {
    pub layout_idx: Option<usize>,
    pub shapes: Vec<Shape>,
    pub background: Option<Fill>,
    pub clr_map_ovr: Option<ClrMapOverride>,
    pub show_master_sp: bool,
    pub hidden: bool,
}

impl Default for Slide {
    fn default() -> Self {
        Self {
            layout_idx: None,
            shapes: Vec::new(),
            background: None,
            clr_map_ovr: None,
            show_master_sp: true,
            hidden: false,
        }
    }
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
    Group(Vec<Shape>, GroupData),
    Chart(ChartData),
    Custom(String), // preset shape name
    CustomGeom(CustomGeometry),
    /// Unsupported content placeholder (SmartArt, OLE, Math, etc.)
    Unsupported(UnsupportedData),
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
    pub flip_h: bool,
    pub flip_v: bool,
    pub fill: Fill,
    pub border: Border,
    pub text_body: Option<TextBody>,
    pub hidden: bool,
    pub placeholder: Option<PlaceholderInfo>,
    pub style_ref: Option<ShapeStyleRef>,
    pub adjust_values: Option<HashMap<String, f64>>,
    pub start_connection: Option<ConnectionRef>,
    pub end_connection: Option<ConnectionRef>,
    pub vertical_text: Option<String>, // "vert", "vert270", "wordArtVert", etc.
    pub vertical_text_explicit: bool,
    pub effects: ShapeEffects,
}

#[derive(Debug, Clone)]
pub struct ConnectionRef {
    pub shape_id: u32,
    pub site_idx: usize,
}

/// Text body
#[derive(Debug, Clone)]
pub struct TextBody {
    pub paragraphs: Vec<TextParagraph>,
    pub list_style: Option<ListStyle>,
    pub vertical_align: VerticalAlign,
    pub vertical_align_explicit: bool,
    pub margin_top_explicit: bool,
    pub margin_bottom_explicit: bool,
    pub margin_left_explicit: bool,
    pub margin_right_explicit: bool,
    pub word_wrap: bool,
    pub word_wrap_explicit: bool,
    pub auto_fit: AutoFit,
    pub margins: TextMargins,
}

impl Default for TextBody {
    fn default() -> Self {
        Self {
            paragraphs: Vec::new(),
            list_style: None,
            vertical_align: VerticalAlign::Top,
            vertical_align_explicit: false,
            margin_top_explicit: false,
            margin_bottom_explicit: false,
            margin_left_explicit: false,
            margin_right_explicit: false,
            word_wrap: true,
            word_wrap_explicit: false,
            auto_fit: AutoFit::None,
            margins: TextMargins::default(),
        }
    }
}

/// Text paragraph
#[derive(Debug, Clone, Default)]
pub struct TextParagraph {
    pub runs: Vec<TextRun>,
    pub alignment: Alignment,
    pub rtl: bool,
    pub line_spacing: Option<SpacingValue>,
    pub space_before: Option<SpacingValue>,
    pub space_after: Option<SpacingValue>,
    pub indent: Option<f64>,
    pub margin_left: Option<f64>,
    pub bullet: Option<Bullet>,
    pub level: u32,
    /// Paragraph-level default run properties (from <a:defRPr> inside <a:pPr>)
    pub def_rpr: Option<ParagraphDefRPr>,
}

/// Paragraph-level default run properties parsed from <a:defRPr> inside <a:pPr>
#[derive(Debug, Clone, Default)]
pub struct ParagraphDefRPr {
    pub font_size: Option<f64>,
    pub letter_spacing: Option<f64>,
    pub baseline: Option<i32>,
    pub underline: Option<UnderlineType>,
    pub strikethrough: Option<StrikethroughType>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub color: Option<Color>,
    pub font_latin: Option<String>,
    pub font_ea: Option<String>,
}

/// Text run (text segment with uniform style)
#[derive(Debug, Clone, Default)]
pub struct TextRun {
    pub text: String,
    pub style: TextStyle,
    pub font: FontStyle,
    pub hyperlink: Option<String>,
    pub is_break: bool, // <a:br> line break
}

/// Picture data
#[derive(Debug, Clone, Default)]
pub struct PictureData {
    pub rel_id: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub crop: Option<CropRect>,
}

/// Image crop rectangle (values 0.0-1.0, representing percentage from each edge)
#[derive(Debug, Clone, Default)]
pub struct CropRect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

/// Chart data (fallback rendering for embedded charts)
#[derive(Debug, Clone, Default)]
pub struct ChartData {
    pub rel_id: String,
    pub preview_image: Option<Vec<u8>>,
    pub preview_mime: Option<String>,
}

/// Table data
#[derive(Debug, Clone, Default)]
pub struct TableData {
    pub rows: Vec<TableRow>,
    pub col_widths: Vec<f64>,
    pub band_row: bool,
    pub band_col: bool,
    pub first_row: bool,
    pub last_row: bool,
    pub first_col: bool,
    pub last_col: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TableRow {
    pub height: f64,
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone)]
pub struct TableCell {
    pub text_body: Option<TextBody>,
    pub fill: Fill,
    pub border_top: Border,
    pub border_bottom: Border,
    pub border_left: Border,
    pub border_right: Border,
    pub col_span: u32,
    pub row_span: u32,
    pub v_merge: bool,
    pub margin_left: f64,   // in pt
    pub margin_right: f64,  // in pt
    pub margin_top: f64,    // in pt
    pub margin_bottom: f64, // in pt
    pub vertical_align: VerticalAlign,
}

impl Default for TableCell {
    fn default() -> Self {
        Self {
            text_body: None,
            fill: Fill::None,
            border_top: Border::default(),
            border_bottom: Border::default(),
            border_left: Border::default(),
            border_right: Border::default(),
            col_span: 0,
            row_span: 0,
            v_merge: false,
            margin_left: 7.2,   // OOXML default 91440 EMU
            margin_right: 7.2,  // OOXML default 91440 EMU
            margin_top: 3.6,    // OOXML default 45720 EMU
            margin_bottom: 3.6, // OOXML default 45720 EMU
            vertical_align: VerticalAlign::Top,
        }
    }
}

/// Group shape data (child offset/extent for coordinate remapping)
#[derive(Debug, Clone, Default)]
pub struct GroupData {
    pub child_offset: Position,
    pub child_extent: Size,
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
    NoAutoFit,
    Normal {
        font_scale: Option<f64>,             // 0.0-1.0 (e.g., 0.625 for 62.5%)
        line_spacing_reduction: Option<f64>, // 0.0-1.0 (e.g., 0.2 for 20%)
    },
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
            top: 3.6, // OOXML default 45720 EMU ~ 3.6pt
            bottom: 3.6,
            left: 7.2, // 91440 EMU ~ 7.2pt
            right: 7.2,
        }
    }
}

/// Type of content that could not be fully rendered
#[derive(Debug, Clone, PartialEq)]
pub enum UnresolvedType {
    SmartArt,
    OleObject,
    MathEquation,
    CustomGeometry,
}

/// Data carried by an Unsupported shape variant
#[derive(Debug, Clone)]
pub struct UnsupportedData {
    /// Human-readable label (e.g. "SmartArt", "OLE Object")
    pub label: String,
    /// Typed classification for programmatic use
    pub element_type: UnresolvedType,
    /// Raw XML snippet captured from the original PPTX
    pub raw_xml: Option<String>,
}

/// Metadata about an element that was rendered as a placeholder
#[derive(Debug, Clone)]
pub struct UnresolvedElement {
    /// 0-based slide index
    pub slide_index: usize,
    /// Type of unresolved content
    pub element_type: UnresolvedType,
    /// Unique ID matching the HTML placeholder element
    pub placeholder_id: String,
    /// Bounding box position in EMU
    pub position: Option<Position>,
    /// Bounding box size in EMU
    pub size: Option<Size>,
    /// Raw XML snippet from the original PPTX
    pub raw_xml: Option<String>,
    /// Structured data model as JSON string (reserved for LLM post-processing)
    pub data_model: Option<String>,
}

/// Bullet
#[derive(Debug, Clone)]
pub enum Bullet {
    Char(BulletChar),
    AutoNum(BulletAutoNum),
    None,
}

/// Character bullet with optional font/size/color
#[derive(Debug, Clone)]
pub struct BulletChar {
    pub char: String,
    pub font: Option<String>,
    pub size_pct: Option<f64>, // percentage of text size, e.g. 1.0 = 100%
    pub color: Option<super::color::Color>,
}

/// Auto-numbered bullet with optional font/size/color
#[derive(Debug, Clone)]
pub struct BulletAutoNum {
    pub num_type: String, // "arabicPeriod", "alphaLcPeriod", etc.
    pub start_at: Option<i32>,
    pub font: Option<String>,
    pub size_pct: Option<f64>,
    pub color: Option<super::color::Color>,
}
