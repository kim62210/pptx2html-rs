use super::color::Color;

/// Text style
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f64>, // in pt
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub color: Color,
    pub baseline: Option<i32>, // superscript(+)/subscript(-) offset (1/1000 %)
    pub letter_spacing: Option<f64>, // in pt
    pub highlight: Option<Color>, // text highlight (background color)
    pub shadow: Option<TextShadow>, // text shadow from effectLst/outerShdw
}

/// Text shadow parameters
#[derive(Debug, Clone)]
pub struct TextShadow {
    pub color: Color,
    pub blur_rad: f64, // blur radius in pt
    pub dist: f64,     // distance in pt
    pub dir: f64,      // direction angle in degrees
}

/// Font style (run-level)
#[derive(Debug, Clone, Default)]
pub struct FontStyle {
    pub latin: Option<String>,
    pub east_asian: Option<String>,
    pub complex_script: Option<String>,
}

/// Text alignment
#[derive(Debug, Clone, Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl Alignment {
    pub fn from_ooxml(val: &str) -> Self {
        match val {
            "ctr" => Self::Center,
            "r" => Self::Right,
            "just" => Self::Justify,
            _ => Self::Left,
        }
    }

    pub fn to_css(&self) -> &str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Justify => "justify",
        }
    }
}

/// Fill (shape/slide background)
#[derive(Debug, Clone, Default)]
pub enum Fill {
    /// No fill specified -- inheritance/theme fallback should apply
    #[default]
    None,
    /// Explicit `<a:noFill>` -- transparent, do NOT apply theme fallback
    NoFill,
    Solid(SolidFill),
    Gradient(GradientFill),
    Image(ImageFill),
}

/// Image fill data for backgrounds
#[derive(Debug, Clone, Default)]
pub struct ImageFill {
    pub rel_id: String,
    pub data: Vec<u8>,
    pub content_type: String,
}

impl Fill {
    /// Extract the primary color reference from this fill (for SVG rendering)
    pub fn color_ref(&self) -> Color {
        match self {
            Fill::Solid(sf) => sf.color.clone(),
            Fill::Gradient(gf) => gf
                .stops
                .first()
                .map(|s| s.color.clone())
                .unwrap_or_else(Color::none),
            _ => Color::none(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SolidFill {
    pub color: Color,
}

#[derive(Debug, Clone, Default)]
pub struct GradientFill {
    pub stops: Vec<GradientStop>,
    pub angle: f64, // in degrees
}

#[derive(Debug, Clone, Default)]
pub struct GradientStop {
    pub position: f64, // 0.0 ~ 1.0
    pub color: Color,
}

/// Shape-level effects (outerShdw, glow from <a:effectLst>)
#[derive(Debug, Clone, Default)]
pub struct ShapeEffects {
    pub outer_shadow: Option<OuterShadow>,
    pub glow: Option<GlowEffect>,
}

/// Outer shadow effect (<a:outerShdw>)
#[derive(Debug, Clone)]
pub struct OuterShadow {
    pub blur_radius: f64, // in pt (EMU / 12700)
    pub distance: f64,    // in pt
    pub direction: f64,   // in degrees (from 60000ths)
    pub color: Color,
    pub alpha: f64, // 0.0-1.0
}

/// Glow effect (<a:glow>)
#[derive(Debug, Clone)]
pub struct GlowEffect {
    pub radius: f64, // in pt
    pub color: Color,
    pub alpha: f64, // 0.0-1.0
}

/// Border
#[derive(Debug, Clone, Default)]
pub struct Border {
    pub width: f64, // in pt
    pub color: Color,
    pub style: BorderStyle,
    pub dash_style: DashStyle,
    pub cap: LineCap,
    pub join: LineJoin,
    pub head_end: Option<LineEnd>,
    pub tail_end: Option<LineEnd>,
    /// Explicit `<a:noFill/>` inside `<a:ln>` — suppress border, do NOT
    /// inherit from theme lnRef (analogous to `Fill::NoFill`).
    pub no_fill: bool,
}

#[derive(Debug, Clone, Default)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
    Dashed,
    Dotted,
}

/// Dash style for SVG stroke-dasharray rendering
#[derive(Debug, Clone, Default)]
pub enum DashStyle {
    #[default]
    Solid,
    Dash,
    Dot,
    DashDot,
    LongDash,
    LongDashDot,
    LongDashDotDot,
    SystemDash,
    SystemDot,
    SystemDashDot,
    SystemDashDotDot,
}

/// Line cap style (ECMA-376 ST_LineCap)
#[derive(Debug, Clone, Default)]
pub enum LineCap {
    #[default]
    Flat,
    Square,
    Round,
}

/// Line join style (ECMA-376 ST_LineJoinType)
#[derive(Debug, Clone, Default)]
pub enum LineJoin {
    #[default]
    Miter,
    Bevel,
    Round,
}

/// Line ending (arrowhead) for connectors/lines
#[derive(Debug, Clone)]
pub struct LineEnd {
    pub end_type: LineEndType,
    pub width: LineEndSize,
    pub length: LineEndSize,
}

/// Line ending arrowhead type (ECMA-376 ST_LineEndType)
#[derive(Debug, Clone, Default)]
pub enum LineEndType {
    #[default]
    None,
    Arrow,
    Triangle,
    Stealth,
    Diamond,
    Oval,
}

/// Line ending size (ECMA-376 ST_LineEndWidth / ST_LineEndLength)
#[derive(Debug, Clone, Default)]
pub enum LineEndSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl LineEndSize {
    /// Fixed pixel size for SVG markers (markerUnits="userSpaceOnUse").
    /// These values produce visually proportional arrowheads regardless
    /// of stroke width, matching typical OOXML rendering.
    pub fn multiplier(&self) -> f64 {
        match self {
            Self::Small => 4.0,
            Self::Medium => 6.0,
            Self::Large => 9.0,
        }
    }
}
