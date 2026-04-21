// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: connectors

use std::collections::HashMap;
pub(super) fn curved_connector2_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 C{c1:.1},{c2:.1} {c3:.1},{c4:.1} {w:.1},{h:.1}",
        c1 = w * 0.5,
        c2 = 0.0,
        c3 = w * 0.5,
        c4 = h,
        w = w,
        h = h
    )
}
pub(super) fn curved_connector3_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let cx = w * adj.get("adj1").copied().unwrap_or(50000.0) / 100_000.0;
    format!(
        "M0,0 C{c1:.1},0 {cx:.1},0 {cx:.1},{cy:.1} C{cx:.1},{h:.1} {c2:.1},{h:.1} {w:.1},{h:.1}",
        c1 = cx / 2.0,
        cx = cx,
        cy = h / 2.0,
        c2 = cx + (w - cx) / 2.0,
        w = w,
        h = h
    )
}
pub(super) fn curved_connector4_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let x_mid = w * a1;
    let y_mid = h * a2;
    format!(
        "M0,0 C{c1:.1},0 {c2:.1},0 {c2:.1},{y1:.1} C{c2:.1},{y2:.1} {c3:.1},{y2:.1} {c3:.1},{y3:.1} C{c3:.1},{h:.1} {c4:.1},{h:.1} {w:.1},{h:.1}",
        c1 = x_mid / 2.0,
        c2 = x_mid,
        y1 = y_mid / 2.0,
        y2 = y_mid,
        c3 = x_mid + (w - x_mid) / 2.0,
        y3 = y_mid + (h - y_mid) / 2.0,
        c4 = w - (w - x_mid) / 4.0,
        w = w,
        h = h
    )
}
pub(super) fn curved_connector5_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(50000.0) / 100_000.0;
    let x1 = w * a1;
    let y_mid = h * a2;
    let x2 = w * a3;
    format!(
        "M0,0 C{c1:.1},0 {c2:.1},0 {c2:.1},{y1:.1} C{c2:.1},{y2:.1} {c3:.1},{y2:.1} {cx:.1},{cy:.1} C{c4:.1},{y2:.1} {c5:.1},{y3:.1} {c5:.1},{y3:.1} C{c5:.1},{h:.1} {c6:.1},{h:.1} {w:.1},{h:.1}",
        c1 = x1 / 2.0,
        c2 = x1,
        y1 = y_mid * 0.4,
        y2 = y_mid * 0.7,
        c3 = x1 + (x2 - x1) * 0.3,
        cx = (x1 + x2) / 2.0,
        cy = y_mid,
        c4 = x2 - (x2 - x1) * 0.3,
        c5 = x2,
        y3 = y_mid + (h - y_mid) * 0.6,
        c6 = x2 + (w - x2) / 2.0,
        w = w,
        h = h
    )
}
pub(super) fn bent_connector2_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},0 L{w:.1},{h:.1}", w = w, h = h)
}
pub(super) fn bent_connector3_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let cx = w * adj1 / 100000.0;
    format!(
        "M0,0 L{cx:.1},0 L{cx:.1},{h:.1} L{w:.1},{h:.1}",
        cx = cx,
        w = w,
        h = h
    )
}
pub(super) fn bent_connector4_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    format!(
        "M0,0 L{cx:.1},0 L{cx:.1},{cy:.1} L{w:.1},{cy:.1} L{w:.1},{h:.1}",
        cx = w * a1,
        cy = h * a2,
        w = w,
        h = h
    )
}
pub(super) fn bent_connector5_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(50000.0) / 100_000.0;
    format!(
        "M0,0 L{x1:.1},0 L{x1:.1},{y1:.1} L{x2:.1},{y1:.1} L{x2:.1},{cy:.1} L{x3:.1},{cy:.1} L{x3:.1},{h:.1} L{w:.1},{h:.1}",
        x1 = w * a1,
        y1 = h * a2,
        x2 = w * 0.5,
        cy = h * a2 + (h * a3 - h * a2) / 2.0,
        x3 = w * a3,
        h = h,
        w = w
    )
}
