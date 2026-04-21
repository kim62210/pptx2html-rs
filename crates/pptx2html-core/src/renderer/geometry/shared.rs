// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: shared

use crate::model::PathFill;
use std::collections::HashMap;
pub(super) type Point = (f64, f64);
pub(super) const CURVED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000112,0.344994 L -0.000112,0.344994 C -0.000112,0.405512 0.046007,0.465129 0.133971,0.517548 0.221710,0.569966 0.347919,0.613386 0.500000,0.643757 0.598988,0.663555 0.707199,0.677278 0.819910,0.684252 L 0.819685,0.494376 0.999888,0.749944 0.819685,0.994263 0.819685,0.804387 0.819685,0.804387 C 0.706974,0.797188 0.598763,0.783465 0.499775,0.763892 0.347694,0.733521 0.221485,0.689876 0.133746,0.637458 0.045782,0.585039 -0.000337,0.525647 -0.000337,0.465129 L -0.000112,0.344994 Z"#;
pub(super) const CURVED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000112,0.349944 L -0.000112,0.349944 C -0.000112,0.411361 0.046007,0.471654 0.133971,0.524972 0.221710,0.578065 0.347919,0.622385 0.500000,0.652981 0.525872,0.658380 0.552643,0.663105 0.579865,0.667604 L 0.579865,0.667604 0.999888,0.850056 0.579865,0.967717 0.579865,0.967717 0.579865,0.967717 C 0.552643,0.963217 0.525872,0.958493 0.499775,0.953093 0.347919,0.922497 0.221710,0.878178 0.133746,0.825084 0.046007,0.771766 -0.000112,0.711474 -0.000112,0.650056 L -0.000112,0.349944 Z"#;
pub(super) const CURVED_LEFT_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.000112,0.749944 L 0.180090,0.494376 0.180090,0.684477 0.180090,0.684477 C 0.292801,0.677278 0.401012,0.663555 0.500000,0.643982 0.652081,0.613611 0.778290,0.569966 0.866029,0.517548 0.924297,0.482677 0.964567,0.444657 0.984814,0.405062 L 0.984814,0.405062 C 0.994938,0.424859 1.000112,0.445107 1.000112,0.465129 1.000112,0.525647 0.953993,0.585264 0.866029,0.637683 0.778290,0.690101 0.652081,0.733521 0.500000,0.763892 0.401012,0.783690 0.292801,0.797413 0.180090,0.804387 L 0.180090,0.994263 0.000112,0.749944 Z"#;
pub(super) const CURVED_LEFT_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000112,0.850056 L 0.419685,0.667604 0.419685,0.667604 0.419685,0.667604 C 0.446907,0.663105 0.473678,0.658380 0.499775,0.652981 0.651631,0.622385 0.777840,0.578065 0.865804,0.524972 0.879078,0.516873 0.891676,0.508549 0.903150,0.500000 L 0.903150,0.500000 C 0.966817,0.546794 0.999663,0.598088 0.999663,0.650056 0.999663,0.711474 0.953543,0.771766 0.865804,0.824859 0.777840,0.878178 0.651631,0.922272 0.499775,0.953093 0.473678,0.958268 0.446907,0.963217 0.419685,0.967717 L 0.419685,0.967717 -0.000112,0.850056 Z"#;
pub(super) const CURVED_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.749719,-0.000112 L 0.994038,0.179865 0.804162,0.179865 0.804162,0.179865 C 0.796963,0.292576 0.783240,0.400787 0.763667,0.499775 0.733296,0.651856 0.689651,0.778065 0.637233,0.865804 0.584814,0.953768 0.525422,0.999888 0.464904,0.999888 0.444657,0.999888 0.424634,0.994713 0.404837,0.984589 L 0.404837,0.984589 C 0.444432,0.964342 0.482452,0.924072 0.517323,0.865804 0.569741,0.778065 0.613386,0.651856 0.643532,0.499775 0.663330,0.400787 0.677053,0.292576 0.684252,0.179865 L 0.494151,0.179865 0.749719,-0.000112 Z"#;
pub(super) const CURVED_UP_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M 0.849831,-0.000112 L 0.970191,0.419685 1.030259,0.419685 1.030259,0.419685 C 1.026209,0.446907 1.021710,0.473678 1.016985,0.499775 0.988864,0.651631 0.948594,0.777840 0.899775,0.865804 0.851181,0.953543 0.796063,0.999663 0.739820,0.999663 0.683577,0.999663 0.628459,0.953543 0.579865,0.865804 0.562092,0.833633 0.545444,0.796288 0.529921,0.754218 L 0.529921,0.754218 C 0.556693,0.681552 0.579190,0.595613 0.596963,0.499775 0.601687,0.473678 0.606187,0.446907 0.610236,0.419685 L 0.670079,0.419685 0.849831,-0.000112 Z"#;
pub(super) const CURVED_DOWN_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.749944,0.999888 L 0.494376,0.819685 0.684477,0.819685 0.684477,0.819685 C 0.677278,0.706974 0.663555,0.598763 0.643982,0.499775 0.613611,0.347694 0.569966,0.221485 0.517548,0.133746 0.465129,0.045782 0.405737,-0.000337 0.345219,-0.000337 L 0.464904,-0.000112 0.464904,-0.000112 C 0.525422,-0.000112 0.585039,0.046007 0.637458,0.133971 0.689876,0.221710 0.733296,0.347919 0.763667,0.499775 0.783465,0.598988 0.797188,0.707199 0.804162,0.819910 L 0.994263,0.819685 0.749944,0.999888 Z"#;
pub(super) const CURVED_DOWN_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M 0.849831,0.999888 L 0.670079,0.579865 0.610236,0.579865 0.610236,0.579865 C 0.606187,0.552643 0.601687,0.525872 0.596963,0.499775 0.568841,0.347919 0.528571,0.221710 0.479753,0.133746 0.431159,0.046007 0.376040,-0.000112 0.319798,-0.000112 L 0.739820,-0.000112 0.739820,-0.000112 C 0.796063,-0.000112 0.851181,0.046007 0.899775,0.133971 0.948369,0.221710 0.988864,0.347919 1.016985,0.499775 1.021710,0.525872 1.026209,0.552643 1.030259,0.579865 L 0.970191,0.579865 0.849831,0.999888 Z"#;
pub(super) fn curved_arrow_adjust_profile(adj: &HashMap<String, f64>) -> f64 {
    let adj1 = adj.get("adj1").copied().unwrap_or(25_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50_000.0);
    let adj3 = adj.get("adj3").copied().unwrap_or(25_000.0);
    let t1 = ((adj1 - 12_000.0) / 30_000.0).clamp(0.0, 1.0);
    let t2 = ((70_000.0 - adj2) / 40_000.0).clamp(0.0, 1.0);
    let t3 = ((adj3 - 18_000.0) / 24_000.0).clamp(0.0, 1.0);
    (t1 + t2 + t3) / 3.0
}
pub(super) fn interpolate_normalized_paths(
    start_path: &str,
    end_path: &str,
    t: f64,
    w: f64,
    h: f64,
) -> String {
    if t <= 0.0 {
        return scale_normalized_path(start_path, w, h);
    }
    if t >= 1.0 {
        return scale_normalized_path(end_path, w, h);
    }

    let start_tokens: Vec<_> = start_path.split_whitespace().collect();
    let end_tokens: Vec<_> = end_path.split_whitespace().collect();
    if start_tokens.len() != end_tokens.len() {
        return scale_normalized_path(start_path, w, h);
    }

    start_tokens
        .iter()
        .zip(end_tokens.iter())
        .map(|(start_token, end_token)| {
            match (start_token.split_once(','), end_token.split_once(',')) {
                (Some((sx, sy)), Some((ex, ey))) => {
                    let sx = sx.parse::<f64>().unwrap_or_default();
                    let sy = sy.parse::<f64>().unwrap_or_default();
                    let ex = ex.parse::<f64>().unwrap_or_default();
                    let ey = ey.parse::<f64>().unwrap_or_default();
                    let x = (sx + (ex - sx) * t) * w;
                    let y = (sy + (ey - sy) * t) * h;
                    format!("{x:.1},{y:.1}")
                }
                _ => (*start_token).to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
pub(super) const CURVED_LEFT_ARROW_MULTI_TIGHT_MAIN_PATH: &str = r#"M0.000000,0.750000 L0.180000,0.494365 L0.180000,0.684365 A1.000000,0.345000 0 0,0 0.709578,0.400565 A1.000000,0.345000 0 0,1 0.180000,0.726328 L0.180000,0.994365 Z"#;
pub(super) const CURVED_LEFT_ARROW_MULTI_TIGHT_SHADE_PATH: &str = r#"M1.000000,0.465000 A1.000000,0.345000 0 0,0 0.000000,0.120000 L0.000000,0.000000 A1.000000,0.345000 0 0,1 1.000000,0.345000 Z"#;
pub(super) const CURVED_LEFT_ARROW_MULTI_TIGHT_OUTLINE_PATH: &str = r#"M1.000000,0.465000 A1.000000,0.345000 0 0,0 0.000000,0.120000 L0.000000,0.000000 A1.000000,0.345000 0 0,1 1.000000,0.345000 L1.000000,0.465000 A1.000000,0.345000 0 0,1 0.468571,0.769782 L0.180000,0.994365 L0.000000,0.750000 L0.180000,0.494365 L0.180000,0.684365 A1.000000,0.345000 0 0,0 0.709578,0.400565"#;
pub(super) const CURVED_LEFT_ARROW_MULTI_WIDE_MAIN_PATH: &str = r#"M0.000000,0.850000 L0.420000,0.667633 L0.420000,0.667633 A1.000000,0.350000 0 0,0 0.608904,0.513837 A1.000000,0.350000 0 0,1 0.420000,0.782278 L0.420000,0.967633 Z"#;
pub(super) const CURVED_LEFT_ARROW_MULTI_WIDE_SHADE_PATH: &str = r#"M1.000000,0.650000 A1.000000,0.350000 0 0,0 0.000000,0.300000 L0.000000,0.000000 A1.000000,0.350000 0 0,1 1.000000,0.350000 Z"#;
pub(super) const CURVED_LEFT_ARROW_MULTI_WIDE_OUTLINE_PATH: &str = r#"M1.000000,0.650000 A1.000000,0.350000 0 0,0 0.000000,0.300000 L0.000000,0.000000 A1.000000,0.350000 0 0,1 1.000000,0.350000 L1.000000,0.650000 A1.000000,0.350000 0 0,1 0.797593,0.861119 L0.420000,0.967633 L0.000000,0.850000 L0.420000,0.667633 L0.420000,0.667633 A1.000000,0.350000 0 0,0 0.608904,0.513837"#;
pub(super) const CURVED_UP_ARROW_MULTI_TIGHT_MAIN_PATH: &str = r#"M0.750000,0.000000 L0.994365,0.180000 L0.804365,0.180000 A0.345000,1.000000 0 0,1 0.478602,0.709578 A0.345000,1.000000 0 0,0 0.762402,0.180000 L0.494365,0.180000 Z"#;
pub(super) const CURVED_UP_ARROW_MULTI_TIGHT_SHADE_PATH: &str = r#"M0.345000,1.000000 A0.345000,1.000000 0 0,1 0.000000,0.000000 L0.120000,0.000000 A0.345000,1.000000 0 0,0 0.465000,1.000000 Z"#;
pub(super) const CURVED_UP_ARROW_MULTI_TIGHT_OUTLINE_PATH: &str = r#"M0.405000,0.984761 A0.345000,1.000000 0 0,0 0.688800,0.455183 L0.494365,0.180000 L0.750000,0.000000 L0.994365,0.180000 L0.804365,0.180000 A0.345000,1.000000 0 0,1 0.499583,0.711429 L0.345000,1.000000 A0.345000,1.000000 0 0,1 0.000000,0.000000 L0.120000,0.000000 A0.345000,1.000000 0 0,0 0.465000,1.000000"#;
pub(super) const CURVED_UP_ARROW_MULTI_WIDE_MAIN_PATH: &str = r#"M0.850000,0.000000 L0.970408,0.420000 L1.030408,0.420000 A0.320000,1.000000 0 0,1 0.762614,0.560861 A0.320000,1.000000 0 0,0 0.858809,0.420000 L0.670408,0.420000 Z"#;
pub(super) const CURVED_UP_ARROW_MULTI_WIDE_SHADE_PATH: &str = r#"M0.320000,1.000000 A0.320000,1.000000 0 0,1 0.000000,0.000000 L0.420000,0.000000 A0.320000,1.000000 0 0,0 0.740000,1.000000 Z"#;
pub(super) const CURVED_UP_ARROW_MULTI_WIDE_OUTLINE_PATH: &str = r#"M0.530000,0.754544 A0.320000,1.000000 0 0,0 0.626195,0.613682 L0.670408,0.420000 L0.850000,0.000000 L0.970408,0.420000 L1.030408,0.420000 A0.320000,1.000000 0 0,1 0.848414,0.597477 L0.320000,1.000000 A0.320000,1.000000 0 0,1 0.000000,0.000000 L0.420000,0.000000 A0.320000,1.000000 0 0,0 0.740000,1.000000"#;
pub(super) const CURVED_DOWN_ARROW_MULTI_TIGHT_MAIN_PATH: &str = r#"M0.750000,1.000000 L0.494365,0.820000 L0.684365,0.820000 A0.345000,1.000000 0 0,0 0.379583,0.288571 L0.465000,0.000000 A0.345000,1.000000 0 0,1 0.769782,0.531429 L0.994365,0.820000 Z"#;
pub(super) const CURVED_DOWN_ARROW_MULTI_TIGHT_SHADE_PATH: &str = r#"M0.405000,0.015239 A0.345000,1.000000 0 0,0 0.080981,1.013388 L0.000000,1.000000 A0.345000,1.000000 0 0,1 0.365981,0.001851 Z"#;
pub(super) const CURVED_DOWN_ARROW_MULTI_TIGHT_OUTLINE_PATH: &str = r#"M0.405000,0.015239 A0.345000,1.000000 0 0,0 0.080981,1.013388 L0.000000,1.000000 A0.345000,1.000000 0 0,1 0.345000,-0.000000 L0.465000,0.000000 A0.345000,1.000000 0 0,1 0.769782,0.531429 L0.994365,0.820000 L0.750000,1.000000 L0.494365,0.820000 L0.684365,0.820000 A0.345000,1.000000 0 0,0 0.379583,0.288571"#;
pub(super) const CURVED_DOWN_ARROW_MULTI_WIDE_MAIN_PATH: &str = r#"M0.850000,1.000000 L0.670408,0.580000 L0.610408,0.580000 A0.320000,1.000000 0 0,0 0.428414,0.402523 L0.740000,0.000000 A0.320000,1.000000 0 0,1 0.921994,0.177477 L0.970408,0.580000 Z"#;
pub(super) const CURVED_DOWN_ARROW_MULTI_WIDE_SHADE_PATH: &str = r#"M0.530000,0.245456 A0.320000,1.000000 0 0,0 0.295799,1.208841 L0.000000,1.000000 A0.320000,1.000000 0 0,1 0.405799,0.036615 Z"#;
pub(super) const CURVED_DOWN_ARROW_MULTI_WIDE_OUTLINE_PATH: &str = r#"M0.530000,0.245456 A0.320000,1.000000 0 0,0 0.295799,1.208841 L0.000000,1.000000 A0.320000,1.000000 0 0,1 0.320000,-0.000000 L0.740000,0.000000 A0.320000,1.000000 0 0,1 0.921994,0.177477 L0.970408,0.580000 L0.850000,1.000000 L0.670408,0.580000 L0.610408,0.580000 A0.320000,1.000000 0 0,0 0.428414,0.402523"#;
pub(super) fn scale_normalized_svg_d(path: &str, w: f64, h: f64) -> String {
    let tokens: Vec<&str> = path.split_whitespace().collect();
    let mut out: Vec<String> = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        if token == "Z" {
            out.push("Z".to_string());
            i += 1;
            continue;
        }
        if let Some(pair) = token.strip_prefix('M').or_else(|| token.strip_prefix('L')) {
            let (x, y) = pair.split_once(',').unwrap_or(("0", "0"));
            let x = x.parse::<f64>().unwrap_or_default() * w;
            let y = y.parse::<f64>().unwrap_or_default() * h;
            out.push(format!("{}{:0.2},{:0.2}", &token[..1], x, y));
            i += 1;
            continue;
        }
        if let Some(pair) = token.strip_prefix('A') {
            let (rx, ry) = pair.split_once(',').unwrap_or(("0", "0"));
            let rx = rx.parse::<f64>().unwrap_or_default() * w;
            let ry = ry.parse::<f64>().unwrap_or_default() * h;
            let rot = tokens.get(i + 1).copied().unwrap_or("0");
            let flags = tokens.get(i + 2).copied().unwrap_or("0,0");
            let end = tokens.get(i + 3).copied().unwrap_or("0,0");
            let (x, y) = end.split_once(',').unwrap_or(("0", "0"));
            let x = x.parse::<f64>().unwrap_or_default() * w;
            let y = y.parse::<f64>().unwrap_or_default() * h;
            out.push(format!("A{rx:.2},{ry:.2} {rot} {flags} {x:.2},{y:.2}"));
            i += 4;
            continue;
        }
        out.push(token.to_string());
        i += 1;
    }
    out.join(" ")
}
pub(super) fn matches_curved_arrow_profile(
    adj: &HashMap<String, f64>,
    a1: f64,
    a2: f64,
    a3: f64,
) -> bool {
    (adj.get("adj1").copied().unwrap_or(25_000.0) - a1).abs() < 0.5
        && (adj.get("adj2").copied().unwrap_or(50_000.0) - a2).abs() < 0.5
        && (adj.get("adj3").copied().unwrap_or(25_000.0) - a3).abs() < 0.5
}
pub(super) fn curved_arrow_multi_svg(
    paths: [(&str, PathFill, bool); 3],
    w: f64,
    h: f64,
) -> CustomGeomSvg {
    CustomGeomSvg {
        paths: paths
            .into_iter()
            .map(|(d, fill, stroke)| CustomGeomPathSvg {
                d: scale_normalized_svg_d(d, w, h),
                fill,
                stroke,
            })
            .collect(),
    }
}
pub(super) fn polygon_path(points: &[Point]) -> String {
    let mut iter = points.iter();
    let Some(&(x0, y0)) = iter.next() else {
        return String::new();
    };

    let mut path = format!("M{x0:.1},{y0:.1}");
    for &(x, y) in iter {
        path.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    path.push_str(" Z");
    path
}
pub(super) fn ellipse_point(cx: f64, cy: f64, rx: f64, ry: f64, angle: f64) -> (f64, f64) {
    (cx + rx * angle.cos(), cy + ry * angle.sin())
}
pub(super) fn scale_unit_point(w: f64, h: f64, ux: f64, uy: f64) -> Point {
    (w * ux, h * uy)
}
pub(super) fn scale_normalized_path(path: &str, w: f64, h: f64) -> String {
    path.split_whitespace()
        .map(|token| {
            if token.len() == 1 && token.chars().all(|c| c.is_ascii_alphabetic()) {
                token.to_string()
            } else if let Some((x, y)) = token.split_once(',') {
                let x = x.parse::<f64>().unwrap_or_default() * w;
                let y = y.parse::<f64>().unwrap_or_default() * h;
                format!("{x:.1},{y:.1}")
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
pub struct CustomGeomSvg {
    pub paths: Vec<CustomGeomPathSvg>,
}
pub struct CustomGeomPathSvg {
    pub d: String,
    pub fill: PathFill,
    pub stroke: bool,
}
