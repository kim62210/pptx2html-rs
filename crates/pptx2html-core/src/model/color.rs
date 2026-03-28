//! OOXML color system (ECMA-376 §20.1.2.3)
//!
//! Color resolution chain: ColorKind → ClrMap mapping → Theme ColorScheme lookup → apply modifiers → ResolvedColor

use crate::model::presentation::{ClrMap, ColorScheme};

/// Color variant
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ColorKind {
    #[default]
    None,
    /// Direct RGB value (e.g. "FF0000")
    Rgb(String),
    /// Theme color reference (e.g. "accent1", "dk1")
    Theme(String),
    /// System color (e.g. "windowText")
    System(String),
    /// Preset color (e.g. "white", "black")
    Preset(String),
}

/// Color modifier (ECMA-376 §20.1.2.3.*)
#[derive(Debug, Clone, PartialEq)]
pub enum ColorModifier {
    /// White blend — preserves original color by val/100000 ratio, rest is white
    Tint(i32),
    /// Black blend — preserves original color by val/100000 ratio, rest is black
    Shade(i32),
    /// Opacity (0=fully transparent, 100000=opaque)
    Alpha(i32),
    /// Luminance multiply (HSL L channel)
    LumMod(i32),
    /// Luminance offset (HSL L channel)
    LumOff(i32),
    /// Saturation multiply (HSL S channel)
    SatMod(i32),
    /// Saturation offset (HSL S channel)
    SatOff(i32),
    /// Hue multiply (HSL H channel)
    HueMod(i32),
    /// Hue offset (HSL H channel)
    HueOff(i32),
    /// Complementary
    Comp,
    /// RGB inverse
    Inv,
    /// Grayscale conversion
    Gray,
}

/// Color + modifier combination
#[derive(Debug, Clone, Default)]
pub struct Color {
    pub kind: ColorKind,
    pub modifiers: Vec<ColorModifier>,
}

/// Resolved RGBA color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// ── Color construction helpers ──

impl Color {
    pub fn none() -> Self {
        Self {
            kind: ColorKind::None,
            modifiers: vec![],
        }
    }

    pub fn rgb(hex: impl Into<String>) -> Self {
        Self {
            kind: ColorKind::Rgb(hex.into()),
            modifiers: vec![],
        }
    }

    pub fn theme(name: impl Into<String>) -> Self {
        Self {
            kind: ColorKind::Theme(name.into()),
            modifiers: vec![],
        }
    }

    pub fn system(name: impl Into<String>) -> Self {
        Self {
            kind: ColorKind::System(name.into()),
            modifiers: vec![],
        }
    }

    pub fn preset(name: impl Into<String>) -> Self {
        Self {
            kind: ColorKind::Preset(name.into()),
            modifiers: vec![],
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self.kind, ColorKind::None)
    }

    pub fn with_modifier(mut self, m: ColorModifier) -> Self {
        self.modifiers.push(m);
        self
    }

    /// Resolve final RGBA color via theme/ClrMap lookup
    pub fn resolve(
        &self,
        scheme: Option<&ColorScheme>,
        clr_map: Option<&ClrMap>,
    ) -> Option<ResolvedColor> {
        let base = match &self.kind {
            ColorKind::None => return None,
            ColorKind::Rgb(hex) => parse_hex_rgb(hex)?,
            ColorKind::Theme(name) => {
                let mapped = clr_map
                    .and_then(|cm| cm.get(name))
                    .map(String::as_str)
                    .unwrap_or(name.as_str());
                let hex = scheme.and_then(|s| s.get(mapped))?;
                parse_hex_rgb(&hex)?
            }
            ColorKind::System(name) => parse_hex_rgb(system_color_fallback(name)?)?,
            ColorKind::Preset(name) => parse_hex_rgb(preset_color(name)?)?,
        };
        Some(apply_modifiers(base, &self.modifiers))
    }

    /// Fallback CSS without theme (backward compatible)
    pub fn to_css(&self) -> Option<String> {
        match &self.kind {
            ColorKind::None => None,
            ColorKind::Rgb(hex) => Some(format!("#{hex}")),
            ColorKind::Theme(name) => Some(format!("#{}", theme_fallback(name))),
            ColorKind::System(name) => Some(format!("#{}", system_color_fallback(name)?)),
            ColorKind::Preset(name) => Some(format!("#{}", preset_color(name)?)),
        }
    }
}

