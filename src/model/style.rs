use super::color::Color;

/// Text style
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f64>,       // in pt
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub color: Color,
    pub baseline: Option<i32>,        // superscript(+)/subscript(-) offset (1/1000 %)
    pub letter_spacing: Option<f64>,  // in pt
    pub highlight: Option<Color>,     // text highlight (background color)
    pub shadow: Option<TextShadow>,   // text shadow from effectLst/outerShdw
}

/// Text shadow parameters
#[derive(Debug, Clone)]
pub struct TextShadow {
    pub color: Color,
    pub blur_rad: f64,  // blur radius in pt
    pub dist: f64,      // distance in pt
    pub dir: f64,       // direction angle in degrees
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
    #[default]
    None,
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
            Fill::Gradient(gf) => {
                gf.stops.first()
                    .map(|s| s.color.clone())
                    .unwrap_or_else(Color::none)
            }
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

/// Border
#[derive(Debug, Clone, Default)]
pub struct Border {
    pub width: f64,       // in pt
    pub color: Color,
    pub style: BorderStyle,
}

#[derive(Debug, Clone, Default)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
    Dashed,
    Dotted,
}
