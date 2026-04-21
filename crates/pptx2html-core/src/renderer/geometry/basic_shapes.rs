// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: basic_shapes

use super::shared::scale_normalized_path;
use std::collections::HashMap;
pub(super) fn rect_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z")
}
pub(super) fn round_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a = adj.get("adj").copied().unwrap_or(16667.0);
    let m = w.min(h);
    let r = (m * a / 100_000.0).min(m / 2.0);
    format!(
        "M{r:.1},0 L{x1:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y1:.1} Q{w:.1},{h:.1} {x1:.1},{h:.1} L{r:.1},{h:.1} Q0,{h:.1} 0,{y1:.1} L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x1 = w - r,
        y1 = h - r,
        w = w,
        h = h
    )
}
pub(super) fn ellipse_path(w: f64, h: f64) -> String {
    let rx = w / 2.0;
    let ry = h / 2.0;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z",
        cx = rx,
        rx = rx,
        ry = ry,
        h = h
    )
}
pub(super) fn triangle_path(w: f64, h: f64) -> String {
    format!("M{:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z", w / 2.0)
}
pub(super) fn rt_triangle_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},{h:.1} L0,{h:.1} Z")
}
pub(super) fn diamond_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    let cy = h / 2.0;
    format!("M{cx:.1},0 L{w:.1},{cy:.1} L{cx:.1},{h:.1} L0,{cy:.1} Z")
}
pub(super) fn parallelogram_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let o = w * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    format!(
        "M{o:.1},0 L{w:.1},0 L{x:.1},{h:.1} L0,{h:.1} Z",
        o = o,
        w = w,
        x = w - o,
        h = h
    )
}
pub(super) const HEXAGON_ADJ_LIGHT_NORMALIZED_PATH: &str = r#"M 0.000000,0.500000 L 0.099888,0.000000 0.899888,0.000000 1.000000,0.500000 0.899888,1.000000 0.099888,1.000000 0.000000,0.500000 Z"#;
pub(super) const HEXAGON_ADJ_DEFAULTISH_NORMALIZED_PATH: &str = r#"M 0.000000,0.500000 L 0.249944,0.000000 0.749831,0.000000 1.000000,0.500000 0.749831,1.000000 0.249944,1.000000 0.000000,0.500000 Z"#;
pub(super) const HEXAGON_ADJ_DEEP_NORMALIZED_PATH: &str = r#"M 0.000000,0.500000 L 0.400000,0.000000 0.600000,0.000000 1.000000,0.500000 0.600000,1.000000 0.400000,1.000000 0.000000,0.500000 Z"#;
pub(super) const HEXAGON_ADJ_EXTREME_NORMALIZED_PATH: &str = r#"M 0.000000,0.500000 L 0.499888,0.000000 0.499888,0.000000 1.000000,0.500000 0.499888,1.000000 0.499888,1.000000 0.000000,0.500000 Z"#;
pub(super) fn hexagon_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let value = adj.get("adj").copied().unwrap_or(25_000.0);
    let anchors = [
        (10_000.0, HEXAGON_ADJ_LIGHT_NORMALIZED_PATH),
        (25_000.0, HEXAGON_ADJ_DEFAULTISH_NORMALIZED_PATH),
        (40_000.0, HEXAGON_ADJ_DEEP_NORMALIZED_PATH),
        (55_000.0, HEXAGON_ADJ_EXTREME_NORMALIZED_PATH),
    ];

    anchors
        .into_iter()
        .min_by(|(ax, _), (ay, _)| {
            let dx = (value - *ax) / 45_000.0;
            let dy = (value - *ay) / 45_000.0;
            (dx * dx)
                .partial_cmp(&(dy * dy))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, path)| path)
        .unwrap_or(HEXAGON_ADJ_DEFAULTISH_NORMALIZED_PATH)
}
pub(super) fn hexagon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if !adj.is_empty() {
        return scale_normalized_path(hexagon_adjust_anchor(adj), w, h);
    }

    let o = w * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    let cy = h / 2.0;
    format!(
        "M{o:.1},0 L{x:.1},0 L{w:.1},{cy:.1} L{x:.1},{h:.1} L{o:.1},{h:.1} L0,{cy:.1} Z",
        o = o,
        x = w - o,
        w = w,
        cy = cy,
        h = h
    )
}
pub(super) const TRAPEZOID_ADJ_LIGHT_NORMALIZED_PATH: &str = r#"M 0.000000,1.000000 L 0.099888,0.000000 0.899888,0.000000 1.000000,1.000000 0.000000,1.000000 Z"#;
pub(super) const TRAPEZOID_ADJ_DEFAULTISH_NORMALIZED_PATH: &str = r#"M 0.000000,1.000000 L 0.249944,0.000000 0.749831,0.000000 1.000000,1.000000 0.000000,1.000000 Z"#;
pub(super) const TRAPEZOID_ADJ_DEEP_NORMALIZED_PATH: &str = r#"M 0.000000,1.000000 L 0.400000,0.000000 0.600000,0.000000 1.000000,1.000000 0.000000,1.000000 Z"#;
pub(super) const TRAPEZOID_ADJ_EXTREME_NORMALIZED_PATH: &str = r#"M 0.000000,1.000000 L 0.499888,0.000000 0.499888,0.000000 1.000000,1.000000 0.000000,1.000000 Z"#;
pub(super) fn trapezoid_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let value = adj.get("adj").copied().unwrap_or(25_000.0);
    let anchors = [
        (10_000.0, TRAPEZOID_ADJ_LIGHT_NORMALIZED_PATH),
        (25_000.0, TRAPEZOID_ADJ_DEFAULTISH_NORMALIZED_PATH),
        (40_000.0, TRAPEZOID_ADJ_DEEP_NORMALIZED_PATH),
        (55_000.0, TRAPEZOID_ADJ_EXTREME_NORMALIZED_PATH),
    ];

    anchors
        .into_iter()
        .min_by(|(ax, _), (ay, _)| {
            let dx = (value - *ax) / 45_000.0;
            let dy = (value - *ay) / 45_000.0;
            (dx * dx)
                .partial_cmp(&(dy * dy))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, path)| path)
        .unwrap_or(TRAPEZOID_ADJ_DEFAULTISH_NORMALIZED_PATH)
}
pub(super) fn trapezoid_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if !adj.is_empty() {
        return scale_normalized_path(trapezoid_adjust_anchor(adj), w, h);
    }

    let o = w * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    format!(
        "M{o:.1},0 L{x:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z",
        o = o,
        x = w - o,
        w = w,
        h = h
    )
}
pub(super) fn pentagon_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    format!(
        "M{cx:.1},0 L{x2:.1},{y1:.1} L{x3:.1},{h:.1} L{x4:.1},{h:.1} L{x1:.1},{y1:.1} Z",
        cx = cx,
        x1 = w * 0.0245,
        x2 = w * 0.9755,
        y1 = h * 0.382,
        x3 = w * 0.7939,
        x4 = w * 0.2061,
        h = h
    )
}
pub(super) fn octagon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a = adj.get("adj").copied().unwrap_or(29289.0);
    let ox = w * a / 100_000.0;
    let oy = h * a / 100_000.0;
    format!(
        "M{ox:.1},0 L{x1:.1},0 L{w:.1},{oy:.1} L{w:.1},{y1:.1} L{x1:.1},{h:.1} L{ox:.1},{h:.1} L0,{y1:.1} L0,{oy:.1} Z",
        ox = ox,
        x1 = w - ox,
        w = w,
        oy = oy,
        y1 = h - oy,
        h = h
    )
}
// Ribbons
pub(super) fn ellipse_ribbon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(12500.0) / 100_000.0;
    let (cy, bh) = (h * (1.0 - a2 + a1), h * a3);
    let _ = a2;
    format!(
        "M0,{cy:.1} Q{cx:.1},{h:.1} {w:.1},{cy:.1} L{w:.1},{bh:.1} Q{cx:.1},0 0,{bh:.1} Z",
        cx = w / 2.0,
        cy = cy,
        w = w,
        bh = bh,
        h = h
    )
}
pub(super) fn ellipse_ribbon2_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(12500.0) / 100_000.0;
    let (cy, bh) = (h * (a2 - a1), h * (1.0 - a3));
    let _ = a2;
    format!(
        "M0,{cy:.1} Q{cx:.1},0 {w:.1},{cy:.1} L{w:.1},{bh:.1} Q{cx:.1},{h:.1} 0,{bh:.1} Z",
        cx = w / 2.0,
        cy = cy,
        w = w,
        bh = bh,
        h = h
    )
}
pub(super) fn non_isosceles_trapezoid_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = w * adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = w * adj.get("adj2").copied().unwrap_or(18750.0) / 100_000.0;
    format!(
        "M{a1:.1},0 L{x:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z",
        a1 = a1,
        x = w - a2,
        w = w,
        h = h
    )
}
