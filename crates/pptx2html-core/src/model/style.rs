use super::color::Color;

/// Underline type (ECMA-376 ST_TextUnderlineType)
#[derive(Debug, Clone, Default, PartialEq)]
pub enum UnderlineType {
    #[default]
    None,
    Single,
    Double,
    Heavy,
    Dotted,
    DottedHeavy,
    Dashed,
    DashHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DotDashHeavy,
    DotDotDash,
    DotDotDashHeavy,
    Wavy,
    WavyHeavy,
    WavyDouble,
}

impl UnderlineType {
    /// Parse OOXML `u` attribute value
    pub fn from_ooxml(val: &str) -> Self {
        match val {
            "sng" => Self::Single,
            "dbl" => Self::Double,
            "heavy" => Self::Heavy,
            "dotted" => Self::Dotted,
            "dottedHeavy" => Self::DottedHeavy,
            "dash" => Self::Dashed,
            "dashHeavy" => Self::DashHeavy,
            "dashLong" => Self::DashLong,
            "dashLongHeavy" => Self::DashLongHeavy,
            "dotDash" => Self::DotDash,
            "dotDashHeavy" => Self::DotDashHeavy,
            "dotDotDash" => Self::DotDotDash,
            "dotDotDashHeavy" => Self::DotDotDashHeavy,
            "wavy" => Self::Wavy,
            "wavyHeavy" => Self::WavyHeavy,
            "wavyDbl" => Self::WavyDouble,
            _ => Self::None,
        }
    }

    /// Generate CSS properties for this underline type
    pub fn to_css(&self) -> Option<String> {
        match self {
            Self::None => Option::None,
            Self::Single => Some("text-decoration: underline".to_string()),
            Self::Double => {
                Some("text-decoration: underline; text-decoration-style: double".to_string())
            }
            Self::Heavy => {
                Some("text-decoration: underline; text-decoration-thickness: 2px".to_string())
            }
            Self::Dotted | Self::DottedHeavy => {
                Some("text-decoration: underline; text-decoration-style: dotted".to_string())
            }
            Self::Dashed | Self::DashHeavy | Self::DashLong | Self::DashLongHeavy => {
                Some("text-decoration: underline; text-decoration-style: dashed".to_string())
            }
            Self::Wavy | Self::WavyHeavy | Self::WavyDouble => {
                Some("text-decoration: underline; text-decoration-style: wavy".to_string())
            }
            Self::DotDash | Self::DotDashHeavy | Self::DotDotDash | Self::DotDotDashHeavy => {
                Some("text-decoration: underline; text-decoration-style: dashed".to_string())
            }
        }
    }
}

/// Strikethrough type (ECMA-376 ST_TextStrikeType)
#[derive(Debug, Clone, Default, PartialEq)]
pub enum StrikethroughType {
    #[default]
    None,
    Single,
    Double,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum TextCapitalization {
    #[default]
    None,
    All,
    Small,
}

impl TextCapitalization {
    pub fn from_ooxml(val: &str) -> Self {
        match val {
            "all" => Self::All,
            "small" => Self::Small,
            _ => Self::None,
        }
    }

    pub fn to_css(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::All => Some("text-transform: uppercase"),
            Self::Small => Some("font-variant: small-caps"),
        }
    }
}

impl StrikethroughType {
    /// Parse OOXML `strike` attribute value
    pub fn from_ooxml(val: &str) -> Self {
        match val {
            "sngStrike" => Self::Single,
            "dblStrike" => Self::Double,
            _ => Self::None,
        }
    }

    /// Generate CSS properties for this strikethrough type
    pub fn to_css(&self) -> Option<&'static str> {
        match self {
            Self::None => Option::None,
            Self::Single => Some("text-decoration: line-through"),
            Self::Double => Some("text-decoration: line-through; text-decoration-style: double"),
        }
    }
}

/// Text style
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f64>, // in pt
    pub bold: bool,
    pub italic: bool,
    pub underline: UnderlineType,
    pub strikethrough: StrikethroughType,
    pub capitalization: TextCapitalization,
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

/// Gradient type (ECMA-376 §20.1.8.46 — a:path, a:lin)
#[derive(Debug, Clone, Default, PartialEq)]
pub enum GradientType {
    #[default]
    Linear, // <a:lin ang="...">
    Radial,      // <a:path path="circle">
    Rectangular, // <a:path path="rect">
    Shape,       // <a:path path="shape">
}

impl GradientType {
    /// Parse OOXML `<a:path path="...">` attribute value
    pub fn from_path_attr(val: &str) -> Self {
        match val {
            "circle" => Self::Radial,
            "rect" => Self::Rectangular,
            "shape" => Self::Shape,
            _ => Self::Radial, // default for unrecognized path types
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GradientFill {
    pub gradient_type: GradientType,
    pub stops: Vec<GradientStop>,
    pub angle: f64, // in degrees (used for Linear)
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
    pub compound: CompoundLine,
    pub alignment: LineAlignment,
    pub join: LineJoin,
    pub miter_limit: Option<f64>,
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

#[derive(Debug, Clone, Default)]
pub enum CompoundLine {
    #[default]
    Single,
    Double,
    ThickThin,
    ThinThick,
    Triple,
}

#[derive(Debug, Clone, Default)]
pub enum LineAlignment {
    #[default]
    Center,
    Inset,
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
    /// Multiplier relative to stroke width for SVG markers
    /// (markerUnits="userSpaceOnUse"). OOXML w/len sm/med/lg map to
    /// proportional multiples of the line width so that thin lines get
    /// small markers and thick lines get proportionally larger ones.
    pub fn multiplier(&self) -> f64 {
        match self {
            Self::Small => 2.0,
            Self::Medium => 3.0,
            Self::Large => 4.5,
        }
    }
}
