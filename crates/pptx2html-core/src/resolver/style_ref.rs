//! Shape style reference resolution (<p:style> element)
//!
//! Resolves fillRef, lnRef, effectRef, and fontRef from the theme's fmtScheme.
//! ECMA-376 §19.3.1.46 (style), §20.1.4.2.9 (fmtScheme)

use crate::model::color::{ColorKind, ResolvedColor};
use crate::model::hierarchy::{FmtScheme, FontRef, StyleRef};
use crate::model::presentation::{ClrMap, ColorScheme, FontScheme};
use crate::model::{Border, BorderStyle, Fill, SolidFill};

/// Resolve fillRef: lookup fmtScheme.fill_style_lst by idx, apply ref color.
///
/// idx 1..=3 -> fill_style_lst[idx-1]
/// idx 1001+ -> bg_fill_style_lst[idx-1001]
/// If the fill from the list uses Color::none() as placeholder, replace with fill_ref.color.
pub fn resolve_fill_ref(
    fill_ref: &StyleRef,
    fmt_scheme: &FmtScheme,
    _scheme: &ColorScheme,
    _clr_map: &ClrMap,
) -> Option<Fill> {
    if fill_ref.idx == 0 {
        return None;
    }

    let base_fill = if fill_ref.idx >= 1001 {
        let list_idx = (fill_ref.idx - 1001) as usize;
        fmt_scheme.bg_fill_style_lst.get(list_idx)
    } else {
        let list_idx = (fill_ref.idx - 1) as usize;
        fmt_scheme.fill_style_lst.get(list_idx)
    };

    let base_fill = base_fill?;

    // If the fill is None or uses a placeholder color (phClr), apply the ref color
    match base_fill {
        Fill::None => {
            if fill_ref.color.is_none() {
                return None;
            }
            Some(Fill::Solid(SolidFill {
                color: fill_ref.color.clone(),
            }))
        }
        Fill::Solid(sf) => {
            let color = if sf.color.is_none() || is_placeholder_color(&sf.color.kind) {
                fill_ref.color.clone()
            } else {
                sf.color.clone()
            };
            Some(Fill::Solid(SolidFill { color }))
        }
        other => Some(other.clone()),
    }
}

/// Resolve lnRef: lookup fmtScheme.ln_style_lst by idx, override color with ln_ref.color.
///
/// idx 1..=N -> ln_style_lst[idx-1]
pub fn resolve_ln_ref(
    ln_ref: &StyleRef,
    fmt_scheme: &FmtScheme,
    _scheme: &ColorScheme,
    _clr_map: &ClrMap,
) -> Option<Border> {
    if ln_ref.idx == 0 {
        return None;
    }

    let list_idx = (ln_ref.idx - 1) as usize;
    let base_border = fmt_scheme.ln_style_lst.get(list_idx)?;

    // Override color with ln_ref.color if the ref has one, or if base uses phClr
    let color =
        if ln_ref.color.kind != ColorKind::None || is_placeholder_color(&base_border.color.kind) {
            ln_ref.color.clone()
        } else {
            base_border.color.clone()
        };

    Some(Border {
        width: base_border.width,
        color,
        style: if base_border.width > 0.0 {
            match base_border.style {
                BorderStyle::None => BorderStyle::Solid,
                ref s => s.clone(),
            }
        } else {
            BorderStyle::None
        },
    })
}

/// Resolve fontRef: major/minor font + color.
///
/// idx "major" -> font_scheme.major_latin
/// idx "minor" -> font_scheme.minor_latin
/// color from font_ref.color resolved via scheme+clr_map
pub fn resolve_font_ref(
    font_ref: &FontRef,
    font_scheme: &FontScheme,
    scheme: &ColorScheme,
    clr_map: &ClrMap,
) -> Option<(String, Option<ResolvedColor>)> {
    let font_name = match font_ref.idx.as_str() {
        "major" => &font_scheme.major_latin,
        "minor" => &font_scheme.minor_latin,
        _ => return None,
    };

    if font_name.is_empty() {
        return None;
    }

    let resolved_color = if !font_ref.color.is_none() {
        font_ref.color.resolve(Some(scheme), Some(clr_map))
    } else {
        None
    };

    Some((font_name.clone(), resolved_color))
}

