// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: chart_shapes
// Chart shapes
pub(super) fn chart_plus_path(w: f64, h: f64) -> String {
    let t = w.min(h) * 0.12;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{cx1:.1},{t:.1} L{cx2:.1},{t:.1} L{cx2:.1},{y:.1} L{cx1:.1},{y:.1} Z M{t:.1},{cy1:.1} L{x:.1},{cy1:.1} L{x:.1},{cy2:.1} L{t:.1},{cy2:.1} Z",
        w = w,
        h = h,
        t = t,
        cx1 = w * 0.35,
        cx2 = w * 0.65,
        cy1 = h * 0.35,
        cy2 = h * 0.65,
        x = w - t,
        y = h - t
    )
}
pub(super) fn chart_star_path(w: f64, h: f64) -> String {
    let t = w.min(h) * 0.12;
    let (cx, cy) = (w / 2.0, h / 2.0);
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{cx:.1},{t:.1} L{x1:.1},{y1:.1} L{x:.1},{cy:.1} L{x1:.1},{y2:.1} L{cx:.1},{y:.1} L{x2:.1},{y2:.1} L{t:.1},{cy:.1} L{x2:.1},{y1:.1} Z",
        w = w,
        h = h,
        cx = cx,
        cy = cy,
        t = t,
        x = w - t,
        y = h - t,
        x1 = w * 0.7,
        x2 = w * 0.3,
        y1 = h * 0.3,
        y2 = h * 0.7
    )
}
pub(super) fn chart_x_path(w: f64, h: f64) -> String {
    let t = w.min(h) * 0.12;
    let s = w.min(h) * 0.06;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{cx:.1},{y1:.1} L{x1:.1},{t:.1} L{x:.1},{y1:.1} L{x2:.1},{cy:.1} L{x:.1},{y2:.1} L{x1:.1},{y:.1} L{cx:.1},{y2:.1} L{x3:.1},{cy:.1} Z",
        w = w,
        h = h,
        cx = w / 2.0,
        cy = h / 2.0,
        t = t,
        x = w - t,
        y = h - t,
        x1 = w / 2.0 + s,
        x2 = w - t - s,
        x3 = t + s,
        y1 = h * 0.35,
        y2 = h * 0.65
    )
}