// ── ResolvedColor ──

impl ResolvedColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_css(&self) -> String {
        if self.a >= 254 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!(
                "rgba({}, {}, {}, {:.2})",
                self.r,
                self.g,
                self.b,
                self.a as f64 / 255.0
            )
        }
    }
}

// ── Apply color modifiers ──

fn apply_modifiers(mut c: ResolvedColor, modifiers: &[ColorModifier]) -> ResolvedColor {
    for m in modifiers {
        match m {
            ColorModifier::Tint(val) => {
                let f = *val as f64 / 100_000.0;
                c.r = clamp_u8(255.0 - (255.0 - c.r as f64) * f);
                c.g = clamp_u8(255.0 - (255.0 - c.g as f64) * f);
                c.b = clamp_u8(255.0 - (255.0 - c.b as f64) * f);
            }
            ColorModifier::Shade(val) => {
                let f = *val as f64 / 100_000.0;
                c.r = clamp_u8(c.r as f64 * f);
                c.g = clamp_u8(c.g as f64 * f);
                c.b = clamp_u8(c.b as f64 * f);
            }
            ColorModifier::Alpha(val) => {
                c.a = clamp_u8(255.0 * *val as f64 / 100_000.0);
            }
            ColorModifier::LumMod(val) => {
                let (h, s, mut l) = rgb_to_hsl(c.r, c.g, c.b);
                l *= *val as f64 / 100_000.0;
                let (r, g, b) = hsl_to_rgb(h, s, l.clamp(0.0, 1.0));
                c.r = r;
                c.g = g;
                c.b = b;
            }
            ColorModifier::LumOff(val) => {
                let (h, s, mut l) = rgb_to_hsl(c.r, c.g, c.b);
                l += *val as f64 / 100_000.0;
                let (r, g, b) = hsl_to_rgb(h, s, l.clamp(0.0, 1.0));
                c.r = r;
                c.g = g;
                c.b = b;
            }
            ColorModifier::SatMod(val) => {
                let (h, mut s, l) = rgb_to_hsl(c.r, c.g, c.b);
                s *= *val as f64 / 100_000.0;
                let (r, g, b) = hsl_to_rgb(h, s.clamp(0.0, 1.0), l);
                c.r = r;
                c.g = g;
                c.b = b;
            }
            ColorModifier::SatOff(val) => {
                let (h, mut s, l) = rgb_to_hsl(c.r, c.g, c.b);
                s += *val as f64 / 100_000.0;
                let (r, g, b) = hsl_to_rgb(h, s.clamp(0.0, 1.0), l);
                c.r = r;
                c.g = g;
                c.b = b;
            }
            ColorModifier::HueMod(val) => {
                let (mut h, s, l) = rgb_to_hsl(c.r, c.g, c.b);
                h *= *val as f64 / 100_000.0;
                h %= 360.0;
                let (r, g, b) = hsl_to_rgb(h, s, l);
                c.r = r;
                c.g = g;
                c.b = b;
            }
            ColorModifier::HueOff(val) => {
                let (mut h, s, l) = rgb_to_hsl(c.r, c.g, c.b);
                h += *val as f64 * 360.0 / 100_000.0;
                h = ((h % 360.0) + 360.0) % 360.0;
                let (r, g, b) = hsl_to_rgb(h, s, l);
                c.r = r;
                c.g = g;
                c.b = b;
            }
            ColorModifier::Inv => {
                c.r ^= 0xFF;
                c.g ^= 0xFF;
                c.b ^= 0xFF;
            }
            ColorModifier::Comp => {
                let (mut h, s, l) = rgb_to_hsl(c.r, c.g, c.b);
                h = (h + 180.0) % 360.0;
                let (r, g, b) = hsl_to_rgb(h, s, l);
                c.r = r;
                c.g = g;
                c.b = b;
            }
            ColorModifier::Gray => {
                let gray = clamp_u8(c.r as f64 * 0.299 + c.g as f64 * 0.587 + c.b as f64 * 0.114);
                c.r = gray;
                c.g = gray;
                c.b = gray;
            }
        }
    }
    c
}

