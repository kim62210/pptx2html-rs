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
