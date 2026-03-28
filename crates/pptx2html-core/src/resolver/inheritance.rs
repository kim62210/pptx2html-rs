//! Property inheritance cascade (slide -> layout -> master)
//!
//! OOXML uses a 3-level hierarchy: SlideMaster -> SlideLayout -> Slide.
//! Properties defined at a lower level (slide) override the same property
//! from a higher level (master). This module resolves effective values.

use crate::model::hierarchy::{ClrMapOverride, FmtScheme, SlideLayout, SlideMaster};
use crate::model::presentation::{ClrMap, ColorScheme};
use crate::model::slide::{Shape, Slide};
use crate::model::{Border, Color, Fill, Position, Size};
use super::style_ref;

/// Resolve effective background for a slide (slide -> layout -> master -> white)
pub fn resolve_background(
    slide: &Slide,
    layout: Option<&SlideLayout>,
    master: Option<&SlideMaster>,
) -> Fill {
    if let Some(ref bg) = slide.background {
        if !matches!(bg, Fill::None) {
            return bg.clone();
        }
    }
    if let Some(l) = layout {
        if let Some(ref bg) = l.background {
            if !matches!(bg, Fill::None) {
                return bg.clone();
            }
        }
    }
    if let Some(m) = master {
        if let Some(ref bg) = m.background {
            if !matches!(bg, Fill::None) {
                return bg.clone();
            }
        }
    }
    Fill::Solid(crate::model::SolidFill {
        color: Color::rgb("FFFFFF"),
    })
}

/// Resolve effective fill for a shape (slide shape -> layout match -> master match -> style_ref)
pub fn resolve_shape_fill(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
) -> Fill {
    if !matches!(shape.fill, Fill::None) {
        return shape.fill.clone();
    }
    if let Some(lm) = layout_match {
        if !matches!(lm.fill, Fill::None) {
            return lm.fill.clone();
        }
    }
    if let Some(mm) = master_match {
        if !matches!(mm.fill, Fill::None) {
            return mm.fill.clone();
        }
    }
    Fill::None
}

/// Resolve effective fill with style_ref fallback (theme-aware)
pub fn resolve_shape_fill_with_theme(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
    fmt_scheme: Option<&FmtScheme>,
    scheme: Option<&ColorScheme>,
    clr_map: Option<&ClrMap>,
) -> Fill {
    let basic = resolve_shape_fill(shape, layout_match, master_match);
    if !matches!(basic, Fill::None) {
        return basic;
    }

    // Try style_ref fillRef as fallback
    if let (Some(style_ref), Some(fmt), Some(cs), Some(cm)) =
        (&shape.style_ref, fmt_scheme, scheme, clr_map)
    {
        if let Some(fill_ref) = &style_ref.fill_ref {
            if let Some(resolved) = style_ref::resolve_fill_ref(fill_ref, fmt, cs, cm) {
                return resolved;
            }
        }
    }

    Fill::None
}

/// Resolve effective border with style_ref fallback (theme-aware)
pub fn resolve_border_with_theme(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
    fmt_scheme: Option<&FmtScheme>,
    scheme: Option<&ColorScheme>,
    clr_map: Option<&ClrMap>,
) -> Border {
    // Check shape's own border first
    if shape.border.width > 0.0 {
        return shape.border.clone();
    }
    // Check layout match
    if let Some(lm) = layout_match {
        if lm.border.width > 0.0 {
            return lm.border.clone();
        }
    }
    // Check master match
    if let Some(mm) = master_match {
        if mm.border.width > 0.0 {
            return mm.border.clone();
        }
    }

    // Try style_ref lnRef as fallback
    if let (Some(sr), Some(fmt), Some(cs), Some(cm)) =
        (&shape.style_ref, fmt_scheme, scheme, clr_map)
    {
        if let Some(ln_ref) = &sr.ln_ref {
            if let Some(resolved) = style_ref::resolve_ln_ref(ln_ref, fmt, cs, cm) {
                return resolved;
            }
        }
    }

    shape.border.clone()
}

/// Resolve effective position/size for a placeholder shape.
/// Use shape's own position if it has non-zero size, otherwise fall back.
pub fn resolve_position(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
) -> (Position, Size) {
    if has_nonzero_size(&shape.size) {
        return (shape.position, shape.size);
    }
    if let Some(lm) = layout_match {
        if has_nonzero_size(&lm.size) {
            return (lm.position, lm.size);
        }
    }
    if let Some(mm) = master_match {
        if has_nonzero_size(&mm.size) {
            return (mm.position, mm.size);
        }
    }
    (shape.position, shape.size)
}

