//! Property inheritance cascade (slide -> layout -> master)
//!
//! OOXML uses a 3-level hierarchy: SlideMaster -> SlideLayout -> Slide.
//! Properties defined at a lower level (slide) override the same property
//! from a higher level (master). This module resolves effective values.

use super::style_ref;
use crate::ProvenanceSource;
use crate::model::hierarchy::{ClrMapOverride, FmtScheme, SlideLayout, SlideMaster};
use crate::model::presentation::{ClrMap, ColorScheme};
use crate::model::slide::{Shape, Slide};
use crate::model::{Border, Color, DashStyle, Fill, LineCap, LineJoin, Position, Size};

/// Resolve effective background for a slide (slide -> layout -> master -> white)
pub fn resolve_background(
    slide: &Slide,
    layout: Option<&SlideLayout>,
    master: Option<&SlideMaster>,
) -> Fill {
    if let Some(ref bg) = slide.background
        && !matches!(bg, Fill::None)
    {
        return bg.clone();
    }
    if let Some(l) = layout
        && let Some(ref bg) = l.background
        && !matches!(bg, Fill::None)
    {
        return bg.clone();
    }
    if let Some(m) = master
        && let Some(ref bg) = m.background
        && !matches!(bg, Fill::None)
    {
        return bg.clone();
    }
    Fill::Solid(crate::model::SolidFill {
        color: Color::rgb("FFFFFF"),
    })
}

pub fn background_source(
    slide: &Slide,
    layout: Option<&SlideLayout>,
    master: Option<&SlideMaster>,
) -> ProvenanceSource {
    if let Some(ref bg) = slide.background
        && !matches!(bg, Fill::None)
    {
        return ProvenanceSource::Slide;
    }
    if let Some(l) = layout
        && let Some(ref bg) = l.background
        && !matches!(bg, Fill::None)
    {
        return ProvenanceSource::LayoutBackground;
    }
    if let Some(m) = master
        && let Some(ref bg) = m.background
        && !matches!(bg, Fill::None)
    {
        return ProvenanceSource::MasterBackground;
    }
    ProvenanceSource::HardcodedDefault
}

