// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: arrow_callouts

use super::shared::scale_normalized_path;
use std::collections::HashMap;
// Arrow callout shapes
pub(super) fn down_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(64977.0) / 100_000.0;
    let (cx, s, ah) = (w / 2.0, w * a1, h * a3);
    let bh = h * a4;
    let _ = a2;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{bh:.1} L{x2:.1},{bh:.1} L{x2:.1},{yh:.1} L{cx:.1},{h:.1} L{x1:.1},{yh:.1} L{x1:.1},{bh:.1} L0,{bh:.1} Z",
        w = w,
        bh = bh,
        x1 = cx - s,
        x2 = cx + s,
        yh = h - ah,
        cx = cx,
        h = h
    )
}
pub(super) fn left_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(64977.0) / 100_000.0;
    let (cy, s, aw) = (h / 2.0, h * a1, w * a3);
    let bx = w * (1.0 - a4);
    let _ = a2;
    format!(
        "M{bx:.1},0 L{w:.1},0 L{w:.1},{h:.1} L{bx:.1},{h:.1} L{bx:.1},{y2:.1} L{aw:.1},{y2:.1} L0,{cy:.1} L{aw:.1},{y1:.1} L{bx:.1},{y1:.1} Z",
        bx = bx,
        w = w,
        h = h,
        y1 = cy - s,
        y2 = cy + s,
        aw = aw,
        cy = cy
    )
}
pub(super) fn right_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(64977.0) / 100_000.0;
    let (cy, s, aw) = (h / 2.0, h * a1, w * a3);
    let bx = w * a4;
    let _ = a2;
    format!(
        "M0,0 L{bx:.1},0 L{bx:.1},{y1:.1} L{xh:.1},{y1:.1} L{w:.1},{cy:.1} L{xh:.1},{y2:.1} L{bx:.1},{y2:.1} L{bx:.1},{h:.1} L0,{h:.1} Z",
        bx = bx,
        y1 = cy - s,
        y2 = cy + s,
        xh = w - aw,
        w = w,
        cy = cy,
        h = h
    )
}
pub(super) fn up_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(64977.0) / 100_000.0;
    let (cx, s, ah) = (w / 2.0, w * a1, h * a3);
    let bh = h * (1.0 - a4);
    let _ = a2;
    format!(
        "M0,{bh:.1} L{x1:.1},{bh:.1} L{x1:.1},{ah:.1} L{cx:.1},0 L{x2:.1},{ah:.1} L{x2:.1},{bh:.1} L{w:.1},{bh:.1} L{w:.1},{h:.1} L0,{h:.1} Z",
        bh = bh,
        x1 = cx - s,
        x2 = cx + s,
        ah = ah,
        cx = cx,
        w = w,
        h = h
    )
}
pub(super) const QUAD_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.976608,0.525362 L 0.824561,0.623188 L 0.812865,0.619565 L 0.812865,0.565217 L 0.754386,0.565217 L 0.754386,0.746377 L 0.602339,0.750000 L 0.602339,0.884058 L 0.695906,0.884058 L 0.695906,0.894928 L 0.520468,1.000000 L 0.485380,1.000000 L 0.309942,0.894928 L 0.309942,0.884058 L 0.403509,0.884058 L 0.403509,0.750000 L 0.251462,0.746377 L 0.251462,0.565217 L 0.192982,0.565217 L 0.192982,0.619565 L 0.175439,0.619565 L 0.000000,0.510870 L 0.000000,0.492754 L 0.175439,0.384058 L 0.192982,0.384058 L 0.192982,0.438406 L 0.251462,0.438406 L 0.251462,0.257246 L 0.403509,0.253623 L 0.403509,0.119565 L 0.309942,0.119565 L 0.309942,0.108696 L 0.485380,0.000000 L 0.520468,0.000000 L 0.695906,0.108696 L 0.695906,0.119565 L 0.602339,0.119565 L 0.602339,0.253623 L 0.754386,0.257246 L 0.754386,0.438406 L 0.812865,0.438406 L 0.812865,0.384058 L 0.824561,0.380435 L 1.000000,0.492754 L 1.000000,0.510870 Z"#;
pub(super) const QUAD_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L 0.149863,0.349795 0.149863,0.424769 0.424769,0.424769 0.424769,0.424769 0.424769,0.424769 0.424769,0.149863 0.349795,0.149863 0.499914,-0.000086 0.649863,0.149863 0.574889,0.149863 0.574889,0.424769 0.574889,0.424769 0.574889,0.424769 0.849795,0.424769 0.849795,0.349795 0.999914,0.499914 0.849795,0.649863 0.849795,0.574889 0.574889,0.574889 0.574889,0.574889 0.574889,0.574889 0.574889,0.849795 0.649863,0.849795 0.499914,0.999914 0.349795,0.849795 0.424769,0.849795 0.424769,0.574889 0.424769,0.574889 0.424769,0.574889 0.149863,0.574889 0.149863,0.649863 -0.000086,0.499914 Z"#;
pub(super) const QUAD_ARROW_CALLOUT_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L 0.149863,0.149863 0.149863,0.324803 0.324803,0.324803 0.324803,0.324803 0.324803,0.324803 0.324803,0.149863 0.149863,0.149863 0.499914,-0.000086 0.849795,0.149863 0.674855,0.149863 0.674855,0.324803 0.674855,0.324803 0.674855,0.324803 0.849795,0.324803 0.849795,0.149863 0.999914,0.499914 0.849795,0.849795 0.849795,0.674855 0.674855,0.674855 0.674855,0.674855 0.674855,0.674855 0.674855,0.849795 0.849795,0.849795 0.499914,0.999914 0.149863,0.849795 0.324803,0.849795 0.324803,0.674855 0.324803,0.674855 0.324803,0.674855 0.149863,0.674855 0.149863,0.849795 -0.000086,0.499914 Z"#;
pub(super) const QUAD_ARROW_CALLOUT_ADJ_LONG_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L -0.000086,-0.000086 -0.000086,0.399777 0.249829,0.399777 0.249829,0.249829 0.399777,0.249829 0.399777,-0.000086 -0.000086,-0.000086 0.499914,-0.000086 0.999914,-0.000086 0.599880,-0.000086 0.599880,0.249829 0.749829,0.249829 0.749829,0.399777 0.999914,0.399777 0.999914,-0.000086 0.999914,0.499914 0.999914,0.999914 0.999914,0.599880 0.749829,0.599880 0.749829,0.749829 0.599880,0.749829 0.599880,0.999914 0.999914,0.999914 0.499914,0.999914 -0.000086,0.999914 0.399777,0.999914 0.399777,0.749829 0.249829,0.749829 0.249829,0.599880 -0.000086,0.599880 -0.000086,0.999914 -0.000086,0.499914 Z"#;
pub(super) const QUAD_ARROW_CALLOUT_ADJ_THICK_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L 0.299812,0.299812 0.299812,0.299812 0.299812,0.299812 0.299812,0.299812 0.299812,0.299812 0.299812,0.299812 0.299812,0.299812 0.499914,-0.000086 0.699846,0.299812 0.699846,0.299812 0.699846,0.299812 0.699846,0.299812 0.699846,0.299812 0.699846,0.299812 0.699846,0.299812 0.999914,0.499914 0.699846,0.699846 0.699846,0.699846 0.699846,0.699846 0.699846,0.699846 0.699846,0.699846 0.699846,0.699846 0.699846,0.699846 0.499914,0.999914 0.299812,0.699846 0.299812,0.699846 0.299812,0.699846 0.299812,0.699846 0.299812,0.699846 0.299812,0.699846 0.299812,0.699846 -0.000086,0.499914 Z"#;
pub(super) fn quad_arrow_callout_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(18_515.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(18_515.0);
    let adj3 = adj.get("adj3").copied().unwrap_or(18_515.0);
    let adj4 = adj.get("adj4").copied().unwrap_or(48_123.0);
    let anchors = [
        (
            15_000.0,
            15_000.0,
            15_000.0,
            15_000.0,
            QUAD_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (
            35_000.0,
            35_000.0,
            35_000.0,
            35_000.0,
            QUAD_ARROW_CALLOUT_ADJ_WIDE_NORMALIZED_PATH,
        ),
        (
            20_000.0,
            50_000.0,
            25_000.0,
            50_000.0,
            QUAD_ARROW_CALLOUT_ADJ_LONG_NORMALIZED_PATH,
        ),
        (
            45_000.0,
            20_000.0,
            45_000.0,
            20_000.0,
            QUAD_ARROW_CALLOUT_ADJ_THICK_NORMALIZED_PATH,
        ),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a2x, a3x, a4x, _), (a1y, a2y, a3y, a4y, _)| {
            let dx1 = (adj1 - *a1x) / 30_000.0;
            let dx2 = (adj2 - *a2x) / 35_000.0;
            let dx3 = (adj3 - *a3x) / 30_000.0;
            let dx4 = (adj4 - *a4x) / 35_000.0;
            let dy1 = (adj1 - *a1y) / 30_000.0;
            let dy2 = (adj2 - *a2y) / 35_000.0;
            let dy3 = (adj3 - *a3y) / 30_000.0;
            let dy4 = (adj4 - *a4y) / 35_000.0;
            (dx1 * dx1 + dx2 * dx2 + dx3 * dx3 + dx4 * dx4)
                .partial_cmp(&(dy1 * dy1 + dy2 * dy2 + dy3 * dy3 + dy4 * dy4))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, _, _, path)| path)
        .unwrap_or(QUAD_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH)
}
pub(super) fn quad_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(QUAD_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH, w, h);
    }

    scale_normalized_path(quad_arrow_callout_adjust_anchor(adj), w, h)
}
pub(super) const LEFT_RIGHT_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.256318,1.000000 L 0.256318,0.639535 L 0.162455,0.633721 L 0.162455,0.755814 L 0.151625,0.761628 L 0.000000,0.517442 L 0.000000,0.488372 L 0.151625,0.244186 L 0.162455,0.250000 L 0.162455,0.372093 L 0.256318,0.366279 L 0.256318,0.000000 L 0.747292,0.000000 L 0.747292,0.372093 L 0.841155,0.372093 L 0.841155,0.250000 L 0.851986,0.244186 L 1.000000,0.488372 L 1.000000,0.517442 L 0.920578,0.651163 L 0.913357,0.651163 L 0.848375,0.761628 L 0.841155,0.755814 L 0.841155,0.633721 L 0.747292,0.633721 L 0.747292,1.000000 Z"#;
pub(super) const LEFT_RIGHT_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L 0.149863,0.349795 0.149863,0.424769 0.424769,0.424769 0.424769,-0.000086 0.574889,-0.000086 0.574889,0.424769 0.849795,0.424769 0.849795,0.349795 0.999914,0.499914 0.849795,0.649863 0.849795,0.574889 0.574889,0.574889 0.574889,0.999914 0.424769,0.999914 0.424769,0.574889 0.149863,0.574889 0.149863,0.649863 -0.000086,0.499914 Z"#;
pub(super) const LEFT_RIGHT_ARROW_CALLOUT_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L 0.349795,0.149863 0.349795,0.324803 0.349795,0.324803 0.349795,-0.000086 0.649863,-0.000086 0.649863,0.324803 0.649863,0.324803 0.649863,0.149863 0.999914,0.499914 0.649863,0.849795 0.649863,0.674855 0.649863,0.674855 0.649863,0.999914 0.349795,0.999914 0.349795,0.674855 0.349795,0.674855 0.349795,0.849795 -0.000086,0.499914 Z"#;
pub(super) const LEFT_RIGHT_ARROW_CALLOUT_ADJ_LONG_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L 0.249829,-0.000086 0.249829,0.399777 0.249829,0.399777 0.249829,-0.000086 0.749829,-0.000086 0.749829,0.399777 0.749829,0.399777 0.749829,-0.000086 0.999914,0.499914 0.749829,0.999914 0.749829,0.599880 0.749829,0.599880 0.749829,0.999914 0.249829,0.999914 0.249829,0.599880 0.249829,0.599880 0.249829,0.999914 -0.000086,0.499914 Z"#;
pub(super) const LEFT_RIGHT_ARROW_CALLOUT_ADJ_THICK_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L 0.449760,0.299812 0.449760,0.299812 0.449760,0.299812 0.449760,-0.000086 0.549897,-0.000086 0.549897,0.299812 0.549897,0.299812 0.549897,0.299812 0.999914,0.499914 0.549897,0.699846 0.549897,0.699846 0.549897,0.699846 0.549897,0.999914 0.449760,0.999914 0.449760,0.699846 0.449760,0.699846 0.449760,0.699846 -0.000086,0.499914 Z"#;
pub(super) fn left_right_arrow_callout_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(25_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(25_000.0);
    let adj3 = adj.get("adj3").copied().unwrap_or(25_000.0);
    let adj4 = adj.get("adj4").copied().unwrap_or(48_123.0);
    let anchors = [
        (
            15_000.0,
            15_000.0,
            15_000.0,
            15_000.0,
            LEFT_RIGHT_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (
            35_000.0,
            35_000.0,
            35_000.0,
            35_000.0,
            LEFT_RIGHT_ARROW_CALLOUT_ADJ_WIDE_NORMALIZED_PATH,
        ),
        (
            20_000.0,
            50_000.0,
            25_000.0,
            50_000.0,
            LEFT_RIGHT_ARROW_CALLOUT_ADJ_LONG_NORMALIZED_PATH,
        ),
        (
            45_000.0,
            20_000.0,
            45_000.0,
            20_000.0,
            LEFT_RIGHT_ARROW_CALLOUT_ADJ_THICK_NORMALIZED_PATH,
        ),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a2x, a3x, a4x, _), (a1y, a2y, a3y, a4y, _)| {
            let dx1 = (adj1 - *a1x) / 30_000.0;
            let dx2 = (adj2 - *a2x) / 35_000.0;
            let dx3 = (adj3 - *a3x) / 30_000.0;
            let dx4 = (adj4 - *a4x) / 35_000.0;
            let dy1 = (adj1 - *a1y) / 30_000.0;
            let dy2 = (adj2 - *a2y) / 35_000.0;
            let dy3 = (adj3 - *a3y) / 30_000.0;
            let dy4 = (adj4 - *a4y) / 35_000.0;
            (dx1 * dx1 + dx2 * dx2 + dx3 * dx3 + dx4 * dx4)
                .partial_cmp(&(dy1 * dy1 + dy2 * dy2 + dy3 * dy3 + dy4 * dy4))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, _, _, path)| path)
        .unwrap_or(LEFT_RIGHT_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH)
}
pub(super) fn left_right_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(LEFT_RIGHT_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH, w, h);
    }

    scale_normalized_path(left_right_arrow_callout_adjust_anchor(adj), w, h)
}
pub(super) const UP_DOWN_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.340580,0.255814 L 0.344203,0.238372 L 0.492754,0.000000 L 0.510870,0.000000 L 0.663043,0.244186 L 0.663043,0.255814 L 1.000000,0.255814 L 1.000000,0.750000 L 0.663043,0.750000 L 0.663043,0.761628 L 0.510870,1.000000 L 0.492754,1.000000 L 0.344203,0.767442 L 0.340580,0.750000 L 0.000000,0.750000 L 0.000000,0.255814 Z"#;
pub(super) const UP_DOWN_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000086,0.424769 L 0.424769,0.424769 0.424769,0.149863 0.349795,0.149863 0.499914,-0.000086 0.649863,0.149863 0.574889,0.149863 0.574889,0.424769 0.999914,0.424769 0.999914,0.574889 0.574889,0.574889 0.574889,0.849795 0.649863,0.849795 0.499914,0.999914 0.349795,0.849795 0.424769,0.849795 0.424769,0.574889 -0.000086,0.574889 -0.000086,0.424769 Z"#;
pub(super) const UP_DOWN_ARROW_CALLOUT_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000086,0.349795 L 0.324803,0.349795 0.324803,0.349795 0.149863,0.349795 0.499914,-0.000086 0.849795,0.349795 0.674855,0.349795 0.674855,0.349795 0.999914,0.349795 0.999914,0.649863 0.674855,0.649863 0.674855,0.649863 0.849795,0.649863 0.499914,0.999914 0.149863,0.649863 0.324803,0.649863 0.324803,0.649863 -0.000086,0.649863 -0.000086,0.349795 Z"#;
pub(super) const UP_DOWN_ARROW_CALLOUT_ADJ_LONG_NORMALIZED_PATH: &str = r#"M -0.000086,0.249829 L 0.399777,0.249829 0.399777,0.249829 -0.000086,0.249829 0.499914,-0.000086 0.999914,0.249829 0.599880,0.249829 0.599880,0.249829 0.999914,0.249829 0.999914,0.749829 0.599880,0.749829 0.599880,0.749829 0.999914,0.749829 0.499914,0.999914 -0.000086,0.749829 0.399777,0.749829 0.399777,0.749829 -0.000086,0.749829 -0.000086,0.249829 Z"#;
pub(super) const UP_DOWN_ARROW_CALLOUT_ADJ_THICK_NORMALIZED_PATH: &str = r#"M -0.000086,0.449760 L 0.299812,0.449760 0.299812,0.449760 0.299812,0.449760 0.499914,-0.000086 0.699846,0.449760 0.699846,0.449760 0.699846,0.449760 0.999914,0.449760 0.999914,0.549897 0.699846,0.549897 0.699846,0.549897 0.699846,0.549897 0.499914,0.999914 0.299812,0.549897 0.299812,0.549897 0.299812,0.549897 -0.000086,0.549897 -0.000086,0.449760 Z"#;
pub(super) fn up_down_arrow_callout_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(25_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(25_000.0);
    let adj3 = adj.get("adj3").copied().unwrap_or(25_000.0);
    let adj4 = adj.get("adj4").copied().unwrap_or(48_123.0);
    let anchors = [
        (
            15_000.0,
            15_000.0,
            15_000.0,
            15_000.0,
            UP_DOWN_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (
            35_000.0,
            35_000.0,
            35_000.0,
            35_000.0,
            UP_DOWN_ARROW_CALLOUT_ADJ_WIDE_NORMALIZED_PATH,
        ),
        (
            20_000.0,
            50_000.0,
            25_000.0,
            50_000.0,
            UP_DOWN_ARROW_CALLOUT_ADJ_LONG_NORMALIZED_PATH,
        ),
        (
            45_000.0,
            20_000.0,
            45_000.0,
            20_000.0,
            UP_DOWN_ARROW_CALLOUT_ADJ_THICK_NORMALIZED_PATH,
        ),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a2x, a3x, a4x, _), (a1y, a2y, a3y, a4y, _)| {
            let dx1 = (adj1 - *a1x) / 30_000.0;
            let dx2 = (adj2 - *a2x) / 35_000.0;
            let dx3 = (adj3 - *a3x) / 30_000.0;
            let dx4 = (adj4 - *a4x) / 35_000.0;
            let dy1 = (adj1 - *a1y) / 30_000.0;
            let dy2 = (adj2 - *a2y) / 35_000.0;
            let dy3 = (adj3 - *a3y) / 30_000.0;
            let dy4 = (adj4 - *a4y) / 35_000.0;
            (dx1 * dx1 + dx2 * dx2 + dx3 * dx3 + dx4 * dx4)
                .partial_cmp(&(dy1 * dy1 + dy2 * dy2 + dy3 * dy3 + dy4 * dy4))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, _, _, path)| path)
        .unwrap_or(UP_DOWN_ARROW_CALLOUT_ADJ_TIGHT_NORMALIZED_PATH)
}
pub(super) fn up_down_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(UP_DOWN_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH, w, h);
    }

    scale_normalized_path(up_down_arrow_callout_adjust_anchor(adj), w, h)
}