/// Resolve effective ClrMap for a slide considering overrides.
/// Check slide clr_map_ovr, then layout clr_map_ovr, then master clr_map.
pub fn resolve_clr_map<'a>(
    slide: &'a Slide,
    layout: Option<&'a SlideLayout>,
    master: &'a SlideMaster,
) -> &'a ClrMap {
    if let Some(ref ovr) = slide.clr_map_ovr {
        match ovr {
            ClrMapOverride::Override(cm) => return cm,
            ClrMapOverride::UseMaster => {}
        }
    }
    if let Some(l) = layout {
        if let Some(ref ovr) = l.clr_map_ovr {
            match ovr {
                ClrMapOverride::Override(cm) => return cm,
                ClrMapOverride::UseMaster => {}
            }
        }
    }
    &master.clr_map
}

fn has_nonzero_size(size: &Size) -> bool {
    size.width.0 != 0 || size.height.0 != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    fn red_fill() -> Fill {
        Fill::Solid(SolidFill {
            color: Color::rgb("FF0000"),
        })
    }

    fn blue_fill() -> Fill {
        Fill::Solid(SolidFill {
            color: Color::rgb("0000FF"),
        })
    }

    fn green_fill() -> Fill {
        Fill::Solid(SolidFill {
            color: Color::rgb("00FF00"),
        })
    }

    // -- resolve_background tests --

    #[test]
    fn bg_slide_wins() {
        let slide = Slide {
            background: Some(red_fill()),
            ..Default::default()
        };
        let layout = SlideLayout {
            background: Some(blue_fill()),
            ..Default::default()
        };
        let master = SlideMaster {
            background: Some(green_fill()),
            ..Default::default()
        };
        let bg = resolve_background(&slide, Some(&layout), Some(&master));
        assert!(matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "FF0000")));
    }

    #[test]
    fn bg_layout_fallback() {
        let slide = Slide::default();
        let layout = SlideLayout {
            background: Some(blue_fill()),
            ..Default::default()
        };
        let bg = resolve_background(&slide, Some(&layout), None);
        assert!(matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "0000FF")));
    }

    #[test]
    fn bg_master_fallback() {
        let slide = Slide::default();
        let master = SlideMaster {
            background: Some(green_fill()),
            ..Default::default()
        };
        let bg = resolve_background(&slide, None, Some(&master));
        assert!(matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "00FF00")));
    }

    #[test]
    fn bg_default_white() {
        let slide = Slide::default();
        let bg = resolve_background(&slide, None, None);
        assert!(matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "FFFFFF")));
    }

    #[test]
    fn bg_skip_fill_none() {
        let slide = Slide {
            background: Some(Fill::None),
            ..Default::default()
        };
        let layout = SlideLayout {
            background: Some(blue_fill()),
            ..Default::default()
        };
        let bg = resolve_background(&slide, Some(&layout), None);
        assert!(matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "0000FF")));
    }

    // -- resolve_shape_fill tests --

    #[test]
    fn shape_fill_from_shape() {
        let shape = Shape {
            fill: red_fill(),
            ..Default::default()
        };
        let fill = resolve_shape_fill(&shape, None, None);
        assert!(matches!(fill, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "FF0000")));
    }

    #[test]
    fn shape_fill_from_layout_match() {
        let shape = Shape::default();
        let layout_match = Shape {
            fill: blue_fill(),
            ..Default::default()
        };
        let fill = resolve_shape_fill(&shape, Some(&layout_match), None);
        assert!(matches!(fill, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "0000FF")));
    }

    #[test]
    fn shape_fill_from_master_match() {
        let shape = Shape::default();
        let layout_match = Shape::default();
        let master_match = Shape {
            fill: green_fill(),
            ..Default::default()
        };
        let fill = resolve_shape_fill(&shape, Some(&layout_match), Some(&master_match));
        assert!(matches!(fill, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "00FF00")));
    }

    #[test]
    fn shape_fill_none_when_all_none() {
        let shape = Shape::default();
        let fill = resolve_shape_fill(&shape, None, None);
        assert!(matches!(fill, Fill::None));
    }

    // -- resolve_position tests --

    #[test]
    fn position_from_shape() {
        let shape = Shape {
            position: Position {
                x: Emu(100),
                y: Emu(200),
            },
            size: Size {
                width: Emu(500),
                height: Emu(300),
            },
            ..Default::default()
        };
        let (pos, sz) = resolve_position(&shape, None, None);
        assert_eq!(pos.x, Emu(100));
        assert_eq!(sz.width, Emu(500));
    }

    #[test]
    fn position_fallback_to_layout() {
        let shape = Shape::default(); // zero size
        let layout_match = Shape {
            position: Position {
                x: Emu(1000),
                y: Emu(2000),
            },
            size: Size {
                width: Emu(5000),
                height: Emu(3000),
            },
            ..Default::default()
        };
        let (pos, sz) = resolve_position(&shape, Some(&layout_match), None);
        assert_eq!(pos.x, Emu(1000));
        assert_eq!(sz.width, Emu(5000));
    }

    #[test]
    fn position_fallback_to_master() {
        let shape = Shape::default();
        let layout_match = Shape::default();
        let master_match = Shape {
            position: Position {
                x: Emu(9000),
                y: Emu(8000),
            },
            size: Size {
                width: Emu(7000),
                height: Emu(6000),
            },
            ..Default::default()
        };
        let (pos, sz) = resolve_position(&shape, Some(&layout_match), Some(&master_match));
        assert_eq!(pos.x, Emu(9000));
        assert_eq!(sz.width, Emu(7000));
    }

    // -- resolve_clr_map tests --

    #[test]
    fn clr_map_from_slide_override() {
        let mut slide_cm = ClrMap::default();
        slide_cm.set("tx1", "accent1");
        let slide = Slide {
            clr_map_ovr: Some(ClrMapOverride::Override(slide_cm)),
            ..Default::default()
        };
        let mut master_cm = ClrMap::default();
        master_cm.set("tx1", "dk1");
        let master = SlideMaster {
            clr_map: master_cm,
            ..Default::default()
        };
        let resolved = resolve_clr_map(&slide, None, &master);
        assert_eq!(resolved.get("tx1"), Some(&"accent1".to_string()));
    }

    #[test]
    fn clr_map_from_layout_override() {
        let slide = Slide::default();
        let mut layout_cm = ClrMap::default();
        layout_cm.set("tx1", "accent2");
        let layout = SlideLayout {
            clr_map_ovr: Some(ClrMapOverride::Override(layout_cm)),
            ..Default::default()
        };
        let mut master_cm = ClrMap::default();
        master_cm.set("tx1", "dk1");
        let master = SlideMaster {
            clr_map: master_cm,
            ..Default::default()
        };
        let resolved = resolve_clr_map(&slide, Some(&layout), &master);
        assert_eq!(resolved.get("tx1"), Some(&"accent2".to_string()));
    }

    #[test]
    fn clr_map_fallback_to_master() {
        let slide = Slide::default();
        let mut master_cm = ClrMap::default();
        master_cm.set("tx1", "dk1");
        let master = SlideMaster {
            clr_map: master_cm,
            ..Default::default()
        };
        let resolved = resolve_clr_map(&slide, None, &master);
        assert_eq!(resolved.get("tx1"), Some(&"dk1".to_string()));
    }

    #[test]
    fn clr_map_use_master_directive() {
        let slide = Slide {
            clr_map_ovr: Some(ClrMapOverride::UseMaster),
            ..Default::default()
        };
        let mut master_cm = ClrMap::default();
        master_cm.set("tx1", "dk1");
        let master = SlideMaster {
            clr_map: master_cm,
            ..Default::default()
        };
        let resolved = resolve_clr_map(&slide, None, &master);
        assert_eq!(resolved.get("tx1"), Some(&"dk1".to_string()));
    }

    #[test]
    fn clr_map_slide_override_beats_layout() {
        let mut slide_cm = ClrMap::default();
        slide_cm.set("tx1", "accent3");
        let slide = Slide {
            clr_map_ovr: Some(ClrMapOverride::Override(slide_cm)),
            ..Default::default()
        };
        let mut layout_cm = ClrMap::default();
        layout_cm.set("tx1", "accent2");
        let layout = SlideLayout {
            clr_map_ovr: Some(ClrMapOverride::Override(layout_cm)),
            ..Default::default()
        };
        let mut master_cm = ClrMap::default();
        master_cm.set("tx1", "dk1");
        let master = SlideMaster {
            clr_map: master_cm,
            ..Default::default()
        };
        let resolved = resolve_clr_map(&slide, Some(&layout), &master);
        assert_eq!(resolved.get("tx1"), Some(&"accent3".to_string()));
    }
}
