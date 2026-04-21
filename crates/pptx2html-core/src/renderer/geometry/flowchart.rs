// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: flowchart

use std::collections::HashMap;
pub(super) fn flowchart_terminator_path(w: f64, h: f64) -> String {
    let r = h / 2.0;
    format!(
        "M{r:.1},0 L{x:.1},0 A{r:.1},{r:.1} 0 0,1 {x:.1},{h:.1} L{r:.1},{h:.1} A{r:.1},{r:.1} 0 0,1 {r:.1},0 Z",
        r = r,
        x = w - r,
        h = h
    )
}
pub(super) fn flowchart_document_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let wh = h * (1.0 - adj.get("adj").copied().unwrap_or(17500.0) / 100_000.0);
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{wh:.1} C{c1:.1},{c2:.1} {c3:.1},{c4:.1} 0,{wh:.1} Z",
        w = w,
        wh = wh,
        c1 = w * 0.7,
        c2 = h * 1.05,
        c3 = w * 0.3,
        c4 = h * 0.7
    )
}
pub(super) fn flowchart_predefined_process_path(w: f64, h: f64) -> String {
    let i = w * 0.1;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{i:.1},0 L{i:.1},{h:.1} M{x:.1},0 L{x:.1},{h:.1}",
        w = w,
        h = h,
        i = i,
        x = w - i
    )
}
pub(super) fn flowchart_alternate_process_path(
    w: f64,
    h: f64,
    adj: &HashMap<String, f64>,
) -> String {
    let r = w.min(h) * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0;
    format!(
        "M{r:.1},0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L{r:.1},{h:.1} Q0,{h:.1} 0,{y:.1} L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x = w - r,
        y = h - r,
        w = w,
        h = h
    )
}
pub(super) fn flowchart_manual_input_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let s = h * adj.get("adj").copied().unwrap_or(20000.0) / 100_000.0;
    format!(
        "M0,{s:.1} L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z",
        s = s,
        w = w,
        h = h
    )
}
pub(super) fn flowchart_input_output_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let o = w * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    format!(
        "M{o:.1},0 L{w:.1},0 L{x:.1},{h:.1} L0,{h:.1} Z",
        o = o,
        w = w,
        x = w - o,
        h = h
    )
}
pub(super) fn flowchart_internal_storage_path(w: f64, h: f64) -> String {
    let i = w.min(h) * 0.15;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{i:.1},0 L{i:.1},{h:.1} M0,{i:.1} L{w:.1},{i:.1}",
        w = w,
        h = h,
        i = i
    )
}
pub(super) fn flowchart_multidocument_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(18750.0) / 100_000.0;
    let (ox, oy, wh) = (w * 0.06, h * 0.06, h * (1.0 - ratio));
    format!(
        "M{ox2:.1},0 L{w:.1},0 L{w:.1},{wh2:.1} C{c1:.1},{c2:.1} {c3:.1},{c4:.1} {ox2:.1},{wh2:.1} Z M{ox:.1},{oy:.1} L{x1:.1},{oy:.1} L{x1:.1},{wh1:.1} C{c5:.1},{c6:.1} {c7:.1},{c8:.1} {ox:.1},{wh1:.1} Z M0,{oy2:.1} L{x0:.1},{oy2:.1} L{x0:.1},{wh:.1} C{c9:.1},{c10:.1} {c11:.1},{c12:.1} 0,{wh:.1} Z",
        ox2 = ox * 2.0,
        w = w,
        wh2 = wh - oy * 2.0,
        c1 = w * 0.7,
        c2 = (wh - oy * 2.0) + h * 0.15,
        c3 = w * 0.35,
        c4 = (wh - oy * 2.0) - h * 0.1,
        ox = ox,
        oy = oy,
        x1 = w - ox,
        wh1 = wh - oy,
        c5 = (w - ox) * 0.7,
        c6 = (wh - oy) + h * 0.15,
        c7 = (w - ox) * 0.35,
        c8 = (wh - oy) - h * 0.1,
        oy2 = oy * 2.0,
        x0 = w - ox * 2.0,
        wh = wh,
        c9 = (w - ox * 2.0) * 0.7,
        c10 = wh + h * 0.15,
        c11 = (w - ox * 2.0) * 0.35,
        c12 = wh - h * 0.1
    )
}
pub(super) fn flowchart_preparation_path(w: f64, h: f64) -> String {
    let o = w * 0.15;
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
pub(super) fn flowchart_manual_operation_path(w: f64, h: f64) -> String {
    let i = w * 0.15;
    format!(
        "M0,0 L{w:.1},0 L{x:.1},{h:.1} L{i:.1},{h:.1} Z",
        w = w,
        x = w - i,
        i = i,
        h = h
    )
}
pub(super) fn flowchart_offpage_connector_path(w: f64, h: f64) -> String {
    let bh = h * 0.8;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{bh:.1} L{cx:.1},{h:.1} L0,{bh:.1} Z",
        w = w,
        bh = bh,
        cx = w / 2.0,
        h = h
    )
}
pub(super) fn flowchart_punched_card_path(w: f64, h: f64) -> String {
    let c = w.min(h) * 0.15;
    format!(
        "M{c:.1},0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} L0,{c:.1} Z",
        c = c,
        w = w,
        h = h
    )
}
pub(super) fn flowchart_punched_tape_path(w: f64, h: f64) -> String {
    let v = h * 0.1;
    format!(
        "M0,{v:.1} C{c1:.1},0 {c2:.1},{v2:.1} {w:.1},{v:.1} L{w:.1},{y:.1} C{c2:.1},{h:.1} {c1:.1},{y2:.1} 0,{y:.1} Z",
        v = v,
        c1 = w * 0.25,
        c2 = w * 0.75,
        v2 = v * 2.0,
        w = w,
        y = h - v,
        h = h,
        y2 = h - v * 2.0
    )
}
pub(super) fn flowchart_summing_junction_path(w: f64, h: f64) -> String {
    let (rx, ry) = (w / 2.0, h / 2.0);
    let (cx, cy) = (rx, ry);
    let (d, dy) = (rx * 0.707, ry * 0.707);
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z M{x1:.1},{y1:.1} L{x2:.1},{y2:.1} M{x3:.1},{y1:.1} L{x4:.1},{y2:.1}",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        x1 = cx - d,
        y1 = cy - dy,
        x2 = cx + d,
        y2 = cy + dy,
        x3 = cx + d,
        x4 = cx - d
    )
}
pub(super) fn flowchart_or_path(w: f64, h: f64) -> String {
    let (rx, ry) = (w / 2.0, h / 2.0);
    let (cx, cy) = (rx, ry);
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z M{cx:.1},0 L{cx:.1},{h:.1} M0,{cy:.1} L{w:.1},{cy:.1}",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        cy = cy,
        w = w
    )
}
pub(super) fn flowchart_collate_path(w: f64, h: f64) -> String {
    let (cx, cy) = (w / 2.0, h / 2.0);
    format!(
        "M0,0 L{w:.1},0 L{cx:.1},{cy:.1} Z M{cx:.1},{cy:.1} L{w:.1},{h:.1} L0,{h:.1} Z",
        w = w,
        cx = cx,
        cy = cy,
        h = h
    )
}
pub(super) fn flowchart_sort_path(w: f64, h: f64) -> String {
    let (cx, cy) = (w / 2.0, h / 2.0);
    format!(
        "M{cx:.1},0 L{w:.1},{cy:.1} L{cx:.1},{h:.1} L0,{cy:.1} Z M0,{cy:.1} L{w:.1},{cy:.1}",
        cx = cx,
        cy = cy,
        w = w,
        h = h
    )
}
pub(super) fn flowchart_extract_path(w: f64, h: f64) -> String {
    format!("M{:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z", w / 2.0, w = w, h = h)
}
pub(super) fn flowchart_merge_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{cx:.1},{h:.1} Z",
        w = w,
        cx = w / 2.0,
        h = h
    )
}
pub(super) fn flowchart_online_storage_path(w: f64, h: f64) -> String {
    let a = w * 0.15;
    let ry = h / 2.0;
    format!(
        "M{a:.1},0 L{w:.1},0 A{a:.1},{ry:.1} 0 0,1 {w:.1},{h:.1} L{a:.1},{h:.1} A{a:.1},{ry:.1} 0 0,0 {a:.1},0 Z",
        a = a,
        w = w,
        ry = ry,
        h = h
    )
}
pub(super) fn flowchart_delay_path(w: f64, h: f64) -> String {
    let rx = w * 0.3;
    let ry = h / 2.0;
    let x = w - rx;
    format!(
        "M0,0 L{x:.1},0 A{rx:.1},{ry:.1} 0 0,1 {x:.1},{h:.1} L0,{h:.1} Z",
        x = x,
        rx = rx,
        ry = ry,
        h = h
    )
}
pub(super) fn flowchart_magnetic_tape_path(w: f64, h: f64) -> String {
    let (rx, ry) = (w / 2.0, h / 2.0);
    let cx = rx;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {x:.1},{y:.1} L{w:.1},{y:.1} L{w:.1},{h:.1} L{cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 0,1 {cx:.1},0 Z",
        cx = cx,
        rx = rx,
        ry = ry,
        x = w * 0.85,
        y = h * 0.85,
        w = w,
        h = h
    )
}
pub(super) fn flowchart_magnetic_disk_path(w: f64, h: f64) -> String {
    let ry = h * 0.12;
    let rx = w / 2.0;
    format!(
        "M0,{ry:.1} A{rx:.1},{ry:.1} 0 0,1 {w:.1},{ry:.1} L{w:.1},{y:.1} A{rx:.1},{ry:.1} 0 0,1 0,{y:.1} Z M0,{ry:.1} A{rx:.1},{ry:.1} 0 0,0 {w:.1},{ry:.1}",
        ry = ry,
        rx = rx,
        w = w,
        y = h - ry
    )
}
pub(super) fn flowchart_magnetic_drum_path(w: f64, h: f64) -> String {
    let rx = w * 0.12;
    let ry = h / 2.0;
    format!(
        "M{rx:.1},0 L{x:.1},0 A{rx:.1},{ry:.1} 0 0,1 {x:.1},{h:.1} L{rx:.1},{h:.1} A{rx:.1},{ry:.1} 0 0,1 {rx:.1},0 Z M{x:.1},0 A{rx:.1},{ry:.1} 0 0,0 {x:.1},{h:.1}",
        rx = rx,
        x = w - rx,
        ry = ry,
        h = h
    )
}
pub(super) fn flowchart_display_path(w: f64, h: f64) -> String {
    let lp = w * 0.15;
    let ra = w * 0.2;
    let cy = h / 2.0;
    let x = w - ra;
    format!(
        "M0,{cy:.1} L{lp:.1},0 L{x:.1},0 A{ra:.1},{cy:.1} 0 0,1 {x:.1},{h:.1} L{lp:.1},{h:.1} Z",
        cy = cy,
        lp = lp,
        x = x,
        ra = ra,
        h = h
    )
}
pub(super) fn flowchart_offline_storage_path(w: f64, h: f64) -> String {
    let i = w * 0.15;
    format!(
        "M0,0 L{w:.1},0 L{x:.1},{h:.1} L{i:.1},{h:.1} Z",
        w = w,
        x = w - i,
        i = i,
        h = h
    )
}
