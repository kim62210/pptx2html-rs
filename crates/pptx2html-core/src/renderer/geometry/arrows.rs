// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: arrows

use super::shared::scale_normalized_path;
use std::collections::HashMap;
pub(super) fn right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(33333.0);
    let s = h * a1 / 100_000.0 / 2.0;
    let hw = w * a2 / 100_000.0;
    let cy = h / 2.0;
    let (yt, yb, xh) = (cy - s, cy + s, w - hw);
    format!(
        "M0,{yt:.1} L{xh:.1},{yt:.1} L{xh:.1},0 L{w:.1},{cy:.1} L{xh:.1},{h:.1} L{xh:.1},{yb:.1} L0,{yb:.1} Z"
    )
}
pub(super) fn left_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(33333.0);
    let s = h * a1 / 100_000.0 / 2.0;
    let hw = w * a2 / 100_000.0;
    let cy = h / 2.0;
    let (yt, yb) = (cy - s, cy + s);
    format!(
        "M{w:.1},{yt:.1} L{hw:.1},{yt:.1} L{hw:.1},0 L0,{cy:.1} L{hw:.1},{h:.1} L{hw:.1},{yb:.1} L{w:.1},{yb:.1} Z"
    )
}
pub(super) fn up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(75000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let s = w * a1 / 100_000.0 / 2.0;
    let hh = h * a2 / 100_000.0;
    let cx = w / 2.0;
    let (xl, xr) = (cx - s, cx + s);
    format!(
        "M{xl:.1},{h:.1} L{xl:.1},{hh:.1} L0,{hh:.1} L{cx:.1},0 L{w:.1},{hh:.1} L{xr:.1},{hh:.1} L{xr:.1},{h:.1} Z"
    )
}
pub(super) fn down_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(75000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let s = w * a1 / 100_000.0 / 2.0;
    let hh = h * a2 / 100_000.0;
    let cx = w / 2.0;
    let (xl, xr, yh) = (cx - s, cx + s, h - hh);
    format!(
        "M{xl:.1},0 L{xr:.1},0 L{xr:.1},{yh:.1} L{w:.1},{yh:.1} L{cx:.1},{h:.1} L0,{yh:.1} L{xl:.1},{yh:.1} Z"
    )
}
pub(super) fn left_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let s = h * a1 / 100_000.0 / 2.0;
    let hw = w * a2 / 100_000.0;
    let cy = h / 2.0;
    let (yt, yb, xr) = (cy - s, cy + s, w - hw);
    format!(
        "M0,{cy:.1} L{hw:.1},0 L{hw:.1},{yt:.1} L{xr:.1},{yt:.1} L{xr:.1},0 L{w:.1},{cy:.1} L{xr:.1},{h:.1} L{xr:.1},{yb:.1} L{hw:.1},{yb:.1} L{hw:.1},{h:.1} Z"
    )
}
pub(super) fn up_down_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let s = w * a1 / 100_000.0 / 2.0;
    let hh = h * a2 / 100_000.0;
    let cx = w / 2.0;
    let (xl, xr, yb) = (cx - s, cx + s, h - hh);
    format!(
        "M{cx:.1},0 L{w:.1},{hh:.1} L{xr:.1},{hh:.1} L{xr:.1},{yb:.1} L{w:.1},{yb:.1} L{cx:.1},{h:.1} L0,{yb:.1} L{xl:.1},{yb:.1} L{xl:.1},{hh:.1} L0,{hh:.1} Z"
    )
}
pub(super) const BENT_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000086,0.999914 L -0.000086,0.424769 -0.000086,0.424769 C -0.000086,0.363317 0.016005,0.302893 0.046816,0.249829 0.077456,0.196594 0.121790,0.152431 0.174855,0.121619 0.228090,0.090979 0.288514,0.074718 0.349966,0.074718 L 0.849795,0.074889 0.849795,-0.000086 0.999914,0.149863 0.849795,0.299812 0.849795,0.224837 0.349795,0.224837 0.349795,0.224837 C 0.314704,0.224837 0.280127,0.234081 0.249829,0.251712 0.219360,0.269172 0.194197,0.294505 0.176566,0.324803 0.159106,0.355272 0.149863,0.389678 0.149863,0.424769 L 0.149863,0.999914 -0.000086,0.999914 Z"#;
pub(super) const BENT_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000086,0.999914 L -0.000086,0.674855 -0.000086,0.674855 C -0.000086,0.587042 0.023023,0.500941 0.066844,0.424940 0.110835,0.348768 0.173827,0.285775 0.250000,0.241784 0.326001,0.197963 0.412102,0.174855 0.499914,0.174855 L 0.649863,0.174855 0.649863,-0.000086 0.999914,0.349795 0.649863,0.699846 0.649863,0.524906 0.499914,0.524906 0.499914,0.524906 C 0.473554,0.524906 0.447706,0.531753 0.424940,0.544933 0.402174,0.558114 0.383174,0.577114 0.369993,0.599880 0.356813,0.622646 0.349966,0.648494 0.349966,0.674855 L 0.349795,0.999914 -0.000086,0.999914 Z"#;
pub(super) const BENT_ARROW_ADJ_TALL_NORMALIZED_PATH: &str = r#"M -0.000086,0.999914 L -0.000086,0.599880 -0.000086,0.599880 C -0.000086,0.512068 0.023023,0.425967 0.066844,0.349966 0.110835,0.273793 0.173827,0.210801 0.249829,0.166809 0.326001,0.122989 0.412102,0.099880 0.499914,0.099880 L 0.499914,0.099880 0.499914,-0.000086 0.999914,0.199846 0.499914,0.399777 0.499914,0.299812 0.499914,0.299812 0.499914,0.299812 C 0.447193,0.299812 0.395498,0.313677 0.349966,0.340038 0.304262,0.366398 0.266433,0.404228 0.240072,0.449760 0.213711,0.495464 0.199846,0.547159 0.199846,0.599880 L 0.199846,0.999914 -0.000086,0.999914 Z"#;
pub(super) const BENT_ARROW_ADJ_THICK_NORMALIZED_PATH: &str = r#"M -0.000086,0.999914 L -0.000086,0.249829 -0.000086,0.249829 C -0.000086,0.206008 0.011383,0.162872 0.033465,0.124872 0.055375,0.086871 0.086871,0.055204 0.124872,0.033293 0.162872,0.011383 0.206008,-0.000086 0.250000,-0.000086 L 0.749829,-0.000086 0.749829,-0.000086 0.999914,0.149863 0.749829,0.299812 0.749829,0.299812 0.299812,0.299812 0.299812,0.299812 0.299812,0.999914 -0.000086,0.999914 Z"#;
pub(super) fn bent_arrow_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(25_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(25_000.0);
    let adj3 = adj.get("adj3").copied().unwrap_or(25_000.0);
    let adj4 = adj.get("adj4").copied().unwrap_or(43_750.0);
    let anchors = [
        (
            15_000.0,
            15_000.0,
            15_000.0,
            35_000.0,
            BENT_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (
            35_000.0,
            35_000.0,
            35_000.0,
            50_000.0,
            BENT_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        ),
        (
            20_000.0,
            20_000.0,
            50_000.0,
            65_000.0,
            BENT_ARROW_ADJ_TALL_NORMALIZED_PATH,
        ),
        (
            45_000.0,
            15_000.0,
            25_000.0,
            25_000.0,
            BENT_ARROW_ADJ_THICK_NORMALIZED_PATH,
        ),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a2x, a3x, a4x, _), (a1y, a2y, a3y, a4y, _)| {
            let dx1 = (adj1 - *a1x) / 30_000.0;
            let dx2 = (adj2 - *a2x) / 20_000.0;
            let dx3 = (adj3 - *a3x) / 35_000.0;
            let dx4 = (adj4 - *a4x) / 40_000.0;
            let dy1 = (adj1 - *a1y) / 30_000.0;
            let dy2 = (adj2 - *a2y) / 20_000.0;
            let dy3 = (adj3 - *a3y) / 35_000.0;
            let dy4 = (adj4 - *a4y) / 40_000.0;
            (dx1 * dx1 + dx2 * dx2 + dx3 * dx3 + dx4 * dx4)
                .partial_cmp(&(dy1 * dy1 + dy2 * dy2 + dy3 * dy3 + dy4 * dy4))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, _, _, path)| path)
        .unwrap_or(BENT_ARROW_ADJ_TIGHT_NORMALIZED_PATH)
}
pub(super) fn bent_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    scale_normalized_path(bent_arrow_adjust_anchor(adj), w, h)
}
pub(super) fn chevron_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let p = w * adj.get("adj").copied().unwrap_or(50000.0) / 100_000.0;
    let cy = h / 2.0;
    let x1 = w - p;
    format!("M0,0 L{x1:.1},0 L{w:.1},{cy:.1} L{x1:.1},{h:.1} L0,{h:.1} L{p:.1},{cy:.1} Z")
}
pub(super) const NOTCHED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000086,0.424769 L 0.849795,0.424769 0.849795,-0.000086 0.999914,0.499914 0.849795,0.999914 0.849795,0.574889 -0.000086,0.574889 0.022338,0.499914 -0.000086,0.424769 Z"#;
pub(super) const NOTCHED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000086,0.324803 L 0.649863,0.324803 0.649863,-0.000086 0.999914,0.499914 0.649863,0.999914 0.649863,0.674855 -0.000086,0.674855 0.122304,0.499914 -0.000086,0.324803 Z"#;
pub(super) const NOTCHED_RIGHT_ARROW_ADJ_LONG_NORMALIZED_PATH: &str = r#"M -0.000086,0.399777 L 0.499914,0.399777 0.499914,-0.000086 0.999914,0.499914 0.499914,0.999914 0.499914,0.599880 -0.000086,0.599880 0.099880,0.499914 -0.000086,0.399777 Z"#;
pub(super) const NOTCHED_RIGHT_ARROW_ADJ_THICK_NORMALIZED_PATH: &str = r#"M -0.000086,0.274820 L 0.799812,0.274820 0.799812,-0.000086 0.999914,0.499914 0.799812,0.999914 0.799812,0.724837 -0.000086,0.724837 0.089781,0.499914 -0.000086,0.274820 Z"#;
pub(super) fn notched_right_arrow_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(50_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(33_333.0);
    let anchors = [
        (
            15_000.0,
            15_000.0,
            NOTCHED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (
            35_000.0,
            35_000.0,
            NOTCHED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        ),
        (
            20_000.0,
            50_000.0,
            NOTCHED_RIGHT_ARROW_ADJ_LONG_NORMALIZED_PATH,
        ),
        (
            45_000.0,
            20_000.0,
            NOTCHED_RIGHT_ARROW_ADJ_THICK_NORMALIZED_PATH,
        ),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a2x, _), (a1y, a2y, _)| {
            let dx1 = (adj1 - *a1x) / 35_000.0;
            let dx2 = (adj2 - *a2x) / 35_000.0;
            let dy1 = (adj1 - *a1y) / 35_000.0;
            let dy2 = (adj2 - *a2y) / 35_000.0;
            (dx1 * dx1 + dx2 * dx2)
                .partial_cmp(&(dy1 * dy1 + dy2 * dy2))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, path)| path)
        .unwrap_or(NOTCHED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH)
}
pub(super) fn notched_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
        let a2 = adj.get("adj2").copied().unwrap_or(33333.0);
        let s = h * a1 / 100_000.0 / 2.0;
        let hw = w * a2 / 100_000.0;
        let cy = h / 2.0;
        let (yt, yb, xh) = (cy - s, cy + s, w - hw);
        let n = hw * 0.5;
        return format!(
            "M0,{yt:.1} L{xh:.1},{yt:.1} L{xh:.1},0 L{w:.1},{cy:.1} L{xh:.1},{h:.1} L{xh:.1},{yb:.1} L0,{yb:.1} L{n:.1},{cy:.1} Z"
        );
    }

    scale_normalized_path(notched_right_arrow_adjust_anchor(adj), w, h)
}
pub(super) const STRIPED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.000000,0.424972 L 0.031046,0.424972 0.031046,0.574803 0.000000,0.574803 0.000000,0.424972 Z M 0.062317,0.424972 L 0.124859,0.424972 0.124859,0.574803 0.062317,0.574803 0.062317,0.424972 Z M 0.156130,0.424972 L 0.849944,0.424972 0.849944,0.000000 1.000000,0.499888 0.849944,1.000000 0.849944,0.574803 0.156130,0.574803 0.156130,0.424972 Z"#;
pub(super) const STRIPED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M 0.000000,0.324859 L 0.031046,0.324859 0.031046,0.674916 0.000000,0.674916 0.000000,0.324859 Z M 0.062317,0.324859 L 0.124859,0.324859 0.124859,0.674916 0.062317,0.674916 0.062317,0.324859 Z M 0.156130,0.324859 L 0.649944,0.324859 0.649944,0.000000 1.000000,0.499888 0.649944,1.000000 0.649944,0.674916 0.156130,0.674916 0.156130,0.324859 Z"#;
pub(super) const STRIPED_RIGHT_ARROW_ADJ_LONG_NORMALIZED_PATH: &str = r#"M 0.000000,0.400000 L 0.031046,0.400000 0.031046,0.600000 0.000000,0.600000 0.000000,0.400000 Z M 0.062317,0.400000 L 0.124859,0.400000 0.124859,0.600000 0.062317,0.600000 0.062317,0.400000 Z M 0.156130,0.400000 L 0.499888,0.400000 0.499888,0.000000 1.000000,0.499888 0.499888,1.000000 0.499888,0.600000 0.156130,0.600000 0.156130,0.400000 Z"#;
pub(super) const STRIPED_RIGHT_ARROW_ADJ_THICK_NORMALIZED_PATH: &str = r#"M 0.000000,0.274916 L 0.031046,0.274916 0.031046,0.724859 0.000000,0.724859 0.000000,0.274916 Z M 0.062317,0.274916 L 0.124859,0.274916 0.124859,0.724859 0.062317,0.724859 0.062317,0.274916 Z M 0.156130,0.274916 L 0.800000,0.274916 0.800000,0.000000 1.000000,0.499888 0.800000,1.000000 0.800000,0.724859 0.156130,0.724859 0.156130,0.274916 Z"#;
pub(super) fn striped_right_arrow_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(50_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(33_333.0);
    let anchors = [
        (
            15_000.0,
            15_000.0,
            STRIPED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (
            35_000.0,
            35_000.0,
            STRIPED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        ),
        (
            20_000.0,
            50_000.0,
            STRIPED_RIGHT_ARROW_ADJ_LONG_NORMALIZED_PATH,
        ),
        (
            45_000.0,
            20_000.0,
            STRIPED_RIGHT_ARROW_ADJ_THICK_NORMALIZED_PATH,
        ),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a2x, _), (a1y, a2y, _)| {
            let dx1 = (adj1 - *a1x) / 35_000.0;
            let dx2 = (adj2 - *a2x) / 35_000.0;
            let dy1 = (adj1 - *a1y) / 35_000.0;
            let dy2 = (adj2 - *a2y) / 35_000.0;
            (dx1 * dx1 + dx2 * dx2)
                .partial_cmp(&(dy1 * dy1 + dy2 * dy2))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, path)| path)
        .unwrap_or(STRIPED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH)
}
pub(super) fn striped_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if !adj.is_empty() {
        return scale_normalized_path(striped_right_arrow_adjust_anchor(adj), w, h);
    }

    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(33333.0);
    let s = h * a1 / 100_000.0 / 2.0;
    let hw = w * a2 / 100_000.0;
    let cy = h / 2.0;
    let (yt, yb, xh) = (cy - s, cy + s, w - hw);
    let (s1, s2, s3, s4) = (w * 0.025, w * 0.05, w * 0.075, w * 0.1);
    format!(
        "M0,{yt:.1} L{s1:.1},{yt:.1} L{s1:.1},{yb:.1} L0,{yb:.1} Z M{s2:.1},{yt:.1} L{s3:.1},{yt:.1} L{s3:.1},{yb:.1} L{s2:.1},{yb:.1} Z M{s4:.1},{yt:.1} L{xh:.1},{yt:.1} L{xh:.1},0 L{w:.1},{cy:.1} L{xh:.1},{h:.1} L{xh:.1},{yb:.1} L{s4:.1},{yb:.1} Z"
    )
}
pub(super) fn home_plate_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    // ECMA-376 homePlate: adj default = 50000 (50% of width for arrow point)
    let a = adj.get("adj").copied().unwrap_or(50000.0);
    let p = w * a / 100_000.0;
    let cy = h / 2.0;
    format!(
        "M0,0 L{x:.1},0 L{w:.1},{cy:.1} L{x:.1},{h:.1} L0,{h:.1} Z",
        x = w - p,
        w = w,
        cy = cy,
        h = h
    )
}
pub(super) fn swoosh_arrow_path(w: f64, h: f64) -> String {
    scale_normalized_path(
        "M 0.036738,0.972701 L 0.028315,0.959195 0.065376,0.834943 0.139498,0.672874 0.245627,0.518908 0.350072,0.410862 0.589283,0.248793 0.862186,0.146149 0.862186,0.027299 0.971685,0.213678 0.895878,0.508103 0.884086,0.402759 0.858817,0.394655 0.676882,0.432471 0.447778,0.524310 0.323118,0.605345 0.191720,0.729598 Z",
        w,
        h,
    )
}