// ── HSL conversion (H: 0-360, S: 0-1, L: 0-1) ──

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let rf = r as f64 / 255.0;
    let gf = g as f64 / 255.0;
    let bf = b as f64 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;
    let l = (max + min) / 2.0;

    if delta < 1e-10 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if (max - rf).abs() < 1e-10 {
        ((gf - bf) / delta) % 6.0
    } else if (max - gf).abs() < 1e-10 {
        (bf - rf) / delta + 2.0
    } else {
        (rf - gf) / delta + 4.0
    };

    let h = ((h * 60.0) + 360.0) % 360.0;
    (h, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    if s < 1e-10 {
        let v = clamp_u8(l * 255.0);
        return (v, v, v);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h_norm = h / 360.0;

    let r = hue_to_rgb(p, q, h_norm + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h_norm);
    let b = hue_to_rgb(p, q, h_norm - 1.0 / 3.0);

    (
        clamp_u8(r * 255.0),
        clamp_u8(g * 255.0),
        clamp_u8(b * 255.0),
    )
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

fn clamp_u8(v: f64) -> u8 {
    v.round().clamp(0.0, 255.0) as u8
}

// ── Hex parsing ──

fn parse_hex_rgb(hex: &str) -> Option<ResolvedColor> {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(ResolvedColor::new(r, g, b))
}

// ── Theme color fallback ──

fn theme_fallback(name: &str) -> &str {
    match name {
        "dk1" | "tx1" => "000000",
        "lt1" | "bg1" => "FFFFFF",
        "dk2" | "tx2" => "44546A",
        "lt2" | "bg2" => "E7E6E6",
        "accent1" => "4472C4",
        "accent2" => "ED7D31",
        "accent3" => "A5A5A5",
        "accent4" => "FFC000",
        "accent5" => "5B9BD5",
        "accent6" => "70AD47",
        "hlink" => "0563C1",
        "folHlink" => "954F72",
        _ => "000000",
    }
}

fn system_color_fallback(name: &str) -> Option<&str> {
    Some(match name {
        "windowText" | "btnText" | "menuText" | "infoText" | "captionText" => "000000",
        "window" | "menu" | "btnFace" | "info" | "btnHighlight" => "FFFFFF",
        "highlight" => "0078D7",
        "highlightText" => "FFFFFF",
        "grayText" => "808080",
        _ => return None,
    })
}

fn preset_color(name: &str) -> Option<&str> {
    Some(match name {
        "black" => "000000",
        "white" => "FFFFFF",
        "red" => "FF0000",
        "green" | "lime" => "00FF00",
        "blue" => "0000FF",
        "yellow" => "FFFF00",
        "cyan" | "aqua" => "00FFFF",
        "magenta" | "fuchsia" => "FF00FF",
        "silver" => "C0C0C0",
        "gray" | "grey" => "808080",
        "maroon" => "800000",
        "olive" => "808000",
        "darkGreen" => "006400",
        "navy" => "000080",
        "teal" => "008080",
        "purple" => "800080",
        "orange" => "FFA500",
        "darkRed" | "dkRed" => "8B0000",
        "darkBlue" | "dkBlue" => "00008B",
        "medBlue" => "0000CD",
        "coral" => "FF7F50",
        "cornflowerBlue" => "6495ED",
        "crimson" => "DC143C",
        "darkCyan" | "dkCyan" => "008B8B",
        "darkGray" | "dkGray" => "A9A9A9",
        "deepPink" => "FF1493",
        "deepSkyBlue" => "00BFFF",
        "dimGray" => "696969",
        "dodgerBlue" => "1E90FF",
        "firebrick" => "B22222",
        "forestGreen" => "228B22",
        "gold" => "FFD700",
        "goldenrod" => "DAA520",
        "hotPink" => "FF69B4",
        "indianRed" => "CD5C5C",
        "indigo" => "4B0082",
        "ivory" => "FFFFF0",
        "khaki" => "F0E68C",
        "lavender" => "E6E6FA",
        "lawnGreen" => "7CFC00",
        "lightBlue" | "ltBlue" => "ADD8E6",
        "lightCoral" => "F08080",
        "lightCyan" => "E0FFFF",
        "lightGray" | "ltGray" => "D3D3D3",
        "lightGreen" => "90EE90",
        "lightPink" => "FFB6C1",
        "lightSalmon" => "FFA07A",
        "lightSeaGreen" => "20B2AA",
        "lightSkyBlue" => "87CEFA",
        "lightSlateGray" => "778899",
        "lightSteelBlue" => "B0C4DE",
        "lightYellow" => "FFFFE0",
        "limeGreen" => "32CD32",
        "medAquamarine" => "66CDAA",
        "medOrchid" => "BA55D3",
        "medPurple" => "9370DB",
        "medSeaGreen" => "3CB371",
        "medSlateBlue" => "7B68EE",
        "medSpringGreen" => "00FA9A",
        "medTurquoise" => "48D1CC",
        "medVioletRed" => "C71585",
        "midnightBlue" => "191970",
        "mistyRose" => "FFE4E1",
        "moccasin" => "FFE4B5",
        "navajoWhite" => "FFDEAD",
        "oldLace" => "FDF5E6",
        "oliveDrab" => "6B8E23",
        "orangeRed" => "FF4500",
        "orchid" => "DA70D6",
        "paleGoldenrod" => "EEE8AA",
        "paleGreen" => "98FB98",
        "paleTurquoise" => "AFEEEE",
        "paleVioletRed" => "DB7093",
        "peachPuff" => "FFDAB9",
        "peru" => "CD853F",
        "pink" => "FFC0CB",
        "plum" => "DDA0DD",
        "powderBlue" => "B0E0E6",
        "rosyBrown" => "BC8F8F",
        "royalBlue" => "4169E1",
        "saddleBrown" => "8B4513",
        "salmon" => "FA8072",
        "sandyBrown" => "F4A460",
        "seaGreen" => "2E8B57",
        "seaShell" => "FFF5EE",
        "sienna" => "A0522D",
        "skyBlue" => "87CEEB",
        "slateBlue" => "6A5ACD",
        "slateGray" => "708090",
        "snow" => "FFFAFA",
        "springGreen" => "00FF7F",
        "steelBlue" => "4682B4",
        "tan" => "D2B48C",
        "thistle" => "D8BFD8",
        "tomato" => "FF6347",
        "turquoise" => "40E0D0",
        "violet" => "EE82EE",
        "wheat" => "F5DEB3",
        "whiteSmoke" => "F5F5F5",
        "yellowGreen" => "9ACD32",
        _ => return None,
    })
}

// ── Modifier parsing helpers ──

impl ColorModifier {
    pub fn from_ooxml(name: &str, val: Option<i32>) -> Option<Self> {
        Some(match name {
            "tint" => Self::Tint(val.unwrap_or(100_000)),
            "shade" => Self::Shade(val.unwrap_or(100_000)),
            "alpha" => Self::Alpha(val.unwrap_or(100_000)),
            "lumMod" => Self::LumMod(val.unwrap_or(100_000)),
            "lumOff" => Self::LumOff(val.unwrap_or(0)),
            "satMod" => Self::SatMod(val.unwrap_or(100_000)),
            "satOff" => Self::SatOff(val.unwrap_or(0)),
            "hueMod" => Self::HueMod(val.unwrap_or(100_000)),
            "hueOff" => Self::HueOff(val.unwrap_or(0)),
            "comp" => Self::Comp,
            "inv" => Self::Inv,
            "gray" => Self::Gray,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_rgb() {
        let c = parse_hex_rgb("FF0000").unwrap();
        assert_eq!(c, ResolvedColor::new(255, 0, 0));

        let c = parse_hex_rgb("#00FF00").unwrap();
        assert_eq!(c, ResolvedColor::new(0, 255, 0));

        assert!(parse_hex_rgb("FFF").is_none());
    }

    #[test]
    fn test_rgb_resolve() {
        let color = Color::rgb("4472C4");
        let resolved = color.resolve(None, None).unwrap();
        assert_eq!(resolved, ResolvedColor::new(0x44, 0x72, 0xC4));
    }

    #[test]
    fn test_theme_resolve_with_scheme() {
        let scheme = ColorScheme {
            accent1: "FF5733".to_string(),
            ..Default::default()
        };
        let color = Color::theme("accent1");
        let resolved = color.resolve(Some(&scheme), None).unwrap();
        assert_eq!(resolved, ResolvedColor::new(0xFF, 0x57, 0x33));
    }

    #[test]
    fn test_theme_with_clrmap() {
        let scheme = ColorScheme {
            dk1: "1A1A1A".to_string(),
            ..Default::default()
        };
        let mut clr_map = ClrMap::default();
        clr_map.set("tx1", "dk1");
        let color = Color::theme("tx1");
        let resolved = color.resolve(Some(&scheme), Some(&clr_map)).unwrap();
        assert_eq!(resolved, ResolvedColor::new(0x1A, 0x1A, 0x1A));
    }

    #[test]
    fn test_tint_modifier() {
        // Pure black + tint 50000 → 50% white blend = ~RGB(128,128,128)
        let color = Color::rgb("000000").with_modifier(ColorModifier::Tint(50000));
        let r = color.resolve(None, None).unwrap();
        assert_eq!(r, ResolvedColor::new(128, 128, 128));
    }

    #[test]
    fn test_shade_modifier() {
        // Pure white + shade 50000 → 50% darkened = ~RGB(128,128,128)
        let color = Color::rgb("FFFFFF").with_modifier(ColorModifier::Shade(50000));
        let r = color.resolve(None, None).unwrap();
        assert_eq!(r, ResolvedColor::new(128, 128, 128));
    }

    #[test]
    fn test_alpha_modifier() {
        let color = Color::rgb("FF0000").with_modifier(ColorModifier::Alpha(50000));
        let r = color.resolve(None, None).unwrap();
        assert_eq!(r.r, 255);
        assert_eq!(r.a, 128);
        assert_eq!(r.to_css(), "rgba(255, 0, 0, 0.50)");
    }

    #[test]
    fn test_lum_mod_off() {
        // accent1 color with lumMod 75000 + lumOff 25000
        let color = Color::rgb("4472C4")
            .with_modifier(ColorModifier::LumMod(75000))
            .with_modifier(ColorModifier::LumOff(25000));
        let r = color.resolve(None, None).unwrap();
        // Result should be a lightened blue
        assert!(r.r > 0x44);
        assert!(r.b > 0xC4 || r.b == 0xC4); // should be brighter
    }

    #[test]
    fn test_inv_modifier() {
        let color = Color::rgb("FF0000").with_modifier(ColorModifier::Inv);
        let r = color.resolve(None, None).unwrap();
        assert_eq!(r, ResolvedColor::new(0, 255, 255));
    }

    #[test]
    fn test_gray_modifier() {
        let color = Color::rgb("FF0000").with_modifier(ColorModifier::Gray);
        let r = color.resolve(None, None).unwrap();
        // Grayscale of red: 0.299*255 ~ 76
        assert_eq!(r.r, r.g);
        assert_eq!(r.g, r.b);
    }

    #[test]
    fn test_hsl_roundtrip() {
        for (r, g, b) in [
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (128, 64, 192),
            (0, 0, 0),
            (255, 255, 255),
        ] {
            let (h, s, l) = rgb_to_hsl(r, g, b);
            let (r2, g2, b2) = hsl_to_rgb(h, s, l);
            assert!(
                (r as i16 - r2 as i16).abs() <= 1,
                "R mismatch for ({r},{g},{b}): got ({r2},{g2},{b2})"
            );
            assert!(
                (g as i16 - g2 as i16).abs() <= 1,
                "G mismatch for ({r},{g},{b}): got ({r2},{g2},{b2})"
            );
            assert!(
                (b as i16 - b2 as i16).abs() <= 1,
                "B mismatch for ({r},{g},{b}): got ({r2},{g2},{b2})"
            );
        }
    }

    #[test]
    fn test_preset_color() {
        let color = Color::preset("red");
        let r = color.resolve(None, None).unwrap();
        assert_eq!(r, ResolvedColor::new(255, 0, 0));
    }

    #[test]
    fn test_system_color() {
        let color = Color::system("windowText");
        let r = color.resolve(None, None).unwrap();
        assert_eq!(r, ResolvedColor::new(0, 0, 0));
    }

    #[test]
    fn test_resolved_color_css() {
        assert_eq!(ResolvedColor::new(255, 0, 0).to_css(), "#FF0000");
        assert_eq!(
            ResolvedColor::with_alpha(255, 0, 0, 128).to_css(),
            "rgba(255, 0, 0, 0.50)"
        );
    }

    #[test]
    fn test_none_resolve() {
        assert!(Color::none().resolve(None, None).is_none());
    }
}