/// Resolve effective fill for a shape (slide shape -> layout match -> master match -> style_ref)
pub fn resolve_shape_fill(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
) -> Fill {
    // Fill::NoFill is an explicit "transparent" -- stop inheritance here
    if matches!(shape.fill, Fill::NoFill) {
        return Fill::NoFill;
    }
    if !matches!(shape.fill, Fill::None) {
        return shape.fill.clone();
    }
    if let Some(lm) = layout_match {
        if matches!(lm.fill, Fill::NoFill) {
            return Fill::NoFill;
        }
        if !matches!(lm.fill, Fill::None) {
            return lm.fill.clone();
        }
    }
    if let Some(mm) = master_match {
        if matches!(mm.fill, Fill::NoFill) {
            return Fill::NoFill;
        }
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
    // NoFill is explicit transparent -- do NOT apply theme fallback
    if matches!(basic, Fill::NoFill) {
        return Fill::NoFill;
    }
    if !matches!(basic, Fill::None) {
        return basic;
    }

    // Try style_ref fillRef as fallback
    if let (Some(style_ref), Some(fmt), Some(cs), Some(cm)) =
        (&shape.style_ref, fmt_scheme, scheme, clr_map)
        && let Some(fill_ref) = &style_ref.fill_ref
        && let Some(resolved) = style_ref::resolve_fill_ref(fill_ref, fmt, cs, cm)
    {
        return resolved;
    }

    Fill::None
}

pub fn shape_fill_source(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
    has_style_ref_fill: bool,
) -> Option<ProvenanceSource> {
    if matches!(shape.fill, Fill::NoFill) || !matches!(shape.fill, Fill::None) {
        return Some(ProvenanceSource::Slide);
    }
    if let Some(lm) = layout_match {
        if matches!(lm.fill, Fill::NoFill) || !matches!(lm.fill, Fill::None) {
            return Some(ProvenanceSource::LayoutPlaceholder);
        }
    }
    if let Some(mm) = master_match {
        if matches!(mm.fill, Fill::NoFill) || !matches!(mm.fill, Fill::None) {
            return Some(ProvenanceSource::MasterPlaceholder);
        }
    }
    if has_style_ref_fill {
        return Some(ProvenanceSource::StyleRef);
    }
    None
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
    // Explicit <a:noFill/> inside <a:ln> — no border, stop inheritance
    // (analogous to Fill::NoFill for shape fills)
    if shape.border.no_fill {
        return Border::default();
    }
    // Check shape's own border first (width > 0, or has meaningful line-end markers/color)
    if shape.border.width > 0.0 || has_border_properties(&shape.border) {
        let mut border = shape.border.clone();
        // If shape has border properties (e.g., head/tail end) but no color,
        // try to resolve color from lnRef style reference
        if border.color.is_none() {
            if let (Some(sr), Some(fmt), Some(cs), Some(cm)) =
                (&shape.style_ref, fmt_scheme, scheme, clr_map)
                && let Some(ln_ref) = &sr.ln_ref
                && let Some(resolved) = style_ref::resolve_ln_ref(ln_ref, fmt, cs, cm)
            {
                border.color = resolved.color;
                if border.width == 0.0 {
                    border.width = resolved.width;
                }
            }
        }
        return border;
    }
    // Check layout match
    if let Some(lm) = layout_match {
        if lm.border.no_fill {
            return Border::default();
        }
        if lm.border.width > 0.0 || has_border_properties(&lm.border) {
            return lm.border.clone();
        }
    }
    // Check master match
    if let Some(mm) = master_match {
        if mm.border.no_fill {
            return Border::default();
        }
        if mm.border.width > 0.0 || has_border_properties(&mm.border) {
            return mm.border.clone();
        }
    }

    // Try style_ref lnRef as fallback
    if let (Some(sr), Some(fmt), Some(cs), Some(cm)) =
        (&shape.style_ref, fmt_scheme, scheme, clr_map)
        && let Some(ln_ref) = &sr.ln_ref
        && let Some(mut resolved) = style_ref::resolve_ln_ref(ln_ref, fmt, cs, cm)
    {
        // Preserve shape's own head_end/tail_end/dash_style/cap/join if lnRef doesn't provide them
        if resolved.head_end.is_none() && shape.border.head_end.is_some() {
            resolved.head_end = shape.border.head_end.clone();
        }
        if resolved.tail_end.is_none() && shape.border.tail_end.is_some() {
            resolved.tail_end = shape.border.tail_end.clone();
        }
        if matches!(resolved.dash_style, DashStyle::Solid)
            && !matches!(shape.border.dash_style, DashStyle::Solid)
        {
            resolved.dash_style = shape.border.dash_style.clone();
        }
        if matches!(resolved.cap, LineCap::Flat)
            && !matches!(shape.border.cap, LineCap::Flat)
        {
            resolved.cap = shape.border.cap.clone();
        }
        if matches!(resolved.join, LineJoin::Miter)
            && !matches!(shape.border.join, LineJoin::Miter)
        {
            resolved.join = shape.border.join.clone();
        }
        return resolved;
    }

    shape.border.clone()
}

pub fn border_source(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
    has_style_ref_line: bool,
) -> Option<ProvenanceSource> {
    if shape.border.no_fill || shape.border.width > 0.0 || has_border_properties(&shape.border) {
        return Some(ProvenanceSource::Slide);
    }
    if let Some(lm) = layout_match {
        if lm.border.no_fill || lm.border.width > 0.0 || has_border_properties(&lm.border) {
            return Some(ProvenanceSource::LayoutPlaceholder);
        }
    }
    if let Some(mm) = master_match {
        if mm.border.no_fill || mm.border.width > 0.0 || has_border_properties(&mm.border) {
            return Some(ProvenanceSource::MasterPlaceholder);
        }
    }
    if has_style_ref_line {
        return Some(ProvenanceSource::StyleRef);
    }
    None
}

/// Check if a border has meaningful properties beyond width (e.g., color, head/tail end)
fn has_border_properties(border: &Border) -> bool {
    !border.color.is_none()
        || border.head_end.is_some()
        || border.tail_end.is_some()
        || !matches!(border.dash_style, DashStyle::Solid)
        || !matches!(border.cap, LineCap::Flat)
        || !matches!(border.join, LineJoin::Miter)
}

/// Resolve effective position/size for a placeholder shape.
/// Use shape's own geometry if it has non-zero size or non-zero position
/// (position (0,0) with non-zero size is valid top-left placement).
/// Falls back to layout then master if the shape has no geometry at all.
pub fn resolve_position(
    shape: &Shape,
    layout_match: Option<&Shape>,
    master_match: Option<&Shape>,
) -> (Position, Size) {
    if has_own_geometry(&shape.position, &shape.size) {
        return (shape.position, shape.size);
    }
    if let Some(lm) = layout_match
        && has_own_geometry(&lm.position, &lm.size)
    {
        return (lm.position, lm.size);
    }
    if let Some(mm) = master_match
        && has_own_geometry(&mm.position, &mm.size)
    {
        return (mm.position, mm.size);
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
    if let Some(l) = layout
        && let Some(ref ovr) = l.clr_map_ovr
    {
        match ovr {
            ClrMapOverride::Override(cm) => return cm,
            ClrMapOverride::UseMaster => {}
        }
    }
    &master.clr_map
}

/// Check if a shape has its own geometry (xfrm was present in the XML).
/// A shape has own geometry if it has non-zero size OR non-zero position.
/// This correctly handles shapes placed at (0,0) with valid size.
fn has_own_geometry(pos: &Position, size: &Size) -> bool {
    size.width.0 != 0 || size.height.0 != 0 || pos.x.0 != 0 || pos.y.0 != 0
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
        assert!(
            matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "FF0000"))
        );
    }

    #[test]
    fn bg_layout_fallback() {
        let slide = Slide::default();
        let layout = SlideLayout {
            background: Some(blue_fill()),
            ..Default::default()
        };
        let bg = resolve_background(&slide, Some(&layout), None);
        assert!(
            matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "0000FF"))
        );
    }

    #[test]
    fn bg_master_fallback() {
        let slide = Slide::default();
        let master = SlideMaster {
            background: Some(green_fill()),
            ..Default::default()
        };
        let bg = resolve_background(&slide, None, Some(&master));
        assert!(
            matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "00FF00"))
        );
    }

    #[test]
    fn bg_default_white() {
        let slide = Slide::default();
        let bg = resolve_background(&slide, None, None);
        assert!(
            matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "FFFFFF"))
        );
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
        assert!(
            matches!(bg, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "0000FF"))
        );
    }

    // -- resolve_shape_fill tests --

    #[test]
    fn shape_fill_from_shape() {
        let shape = Shape {
            fill: red_fill(),
            ..Default::default()
        };
        let fill = resolve_shape_fill(&shape, None, None);
        assert!(
            matches!(fill, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "FF0000"))
        );
    }

    #[test]
    fn shape_fill_from_layout_match() {
        let shape = Shape::default();
        let layout_match = Shape {
            fill: blue_fill(),
            ..Default::default()
        };
        let fill = resolve_shape_fill(&shape, Some(&layout_match), None);
        assert!(
            matches!(fill, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "0000FF"))
        );
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
        assert!(
            matches!(fill, Fill::Solid(ref sf) if matches!(sf.color.kind, ColorKind::Rgb(ref s) if s == "00FF00"))
        );
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

    #[test]
    fn position_nonzero_position_zero_size_uses_own() {
        // Shape at (100, 200) with zero size should still use its own position
        // (not fall through to layout) because it has explicit geometry
        let shape = Shape {
            position: Position {
                x: Emu(100),
                y: Emu(200),
            },
            size: Size::default(), // zero
            ..Default::default()
        };
        let layout_match = Shape {
            position: Position {
                x: Emu(5000),
                y: Emu(6000),
            },
            size: Size {
                width: Emu(3000),
                height: Emu(4000),
            },
            ..Default::default()
        };
        let (pos, _) = resolve_position(&shape, Some(&layout_match), None);
        assert_eq!(pos.x, Emu(100));
        assert_eq!(pos.y, Emu(200));
    }

    #[test]
    fn position_zero_position_nonzero_size_uses_own() {
        // Shape at (0, 0) with valid size should use its own position (0,0)
        // not fall through to layout
        let shape = Shape {
            position: Position::default(), // (0, 0)
            size: Size {
                width: Emu(5000),
                height: Emu(3000),
            },
            ..Default::default()
        };
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
        let (pos, _) = resolve_position(&shape, Some(&layout_match), None);
        assert_eq!(pos.x, Emu(0));
        assert_eq!(pos.y, Emu(0));
    }

    #[test]
    fn position_negative_coordinate_uses_own() {
        // Picture shape at (0, -6350) with valid size should use its own geometry.
        // Negative coordinates are valid in OOXML (shape extends above slide edge).
        let shape = Shape {
            position: Position {
                x: Emu(0),
                y: Emu(-6350),
            },
            size: Size {
                width: Emu(9144000),
                height: Emu(952500),
            },
            ..Default::default()
        };
        let layout_match = Shape {
            position: Position {
                x: Emu(628650),
                y: Emu(365125),
            },
            size: Size {
                width: Emu(7886700),
                height: Emu(1325563),
            },
            ..Default::default()
        };
        let (pos, sz) = resolve_position(&shape, Some(&layout_match), None);
        assert_eq!(pos.x, Emu(0));
        assert_eq!(pos.y, Emu(-6350));
        assert_eq!(sz.width, Emu(9144000));
        assert_eq!(sz.height, Emu(952500));
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
