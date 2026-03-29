/// EMU (English Metric Unit) — base length unit in OOXML
/// 1 inch = 914400 EMU, 1 pt = 12700 EMU, 1 cm = 360000 EMU
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Emu(pub i64);

impl Emu {
    pub fn to_px(self) -> f64 {
        // At 96 DPI: 1 inch = 914400 EMU = 96 px
        self.0 as f64 / 914400.0 * 96.0
    }

    pub fn to_pt(self) -> f64 {
        self.0 as f64 / 12700.0
    }

    pub fn to_cm(self) -> f64 {
        self.0 as f64 / 360000.0
    }

    /// Parse an EMU value from string, returning zero for invalid input.
    pub fn parse_emu(s: &str) -> Self {
        Self(s.parse::<i64>().unwrap_or(0))
    }
}

/// 2D position (relative to top-left origin)
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: Emu,
    pub y: Emu,
}

/// 2D size
#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: Emu,
    pub height: Emu,
}

/// Custom geometry parsed from `<a:custGeom>`
#[derive(Debug, Clone)]
pub struct CustomGeometry {
    pub paths: Vec<GeometryPath>,
}

/// A single path inside `<a:pathLst>`
#[derive(Debug, Clone)]
pub struct GeometryPath {
    pub width: f64,
    pub height: f64,
    pub commands: Vec<PathCommand>,
    pub fill: PathFill,
}

/// DrawingML path command
#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    CubicBezTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    QuadBezTo {
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    ArcTo {
        wr: f64,
        hr: f64,
        start_angle: f64,
        swing_angle: f64,
    },
    Close,
}

/// Path fill mode
#[derive(Debug, Clone, Default)]
pub enum PathFill {
    #[default]
    Norm,
    None,
    Lighten,
    Darken,
    LightenLess,
    DarkenLess,
}