/// Check if a color is the OOXML placeholder color (phClr)
/// fmtScheme uses phClr as a marker meaning "replace with the style ref's color"
fn is_placeholder_color(kind: &ColorKind) -> bool {
    matches!(kind, ColorKind::Theme(name) if name == "phClr")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::color::{Color, ColorKind};
    use crate::model::hierarchy::{FmtScheme, FontRef, StyleRef};
    use crate::model::presentation::{ClrMap, ColorScheme, FontScheme};
    use crate::model::{Border, BorderStyle, Fill, SolidFill};

    fn test_scheme() -> ColorScheme {
        ColorScheme {
            dk1: "000000".to_string(),
            lt1: "FFFFFF".to_string(),
            accent1: "4472C4".to_string(),
            ..Default::default()
        }
    }

    fn test_fmt_scheme() -> FmtScheme {
        FmtScheme {
            fill_style_lst: vec![
                Fill::Solid(SolidFill {
                    color: Color::none(),
                }),
                Fill::Solid(SolidFill {
                    color: Color::rgb("AABBCC"),
                }),
                Fill::Solid(SolidFill {
                    color: Color::none(),
                }),
            ],
            ln_style_lst: vec![
                Border {
                    width: 0.75,
                    color: Color::none(),
                    style: BorderStyle::Solid,
                },
                Border {
                    width: 1.5,
                    color: Color::none(),
                    style: BorderStyle::Solid,
                },
            ],
            bg_fill_style_lst: vec![Fill::Solid(SolidFill {
                color: Color::none(),
            })],
        }
    }

    #[test]
    fn fill_ref_idx_zero_returns_none() {
        let sr = StyleRef {
            idx: 0,
            color: Color::theme("accent1"),
        };
        assert!(
            resolve_fill_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).is_none()
        );
    }

    #[test]
    fn fill_ref_idx_1_replaces_placeholder_color() {
        let sr = StyleRef {
            idx: 1,
            color: Color::theme("accent1"),
        };
        let fill =
            resolve_fill_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).unwrap();
        match fill {
            Fill::Solid(sf) => {
                assert!(matches!(sf.color.kind, ColorKind::Theme(ref n) if n == "accent1"));
            }
            other => panic!("Expected SolidFill, got {other:?}"),
        }
    }

    #[test]
    fn fill_ref_idx_2_keeps_existing_color() {
        let sr = StyleRef {
            idx: 2,
            color: Color::theme("accent1"),
        };
        let fill =
            resolve_fill_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).unwrap();
        match fill {
            Fill::Solid(sf) => {
                assert!(matches!(sf.color.kind, ColorKind::Rgb(ref v) if v == "AABBCC"));
            }
            other => panic!("Expected SolidFill, got {other:?}"),
        }
    }

    #[test]
    fn fill_ref_bg_idx_1001() {
        let sr = StyleRef {
            idx: 1001,
            color: Color::theme("accent1"),
        };
        let fill =
            resolve_fill_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).unwrap();
        match fill {
            Fill::Solid(sf) => {
                assert!(matches!(sf.color.kind, ColorKind::Theme(ref n) if n == "accent1"));
            }
            other => panic!("Expected SolidFill, got {other:?}"),
        }
    }

    #[test]
    fn fill_ref_out_of_range_returns_none() {
        let sr = StyleRef {
            idx: 99,
            color: Color::theme("accent1"),
        };
        assert!(
            resolve_fill_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).is_none()
        );
    }

    #[test]
    fn ln_ref_idx_zero_returns_none() {
        let sr = StyleRef {
            idx: 0,
            color: Color::theme("accent1"),
        };
        assert!(
            resolve_ln_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).is_none()
        );
    }

    #[test]
    fn ln_ref_idx_1_overrides_color() {
        let sr = StyleRef {
            idx: 1,
            color: Color::theme("accent1"),
        };
        let border =
            resolve_ln_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).unwrap();
        assert!((border.width - 0.75).abs() < 0.01);
        assert!(matches!(border.color.kind, ColorKind::Theme(ref n) if n == "accent1"));
        assert!(matches!(border.style, BorderStyle::Solid));
    }

    #[test]
    fn ln_ref_idx_2() {
        let sr = StyleRef {
            idx: 2,
            color: Color::rgb("FF0000"),
        };
        let border =
            resolve_ln_ref(&sr, &test_fmt_scheme(), &test_scheme(), &ClrMap::default()).unwrap();
        assert!((border.width - 1.5).abs() < 0.01);
        assert!(matches!(border.color.kind, ColorKind::Rgb(ref v) if v == "FF0000"));
    }

    #[test]
    fn font_ref_major() {
        let fr = FontRef {
            idx: "major".to_string(),
            color: Color::theme("dk1"),
        };
        let fs = FontScheme {
            major_latin: "Calibri Light".to_string(),
            minor_latin: "Calibri".to_string(),
            ..Default::default()
        };
        let result = resolve_font_ref(&fr, &fs, &test_scheme(), &ClrMap::default()).unwrap();
        assert_eq!(result.0, "Calibri Light");
        assert!(result.1.is_some());
    }

    #[test]
    fn font_ref_minor() {
        let fr = FontRef {
            idx: "minor".to_string(),
            color: Color::none(),
        };
        let fs = FontScheme {
            major_latin: "Calibri Light".to_string(),
            minor_latin: "Calibri".to_string(),
            ..Default::default()
        };
        let result = resolve_font_ref(&fr, &fs, &test_scheme(), &ClrMap::default()).unwrap();
        assert_eq!(result.0, "Calibri");
        assert!(result.1.is_none());
    }

    #[test]
    fn font_ref_unknown_idx_returns_none() {
        let fr = FontRef {
            idx: "unknown".to_string(),
            color: Color::none(),
        };
        let fs = FontScheme::default();
        assert!(resolve_font_ref(&fr, &fs, &test_scheme(), &ClrMap::default()).is_none());
    }
}
