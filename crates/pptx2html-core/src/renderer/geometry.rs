//! Preset shape SVG path generation for all 187 OOXML preset geometries.
//! Generates SVG `<path>` elements parameterized by width, height, and adjust values.
//! Covers flowchart, action buttons, stars, callouts, math shapes, arrow callouts,
//! brackets/braces, chart shapes, scrolls, tabs, ribbons, circular arrows, and more
//! per ECMA-376 Part 1 section 20.1.10.

use std::collections::HashMap;

type Point = (f64, f64);
type PolylineSides = (Vec<Point>, Vec<Point>);

/// Returns true if this preset shape needs `fill-rule="evenodd"` to render holes correctly.
/// Shapes with inner cutouts (donut, frame, noSmoking, blockArc, etc.) use two subpaths
/// where the inner subpath winds in the opposite direction to create a hole.
pub fn needs_evenodd_fill(name: &str) -> bool {
    matches!(
        name,
        "donut"
            | "frame"
            | "noSmoking"
            | "blockArc"
            | "bracePair"
            | "bracketPair"
            | "bevel"
            | "can"
    )
}

/// Generate an SVG path string for a named preset shape.
/// Returns `None` if the shape name is not supported.
pub fn preset_shape_svg(
    name: &str,
    w: f64,
    h: f64,
    adjust_values: &HashMap<String, f64>,
) -> Option<String> {
    match name {
        // Basic shapes
        "rect" => Some(rect_path(w, h)),
        "roundRect" => Some(round_rect_path(w, h, adjust_values)),
        "ellipse" => Some(ellipse_path(w, h)),
        "triangle" | "isosTriangle" => Some(triangle_path(w, h)),
        "rtTriangle" => Some(rt_triangle_path(w, h)),
        "diamond" => Some(diamond_path(w, h)),
        "parallelogram" => Some(parallelogram_path(w, h, adjust_values)),
        "trapezoid" => Some(trapezoid_path(w, h, adjust_values)),
        "pentagon" => Some(pentagon_path(w, h)),
        "hexagon" => Some(hexagon_path(w, h, adjust_values)),
        "octagon" => Some(octagon_path(w, h, adjust_values)),
        // Snip/Fold/Round corner rectangles
        "snip1Rect" => Some(snip1_rect_path(w, h, adjust_values)),
        "snip2SameRect" => Some(snip2_same_rect_path(w, h, adjust_values)),
        "snip2DiagRect" => Some(snip2_diag_rect_path(w, h, adjust_values)),
        "snipRoundRect" => Some(snip_round_rect_path(w, h, adjust_values)),
        "round1Rect" => Some(round1_rect_path(w, h, adjust_values)),
        "round2SameRect" => Some(round2_same_rect_path(w, h, adjust_values)),
        "round2DiagRect" => Some(round2_diag_rect_path(w, h, adjust_values)),
        "foldCorner" | "foldedCorner" => Some(fold_corner_path(w, h, adjust_values)),
        // Rectangles & related
        "diagStripe" => Some(diag_stripe_path(w, h, adjust_values)),
        "corner" => Some(corner_path(w, h, adjust_values)),
        "plaque" => Some(plaque_path(w, h, adjust_values)),
        "bracePair" => Some(brace_pair_path(w, h, adjust_values)),
        "bracketPair" => Some(bracket_pair_path(w, h, adjust_values)),
        "halfFrame" => Some(half_frame_path(w, h, adjust_values)),
        "line" => Some(line_path(w, h)),
        // Arrows
        "rightArrow" => Some(right_arrow_path(w, h, adjust_values)),
        "leftArrow" => Some(left_arrow_path(w, h, adjust_values)),
        "upArrow" => Some(up_arrow_path(w, h, adjust_values)),
        "downArrow" => Some(down_arrow_path(w, h, adjust_values)),
        "leftRightArrow" => Some(left_right_arrow_path(w, h, adjust_values)),
        "upDownArrow" => Some(up_down_arrow_path(w, h, adjust_values)),
        "bentArrow" => Some(bent_arrow_path(w, h, adjust_values)),
        "chevron" => Some(chevron_path(w, h, adjust_values)),
        "notchedRightArrow" => Some(notched_right_arrow_path(w, h, adjust_values)),
        "stripedRightArrow" => Some(striped_right_arrow_path(w, h, adjust_values)),
        "curvedRightArrow" => Some(curved_right_arrow_path(w, h, adjust_values)),
        "curvedLeftArrow" => Some(curved_left_arrow_path(w, h, adjust_values)),
        "curvedUpArrow" => Some(curved_up_arrow_path(w, h, adjust_values)),
        "curvedDownArrow" => Some(curved_down_arrow_path(w, h, adjust_values)),
        "circularArrow" => Some(circular_arrow_path(w, h, adjust_values)),
        "bentUpArrow" => Some(bent_up_arrow_path(w, h, adjust_values)),
        "uturnArrow" => Some(uturn_arrow_path(w, h, adjust_values)),
        "leftRightUpArrow" => Some(left_right_up_arrow_path(w, h, adjust_values)),
        "quadArrow" => Some(quad_arrow_path(w, h, adjust_values)),
        "leftUpArrow" => Some(left_up_arrow_path(w, h, adjust_values)),
        "homePlate" => Some(home_plate_path(w, h, adjust_values)),
        // Callouts
        "wedgeRoundRectCallout" => Some(wedge_round_rect_callout_path(w, h, adjust_values)),
        "wedgeEllipseCallout" => Some(wedge_ellipse_callout_path(w, h, adjust_values)),
        "cloudCallout" => Some(cloud_callout_path(w, h, adjust_values)),
        "callout1" => Some(callout1_path(w, h)),
        "callout2" => Some(callout2_path(w, h)),
        "callout3" => Some(callout3_path(w, h)),
        "borderCallout1" => Some(border_callout1_path(w, h)),
        "borderCallout2" => Some(border_callout2_path(w, h)),
        "borderCallout3" => Some(border_callout3_path(w, h)),
        "accentCallout1" => Some(accent_callout1_path(w, h)),
        "accentCallout2" => Some(accent_callout2_path(w, h)),
        "accentCallout3" => Some(accent_callout3_path(w, h)),
        "accentBorderCallout1" => Some(accent_border_callout1_path(w, h)),
        "accentBorderCallout2" => Some(accent_border_callout2_path(w, h)),
        "accentBorderCallout3" => Some(accent_border_callout3_path(w, h)),
        "wedgeRectCallout" => Some(wedge_rect_callout_path(w, h, adjust_values)),
        // Flowchart
        "flowChartProcess" => Some(rect_path(w, h)),
        "flowChartDecision" => Some(diamond_path(w, h)),
        "flowChartTerminator" => Some(flowchart_terminator_path(w, h)),
        "flowChartDocument" => Some(flowchart_document_path(w, h, adjust_values)),
        "flowChartPredefinedProcess" => Some(flowchart_predefined_process_path(w, h)),
        "flowChartAlternateProcess" => Some(flowchart_alternate_process_path(w, h, adjust_values)),
        "flowChartManualInput" => Some(flowchart_manual_input_path(w, h, adjust_values)),
        "flowChartConnector" => Some(ellipse_path(w, h)),
        "flowChartInputOutput" => Some(flowchart_input_output_path(w, h, adjust_values)),
        "flowChartInternalStorage" => Some(flowchart_internal_storage_path(w, h)),
        "flowChartMultidocument" => Some(flowchart_multidocument_path(w, h, adjust_values)),
        "flowChartPreparation" => Some(flowchart_preparation_path(w, h)),
        "flowChartManualOperation" => Some(flowchart_manual_operation_path(w, h)),
        "flowChartOffpageConnector" => Some(flowchart_offpage_connector_path(w, h)),
        "flowChartPunchedCard" => Some(flowchart_punched_card_path(w, h)),
        "flowChartPunchedTape" => Some(flowchart_punched_tape_path(w, h)),
        "flowChartSummingJunction" => Some(flowchart_summing_junction_path(w, h)),
        "flowChartOr" => Some(flowchart_or_path(w, h)),
        "flowChartCollate" => Some(flowchart_collate_path(w, h)),
        "flowChartSort" => Some(flowchart_sort_path(w, h)),
        "flowChartExtract" => Some(flowchart_extract_path(w, h)),
        "flowChartMerge" => Some(flowchart_merge_path(w, h)),
        "flowChartOnlineStorage" => Some(flowchart_online_storage_path(w, h)),
        "flowChartDelay" => Some(flowchart_delay_path(w, h)),
        "flowChartMagneticTape" => Some(flowchart_magnetic_tape_path(w, h)),
        "flowChartMagneticDisk" => Some(flowchart_magnetic_disk_path(w, h)),
        "flowChartMagneticDrum" => Some(flowchart_magnetic_drum_path(w, h)),
        "flowChartDisplay" => Some(flowchart_display_path(w, h)),
        // Action buttons
        "actionButtonBlank" => Some(action_button_blank_path(w, h)),
        "actionButtonHome" => Some(action_button_icon_path(w, h, "home")),
        "actionButtonHelp" => Some(action_button_icon_path(w, h, "help")),
        "actionButtonInformation" => Some(action_button_icon_path(w, h, "info")),
        "actionButtonBackPrevious" => Some(action_button_icon_path(w, h, "back")),
        "actionButtonForwardNext" => Some(action_button_icon_path(w, h, "forward")),
        "actionButtonBeginning" => Some(action_button_icon_path(w, h, "beginning")),
        "actionButtonEnd" => Some(action_button_icon_path(w, h, "end")),
        "actionButtonReturn" => Some(action_button_icon_path(w, h, "return")),
        "actionButtonDocument" => Some(action_button_icon_path(w, h, "document")),
        "actionButtonSound" => Some(action_button_icon_path(w, h, "sound")),
        "actionButtonMovie" => Some(action_button_icon_path(w, h, "movie")),
        // Stars & seals
        "star4" => Some(star4_path(w, h, adjust_values)),
        "star5" => Some(star5_path(w, h, adjust_values)),
        "star6" => Some(star6_path(w, h, adjust_values)),
        "star7" => Some(star_n_path(w, h, 7, adjust_values, 34601.0)),
        "star8" => Some(star_n_path(w, h, 8, adjust_values, 34601.0)),
        "star10" => Some(star_n_path(w, h, 10, adjust_values, 42533.0)),
        "star12" => Some(star_n_path(w, h, 12, adjust_values, 37500.0)),
        "star16" => Some(star_n_path(w, h, 16, adjust_values, 37500.0)),
        "star24" => Some(star_n_path(w, h, 24, adjust_values, 37500.0)),
        "star32" => Some(star_n_path(w, h, 32, adjust_values, 37500.0)),
        "irregularSeal1" => Some(irregular_seal1_path(w, h)),
        "irregularSeal2" => Some(irregular_seal2_path(w, h)),
        // Math
        "mathEqual" => Some(math_equal_path(w, h, adjust_values)),
        "mathNotEqual" => Some(math_not_equal_path(w, h, adjust_values)),
        "mathMultiply" => Some(math_multiply_path(w, h, adjust_values)),
        "mathDivide" => Some(math_divide_path(w, h, adjust_values)),
        // Other
        "heart" => Some(heart_path(w, h)),
        "plus" => Some(preset_plus_path(w, h, adjust_values)),
        "mathPlus" => Some(plus_path(w, h, adjust_values)),
        "mathMinus" => Some(math_minus_path(w, h, adjust_values)),
        "lightningBolt" => Some(lightning_bolt_path(w, h)),
        "cloud" => Some(cloud_path(w, h)),
        "frame" => Some(frame_path(w, h, adjust_values)),
        "ribbon" => Some(ribbon_path(w, h, adjust_values)),
        "ribbon2" => Some(ribbon2_path(w, h, adjust_values)),
        "donut" => Some(donut_path(w, h, adjust_values)),
        "noSmoking" => Some(no_smoking_path(w, h, adjust_values)),
        "blockArc" => Some(block_arc_path(w, h, adjust_values)),
        "smileyFace" => Some(smiley_face_path(w, h, adjust_values)),
        "can" => Some(can_path(w, h, adjust_values)),
        "cube" => Some(cube_path(w, h, adjust_values)),
        "moon" => Some(moon_path(w, h, adjust_values)),
        "sun" => Some(sun_path(w, h, adjust_values)),
        "bevel" => Some(bevel_path(w, h, adjust_values)),
        "gear6" => Some(gear_path(w, h, 6)),
        "gear9" => Some(gear_path(w, h, 9)),
        "pie" => Some(pie_path(w, h, adjust_values)),
        "pieWedge" => Some(pie_wedge_path(w, h)),
        "arc" => Some(arc_path(w, h, adjust_values)),
        "wave" => Some(wave_path(w, h, adjust_values)),
        "doubleWave" => Some(double_wave_path(w, h, adjust_values)),
        "decagon" => Some(regular_polygon_path(w, h, 10)),
        "dodecagon" => Some(regular_polygon_path(w, h, 12)),
        "funnel" => Some(funnel_path(w, h)),
        "teardrop" => Some(teardrop_path(w, h, adjust_values)),
        "heptagon" => Some(regular_polygon_path(w, h, 7)),
        // Arrow callouts
        "downArrowCallout" => Some(down_arrow_callout_path(w, h, adjust_values)),
        "leftArrowCallout" => Some(left_arrow_callout_path(w, h, adjust_values)),
        "rightArrowCallout" => Some(right_arrow_callout_path(w, h, adjust_values)),
        "upArrowCallout" => Some(up_arrow_callout_path(w, h, adjust_values)),
        "quadArrowCallout" => Some(quad_arrow_callout_path(w, h, adjust_values)),
        "leftRightArrowCallout" => Some(left_right_arrow_callout_path(w, h, adjust_values)),
        "upDownArrowCallout" => Some(up_down_arrow_callout_path(w, h, adjust_values)),
        // Brackets and braces
        "leftBrace" => Some(left_brace_path(w, h, adjust_values)),
        "rightBrace" => Some(right_brace_path(w, h, adjust_values)),
        "leftBracket" => Some(left_bracket_path(w, h, adjust_values)),
        "rightBracket" => Some(right_bracket_path(w, h, adjust_values)),
        // Chart shapes
        "chartPlus" => Some(chart_plus_path(w, h)),
        "chartStar" => Some(chart_star_path(w, h)),
        "chartX" => Some(chart_x_path(w, h)),
        // Scrolls
        "horizontalScroll" => Some(horizontal_scroll_path(w, h, adjust_values)),
        "verticalScroll" => Some(vertical_scroll_path(w, h, adjust_values)),
        // Tabs
        "cornerTabs" => Some(corner_tabs_path(w, h)),
        "plaqueTabs" => Some(plaque_tabs_path(w, h)),
        "squareTabs" => Some(square_tabs_path(w, h)),
        // Ribbons
        "ellipseRibbon" => Some(ellipse_ribbon_path(w, h, adjust_values)),
        "ellipseRibbon2" => Some(ellipse_ribbon2_path(w, h, adjust_values)),
        // Circular arrows
        "leftCircularArrow" => Some(left_circular_arrow_path(w, h)),
        "leftRightCircularArrow" => Some(left_right_circular_arrow_path(w, h)),
        // Misc
        "chord" => Some(chord_path(w, h, adjust_values)),
        "lineInv" => Some(line_inv_path(w, h)),
        "nonIsoscelesTrapezoid" => Some(non_isosceles_trapezoid_path(w, h, adjust_values)),
        "swooshArrow" => Some(swoosh_arrow_path(w, h)),
        "leftRightRibbon" => Some(left_right_ribbon_path(w, h, adjust_values)),
        // Additional ECMA-376 ST_ShapeType shapes
        "flowChartOfflineStorage" => Some(flowchart_offline_storage_path(w, h)),
        "cross" => Some(preset_plus_path(w, h, adjust_values)),
        "straightConnector1" => Some(line_path(w, h)),
        "curvedConnector2" => Some(curved_connector2_path(w, h)),
        "curvedConnector3" => Some(curved_connector3_path(w, h, adjust_values)),
        "curvedConnector4" => Some(curved_connector4_path(w, h, adjust_values)),
        "curvedConnector5" => Some(curved_connector5_path(w, h, adjust_values)),
        "bentConnector2" => Some(bent_connector2_path(w, h)),
        "bentConnector3" => Some(bent_connector3_path(w, h, adjust_values)),
        "bentConnector4" => Some(bent_connector4_path(w, h, adjust_values)),
        "bentConnector5" => Some(bent_connector5_path(w, h, adjust_values)),
        _ => None,
    }
}

fn rect_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z")
}
fn round_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn ellipse_path(w: f64, h: f64) -> String {
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
fn triangle_path(w: f64, h: f64) -> String {
    format!("M{:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z", w / 2.0)
}
fn rt_triangle_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},{h:.1} L0,{h:.1} Z")
}
fn diamond_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    let cy = h / 2.0;
    format!("M{cx:.1},0 L{w:.1},{cy:.1} L{cx:.1},{h:.1} L0,{cy:.1} Z")
}
fn parallelogram_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let o = w * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    format!(
        "M{o:.1},0 L{w:.1},0 L{x:.1},{h:.1} L0,{h:.1} Z",
        o = o,
        w = w,
        x = w - o,
        h = h
    )
}
fn trapezoid_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let o = w * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    format!(
        "M{o:.1},0 L{x:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z",
        o = o,
        x = w - o,
        w = w,
        h = h
    )
}
fn pentagon_path(w: f64, h: f64) -> String {
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
fn hexagon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn octagon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn snip1_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let d = w.min(h) * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0;
    format!(
        "M0,0 L{x:.1},0 L{w:.1},{d:.1} L{w:.1},{h:.1} L0,{h:.1} Z",
        x = w - d,
        w = w,
        d = d,
        h = h
    )
}
fn snip2_same_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let d = w.min(h) * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0;
    format!(
        "M{d:.1},0 L{x:.1},0 L{w:.1},{d:.1} L{w:.1},{h:.1} L0,{h:.1} L0,{d:.1} Z",
        d = d,
        x = w - d,
        w = w,
        h = h
    )
}
fn snip2_diag_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let d = w.min(h) * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0;
    format!(
        "M{d:.1},0 L{x:.1},0 L{w:.1},{d:.1} L{w:.1},{y:.1} L{x:.1},{h:.1} L0,{h:.1} Z",
        d = d,
        x = w - d,
        w = w,
        y = h - d,
        h = h
    )
}
fn snip_round_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let m = w.min(h);
    let r = (m * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0).min(m / 2.0);
    format!(
        "M{r:.1},0 L{x:.1},0 L{w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L{r:.1},{h:.1} Q0,{h:.1} 0,{y:.1} L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x = w - r,
        w = w,
        y = h - r,
        h = h
    )
}
fn round1_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let m = w.min(h);
    let r = (m * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0).min(m / 2.0);
    format!(
        "M0,0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{h:.1} L0,{h:.1} Z",
        x = w - r,
        w = w,
        r = r,
        h = h
    )
}
fn round2_same_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let m = w.min(h);
    let r = (m * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0).min(m / 2.0);
    format!(
        "M{r:.1},0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{h:.1} L0,{h:.1} L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x = w - r,
        w = w,
        h = h
    )
}
fn round2_diag_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let m = w.min(h);
    let r = (m * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0).min(m / 2.0);
    format!(
        "M{r:.1},0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L0,{h:.1} L0,0 Z",
        r = r,
        x = w - r,
        w = w,
        y = h - r,
        h = h
    )
}
fn fold_corner_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let d = w.min(h) * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{y:.1} L{x:.1},{h:.1} L0,{h:.1} Z M{x:.1},{h:.1} L{w:.1},{y:.1} L{x:.1},{y:.1} Z",
        w = w,
        x = w - d,
        y = h - d,
        h = h
    )
}
fn diag_stripe_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(50000.0) / 100_000.0;
    let top_left_x = w * (0.2 + ratio.clamp(0.0, 1.0) * 0.3);
    let left_y = h * (0.7 - ratio.clamp(0.0, 1.0) * 0.3);
    format!(
        "M0,{h:.1} L0,{left_y:.1} L{top_left_x:.1},0 L{w:.1},0 Z",
        h = h,
        left_y = left_y,
        top_left_x = top_left_x,
        w = w
    )
}
fn corner_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let dx = w * adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let dy = h * adj.get("adj1").copied().unwrap_or(50000.0) / 100_000.0;
    format!(
        "M0,0 L{dx:.1},0 L{dx:.1},{y:.1} L{w:.1},{y:.1} L{w:.1},{h:.1} L0,{h:.1} Z",
        dx = dx,
        y = h - dy,
        w = w,
        h = h
    )
}
fn plaque_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let m = w.min(h);
    let r = (m * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0).min(m / 2.0);
    format!(
        "M0,{r:.1} Q0,0 {r:.1},0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L{r:.1},{h:.1} Q0,{h:.1} 0,{y:.1} Z",
        r = r,
        x = w - r,
        w = w,
        y = h - r,
        h = h
    )
}
fn brace_pair_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let r = w.min(h) * adj.get("adj").copied().unwrap_or(8333.0) / 100_000.0;
    let cy = h / 2.0;
    let corner = (r * 1.6).max(w.min(h) * 0.08);
    let pinch = r.max(w.min(h) * 0.05);
    let x1 = w - corner;
    let y1 = cy - r;
    let y2 = cy + r;
    let y3 = h - corner;
    format!(
        "M{corner:.1},0 L{x1:.1},0 Q{w:.1},0 {w:.1},{corner:.1} L{w:.1},{y1:.1} Q{w:.1},{cy:.1} {x_in:.1},{cy:.1} Q{w:.1},{cy:.1} {w:.1},{y2:.1} L{w:.1},{y3:.1} Q{w:.1},{h:.1} {x1:.1},{h:.1} L{corner:.1},{h:.1} Q0,{h:.1} 0,{y3:.1} L0,{y2:.1} Q0,{cy:.1} {pinch:.1},{cy:.1} Q0,{cy:.1} 0,{y1:.1} L0,{corner:.1} Q0,0 {corner:.1},0 Z",
        corner = corner,
        x1 = x1,
        w = w,
        y1 = y1,
        cy = cy,
        x_in = w - pinch,
        y2 = y2,
        y3 = y3,
        h = h,
        pinch = pinch
    )
}
fn bracket_pair_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.025258,0.172501 L 0.025258,0.172501 C 0.025258,0.145695 0.029703,0.119205 0.038392,0.095869 0.047080,0.072532 0.059406,0.052980 0.074358,0.039420 0.089311,0.026175 0.106284,0.018921 0.123661,0.018921 L 0.876339,0.019237 0.876339,0.019237 C 0.893514,0.019237 0.910487,0.026175 0.925439,0.039735 0.940392,0.053295 0.952920,0.072532 0.961608,0.095869 0.970095,0.119205 0.974742,0.145695 0.974742,0.172816 L 0.974742,0.172816 0.974742,0.786818 0.974742,0.786818 C 0.974742,0.813623 0.970297,0.840114 0.961608,0.863450 0.952920,0.886787 0.940594,0.906339 0.925642,0.919899 0.910689,0.933144 0.893716,0.940397 0.876339,0.940397 L 0.123459,0.940397 0.123459,0.940397 C 0.106284,0.940397 0.089311,0.933459 0.074358,0.919899 0.059406,0.906339 0.046878,0.887102 0.038190,0.863765 0.029703,0.840429 0.025056,0.813939 0.025056,0.786818 L 0.025258,0.172501 Z",
            w,
            h,
        );
    }

    let r = w.min(h) * adj.get("adj").copied().unwrap_or(16667.0) / 100_000.0;
    format!(
        "M{r:.1},0 L0,0 L0,{h:.1} L{r:.1},{h:.1} M{x:.1},0 L{w:.1},0 L{w:.1},{h:.1} L{x:.1},{h:.1}",
        r = r,
        x = w - r,
        w = w,
        h = h
    )
}
fn half_frame_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ty = h * adj.get("adj1").copied().unwrap_or(33333.0) / 100_000.0;
    let tx = w * adj.get("adj2").copied().unwrap_or(33333.0) / 100_000.0;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{ty:.1} L{tx:.1},{ty:.1} L{tx:.1},{h:.1} L0,{h:.1} Z",
        w = w,
        ty = ty,
        tx = tx,
        h = h
    )
}
fn line_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},{h:.1}")
}
fn right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn left_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn down_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn left_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn up_down_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn bent_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(43750.0) / 100_000.0;
    let s = h * a1;
    let c = h * a2;
    let xh = w * (1.0 - a3);
    let (yt, yb) = (c - s / 2.0, c + s / 2.0);
    let hd = (c + h * a4.clamp(0.1, 0.9)).min(h);
    format!(
        "M0,{h:.1} L0,{yb:.1} L{xh:.1},{yb:.1} L{xh:.1},0 L{w:.1},{c:.1} L{xh:.1},{hd:.1} L{xh:.1},{yt:.1} L{s:.1},{yt:.1} L{s:.1},{h:.1} Z",
        h = h,
        yb = yb,
        xh = xh,
        w = w,
        c = c,
        hd = hd,
        yt = yt,
        s = s
    )
}
fn chevron_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let p = w * adj.get("adj").copied().unwrap_or(50000.0) / 100_000.0;
    let cy = h / 2.0;
    let x1 = w - p;
    format!("M0,0 L{x1:.1},0 L{w:.1},{cy:.1} L{x1:.1},{h:.1} L0,{h:.1} L{p:.1},{cy:.1} Z")
}
fn notched_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let a2 = adj.get("adj2").copied().unwrap_or(33333.0);
    let s = h * a1 / 100_000.0 / 2.0;
    let hw = w * a2 / 100_000.0;
    let cy = h / 2.0;
    let (yt, yb, xh) = (cy - s, cy + s, w - hw);
    let n = hw * 0.5;
    format!(
        "M0,{yt:.1} L{xh:.1},{yt:.1} L{xh:.1},0 L{w:.1},{cy:.1} L{xh:.1},{h:.1} L{xh:.1},{yb:.1} L0,{yb:.1} L{n:.1},{cy:.1} Z"
    )
}
fn striped_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn curved_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.000342,0.388195 L 0.000342,0.388195 C 0.000342,0.456269 0.046543,0.523266 0.134155,0.582292 0.222108,0.641318 0.348049,0.690220 0.500342,0.724257 0.578371,0.741922 0.662560,0.755278 0.750171,0.764110 L 0.749829,0.685480 1.000000,0.855019 0.749829,1.000000 0.749829,0.921370 0.749829,0.921370 C 0.662218,0.912538 0.578371,0.899181 0.500000,0.881732 0.348049,0.847695 0.221766,0.798578 0.134155,0.739552 0.046201,0.680526 0.000000,0.613744 0.000000,0.545455 L 0.000342,0.388195 0.000342,0.388195 C 0.000342,0.320121 0.046543,0.253124 0.134155,0.194097 0.222108,0.135071 0.348049,0.086170 0.500342,0.052133 0.652293,0.017880 0.824435,0.000000 1.000000,0.000000 L 1.000000,0.157475 1.000000,0.157475 C 0.824435,0.157475 0.652293,0.175355 0.500342,0.209393 0.348049,0.243645 0.222108,0.292546 0.134155,0.351573 0.080767,0.387333 0.042779,0.426325 0.021218,0.467040",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let thickness = w.min(h) * (0.20 + a1.clamp(0.0, 1.0) * 0.08);
    let cx = w * (0.60 + a2.clamp(0.0, 1.0) * 0.04);
    let cy = h * (0.46 - a2.clamp(0.0, 1.0) * 0.04);
    let rx = w * (0.56 + a3.clamp(0.0, 1.0) * 0.06);
    let ry = h * (0.36 + a2.clamp(0.0, 1.0) * 0.08);
    let start_angle = 5.10 - a3.clamp(0.0, 1.0) * 0.18;
    let end_angle = 0.78 + a1.clamp(0.0, 1.0) * 0.12;
    let centerline = sample_ellipse_arc_points(cx, cy, rx, ry, start_angle, end_angle, 28);
    ribbon_path_from_centerline(&centerline, thickness, false, true)
}
fn curved_left_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 1.000000,0.564799 L 1.000000,0.564799 1.000000,0.564799 C 1.000000,0.509797 0.953820,0.455483 0.865929,0.408044 0.778251,0.360261 0.652054,0.320729 0.499894,0.292884 0.347946,0.265383 0.175569,0.250945 0.000000,0.250945 L 0.000000,0.000000 0.000000,0.000000 C 0.175569,0.000000 0.347946,0.014438 0.499894,0.041939 0.652054,0.069440 0.778251,0.109316 0.865929,0.156755 0.953820,0.204538 1.000000,0.258852 1.000000,0.313854 L 1.000000,0.313854 1.000000,0.564799 1.000000,0.564799 C 1.000000,0.619801 0.953820,0.674115 0.865929,0.721554 0.778251,0.769337 0.652054,0.808869 0.500106,0.836714 0.393701,0.855964 0.276867,0.868683 0.155352,0.874871 L 0.155352,1.000000 0.000000,0.752836 0.155352,0.498109 0.155352,0.623582 0.155352,0.623582 C 0.276655,0.617738 0.393488,0.604675 0.499894,0.585425 0.652054,0.557924 0.778251,0.518047 0.865929,0.470608 0.884656,0.460296 0.901468,0.449983 0.916365,0.438982",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let body_top = h * a1.clamp(0.08, 0.42);
    let body_bottom = h - body_top;
    let ctrl_y = h * (0.1 + a2.clamp(0.0, 1.0) * 0.4);
    let tail_ctrl_y = h * (0.6 + a2.clamp(0.0, 1.0) * 0.2);
    let ctrl_front_x = w * (0.85 - a2.clamp(0.0, 1.0) * 0.3);
    let ctrl_back_x = w * (0.75 - a2.clamp(0.0, 1.0) * 0.4);
    let tail_x = w * (0.95 - a2.clamp(0.0, 1.0) * 0.3);
    let cy = h / 2.0;
    let hw = w * a3;
    format!(
        "M{w:.1},{h:.1} C{w:.1},{ctrl_y:.1} {ctrl_front_x:.1},{body_top:.1} {hw:.1},{body_top:.1} L{hw:.1},0 L0,{cy:.1} L{hw:.1},{h:.1} L{hw:.1},{body_bottom:.1} C{ctrl_back_x:.1},{body_bottom:.1} {tail_x:.1},{tail_ctrl_y:.1} {tail_x:.1},{h:.1} Z",
        w = w,
        h = h,
        ctrl_y = ctrl_y,
        ctrl_front_x = ctrl_front_x,
        body_top = body_top,
        hw = hw,
        cy = cy,
        body_bottom = body_bottom,
        ctrl_back_x = ctrl_back_x,
        tail_x = tail_x,
        tail_ctrl_y = tail_ctrl_y
    )
}
fn curved_up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.441878,0.916434 L 0.441878,0.916434 C 0.452656,0.901448 0.463177,0.884684 0.473441,0.865888 0.521427,0.778258 0.561201,0.652019 0.588915,0.499873 0.603285,0.421641 0.614062,0.337567 0.621247,0.249936 L 0.494996,0.249936 0.757506,0.000000 1.000000,0.249936 0.873749,0.249936 0.873749,0.249936 C 0.866564,0.337567 0.855787,0.421641 0.841416,0.499873 0.813703,0.652019 0.773929,0.778258 0.725943,0.865888 0.677957,0.953772 0.623557,1.000000 0.568129,1.000000 L 0.315627,1.000000 0.315627,1.000000 C 0.260200,1.000000 0.205799,0.953772 0.157814,0.866142 0.109828,0.778258 0.070054,0.652019 0.042340,0.500127 0.014627,0.347981 0.000000,0.175514 0.000000,0.000000 L 0.252502,0.000000 0.252502,0.000000 C 0.252502,0.175514 0.267129,0.347981 0.294842,0.500127 0.322556,0.652019 0.362330,0.778258 0.410316,0.866142 0.458301,0.953772 0.512702,1.000000 0.568129,1.000000",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let cx = w / 2.0;
    let hh = h * a3;
    let body_left = w * a1.clamp(0.08, 0.42);
    let body_right = w - body_left;
    let ctrl_x = w * (0.1 + a2.clamp(0.0, 1.0) * 0.4);
    let tail_ctrl_x = w * (0.6 + a2.clamp(0.0, 1.0) * 0.2);
    let ctrl_top_y = h * (0.9 - a2.clamp(0.0, 1.0) * 0.4);
    let tail_top_y = h * (0.45 + a2.clamp(0.0, 1.0) * 0.2);
    let tail_y = h * (0.6 + a2.clamp(0.0, 1.0) * 0.4);
    format!(
        "M{w:.1},{h:.1} C{body_left_plus:.1},{h:.1} {ctrl_x:.1},{ctrl_top_y:.1} {ctrl_x:.1},{hh:.1} L0,{hh:.1} L{cx:.1},0 L{w:.1},{hh:.1} L{body_right:.1},{hh:.1} C{body_right:.1},{tail_top_y:.1} {tail_ctrl_x:.1},{tail_y:.1} {w:.1},{tail_y:.1} Z",
        w = w,
        h = h,
        body_left_plus = (body_left + w * 0.05).min(w * 0.45),
        ctrl_x = ctrl_x,
        ctrl_top_y = ctrl_top_y,
        hh = hh,
        cx = cx,
        body_right = body_right,
        tail_top_y = tail_top_y,
        tail_ctrl_x = tail_ctrl_x,
        tail_y = tail_y
    )
}
fn curved_down_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.438982,0.083617 L 0.438982,0.083617 C 0.428326,0.098511 0.417669,0.115319 0.407700,0.134043 0.359917,0.221915 0.320385,0.348085 0.292884,0.500000 0.265040,0.652128 0.250602,0.824468 0.250602,1.000000 L 0.000000,1.000000 0.000000,1.000000 C 0.000000,0.824468 0.014438,0.652128 0.041939,0.500213 0.069440,0.348085 0.109316,0.221915 0.156755,0.134255 0.204538,0.046383 0.258852,0.000213 0.313854,0.000213 L 0.564799,0.000213 0.564799,0.000213 C 0.619801,0.000213 0.674115,0.046383 0.721554,0.134255 0.769337,0.221915 0.808869,0.348085 0.836714,0.500000 0.855964,0.606383 0.868683,0.723191 0.874871,0.844681 L 1.000000,0.844468 0.752836,1.000000 0.498109,0.844468 0.623582,0.844468 0.623582,0.844468 C 0.617738,0.723191 0.604675,0.606383 0.585425,0.500000 0.557924,0.347872 0.518047,0.221702 0.470608,0.134043 0.422826,0.046170 0.368855,0.000000 0.313510,0.000000",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let cx = w / 2.0;
    let hy = h * (1.0 - a3);
    let body_left = w * a1.clamp(0.08, 0.42);
    let body_right = w - body_left;
    let ctrl_x = w * (0.1 + a2.clamp(0.0, 1.0) * 0.4);
    let tail_ctrl_x = w * (0.6 + a2.clamp(0.0, 1.0) * 0.2);
    let ctrl_y = h * (0.1 + a2.clamp(0.0, 1.0) * 0.4);
    let tail_ctrl_y = h * (0.45 + a2.clamp(0.0, 1.0) * 0.2);
    let tail_y = h * (0.6 - a2.clamp(0.0, 1.0) * 0.2);
    format!(
        "M{w:.1},0 C{body_left_plus:.1},0 {ctrl_x:.1},{ctrl_y:.1} {ctrl_x:.1},{hy:.1} L0,{hy:.1} L{cx:.1},{h:.1} L{w:.1},{hy:.1} L{body_right:.1},{hy:.1} C{body_right:.1},{tail_ctrl_y:.1} {tail_ctrl_x:.1},{tail_y:.1} {w:.1},{tail_y:.1} Z",
        w = w,
        body_left_plus = (body_left + w * 0.05).min(w * 0.45),
        ctrl_x = ctrl_x,
        ctrl_y = ctrl_y,
        hy = hy,
        cx = cx,
        h = h,
        body_right = body_right,
        tail_ctrl_y = tail_ctrl_y,
        tail_ctrl_x = tail_ctrl_x,
        tail_y = tail_y
    )
}
fn circular_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(12500.0) / 100_000.0;
    let a5 = adj.get("adj5").copied().unwrap_or(12500.0) / 100_000.0;
    let thickness = w.min(h) * (0.16 + a5.clamp(0.0, 1.0) * 0.12);
    let cx = w * 0.50;
    let cy = h * (0.82 - a1.clamp(-0.4, 0.6) * 0.08);
    let rx = w * (0.46 + a1.clamp(-0.4, 0.6) * 0.04);
    let ry = h * (0.75 + a5.clamp(0.0, 1.0) * 0.06);
    let start_angle = 3.30 + a1.clamp(-0.4, 0.6) * 0.18;
    let end_angle = 5.90 - a1.clamp(-0.4, 0.6) * 0.50;
    let centerline = sample_ellipse_arc_points(cx, cy, rx, ry, start_angle, end_angle, 26);
    ribbon_path_from_centerline(&centerline, thickness, false, true)
}
fn bent_up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.025061,0.709779 L 0.753234,0.709779 0.753234,0.249211 0.679466,0.249211 0.827001,0.018927 0.974737,0.249211 0.900768,0.249211 0.900768,0.940379 0.025061,0.940379 0.025061,0.709779 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let s = w * a1;
    let hh = h * a2;
    let xm = w - s;
    let cx = xm + s / 2.0;
    let xl = xm - s * (0.2 + a3.clamp(0.0, 1.0) * 0.8);
    format!(
        "M0,{h:.1} L0,{y1:.1} L{xm:.1},{y1:.1} L{xm:.1},{hh:.1} L{xl:.1},{hh:.1} L{cx:.1},0 L{w:.1},{hh:.1} L{w:.1},{y1:.1} L{s:.1},{y1:.1} L{s:.1},{h:.1} Z",
        h = h,
        y1 = h - s,
        xm = xm,
        hh = hh,
        xl = xl,
        cx = cx,
        w = w,
        s = s
    )
}
fn uturn_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.266000,0.972000 L 0.028000,0.962000 0.028000,0.418000 0.044000,0.322000 0.124000,0.174000 0.250000,0.072000 0.394000,0.028000 0.554000,0.040000 0.686000,0.104000 0.776000,0.194000 0.844000,0.334000 0.856000,0.490000 0.972000,0.502000 0.730000,0.736000 0.496000,0.502000 0.612000,0.486000 0.608000,0.406000 0.576000,0.338000 0.482000,0.276000 0.362000,0.292000 0.308000,0.338000 0.276000,0.410000 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(43750.0) / 100_000.0;
    let a5 = adj.get("adj5").copied().unwrap_or(75000.0) / 100_000.0;
    let s = w * a1;
    let rx = w * 0.35;
    let ry = h * a2;
    let c = ry;
    let xl = w * (0.08 + a3.clamp(0.0, 1.0) * 0.2);
    let xr = w * (0.68 + a5.clamp(0.0, 1.0) * 0.2);
    let hh = h * (0.08 + a4.clamp(0.0, 1.0) * 0.2);
    let yh = h - hh;
    let cx = xr + (w - xr) * (0.25 + a5.clamp(0.0, 1.0) * 0.5);
    let xm = xr - s * (0.3 + a4.clamp(0.0, 1.0) * 0.6);
    let xr2 = xr - s * (0.4 + a4.clamp(0.0, 1.0) * 0.6);
    let xl2 = xl + s * (0.8 + a3.clamp(0.0, 1.0) * 0.6);
    format!(
        "M{xl:.1},{h:.1} L{xl:.1},{c:.1} A{rx:.1},{ry:.1} 0 0,1 {xr:.1},{c:.1} L{xr:.1},{yh:.1} L{w:.1},{yh:.1} L{cx:.1},{h:.1} L{xm:.1},{yh:.1} L{xr2:.1},{yh:.1} L{xr2:.1},{c:.1} A{rx2:.1},{ry2:.1} 0 0,0 {xl2:.1},{c:.1} L{xl2:.1},{h:.1} Z",
        xl = xl,
        h = h,
        c = c,
        rx = rx,
        ry = ry,
        xr = xr,
        yh = yh,
        w = w,
        cx = cx,
        xm = xm,
        xr2 = xr2,
        rx2 = (rx - s).max(0.1),
        ry2 = (ry - s).max(0.1),
        xl2 = xl2
    )
}
fn left_right_up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        let points = [
            scale_unit_point(w, h, 0.000000, 0.749743),
            scale_unit_point(w, h, 0.155352, 0.499829),
            scale_unit_point(w, h, 0.155352, 0.624786),
            scale_unit_point(w, h, 0.422217, 0.624786),
            scale_unit_point(w, h, 0.422217, 0.249914),
            scale_unit_point(w, h, 0.344541, 0.249914),
            scale_unit_point(w, h, 0.499894, 0.000000),
            scale_unit_point(w, h, 0.655246, 0.249914),
            scale_unit_point(w, h, 0.577570, 0.249914),
            scale_unit_point(w, h, 0.577570, 0.624786),
            scale_unit_point(w, h, 0.844435, 0.624786),
            scale_unit_point(w, h, 0.844435, 0.499829),
            scale_unit_point(w, h, 1.000000, 0.749743),
            scale_unit_point(w, h, 0.844435, 1.000000),
            scale_unit_point(w, h, 0.844435, 0.874700),
            scale_unit_point(w, h, 0.155352, 0.874700),
            scale_unit_point(w, h, 0.155352, 1.000000),
        ];
        return polygon_path(&points);
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let s = w.min(h) * a1;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let hd = w * (0.1 + a2.clamp(0.0, 1.0) * 0.25);
    let hh = h * (0.1 + a3.clamp(0.0, 1.0) * 0.25);
    format!(
        "M0,{cy:.1} L{hd:.1},{y1:.1} L{hd:.1},{y2:.1} L{x1:.1},{y2:.1} L{x1:.1},{hh:.1} L{x2:.1},{hh:.1} L{cx:.1},0 L{x3:.1},{hh:.1} L{x4:.1},{hh:.1} L{x4:.1},{y2:.1} L{x5:.1},{y2:.1} L{x5:.1},{y1:.1} L{w:.1},{cy:.1} L{x5:.1},{y3:.1} L{x5:.1},{y4:.1} L{hd:.1},{y4:.1} L{hd:.1},{y3:.1} Z",
        cy = cy,
        hd = hd,
        y1 = cy - s * 1.5,
        y2 = cy - s / 2.0,
        x1 = cx - s / 2.0,
        hh = hh,
        x2 = cx - s * 1.5,
        cx = cx,
        x3 = cx + s * 1.5,
        x4 = cx + s / 2.0,
        x5 = w - hd,
        w = w,
        y3 = cy + s * 1.5,
        y4 = cy + s / 2.0
    )
}
fn quad_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(22500.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(22500.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(22500.0) / 100_000.0;
    let s = w.min(h) * a1;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (hw, hh) = (w * a2, h * a3);
    format!(
        "M{cx:.1},0 L{x3:.1},{hh:.1} L{x2:.1},{hh:.1} L{x2:.1},{y1:.1} L{hw:.1},{y1:.1} L{hw:.1},{y2:.1} L0,{cy:.1} L{hw:.1},{y3:.1} L{hw:.1},{y4:.1} L{x2:.1},{y4:.1} L{x2:.1},{y5:.1} L{x3:.1},{y5:.1} L{cx:.1},{h:.1} L{x4:.1},{y5:.1} L{x5:.1},{y5:.1} L{x5:.1},{y4:.1} L{x6:.1},{y4:.1} L{x6:.1},{y3:.1} L{w:.1},{cy:.1} L{x6:.1},{y2:.1} L{x6:.1},{y1:.1} L{x5:.1},{y1:.1} L{x5:.1},{hh:.1} L{x4:.1},{hh:.1} Z",
        cx = cx,
        cy = cy,
        hw = hw,
        hh = hh,
        x2 = cx - s / 2.0,
        x3 = cx - s * 1.5,
        x4 = cx + s * 1.5,
        x5 = cx + s / 2.0,
        x6 = w - hw,
        w = w,
        h = h,
        y1 = cy - s / 2.0,
        y2 = cy - s * 1.5,
        y3 = cy + s * 1.5,
        y4 = cy + s / 2.0,
        y5 = h - hh
    )
}
fn left_up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.039117,0.814066 L 0.269401,0.666532 0.269401,0.740299 0.614826,0.740299 0.614826,0.159660 0.499685,0.159660 0.729968,0.012126 0.960568,0.159660 0.845110,0.159660 0.845110,0.887833 0.269401,0.887833 0.269401,0.961803 0.039117,0.814066 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let s = w.min(h) * a1;
    let (hd, hh) = (w * a2, h * a2);
    let body_reach = w * (0.2 + a3.clamp(0.0, 1.0) * 0.35);
    format!(
        "M0,{cy:.1} L{hd:.1},{y1:.1} L{hd:.1},{y2:.1} L{x1:.1},{y2:.1} L{x1:.1},{hh:.1} L{x2:.1},{hh:.1} L{x3:.1},0 L{w:.1},{hh:.1} L{x4:.1},{hh:.1} L{x4:.1},{h:.1} L{hd:.1},{h:.1} L{hd:.1},{y3:.1} Z",
        cy = h / 2.0,
        hd = hd,
        y1 = h / 2.0 - s,
        y2 = h / 2.0 - s / 2.0,
        x1 = (w - body_reach).max(hd),
        hh = hh,
        x2 = (w - body_reach - s * 0.5).max(hd),
        x3 = w - s,
        w = w,
        x4 = w - s / 2.0,
        h = h,
        y3 = h / 2.0 + s
    )
}
fn home_plate_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn wedge_round_rect_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(-20833.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(62500.0) / 100_000.0;
    let r = w.min(h) * 0.06;
    let tt = w * (0.5 + a1);
    let ty = h * a2;
    format!(
        "M{r:.1},0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L{t2:.1},{h:.1} L{tt:.1},{ty:.1} L{t1:.1},{h:.1} L{r:.1},{h:.1} Q0,{h:.1} 0,{y:.1} L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x = w - r,
        y = h - r,
        w = w,
        h = h,
        t1 = w * 0.1,
        t2 = w * 0.2,
        tt = tt,
        ty = ty
    )
}
fn wedge_ellipse_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(-20833.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(62500.0) / 100_000.0;
    let rx = w / 2.0;
    let ry = h / 2.0;
    let tt = w * (0.5 + a1);
    let ty = h * a2;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z M{t1:.1},{h1:.1} L{tt:.1},{ty:.1} L{t2:.1},{h2:.1}",
        cx = rx,
        rx = rx,
        ry = ry,
        h = h,
        t1 = w * 0.35,
        h1 = h * 0.93,
        tt = tt,
        ty = ty,
        t2 = w * 0.45,
        h2 = h * 0.93
    )
}
fn cloud_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(-20833.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(62500.0) / 100_000.0;
    let bx = w * (0.18 + a1.clamp(-0.5, 0.5) * 0.25);
    let by = h * (0.75 + a2.clamp(0.0, 1.0) * 0.35);
    let br = w * (0.03 + a2.clamp(0.0, 1.0) * 0.03);
    format!(
        "M{x1:.1},{y1:.1} C{x1:.1},{y0:.1} {x2:.1},{y0:.1} {x3:.1},{y0:.1} C{x4:.1},{y0:.1} {x5:.1},{y2:.1} {x5:.1},{y3:.1} C{x5:.1},{y4:.1} {x4:.1},{y5:.1} {x3:.1},{y5:.1} C{x2:.1},{y5:.1} {x1:.1},{y4:.1} {x1:.1},{y3:.1} C{x0:.1},{y3:.1} {x0:.1},{y1:.1} {x1:.1},{y1:.1} Z M{bx1:.1},{by1:.1} A{br:.1},{br:.1} 0 1,1 {bx2:.1},{by2:.1} A{br:.1},{br:.1} 0 1,1 {bx1:.1},{by1:.1} Z",
        x0 = w * 0.05,
        x1 = w * 0.2,
        x2 = w * 0.35,
        x3 = w * 0.55,
        x4 = w * 0.8,
        x5 = w * 0.95,
        y0 = h * 0.1,
        y1 = h * 0.35,
        y2 = h * 0.25,
        y3 = h * 0.55,
        y4 = h * 0.75,
        y5 = h * 0.85,
        bx1 = bx,
        by1 = by,
        bx2 = bx - br * 1.25,
        by2 = by + br * 2.5,
        br = br
    )
}
fn callout1_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{a:.1},{h:.1} L{t:.1},{ty:.1}",
        w = w,
        h = h,
        a = w * 0.4,
        t = w * -0.1,
        ty = h * 1.2
    )
}
fn callout2_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{a:.1},{h:.1} L{m:.1},{my:.1} L{t:.1},{ty:.1}",
        w = w,
        h = h,
        a = w * 0.4,
        m = w * 0.2,
        my = h * 1.1,
        t = w * -0.1,
        ty = h * 1.3
    )
}
fn callout3_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{a:.1},{h:.1} L{m1:.1},{m1y:.1} L{m2:.1},{m2y:.1} L{t:.1},{ty:.1}",
        w = w,
        h = h,
        a = w * 0.4,
        m1 = w * 0.3,
        m1y = h * 1.05,
        m2 = w * 0.1,
        m2y = h * 1.2,
        t = w * -0.15,
        ty = h * 1.4
    )
}
fn border_callout1_path(w: f64, h: f64) -> String {
    callout1_path(w, h)
}
fn border_callout2_path(w: f64, h: f64) -> String {
    callout2_path(w, h)
}
fn border_callout3_path(w: f64, h: f64) -> String {
    callout3_path(w, h)
}
fn accent_callout1_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M0,0 L0,{h:.1} M{a:.1},{h:.1} L{t:.1},{ty:.1}",
        w = w,
        h = h,
        a = w * 0.4,
        t = w * -0.1,
        ty = h * 1.2
    )
}
fn accent_callout2_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M0,0 L0,{h:.1} M{a:.1},{h:.1} L{m:.1},{my:.1} L{t:.1},{ty:.1}",
        w = w,
        h = h,
        a = w * 0.4,
        m = w * 0.2,
        my = h * 1.1,
        t = w * -0.1,
        ty = h * 1.3
    )
}
fn accent_callout3_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M0,0 L0,{h:.1} M{a:.1},{h:.1} L{m1:.1},{m1y:.1} L{m2:.1},{m2y:.1} L{t:.1},{ty:.1}",
        w = w,
        h = h,
        a = w * 0.4,
        m1 = w * 0.3,
        m1y = h * 1.05,
        m2 = w * 0.1,
        m2y = h * 1.2,
        t = w * -0.15,
        ty = h * 1.4
    )
}
fn accent_border_callout1_path(w: f64, h: f64) -> String {
    accent_callout1_path(w, h)
}
fn accent_border_callout2_path(w: f64, h: f64) -> String {
    accent_callout2_path(w, h)
}
fn accent_border_callout3_path(w: f64, h: f64) -> String {
    accent_callout3_path(w, h)
}
fn wedge_rect_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(-20833.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(62500.0) / 100_000.0;
    let tt = w * (0.5 + a1);
    let ty = h * a2;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L{t2:.1},{h:.1} L{tt:.1},{ty:.1} L{t1:.1},{h:.1} L0,{h:.1} Z",
        w = w,
        h = h,
        t1 = w * 0.1,
        t2 = w * 0.2,
        tt = tt,
        ty = ty
    )
}
fn flowchart_terminator_path(w: f64, h: f64) -> String {
    let r = h / 2.0;
    format!(
        "M{r:.1},0 L{x:.1},0 A{r:.1},{r:.1} 0 0,1 {x:.1},{h:.1} L{r:.1},{h:.1} A{r:.1},{r:.1} 0 0,1 {r:.1},0 Z",
        r = r,
        x = w - r,
        h = h
    )
}
fn flowchart_document_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn flowchart_predefined_process_path(w: f64, h: f64) -> String {
    let i = w * 0.1;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{i:.1},0 L{i:.1},{h:.1} M{x:.1},0 L{x:.1},{h:.1}",
        w = w,
        h = h,
        i = i,
        x = w - i
    )
}
fn flowchart_alternate_process_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn flowchart_manual_input_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let s = h * adj.get("adj").copied().unwrap_or(20000.0) / 100_000.0;
    format!(
        "M0,{s:.1} L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z",
        s = s,
        w = w,
        h = h
    )
}
fn flowchart_input_output_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let o = w * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    format!(
        "M{o:.1},0 L{w:.1},0 L{x:.1},{h:.1} L0,{h:.1} Z",
        o = o,
        w = w,
        x = w - o,
        h = h
    )
}
fn flowchart_internal_storage_path(w: f64, h: f64) -> String {
    let i = w.min(h) * 0.15;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{i:.1},0 L{i:.1},{h:.1} M0,{i:.1} L{w:.1},{i:.1}",
        w = w,
        h = h,
        i = i
    )
}
fn flowchart_multidocument_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn flowchart_preparation_path(w: f64, h: f64) -> String {
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
fn flowchart_manual_operation_path(w: f64, h: f64) -> String {
    let i = w * 0.15;
    format!(
        "M0,0 L{w:.1},0 L{x:.1},{h:.1} L{i:.1},{h:.1} Z",
        w = w,
        x = w - i,
        i = i,
        h = h
    )
}
fn flowchart_offpage_connector_path(w: f64, h: f64) -> String {
    let bh = h * 0.8;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{bh:.1} L{cx:.1},{h:.1} L0,{bh:.1} Z",
        w = w,
        bh = bh,
        cx = w / 2.0,
        h = h
    )
}
fn flowchart_punched_card_path(w: f64, h: f64) -> String {
    let c = w.min(h) * 0.15;
    format!(
        "M{c:.1},0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} L0,{c:.1} Z",
        c = c,
        w = w,
        h = h
    )
}
fn flowchart_punched_tape_path(w: f64, h: f64) -> String {
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
fn flowchart_summing_junction_path(w: f64, h: f64) -> String {
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
fn flowchart_or_path(w: f64, h: f64) -> String {
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
fn flowchart_collate_path(w: f64, h: f64) -> String {
    let (cx, cy) = (w / 2.0, h / 2.0);
    format!(
        "M0,0 L{w:.1},0 L{cx:.1},{cy:.1} Z M{cx:.1},{cy:.1} L{w:.1},{h:.1} L0,{h:.1} Z",
        w = w,
        cx = cx,
        cy = cy,
        h = h
    )
}
fn flowchart_sort_path(w: f64, h: f64) -> String {
    let (cx, cy) = (w / 2.0, h / 2.0);
    format!(
        "M{cx:.1},0 L{w:.1},{cy:.1} L{cx:.1},{h:.1} L0,{cy:.1} Z M0,{cy:.1} L{w:.1},{cy:.1}",
        cx = cx,
        cy = cy,
        w = w,
        h = h
    )
}
fn flowchart_extract_path(w: f64, h: f64) -> String {
    format!("M{:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z", w / 2.0, w = w, h = h)
}
fn flowchart_merge_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 L{cx:.1},{h:.1} Z",
        w = w,
        cx = w / 2.0,
        h = h
    )
}
fn flowchart_online_storage_path(w: f64, h: f64) -> String {
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
fn flowchart_delay_path(w: f64, h: f64) -> String {
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
fn flowchart_magnetic_tape_path(w: f64, h: f64) -> String {
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
fn flowchart_magnetic_disk_path(w: f64, h: f64) -> String {
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
fn flowchart_magnetic_drum_path(w: f64, h: f64) -> String {
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
fn flowchart_display_path(w: f64, h: f64) -> String {
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
fn action_button_blank_path(w: f64, h: f64) -> String {
    let r = w.min(h) * 0.05;
    format!(
        "M{r:.1},0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L{r:.1},{h:.1} Q0,{h:.1} 0,{y:.1} L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x = w - r,
        y = h - r,
        w = w,
        h = h
    )
}
fn action_button_icon_path(w: f64, h: f64, icon: &str) -> String {
    let r = w.min(h) * 0.05;
    let btn = format!(
        "M{r:.1},0 L{x:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L{r:.1},{h:.1} Q0,{h:.1} 0,{y:.1} L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x = w - r,
        y = h - r,
        w = w,
        h = h
    );
    let (ix, iy, iw, ih) = (w * 0.25, h * 0.25, w * 0.5, h * 0.5);
    let (icx, icy) = (w * 0.5, h * 0.5);
    let ip = match icon {
        "home" => format!(
            "M{cx:.1},{iy:.1} L{x2:.1},{m:.1} L{x2:.1},{y2:.1} L{x1:.1},{y2:.1} L{x1:.1},{m:.1} Z",
            cx = icx,
            iy = iy,
            x1 = ix,
            x2 = ix + iw,
            m = iy + ih * 0.45,
            y2 = iy + ih
        ),
        "help" => {
            let qr = iw * 0.25;
            format!(
                "M{cx:.1},{y1:.1} A{qr:.1},{qr:.1} 0 1,1 {cx:.1},{m:.1} L{cx:.1},{y2:.1} M{cx:.1},{y3:.1} L{cx:.1},{y4:.1}",
                cx = icx,
                y1 = iy + ih * 0.15,
                qr = qr,
                m = iy + ih * 0.55,
                y2 = iy + ih * 0.65,
                y3 = iy + ih * 0.8,
                y4 = iy + ih * 0.85
            )
        }
        "info" => format!(
            "M{cx:.1},{y1:.1} L{cx:.1},{y2:.1} M{b1:.1},{y3:.1} L{b2:.1},{y3:.1} L{b2:.1},{y4:.1} L{b1:.1},{y4:.1} Z",
            cx = icx,
            y1 = iy + ih * 0.15,
            y2 = iy + ih * 0.25,
            b1 = icx - iw * 0.08,
            b2 = icx + iw * 0.08,
            y3 = iy + ih * 0.35,
            y4 = iy + ih * 0.9
        ),
        "back" => format!(
            "M{x2:.1},{iy:.1} L{x1:.1},{cy:.1} L{x2:.1},{y2:.1} Z",
            x1 = ix,
            x2 = ix + iw,
            iy = iy,
            cy = icy,
            y2 = iy + ih
        ),
        "forward" => format!(
            "M{x1:.1},{iy:.1} L{x2:.1},{cy:.1} L{x1:.1},{y2:.1} Z",
            x1 = ix,
            x2 = ix + iw,
            iy = iy,
            cy = icy,
            y2 = iy + ih
        ),
        "beginning" => {
            let bw = iw * 0.12;
            format!(
                "M{x1:.1},{iy:.1} L{xb:.1},{iy:.1} L{xb:.1},{y2:.1} L{x1:.1},{y2:.1} Z M{x2:.1},{iy:.1} L{xb:.1},{cy:.1} L{x2:.1},{y2:.1} Z",
                x1 = ix,
                xb = ix + bw,
                iy = iy,
                y2 = iy + ih,
                x2 = ix + iw,
                cy = icy
            )
        }
        "end" => {
            let bw = iw * 0.12;
            format!(
                "M{x1:.1},{iy:.1} L{xb:.1},{cy:.1} L{x1:.1},{y2:.1} Z M{xb:.1},{iy:.1} L{x2:.1},{iy:.1} L{x2:.1},{y2:.1} L{xb:.1},{y2:.1} Z",
                x1 = ix,
                xb = ix + iw - bw,
                cy = icy,
                x2 = ix + iw,
                iy = iy,
                y2 = iy + ih
            )
        }
        "return" => format!(
            "M{x2:.1},{m:.1} L{x1:.1},{m:.1} A{rx:.1},{ry:.1} 0 0,1 {x2:.1},{iy:.1}",
            x1 = ix,
            x2 = ix + iw,
            m = icy,
            rx = iw * 0.4,
            ry = ih * 0.3,
            iy = iy + ih * 0.2
        ),
        "document" => {
            let f = iw * 0.2;
            format!(
                "M{x1:.1},{iy:.1} L{x3:.1},{iy:.1} L{x2:.1},{y1:.1} L{x2:.1},{y2:.1} L{x1:.1},{y2:.1} Z M{x3:.1},{iy:.1} L{x3:.1},{y1:.1} L{x2:.1},{y1:.1}",
                x1 = ix,
                x2 = ix + iw,
                x3 = ix + iw - f,
                iy = iy,
                y1 = iy + f,
                y2 = iy + ih
            )
        }
        "sound" => format!(
            "M{x1:.1},{y1:.1} L{x2:.1},{y1:.1} L{x3:.1},{iy:.1} L{x3:.1},{y2:.1} L{x2:.1},{y3:.1} L{x1:.1},{y3:.1} Z",
            x1 = ix,
            x2 = ix + iw * 0.3,
            x3 = ix + iw * 0.6,
            iy = iy,
            y1 = iy + ih * 0.3,
            y2 = iy + ih,
            y3 = iy + ih * 0.7
        ),
        "movie" => format!(
            "M{x1:.1},{iy:.1} L{x2:.1},{iy:.1} L{x2:.1},{y2:.1} L{x1:.1},{y2:.1} Z M{x1:.1},{y1:.1} L{x2:.1},{y1:.1}",
            x1 = ix,
            x2 = ix + iw,
            iy = iy,
            y1 = iy + ih * 0.2,
            y2 = iy + ih
        ),
        _ => String::new(),
    };
    format!("{btn} {ip}")
}
fn star4_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(18000.0) / 100_000.0;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (ix, iy) = (cx * (1.0 - ratio), cy * (1.0 - ratio));
    format!(
        "M{cx:.1},0 L{ix2:.1},{iy:.1} L{w:.1},{cy:.1} L{ix2:.1},{iy2:.1} L{cx:.1},{h:.1} L{ix:.1},{iy2:.1} L0,{cy:.1} L{ix:.1},{iy:.1} Z",
        cx = cx,
        cy = cy,
        w = w,
        h = h,
        ix = ix,
        ix2 = w - ix,
        iy = iy,
        iy2 = h - iy
    )
}
fn star5_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (ro_x, ro_y) = (cx, cy);
    let (ri_x, ri_y) = (cx * ratio * 2.0, cy * ratio * 2.0);
    let n = 5;
    let t = n * 2;
    let st = -std::f64::consts::FRAC_PI_2;
    let mut pts: Vec<(f64, f64)> = Vec::with_capacity(t as usize);
    for i in 0..t {
        let a = st + 2.0 * std::f64::consts::PI * (i as f64) / (t as f64);
        let (rx, ry) = if i % 2 == 0 {
            (ro_x, ro_y)
        } else {
            (ri_x, ri_y)
        };
        pts.push((cx + rx * a.cos(), cy + ry * a.sin()));
    }
    let mut s = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for &(x, y) in &pts[1..] {
        s.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    s.push_str(" Z");
    s
}
fn star6_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(28868.0) / 100_000.0;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (ro_x, ro_y) = (cx, cy);
    let (ri_x, ri_y) = (cx * ratio * 2.0, cy * ratio * 2.0);
    let n = 6;
    let t = n * 2;
    let st = -std::f64::consts::FRAC_PI_2;
    let mut pts: Vec<(f64, f64)> = Vec::with_capacity(t as usize);
    for i in 0..t {
        let a = st + 2.0 * std::f64::consts::PI * (i as f64) / (t as f64);
        let (rx, ry) = if i % 2 == 0 {
            (ro_x, ro_y)
        } else {
            (ri_x, ri_y)
        };
        pts.push((cx + rx * a.cos(), cy + ry * a.sin()));
    }
    let mut s = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for &(x, y) in &pts[1..] {
        s.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    s.push_str(" Z");
    s
}
fn star_n_path(w: f64, h: f64, n: u32, adj: &HashMap<String, f64>, default_adj: f64) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(default_adj) / 100_000.0;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (ro, ri) = (cx, cx * ratio * 2.0);
    let (ryo, ryi) = (cy, cy * ratio * 2.0);
    let t = n * 2;
    let st = -std::f64::consts::FRAC_PI_2;
    let mut pts: Vec<(f64, f64)> = Vec::with_capacity(t as usize);
    for i in 0..t {
        let a = st + 2.0 * std::f64::consts::PI * (i as f64) / (t as f64);
        let (rx, ry) = if i % 2 == 0 { (ro, ryo) } else { (ri, ryi) };
        pts.push((cx + rx * a.cos(), cy + ry * a.sin()));
    }
    let mut s = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for &(x, y) in &pts[1..] {
        s.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    s.push_str(" Z");
    s
}
fn irregular_seal1_path(w: f64, h: f64) -> String {
    let p = [
        (0.5, 0.0),
        (0.6, 0.15),
        (0.75, 0.05),
        (0.7, 0.25),
        (1.0, 0.2),
        (0.8, 0.4),
        (0.95, 0.55),
        (0.75, 0.55),
        (0.85, 0.75),
        (0.65, 0.65),
        (0.6, 0.9),
        (0.45, 0.7),
        (0.3, 1.0),
        (0.35, 0.7),
        (0.1, 0.8),
        (0.2, 0.55),
        (0.0, 0.5),
        (0.2, 0.35),
        (0.05, 0.15),
        (0.3, 0.25),
        (0.35, 0.05),
        (0.45, 0.2),
    ];
    let mut s = format!("M{:.1},{:.1}", p[0].0 * w, p[0].1 * h);
    for &(px, py) in &p[1..] {
        s.push_str(&format!(" L{:.1},{:.1}", px * w, py * h));
    }
    s.push_str(" Z");
    s
}
fn irregular_seal2_path(w: f64, h: f64) -> String {
    let p = [
        (0.45, 0.0),
        (0.55, 0.1),
        (0.7, 0.0),
        (0.65, 0.2),
        (0.9, 0.1),
        (0.78, 0.3),
        (1.0, 0.35),
        (0.82, 0.45),
        (0.95, 0.6),
        (0.72, 0.6),
        (0.8, 0.8),
        (0.6, 0.7),
        (0.55, 1.0),
        (0.45, 0.75),
        (0.25, 0.9),
        (0.3, 0.65),
        (0.05, 0.7),
        (0.2, 0.5),
        (0.0, 0.35),
        (0.22, 0.35),
        (0.1, 0.15),
        (0.35, 0.2),
    ];
    let mut s = format!("M{:.1},{:.1}", p[0].0 * w, p[0].1 * h);
    for &(px, py) in &p[1..] {
        s.push_str(&format!(" L{:.1},{:.1}", px * w, py * h));
    }
    s.push_str(" Z");
    s
}
fn math_equal_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.112890,0.196584 L 0.883509,0.196584 0.883509,0.491445 0.112890,0.491445 0.112890,0.196584 Z M 0.112890,0.549220 L 0.883509,0.549220 0.883509,0.843965 0.112890,0.843965 0.112890,0.549220 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(23520.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(11760.0) / 100_000.0;
    let bar_h = h * a2;
    let gap = h * a1;
    let cy = h / 2.0;
    let y1 = cy - gap / 2.0 - bar_h;
    let y2 = cy - gap / 2.0;
    let y3 = cy + gap / 2.0;
    let y4 = cy + gap / 2.0 + bar_h;
    format!(
        "M0,{y1:.1} L{w:.1},{y1:.1} L{w:.1},{y2:.1} L0,{y2:.1} Z M0,{y3:.1} L{w:.1},{y3:.1} L{w:.1},{y4:.1} L0,{y4:.1} Z",
        y1 = y1,
        y2 = y2,
        y3 = y3,
        y4 = y4,
        w = w
    )
}
fn math_not_equal_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.033486,0.208517 L 0.485552,0.208517 0.544693,0.018927 0.719147,0.092744 0.682960,0.208517 0.966244,0.208517 0.966244,0.425237 0.615447,0.425237 0.581691,0.533754 0.966244,0.533754 0.966244,0.750473 0.514178,0.750473 0.455036,0.940379 0.280583,0.866246 0.316770,0.750473 0.033486,0.750473 0.033486,0.533754 0.384283,0.533754 0.418039,0.425237 0.033486,0.425237 0.033486,0.208517 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(23520.0) / 100_000.0;
    let a2_angle = adj.get("adj2").copied().unwrap_or(6600000.0);
    let a3 = adj.get("adj3").copied().unwrap_or(11760.0) / 100_000.0;
    let bar_h = h * a3;
    let gap = h * a1;
    let cy = h / 2.0;
    let y1 = cy - gap / 2.0 - bar_h;
    let y2 = cy - gap / 2.0;
    let y3 = cy + gap / 2.0;
    let y4 = cy + gap / 2.0 + bar_h;
    let skew = (a2_angle - 6_600_000.0) / 21_600_000.0;
    let x1 = w * (0.65 + skew * 0.6);
    let x2 = w * (0.35 - skew * 0.6);
    format!(
        "M0,{y1:.1} L{w:.1},{y1:.1} L{w:.1},{y2:.1} L0,{y2:.1} Z M0,{y3:.1} L{w:.1},{y3:.1} L{w:.1},{y4:.1} L0,{y4:.1} Z M{x1:.1},0 L{x2:.1},{h:.1}",
        y1 = y1,
        y2 = y2,
        y3 = y3,
        y4 = y4,
        w = w,
        x1 = x1,
        x2 = x2,
        h = h
    )
}
fn math_multiply_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.052744,0.138225 L 0.301148,0.019653 0.499787,0.265640 0.698426,0.019653 0.946831,0.138225 0.671629,0.478873 0.946831,0.819522 0.698426,0.938094 0.499787,0.692106 0.301148,0.938094 0.052744,0.819522 0.327946,0.478873 0.052744,0.138225 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(23520.0) / 100_000.0;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let d = w.min(h) * a1;
    let t = w.min(h) * 0.06;
    format!(
        "M{x1:.1},{y1t:.1} L{x1t:.1},{y1:.1} L{cx:.1},{y3:.1} L{x2t:.1},{y1:.1} L{x2:.1},{y1t:.1} L{x4:.1},{cy:.1} L{x2:.1},{y2t:.1} L{x2t:.1},{y2:.1} L{cx:.1},{y4:.1} L{x1t:.1},{y2:.1} L{x1:.1},{y2t:.1} L{x3:.1},{cy:.1} Z",
        cx = cx,
        cy = cy,
        x1 = cx - d,
        y1 = cy - d,
        x1t = cx - d + t,
        y1t = cy - d + t,
        x2 = cx + d,
        y2 = cy + d,
        x2t = cx + d - t,
        y2t = cy + d - t,
        x3 = cx - d + t * 0.5,
        x4 = cx + d - t * 0.5,
        y3 = cy - d + t * 0.5,
        y4 = cy + d - t * 0.5
    )
}
fn math_divide_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.499791,0.015629 L 0.499791,0.015629 C 0.540292,0.015629 0.579958,0.022402 0.615031,0.034905 0.650104,0.047669 0.679332,0.065642 0.699791,0.087523 0.719833,0.109403 0.730689,0.134410 0.730689,0.159677 0.730689,0.184944 0.719833,0.209690 0.699791,0.231571 0.679332,0.253451 0.650104,0.271685 0.615031,0.284189 0.579958,0.296952 0.540292,0.303464 0.499791,0.303464 0.459290,0.303464 0.419624,0.296952 0.384551,0.284189 0.349478,0.271685 0.320251,0.253451 0.299791,0.231571 0.279749,0.209690 0.268894,0.184944 0.268894,0.159677 0.268894,0.134410 0.279749,0.109403 0.299791,0.087523 0.320251,0.065642 0.349478,0.047669 0.384551,0.034905 0.419624,0.022402 0.459290,0.015629 0.499791,0.015629 Z M 0.499791,0.950768 L 0.499791,0.950768 C 0.459290,0.950768 0.419624,0.943996 0.384551,0.931493 0.349478,0.918729 0.320251,0.900755 0.299791,0.878875 0.279749,0.856994 0.268894,0.831987 0.268894,0.806721 0.268894,0.781454 0.279749,0.756707 0.299791,0.734827 0.320251,0.712946 0.349478,0.694712 0.384551,0.682209 0.419624,0.669445 0.459290,0.662933 0.499791,0.662933 0.540292,0.662933 0.579958,0.669445 0.615031,0.682209 0.650104,0.694712 0.679332,0.712946 0.699791,0.734827 0.719833,0.756707 0.730689,0.781454 0.730689,0.806721 0.730689,0.831987 0.719833,0.856994 0.699791,0.878875 0.679332,0.900755 0.650104,0.918729 0.615031,0.931493 0.579958,0.943996 0.540292,0.950768 0.499791,0.950768 Z M 0.051775,0.339151 L 0.947808,0.339151 0.947808,0.627247 0.051775,0.627247 0.051775,0.339151 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(23520.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(5765.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(11760.0) / 100_000.0;
    let cx = w / 2.0;
    let dr = w.min(h) * a2;
    let bar_h = h * a3;
    let gap = h * a1;
    let cy = h / 2.0;
    let y1 = cy - bar_h / 2.0;
    let y2 = cy + bar_h / 2.0;
    let dot_top = cy - gap / 2.0 - bar_h / 2.0;
    let dot_bot = cy + gap / 2.0 + bar_h / 2.0;
    format!(
        "M0,{y1:.1} L{w:.1},{y1:.1} L{w:.1},{y2:.1} L0,{y2:.1} Z M{cx:.1},{d1:.1} A{dr:.1},{dr:.1} 0 1,1 {cx:.1},{d1b:.1} A{dr:.1},{dr:.1} 0 1,1 {cx:.1},{d1:.1} Z M{cx:.1},{d2:.1} A{dr:.1},{dr:.1} 0 1,1 {cx:.1},{d2b:.1} A{dr:.1},{dr:.1} 0 1,1 {cx:.1},{d2:.1} Z",
        y1 = y1,
        y2 = y2,
        w = w,
        cx = cx,
        dr = dr,
        d1 = dot_top - dr,
        d1b = dot_top + dr,
        d2 = dot_bot - dr,
        d2b = dot_bot + dr
    )
}
fn heart_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    format!(
        "M{cx:.1},{h:.1} C{c1:.1},{c1y:.1} 0,{c2:.1} 0,{c3:.1} C0,{c4:.1} {c5:.1},0 {cx:.1},{c6:.1} C{c7:.1},0 {w:.1},{c4:.1} {w:.1},{c3:.1} C{w:.1},{c2:.1} {c8:.1},{c1y:.1} {cx:.1},{h:.1} Z",
        cx = cx,
        w = w,
        h = h,
        c1 = w * 0.15,
        c1y = h * 0.75,
        c2 = h * 0.5,
        c3 = h * 0.3,
        c4 = h * 0.1,
        c5 = w * 0.15,
        c6 = h * 0.2,
        c7 = w * 0.85,
        c8 = w * 0.85
    )
}
fn plus_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.352280,0.110805 L 0.647720,0.110805 0.647720,0.408107 0.899713,0.408107 0.899713,0.591893 0.647720,0.591893 0.647720,0.889195 0.352280,0.889195 0.352280,0.591893 0.100287,0.591893 0.100287,0.408107 0.352280,0.408107 0.352280,0.110805 Z",
            w,
            h,
        );
    }

    let a = adj.get("adj").copied().unwrap_or(25000.0);
    let ax = w * a / 100_000.0;
    let ay = h * a / 100_000.0;
    let (x1, y1) = (w - ax, h - ay);
    format!(
        "M{ax:.1},0 L{x1:.1},0 L{x1:.1},{ay:.1} L{w:.1},{ay:.1} L{w:.1},{y1:.1} L{x1:.1},{y1:.1} L{x1:.1},{h:.1} L{ax:.1},{h:.1} L{ax:.1},{y1:.1} L0,{y1:.1} L0,{ay:.1} L{ax:.1},{ay:.1} Z"
    )
}
fn preset_plus_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.733333,1.002075 L 0.254167,0.997925 0.239583,0.979253 0.239583,0.771784 0.229167,0.761411 0.033333,0.761411 -0.002083,0.738589 -0.002083,0.261411 0.008333,0.238589 0.239583,0.232365 0.239583,0.024896 0.250000,-0.002075 0.741667,-0.002075 0.756250,0.020747 0.760417,0.232365 0.983333,0.238589 0.997917,0.257261 1.002083,0.734440 0.991667,0.753112 0.962500,0.761411 0.766667,0.761411 0.756250,0.780083 0.756250,0.983402 Z",
            w,
            h,
        );
    }

    plus_path(w, h, adj)
}
fn math_minus_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.100287,0.363463 L 0.899713,0.363463 0.899713,0.636537 0.100287,0.636537 0.100287,0.363463 Z",
            w,
            h,
        );
    }

    let a1 = adj.get("adj1").copied().unwrap_or(11760.0) / 100_000.0;
    let bar_h = h * a1;
    let cy = h / 2.0;
    format!(
        "M0,{y1:.1} L{w:.1},{y1:.1} L{w:.1},{y2:.1} L0,{y2:.1} Z",
        y1 = cy - bar_h / 2.0,
        y2 = cy + bar_h / 2.0,
        w = w
    )
}
fn lightning_bolt_path(w: f64, h: f64) -> String {
    scale_normalized_path(
        "M 0.398471,0.014333 L 0.589345,0.279025 0.510750,0.310081 0.751314,0.537028 0.672480,0.575012 0.970139,0.954849 0.465361,0.663641 0.561634,0.623268 0.248208,0.436694 0.360487,0.379121 0.029623,0.183708 0.398471,0.014333 Z",
        w,
        h,
    )
}
fn cloud_path(w: f64, h: f64) -> String {
    format!(
        "M{x1:.1},{y1:.1} C{x1:.1},{y0:.1} {x2:.1},{y0a:.1} {x3:.1},{y0a:.1} C{x3a:.1},{yt:.1} {x4:.1},{yt:.1} {x5:.1},{y0a:.1} C{x5a:.1},{yt:.1} {x6:.1},{yt2:.1} {x7:.1},{y2:.1} C{x8:.1},{y2:.1} {x8:.1},{y3:.1} {x7:.1},{y4:.1} C{x8:.1},{y5:.1} {x7:.1},{y6:.1} {x5:.1},{y6:.1} C{x4:.1},{y7:.1} {x2a:.1},{y7:.1} {x2:.1},{y6:.1} C{x1a:.1},{y7:.1} {x0:.1},{y5:.1} {x0:.1},{y4:.1} C{x0:.1},{y3:.1} {x1a:.1},{y1:.1} {x1:.1},{y1:.1} Z",
        x0 = w * 0.05,
        x1 = w * 0.18,
        x1a = w * 0.08,
        x2 = w * 0.3,
        x2a = w * 0.35,
        x3 = w * 0.4,
        x3a = w * 0.42,
        x4 = w * 0.55,
        x5 = w * 0.68,
        x5a = w * 0.78,
        x6 = w * 0.88,
        x7 = w * 0.85,
        x8 = w * 0.95,
        yt = h * 0.1,
        yt2 = h * 0.12,
        y0 = h * 0.25,
        y0a = h * 0.22,
        y1 = h * 0.35,
        y2 = h * 0.3,
        y3 = h * 0.45,
        y4 = h * 0.55,
        y5 = h * 0.7,
        y6 = h * 0.75,
        y7 = h * 0.85
    )
}
fn frame_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let t = w.min(h) * adj.get("adj").copied().unwrap_or(12500.0) / 100_000.0;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{t:.1},{t:.1} L{t:.1},{y:.1} L{x:.1},{y:.1} L{x:.1},{t:.1} Z",
        w = w,
        h = h,
        t = t,
        x = w - t,
        y = h - t
    )
}
fn ribbon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let f = w * adj.get("adj1").copied().unwrap_or(16667.0) / 100_000.0;
    let ratio2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let (bt, bb) = (h * (1.0 - ratio2), h * ratio2);
    format!(
        "M0,{bt:.1} L{f:.1},{bt:.1} L{f:.1},0 L{x:.1},0 L{x:.1},{bt:.1} L{w:.1},{bt:.1} L{x2:.1},{m:.1} L{w:.1},{bb:.1} L{x:.1},{bb:.1} L{x:.1},{h:.1} L{f:.1},{h:.1} L{f:.1},{bb:.1} L0,{bb:.1} L{f2:.1},{m:.1} Z",
        bt = bt,
        bb = bb,
        f = f,
        x = w - f,
        x2 = w - f * 0.3,
        f2 = f * 0.3,
        m = (bt + bb) / 2.0,
        w = w,
        h = h
    )
}
fn ribbon2_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    ribbon_path(w, h, adj)
}
fn donut_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let t = w.min(h) * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    let (ro, ryo) = (w / 2.0, h / 2.0);
    let cx = ro;
    format!(
        "M{cx:.1},0 A{ro:.1},{ryo:.1} 0 1,1 {cx:.1},{h:.1} A{ro:.1},{ryo:.1} 0 1,1 {cx:.1},0 Z M{cx:.1},{t:.1} A{ri:.1},{ryi:.1} 0 1,0 {cx:.1},{y:.1} A{ri:.1},{ryi:.1} 0 1,0 {cx:.1},{t:.1} Z",
        cx = cx,
        ro = ro,
        ryo = ryo,
        h = h,
        t = t,
        ri = (ro - t).max(0.1),
        ryi = (ryo - t).max(0.1),
        y = h - t
    )
}
fn no_smoking_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let (rx, ry) = (w / 2.0, h / 2.0);
    let cx = rx;
    let t = w.min(h) * adj.get("adj").copied().unwrap_or(18750.0) / 100_000.0;
    let a = std::f64::consts::FRAC_PI_4;
    let (ca, sa) = (a.cos(), a.sin());
    let (bx1, by1) = (cx - rx * ca, ry - ry * sa);
    let (bx2, by2) = (cx + rx * ca, ry + ry * sa);
    let (dx, dy) = (t / 2.0 * sa, t / 2.0 * ca);
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z M{a:.1},{b:.1} L{c:.1},{d:.1} L{e:.1},{f:.1} L{g:.1},{hh:.1} Z",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        a = bx1 + dx,
        b = by1 - dy,
        c = bx2 + dx,
        d = by2 - dy,
        e = bx2 - dx,
        f = by2 + dy,
        g = bx1 - dx,
        hh = by1 + dy
    )
}
fn block_arc_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        let (sx, sy) = scale_unit_point(w, h, 0.000000, 0.499873);
        let (c1x, c1y) = scale_unit_point(w, h, 0.000000, 0.411989);
        let (c2x, c2y) = scale_unit_point(w, h, 0.023114, 0.325883);
        let (x1, y1) = scale_unit_point(w, h, 0.067056, 0.249936);
        let (c3x, c3y) = scale_unit_point(w, h, 0.110998, 0.173990);
        let (c4x, c4y) = scale_unit_point(w, h, 0.173990, 0.110744);
        let (x2, y2) = scale_unit_point(w, h, 0.249936, 0.066802);
        let (c5x, c5y) = scale_unit_point(w, h, 0.325883, 0.022860);
        let (c6x, c6y) = scale_unit_point(w, h, 0.412243, 0.000000);
        let (x3, y3) = scale_unit_point(w, h, 0.500127, 0.000000);
        let (c7x, c7y) = scale_unit_point(w, h, 0.587757, 0.000000);
        let (c8x, c8y) = scale_unit_point(w, h, 0.674117, 0.022860);
        let (x4, y4) = scale_unit_point(w, h, 0.750064, 0.066802);
        let (c9x, c9y) = scale_unit_point(w, h, 0.826010, 0.110744);
        let (c10x, c10y) = scale_unit_point(w, h, 0.889002, 0.173990);
        let (x5, y5) = scale_unit_point(w, h, 0.932944, 0.249936);
        let (c11x, c11y) = scale_unit_point(w, h, 0.976886, 0.325883);
        let (c12x, c12y) = scale_unit_point(w, h, 1.000000, 0.411989);
        let (x6, y6) = scale_unit_point(w, h, 1.000000, 0.499873);
        let (lx1, ly1) = scale_unit_point(w, h, 0.749809, 0.499873);
        let (ic1x, ic1y) = scale_unit_point(w, h, 0.749809, 0.455931);
        let (ic2x, ic2y) = scale_unit_point(w, h, 0.738379, 0.412751);
        let (ix1, iy1) = scale_unit_point(w, h, 0.716281, 0.374905);
        let (ic3x, ic3y) = scale_unit_point(w, h, 0.694437, 0.336805);
        let (ic4x, ic4y) = scale_unit_point(w, h, 0.662687, 0.305309);
        let (ix2, iy2) = scale_unit_point(w, h, 0.624841, 0.283465);
        let (ic5x, ic5y) = scale_unit_point(w, h, 0.586741, 0.261367);
        let (ic6x, ic6y) = scale_unit_point(w, h, 0.543815, 0.249936);
        let (ix3, iy3) = scale_unit_point(w, h, 0.499873, 0.249936);
        let (ic7x, ic7y) = scale_unit_point(w, h, 0.455931, 0.249936);
        let (ic8x, ic8y) = scale_unit_point(w, h, 0.412751, 0.261367);
        let (ix4, iy4) = scale_unit_point(w, h, 0.374905, 0.283465);
        let (ic9x, ic9y) = scale_unit_point(w, h, 0.336805, 0.305309);
        let (ic10x, ic10y) = scale_unit_point(w, h, 0.305309, 0.336805);
        let (ix5, iy5) = scale_unit_point(w, h, 0.283211, 0.374905);
        let (ic11x, ic11y) = scale_unit_point(w, h, 0.261367, 0.412751);
        let (ic12x, ic12y) = scale_unit_point(w, h, 0.249936, 0.455931);
        let (ix6, iy6) = scale_unit_point(w, h, 0.249936, 0.499873);

        return format!(
            "M{sx:.1},{sy:.1} C{c1x:.1},{c1y:.1} {c2x:.1},{c2y:.1} {x1:.1},{y1:.1} C{c3x:.1},{c3y:.1} {c4x:.1},{c4y:.1} {x2:.1},{y2:.1} C{c5x:.1},{c5y:.1} {c6x:.1},{c6y:.1} {x3:.1},{y3:.1} C{c7x:.1},{c7y:.1} {c8x:.1},{c8y:.1} {x4:.1},{y4:.1} C{c9x:.1},{c9y:.1} {c10x:.1},{c10y:.1} {x5:.1},{y5:.1} C{c11x:.1},{c11y:.1} {c12x:.1},{c12y:.1} {x6:.1},{y6:.1} L{lx1:.1},{ly1:.1} C{ic1x:.1},{ic1y:.1} {ic2x:.1},{ic2y:.1} {ix1:.1},{iy1:.1} C{ic3x:.1},{ic3y:.1} {ic4x:.1},{ic4y:.1} {ix2:.1},{iy2:.1} C{ic5x:.1},{ic5y:.1} {ic6x:.1},{ic6y:.1} {ix3:.1},{iy3:.1} C{ic7x:.1},{ic7y:.1} {ic8x:.1},{ic8y:.1} {ix4:.1},{iy4:.1} C{ic9x:.1},{ic9y:.1} {ic10x:.1},{ic10y:.1} {ix5:.1},{iy5:.1} C{ic11x:.1},{ic11y:.1} {ic12x:.1},{ic12y:.1} {ix6:.1},{iy6:.1} L{sx:.1},{sy:.1} Z"
        );
    }

    let adj1 = adj.get("adj1").copied().unwrap_or(10800000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(0.0);
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let (ro, ryo) = (w / 2.0, h / 2.0);
    let t = w.min(h) * a3;
    let (cx, cy) = (ro, ryo);
    let start_angle = -std::f64::consts::FRAC_PI_2 + adj2 / 21_600_000.0 * std::f64::consts::TAU;
    let end_angle =
        std::f64::consts::PI + (adj1 - 10_800_000.0) / 21_600_000.0 * std::f64::consts::TAU;
    let (sx, sy) = ellipse_point(cx, cy, ro, ryo, start_angle);
    let (ex, ey) = ellipse_point(cx, cy, ro, ryo, end_angle);
    let ri = (ro - t).max(0.1);
    let ryi = (ryo - t).max(0.1);
    let (isx, isy) = ellipse_point(cx, cy, ri, ryi, start_angle);
    let (iex, iey) = ellipse_point(cx, cy, ri, ryi, end_angle);
    format!(
        "M{sx:.1},{sy:.1} A{ro:.1},{ryo:.1} 0 1,1 {ex:.1},{ey:.1} L{iex:.1},{iey:.1} A{ri:.1},{ryi:.1} 0 1,0 {isx:.1},{isy:.1} Z",
        sx = sx,
        sy = sy,
        ro = ro,
        ryo = ryo,
        ex = ex,
        ey = ey,
        iex = iex,
        iey = iey,
        ri = ri,
        ryi = ryi,
        isx = isx,
        isy = isy
    )
}
fn smiley_face_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let smile_adj = adj.get("adj").copied().unwrap_or(4653.0) / 100_000.0;
    let (rx, ry) = (w / 2.0, h / 2.0);
    let cx = rx;
    let (erx, ery) = (w * 0.05, h * 0.06);
    let (lcx, rcx, ecy) = (w * 0.35, w * 0.65, h * 0.38);
    let (sx1, sx2, sy) = (w * 0.3, w * 0.7, h * 0.6);
    let scy = sy + h * smile_adj;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z M{lr:.1},{ecy:.1} A{erx:.1},{ery:.1} 0 1,1 {ll:.1},{ecy:.1} A{erx:.1},{ery:.1} 0 1,1 {lr:.1},{ecy:.1} Z M{rr:.1},{ecy:.1} A{erx:.1},{ery:.1} 0 1,1 {rl:.1},{ecy:.1} A{erx:.1},{ery:.1} 0 1,1 {rr:.1},{ecy:.1} Z M{sx1:.1},{sy:.1} Q{cx:.1},{scy:.1} {sx2:.1},{sy:.1}",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        lr = lcx + erx,
        ll = lcx - erx,
        rr = rcx + erx,
        rl = rcx - erx,
        ecy = ecy,
        erx = erx,
        ery = ery,
        sx1 = sx1,
        sx2 = sx2,
        sy = sy,
        scy = scy
    )
}
fn can_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ry = h * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    let rx = w / 2.0;
    let (bt, bb) = (ry, h - ry);
    format!(
        "M0,{bt:.1} L0,{bb:.1} A{rx:.1},{ry:.1} 0 0,0 {w:.1},{bb:.1} L{w:.1},{bt:.1} A{rx:.1},{ry:.1} 0 0,0 0,{bt:.1} Z M0,{bt:.1} A{rx:.1},{ry:.1} 0 0,1 {w:.1},{bt:.1} A{rx:.1},{ry:.1} 0 0,1 0,{bt:.1} Z",
        bt = bt,
        bb = bb,
        rx = rx,
        ry = ry,
        w = w
    )
}
fn cube_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let d = w.min(h) * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    format!(
        "M0,{d:.1} L{d:.1},0 L{w:.1},0 L{w:.1},{y:.1} L{x:.1},{h:.1} L0,{h:.1} Z M0,{d:.1} L{d:.1},0 L{w:.1},0 L{x:.1},{d:.1} Z M{x:.1},{d:.1} L{w:.1},0 L{w:.1},{y:.1} L{x:.1},{h:.1} Z",
        d = d,
        x = w - d,
        y = h - d,
        w = w,
        h = h
    )
}
fn moon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(50000.0) / 100_000.0;
    let tip_x = w;
    let outer_rx = w * 0.95;
    let outer_ry = h / 2.0;
    let inner_rx = w * (0.45 + ratio.clamp(0.0, 1.0) * 0.20);
    let inner_ry = h * (0.45 + ratio.clamp(0.0, 1.0) * 0.05);
    format!(
        "M{tip_x:.1},0 A{outer_rx:.1},{outer_ry:.1} 0 1,0 {tip_x:.1},{h:.1} A{inner_rx:.1},{inner_ry:.1} 0 1,1 {tip_x:.1},0 Z",
        tip_x = tip_x,
        outer_rx = outer_rx,
        outer_ry = outer_ry,
        h = h,
        inner_rx = inner_rx,
        inner_ry = inner_ry
    )
}
fn sun_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let (cx, cy) = (w / 2.0, h / 2.0);
    let rb = w.min(h) * adj.get("adj").copied().unwrap_or(25000.0) / 100_000.0;
    let mut p = format!(
        "M{x1:.1},{cy:.1} A{r:.1},{r:.1} 0 1,1 {x2:.1},{cy:.1} A{r:.1},{r:.1} 0 1,1 {x1:.1},{cy:.1} Z",
        x1 = cx - rb,
        x2 = cx + rb,
        cy = cy,
        r = rb
    );
    let (ri, ro, rw) = (w.min(h) * 0.32, w.min(h) * 0.48, w.min(h) * 0.04);
    for i in 0..8 {
        let a = std::f64::consts::PI * (i as f64) / 4.0 - std::f64::consts::FRAC_PI_2;
        let (ca, sa) = (a.cos(), a.sin());
        let (pa, pb) = (
            (a + std::f64::consts::FRAC_PI_2).cos(),
            (a + std::f64::consts::FRAC_PI_2).sin(),
        );
        p.push_str(&format!(
            " M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z",
            cx + ri * ca + rw * pa,
            cy + ri * sa + rw * pb,
            cx + ro * ca,
            cy + ro * sa,
            cx + ri * ca - rw * pa,
            cy + ri * sa - rw * pb
        ));
    }
    p
}
fn bevel_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let t = w.min(h) * adj.get("adj").copied().unwrap_or(12500.0) / 100_000.0;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M0,0 L{t:.1},{t:.1} M{w:.1},0 L{x:.1},{t:.1} M{w:.1},{h:.1} L{x:.1},{y:.1} M0,{h:.1} L{t:.1},{y:.1}",
        w = w,
        h = h,
        t = t,
        x = w - t,
        y = h - t
    )
}
fn gear_path(w: f64, h: f64, teeth: u32) -> String {
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (ro, ri, rh) = (w.min(h) * 0.48, w.min(h) * 0.38, w.min(h) * 0.15);
    let t = teeth * 2;
    let mut pts: Vec<(f64, f64)> = Vec::with_capacity(t as usize);
    for i in 0..t {
        let a = 2.0 * std::f64::consts::PI * (i as f64) / (t as f64) - std::f64::consts::FRAC_PI_2;
        let r = if i % 2 == 0 { ro } else { ri };
        pts.push((cx + r * a.cos(), cy + r * a.sin()));
    }
    let mut p = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for &(x, y) in &pts[1..] {
        p.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    p.push_str(" Z");
    p.push_str(&format!(
        " M{:.1},{:.1} A{r:.1},{r:.1} 0 1,0 {:.1},{:.1} A{r:.1},{r:.1} 0 1,0 {:.1},{:.1} Z",
        cx - rh,
        cy,
        cx + rh,
        cy,
        cx - rh,
        cy,
        r = rh
    ));
    p
}
fn pie_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(0.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(16200000.0);
    let (rx, ry) = (w / 2.0, h / 2.0);
    let (cx, cy) = (rx, ry);
    let start_angle = -std::f64::consts::FRAC_PI_2 + adj1 / 21_600_000.0 * std::f64::consts::TAU;
    let sweep_angle = -(adj2 / 21_600_000.0 * std::f64::consts::TAU);
    let end_angle = start_angle + sweep_angle;
    let (sx, sy) = ellipse_point(cx, cy, rx, ry, start_angle);
    let (ex, ey) = ellipse_point(cx, cy, rx, ry, end_angle);
    format!(
        "M{cx:.1},{cy:.1} L{sx:.1},{sy:.1} A{rx:.1},{ry:.1} 0 {large_arc},{sweep_flag} {ex:.1},{ey:.1} Z",
        cx = cx,
        cy = cy,
        sx = sx,
        sy = sy,
        rx = rx,
        ry = ry,
        large_arc = if sweep_angle.abs() > std::f64::consts::PI {
            1
        } else {
            0
        },
        sweep_flag = if sweep_angle > 0.0 { 1 } else { 0 },
        ex = ex,
        ey = ey
    )
}
fn pie_wedge_path(w: f64, h: f64) -> String {
    format!(
        "M0,0 L{w:.1},0 A{w:.1},{h:.1} 0 0,1 0,{h:.1} Z",
        w = w,
        h = h
    )
}
fn arc_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(16200000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(0.0);
    let (rx, ry) = (w / 2.0, h / 2.0);
    let (cx, cy) = (rx, ry);
    let start_angle = std::f64::consts::PI + adj2 / 21_600_000.0 * std::f64::consts::TAU;
    let end_angle = (adj1 - 16_200_000.0) / 21_600_000.0 * std::f64::consts::TAU;
    let (sx, sy) = ellipse_point(cx, cy, rx, ry, start_angle);
    let (ex, ey) = ellipse_point(cx, cy, rx, ry, end_angle);
    format!(
        "M{sx:.1},{sy:.1} A{rx:.1},{ry:.1} 0 0,1 {ex:.1},{ey:.1}",
        sx = sx,
        sy = sy,
        rx = rx,
        ry = ry,
        ex = ex,
        ey = ey
    )
}

fn sample_ellipse_arc_points(
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    start_angle: f64,
    end_angle: f64,
    steps: usize,
) -> Vec<Point> {
    let mut points = Vec::with_capacity(steps.saturating_add(1));
    if steps == 0 {
        points.push(ellipse_point(cx, cy, rx, ry, start_angle));
        return points;
    }

    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let angle = start_angle + (end_angle - start_angle) * t;
        points.push(ellipse_point(cx, cy, rx, ry, angle));
    }
    points
}

fn normalize_vector(dx: f64, dy: f64) -> (f64, f64) {
    let len = (dx * dx + dy * dy).sqrt();
    if len <= f64::EPSILON {
        (0.0, 0.0)
    } else {
        (dx / len, dy / len)
    }
}

fn offset_polyline(points: &[Point], half_width: f64) -> PolylineSides {
    if points.len() < 2 {
        return (points.to_vec(), points.to_vec());
    }

    let mut left = Vec::with_capacity(points.len());
    let mut right = Vec::with_capacity(points.len());

    for i in 0..points.len() {
        let (px, py) = points[i];
        let prev_dir = if i == 0 {
            let (nx, ny) = points[i + 1];
            normalize_vector(nx - px, ny - py)
        } else {
            let (bx, by) = points[i - 1];
            normalize_vector(px - bx, py - by)
        };
        let next_dir = if i + 1 == points.len() {
            let (bx, by) = points[i - 1];
            normalize_vector(px - bx, py - by)
        } else {
            let (nx, ny) = points[i + 1];
            normalize_vector(nx - px, ny - py)
        };

        let prev_normal = (-prev_dir.1, prev_dir.0);
        let next_normal = (-next_dir.1, next_dir.0);
        let mut normal = (prev_normal.0 + next_normal.0, prev_normal.1 + next_normal.1);
        if normal.0.abs() <= f64::EPSILON && normal.1.abs() <= f64::EPSILON {
            normal = prev_normal;
        }
        let normal = normalize_vector(normal.0, normal.1);

        left.push((px + normal.0 * half_width, py + normal.1 * half_width));
        right.push((px - normal.0 * half_width, py - normal.1 * half_width));
    }

    (left, right)
}

fn polygon_path(points: &[Point]) -> String {
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

fn ribbon_path_from_centerline(
    centerline: &[Point],
    thickness: f64,
    start_head: bool,
    end_head: bool,
) -> String {
    if centerline.len() < 2 {
        return String::new();
    }

    let head_len = thickness * 1.35;
    let half_width = thickness / 2.0;
    let (left, right) = offset_polyline(centerline, half_width);
    let start_dir = normalize_vector(
        centerline[1].0 - centerline[0].0,
        centerline[1].1 - centerline[0].1,
    );
    let end_dir = normalize_vector(
        centerline[centerline.len() - 1].0 - centerline[centerline.len() - 2].0,
        centerline[centerline.len() - 1].1 - centerline[centerline.len() - 2].1,
    );

    let mut polygon = Vec::with_capacity(centerline.len() * 2 + 2);
    if start_head {
        let mid = (
            (left[0].0 + right[0].0) / 2.0,
            (left[0].1 + right[0].1) / 2.0,
        );
        polygon.push((
            mid.0 - start_dir.0 * head_len,
            mid.1 - start_dir.1 * head_len,
        ));
    }
    polygon.extend(left.iter().copied());
    if end_head {
        let last = centerline.len() - 1;
        let mid = (
            (left[last].0 + right[last].0) / 2.0,
            (left[last].1 + right[last].1) / 2.0,
        );
        polygon.push((mid.0 + end_dir.0 * head_len, mid.1 + end_dir.1 * head_len));
    }
    polygon.extend(right.iter().rev().copied());
    polygon_path(&polygon)
}

fn ellipse_point(cx: f64, cy: f64, rx: f64, ry: f64, angle: f64) -> (f64, f64) {
    (cx + rx * angle.cos(), cy + ry * angle.sin())
}

fn scale_unit_point(w: f64, h: f64, ux: f64, uy: f64) -> Point {
    (w * ux, h * uy)
}

fn scale_normalized_path(path: &str, w: f64, h: f64) -> String {
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

fn scale_normalized_path_about_center(
    path: &str,
    w: f64,
    h: f64,
    scale_x: f64,
    scale_y: f64,
) -> String {
    path.split_whitespace()
        .map(|token| {
            if token.len() == 1 && token.chars().all(|c| c.is_ascii_alphabetic()) {
                token.to_string()
            } else if let Some((x, y)) = token.split_once(',') {
                let x = (x.parse::<f64>().unwrap_or_default() - 0.5) * scale_x + 0.5;
                let y = (y.parse::<f64>().unwrap_or_default() - 0.5) * scale_y + 0.5;
                format!("{:.1},{:.1}", x * w, y * h)
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn wave_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a = h * adj.get("adj1").copied().unwrap_or(12500.0) / 100_000.0;
    let adj2 = adj.get("adj2").copied().unwrap_or(0.0) / 100_000.0;
    let phase = w * adj2 * 0.15;
    format!(
        "M0,{a:.1} C{c1:.1},0 {c2:.1},{a2:.1} {w:.1},{a:.1} L{w:.1},{y1:.1} C{c3:.1},{h:.1} {c4:.1},{y2:.1} 0,{y1:.1} Z",
        a = a,
        c1 = w * 0.25 + phase,
        c2 = w * 0.75 - phase,
        a2 = a * 2.0,
        w = w,
        y1 = h - a,
        h = h,
        c3 = w * 0.75 + phase,
        c4 = w * 0.25 - phase,
        y2 = h - a * 2.0
    )
}
fn double_wave_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a = h * adj.get("adj1").copied().unwrap_or(6250.0) / 100_000.0;
    let adj2 = adj.get("adj2").copied().unwrap_or(0.0) / 100_000.0;
    let phase = w * adj2 * 0.08;
    format!(
        "M0,{a:.1} C{c1:.1},0 {c2:.1},{a2:.1} {cx:.1},{a:.1} C{c3:.1},0 {c4:.1},{a2:.1} {w:.1},{a:.1} L{w:.1},{y1:.1} C{c5:.1},{h:.1} {c6:.1},{y2:.1} {cx:.1},{y1:.1} C{c7:.1},{h:.1} {c8:.1},{y2:.1} 0,{y1:.1} Z",
        a = a,
        a2 = a * 2.0,
        c1 = w * 0.125 + phase,
        c2 = w * 0.375 - phase,
        cx = w * 0.5,
        c3 = w * 0.625 + phase,
        c4 = w * 0.875 - phase,
        w = w,
        y1 = h - a,
        y2 = h - a * 2.0,
        c5 = w * 0.875 + phase,
        c6 = w * 0.625 - phase,
        c7 = w * 0.375 + phase,
        c8 = w * 0.125 - phase,
        h = h
    )
}
fn regular_polygon_path(w: f64, h: f64, sides: u32) -> String {
    let (cx, cy) = (w / 2.0, h / 2.0);
    let st = -std::f64::consts::FRAC_PI_2;
    let mut pts: Vec<(f64, f64)> = Vec::with_capacity(sides as usize);
    for i in 0..sides {
        let a = st + 2.0 * std::f64::consts::PI * (i as f64) / (sides as f64);
        pts.push((cx + cx * a.cos(), cy + cy * a.sin()));
    }
    let mut p = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
    for &(x, y) in &pts[1..] {
        p.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    p.push_str(" Z");
    p
}
fn funnel_path(w: f64, h: f64) -> String {
    let (sx, sy) = scale_unit_point(w, h, 0.004682, 0.284492);
    let (c1x, c1y) = scale_unit_point(w, h, 0.001490, 0.272852);
    let (c2x, c2y) = scale_unit_point(w, h, -0.000213, 0.261212);
    let (x1, y1) = scale_unit_point(w, h, -0.000213, 0.249572);
    let (c3x, c3y) = scale_unit_point(w, h, 0.022984, 0.205751);
    let (c4x, c4y) = scale_unit_point(w, h, 0.066823, 0.162616);
    let (x2, y2) = scale_unit_point(w, h, 0.110662, 0.124615);
    let (c5x, c5y) = scale_unit_point(w, h, 0.173867, 0.086614);
    let (c6x, c6y) = scale_unit_point(w, h, 0.249840, 0.055118);
    let (x3, y3) = scale_unit_point(w, h, 0.325814, 0.033208);
    let (c7x, c7y) = scale_unit_point(w, h, 0.412003, 0.011298);
    let (c8x, c8y) = scale_unit_point(w, h, 0.499894, -0.000342);
    let (x4, y4) = scale_unit_point(w, h, 0.587572, -0.000342);
    let (c9x, c9y) = scale_unit_point(w, h, 0.673760, 0.011298);
    let (c10x, c10y) = scale_unit_point(w, h, 0.749734, 0.033208);
    let (x5, y5) = scale_unit_point(w, h, 0.825920, 0.055118);
    let (c11x, c11y) = scale_unit_point(w, h, 0.888913, 0.086614);
    let (c12x, c12y) = scale_unit_point(w, h, 0.932752, 0.124615);
    let (x6, y6) = scale_unit_point(w, h, 0.976804, 0.162616);
    let (c13x, c13y) = scale_unit_point(w, h, 0.999787, 0.205751);
    let (c14x, c14y) = scale_unit_point(w, h, 0.999787, 0.249572);
    let (x7, y7) = scale_unit_point(w, h, 0.999787, 0.261212);
    let (c15x, c15y) = scale_unit_point(w, h, 0.998085, 0.272852);
    let (c16x, c16y) = scale_unit_point(w, h, 0.994893, 0.284492);
    let (x8, y8) = scale_unit_point(w, h, 0.623750, 0.945909);
    let (c17x, c17y) = scale_unit_point(w, h, 0.621622, 0.953783);
    let (c18x, c18y) = scale_unit_point(w, h, 0.616301, 0.961657);
    let (x9, y9) = scale_unit_point(w, h, 0.608215, 0.968504);
    let (c19x, c19y) = scale_unit_point(w, h, 0.597148, 0.978090);
    let (c20x, c20y) = scale_unit_point(w, h, 0.581400, 0.985964);
    let (x10, y10) = scale_unit_point(w, h, 0.562460, 0.991441);
    let (c21x, c21y) = scale_unit_point(w, h, 0.543520, 0.996919);
    let (c22x, c22y) = scale_unit_point(w, h, 0.521813, 0.999658);
    let (x11, y11) = scale_unit_point(w, h, 0.499894, 0.999658);
    let (c23x, c23y) = scale_unit_point(w, h, 0.477974, 0.999658);
    let (c24x, c24y) = scale_unit_point(w, h, 0.456480, 0.996919);
    let (x12, y12) = scale_unit_point(w, h, 0.437540, 0.991441);
    let (c25x, c25y) = scale_unit_point(w, h, 0.418387, 0.985964);
    let (c26x, c26y) = scale_unit_point(w, h, 0.402639, 0.978090);
    let (x13, y13) = scale_unit_point(w, h, 0.391785, 0.968504);
    let (c27x, c27y) = scale_unit_point(w, h, 0.383699, 0.961657);
    let (c28x, c28y) = scale_unit_point(w, h, 0.378378, 0.953783);
    let (x14, y14) = scale_unit_point(w, h, 0.376250, 0.945909);
    format!(
        "M{sx:.1},{sy:.1} C{c1x:.1},{c1y:.1} {c2x:.1},{c2y:.1} {x1:.1},{y1:.1} C{c3x:.1},{c3y:.1} {c4x:.1},{c4y:.1} {x2:.1},{y2:.1} C{c5x:.1},{c5y:.1} {c6x:.1},{c6y:.1} {x3:.1},{y3:.1} C{c7x:.1},{c7y:.1} {c8x:.1},{c8y:.1} {x4:.1},{y4:.1} C{c9x:.1},{c9y:.1} {c10x:.1},{c10y:.1} {x5:.1},{y5:.1} C{c11x:.1},{c11y:.1} {c12x:.1},{c12y:.1} {x6:.1},{y6:.1} C{c13x:.1},{c13y:.1} {c14x:.1},{c14y:.1} {x7:.1},{y7:.1} C{c15x:.1},{c15y:.1} {c16x:.1},{c16y:.1} {x8:.1},{y8:.1} C{c17x:.1},{c17y:.1} {c18x:.1},{c18y:.1} {x9:.1},{y9:.1} C{c19x:.1},{c19y:.1} {c20x:.1},{c20y:.1} {x10:.1},{y10:.1} C{c21x:.1},{c21y:.1} {c22x:.1},{c22y:.1} {x11:.1},{y11:.1} C{c23x:.1},{c23y:.1} {c24x:.1},{c24y:.1} {x12:.1},{y12:.1} C{c25x:.1},{c25y:.1} {c26x:.1},{c26y:.1} {x13:.1},{y13:.1} C{c27x:.1},{c27y:.1} {c28x:.1},{c28y:.1} {x14:.1},{y14:.1} L{sx:.1},{sy:.1} Z"
    )
}
fn teardrop_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let ratio = adj.get("adj").copied().unwrap_or(100000.0) / 100_000.0;
    let (rx, ry) = (w / 2.0, h / 2.0);
    let tip_x = rx + rx * ratio;
    format!(
        "M{tip_x:.1},0 L{tip_x:.1},{ry:.1} A{rx:.1},{ry:.1} 0 1,1 {rx:.1},0 Z",
        tip_x = tip_x.min(w),
        rx = rx,
        ry = ry
    )
}

// Arrow callout shapes
fn down_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn left_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn right_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn up_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn quad_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(18515.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(18515.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(18515.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(48123.0) / 100_000.0;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (sx, sy) = (w * a1, h * a1);
    let (ax, ay) = (w * a2, h * a2);
    let (bx, by) = (w * a4, h * a4);
    let _ = a3;
    format!(
        "M{cx:.1},0 L{x4:.1},{ay:.1} L{x3:.1},{ay:.1} L{x3:.1},{by:.1} L{bx:.1},{by:.1} L{bx:.1},{y1:.1} L{ax:.1},{y1:.1} L0,{cy:.1} L{ax:.1},{y2:.1} L{bx:.1},{y2:.1} L{bx:.1},{y4:.1} L{x3:.1},{y4:.1} L{x3:.1},{y3:.1} L{x4:.1},{y3:.1} L{cx:.1},{h:.1} L{x5:.1},{y3:.1} L{x6:.1},{y3:.1} L{x6:.1},{y4:.1} L{x7:.1},{y4:.1} L{x7:.1},{y2:.1} L{x8:.1},{y2:.1} L{w:.1},{cy:.1} L{x8:.1},{y1:.1} L{x7:.1},{y1:.1} L{x7:.1},{by:.1} L{x6:.1},{by:.1} L{x6:.1},{ay:.1} L{x5:.1},{ay:.1} Z",
        cx = cx,
        cy = cy,
        w = w,
        h = h,
        ax = ax,
        ay = ay,
        bx = bx,
        by = by,
        x3 = cx - sx,
        x4 = cx - sy,
        x5 = cx + sy,
        x6 = cx + sx,
        x7 = w - bx,
        x8 = w - ax,
        y1 = cy - sy,
        y2 = cy + sy,
        y3 = h - ay,
        y4 = h - by
    )
}
fn left_right_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(48123.0) / 100_000.0;
    let (cy, s, aw) = (h / 2.0, h * a1, w * a3);
    let (bx1, bx2) = (w * (1.0 - a4), w * a4);
    let _ = a2;
    format!(
        "M{bx1:.1},0 L{bx2:.1},0 L{bx2:.1},{y1:.1} L{xh:.1},{y1:.1} L{w:.1},{cy:.1} L{xh:.1},{y2:.1} L{bx2:.1},{y2:.1} L{bx2:.1},{h:.1} L{bx1:.1},{h:.1} L{bx1:.1},{y2:.1} L{aw:.1},{y2:.1} L0,{cy:.1} L{aw:.1},{y1:.1} L{bx1:.1},{y1:.1} Z",
        bx1 = bx1,
        bx2 = bx2,
        y1 = cy - s,
        y2 = cy + s,
        aw = aw,
        xh = w - aw,
        w = w,
        cy = cy,
        h = h
    )
}
fn up_down_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(25000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let a4 = adj.get("adj4").copied().unwrap_or(48123.0) / 100_000.0;
    let (cx, s, ah) = (w / 2.0, w * a1, h * a3);
    let (by1, by2) = (h * (1.0 - a4), h * a4);
    let _ = a2;
    format!(
        "M0,{by1:.1} L{x1:.1},{by1:.1} L{x1:.1},{ah:.1} L{cx:.1},0 L{x2:.1},{ah:.1} L{x2:.1},{by1:.1} L{w:.1},{by1:.1} L{w:.1},{by2:.1} L{x2:.1},{by2:.1} L{x2:.1},{yh:.1} L{cx:.1},{h:.1} L{x1:.1},{yh:.1} L{x1:.1},{by2:.1} L0,{by2:.1} Z",
        by1 = by1,
        by2 = by2,
        x1 = cx - s,
        x2 = cx + s,
        ah = ah,
        yh = h - ah,
        cx = cx,
        w = w,
        h = h
    )
}

// Brackets and braces
fn left_brace_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.970146,0.954632 L 0.970146,0.954632 C 0.887509,0.954632 0.806544,0.951051 0.735133,0.944126 0.663721,0.937202 0.604251,0.927412 0.562933,0.915473 0.521615,0.903534 0.500119,0.890162 0.500119,0.876313 L 0.499881,0.562798 0.499881,0.562798 0.499881,0.562798 C 0.499881,0.548949 0.478147,0.535578 0.436828,0.523639 0.395510,0.511700 0.336279,0.501910 0.264867,0.494986 0.193456,0.488061 0.112252,0.484479 0.029615,0.484479 L 0.029615,0.484479 C 0.112252,0.484479 0.193456,0.480898 0.264867,0.473973 0.336279,0.467049 0.395510,0.457259 0.436828,0.445320 0.478147,0.433381 0.499881,0.419771 0.499881,0.406160 L 0.499881,0.092884 0.499881,0.092884 C 0.499881,0.079035 0.521615,0.065664 0.562933,0.053725 0.604251,0.041786 0.663482,0.031996 0.734894,0.025072 0.806305,0.018147 0.887509,0.014565 0.970146,0.014565 L 0.970146,0.954632 Z",
            w,
            h,
        );
    }

    let r = h * adj.get("adj1").copied().unwrap_or(8333.0) / 100_000.0;
    let cy = h * adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let x = w * 0.7;
    format!(
        "M{x:.1},0 Q{xm:.1},0 {xm:.1},{r:.1} L{xm:.1},{y1:.1} Q{xm:.1},{cy:.1} 0,{cy:.1} Q{xm:.1},{cy:.1} {xm:.1},{y2:.1} L{xm:.1},{y3:.1} Q{xm:.1},{h:.1} {x:.1},{h:.1}",
        x = x,
        xm = x * 0.5,
        r = r,
        y1 = cy - r,
        cy = cy,
        y2 = cy + r,
        y3 = h - r,
        h = h
    )
}
fn right_brace_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.029854,0.014333 L 0.029854,0.014333 C 0.112491,0.014333 0.193456,0.017917 0.264867,0.024845 0.336279,0.031773 0.395749,0.041567 0.437067,0.053512 0.478385,0.065456 0.500119,0.078834 0.500119,0.092690 L 0.500119,0.092690 0.499881,0.406116 0.499881,0.406116 C 0.499881,0.419971 0.521615,0.433349 0.562933,0.445294 0.604251,0.457238 0.663482,0.467033 0.734894,0.473961 0.806305,0.480889 0.887509,0.484472 0.970146,0.484472 L 0.970146,0.484472 C 0.887509,0.484472 0.806305,0.488055 0.734894,0.494983 0.663482,0.501911 0.604251,0.511706 0.562933,0.523650 0.521615,0.535595 0.499881,0.549212 0.499881,0.562828 L 0.499881,0.876254 0.499881,0.876254 C 0.499881,0.890110 0.478147,0.903488 0.436828,0.915432 0.395510,0.927377 0.336279,0.937172 0.264867,0.944099 0.193456,0.951027 0.112252,0.954611 0.029854,0.954611 L 0.029854,0.014333 Z",
            w,
            h,
        );
    }

    let r = h * adj.get("adj1").copied().unwrap_or(8333.0) / 100_000.0;
    let cy = h * adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let x = w * 0.3;
    format!(
        "M{x:.1},0 Q{xm:.1},0 {xm:.1},{r:.1} L{xm:.1},{y1:.1} Q{xm:.1},{cy:.1} {w:.1},{cy:.1} Q{xm:.1},{cy:.1} {xm:.1},{y2:.1} L{xm:.1},{y3:.1} Q{xm:.1},{h:.1} {x:.1},{h:.1}",
        x = x,
        xm = w - x * 0.5,
        r = r,
        y1 = cy - r,
        cy = cy,
        y2 = cy + r,
        y3 = h - r,
        h = h,
        w = w
    )
}
fn left_bracket_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let r = h * adj.get("adj").copied().unwrap_or(8333.0) / 100_000.0;
    let x = w * 0.7;
    format!(
        "M{x:.1},0 L{r:.1},0 Q0,0 0,{r:.1} L0,{y:.1} Q0,{h:.1} {r:.1},{h:.1} L{x:.1},{h:.1}",
        x = x,
        r = r,
        y = h - r,
        h = h
    )
}
fn right_bracket_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let r = h * adj.get("adj").copied().unwrap_or(8333.0) / 100_000.0;
    let x = w * 0.3;
    format!(
        "M{x:.1},0 L{xr:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y:.1} Q{w:.1},{h:.1} {xr:.1},{h:.1} L{x:.1},{h:.1}",
        x = x,
        xr = w - r,
        w = w,
        r = r,
        y = h - r,
        h = h
    )
}

// Chart shapes
fn chart_plus_path(w: f64, h: f64) -> String {
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
fn chart_star_path(w: f64, h: f64) -> String {
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
fn chart_x_path(w: f64, h: f64) -> String {
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

// Scrolls
fn horizontal_scroll_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let r = w.min(h) * adj.get("adj").copied().unwrap_or(12500.0) / 100_000.0;
    let r2 = r / 2.0;
    let (x, y1, y2) = (w - r, h - r, h - r2);
    format!(
        "M{r:.1},{r:.1} L{x:.1},{r:.1} A{r2:.1},{r2:.1} 0 0,1 {x:.1},{r2:.1} A{r2:.1},{r2:.1} 0 0,1 {w:.1},{r:.1} L{w:.1},{y1:.1} A{r2:.1},{r2:.1} 0 0,1 {w:.1},{y2:.1} A{r2:.1},{r2:.1} 0 0,1 {x:.1},{y1:.1} L{r:.1},{y1:.1} A{r2:.1},{r2:.1} 0 0,1 {r:.1},{y2:.1} A{r2:.1},{r2:.1} 0 0,1 0,{y1:.1} L0,{r:.1} A{r2:.1},{r2:.1} 0 0,1 0,{r2:.1} A{r2:.1},{r2:.1} 0 0,1 {r:.1},{r:.1} Z",
        r = r,
        r2 = r2,
        x = x,
        w = w,
        y1 = y1,
        y2 = y2
    )
}
fn vertical_scroll_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let r = w.min(h) * adj.get("adj").copied().unwrap_or(12500.0) / 100_000.0;
    let body_left = r * 0.8;
    let body_top = r * 1.2;
    let body_right = w - r * 0.4;
    let body_bottom = h - r * 0.2;
    let body_radius = r * 0.45;
    let lip_left = r * 0.35;
    let lip_right = w;
    let lip_height = r * 1.1;
    let curl_r = r * 0.45;
    format!(
        "M{body_left:.1},{body_top:.1} L{body_right_minus:.1},{body_top:.1} Q{body_right:.1},{body_top:.1} {body_right:.1},{body_top_plus:.1} L{body_right:.1},{body_bottom_minus:.1} Q{body_right:.1},{body_bottom:.1} {body_right_minus:.1},{body_bottom:.1} L{body_left:.1},{body_bottom:.1} Q0,{body_bottom:.1} 0,{body_bottom_minus:.1} L0,{body_top_plus:.1} Q0,{body_top:.1} {body_left:.1},{body_top:.1} Z \
         M{lip_left_plus:.1},0 L{lip_right_minus:.1},0 Q{lip_right:.1},0 {lip_right:.1},{curl_r:.1} Q{lip_right:.1},{lip_height:.1} {lip_right_minus:.1},{lip_height:.1} L{lip_left_plus:.1},{lip_height:.1} Q{lip_left:.1},{lip_height:.1} {lip_left:.1},{curl_r:.1} Q{lip_left:.1},0 {lip_left_plus:.1},0 Z \
         M{curl_center:.1},{curl_r:.1} A{curl_r:.1},{curl_r:.1} 0 1,1 {curl_center:.1},{lip_height_minus:.1} A{curl_r:.1},{curl_r:.1} 0 1,1 {curl_center:.1},{curl_r:.1} Z \
         M{curl_center_bottom:.1},{body_bottom_minus_curl:.1} A{curl_r:.1},{curl_r:.1} 0 1,1 {curl_center_bottom:.1},{body_bottom_plus_curl:.1} A{curl_r:.1},{curl_r:.1} 0 1,1 {curl_center_bottom:.1},{body_bottom_minus_curl:.1} Z",
        body_left = body_left,
        body_right = body_right,
        body_right_minus = body_right - body_radius,
        body_top = body_top,
        body_top_plus = body_top + body_radius,
        body_bottom = body_bottom,
        body_bottom_minus = body_bottom - body_radius,
        lip_left = lip_left,
        lip_left_plus = lip_left + curl_r,
        lip_right = lip_right,
        lip_right_minus = lip_right - curl_r,
        lip_height = lip_height,
        lip_height_minus = lip_height - curl_r,
        curl_r = curl_r,
        curl_center = lip_left + curl_r,
        curl_center_bottom = curl_r,
        body_bottom_minus_curl = body_bottom - curl_r * 2.0,
        body_bottom_plus_curl = body_bottom
    )
}

// Tabs
fn corner_tabs_path(w: f64, h: f64) -> String {
    scale_normalized_path_about_center(
        "M 0.000000,0.000000 L 0.120000,0.000000 0.000000,0.120000 Z M 0.880000,0.000000 L 1.000000,0.000000 1.000000,0.120000 Z M 1.000000,0.880000 L 1.000000,1.000000 0.880000,1.000000 Z M 0.000000,0.880000 L 0.120000,1.000000 0.000000,1.000000 Z",
        w,
        h,
        1.045,
        1.040,
    )
}
fn plaque_tabs_path(w: f64, h: f64) -> String {
    let r = w.min(h) * 0.12;
    format!(
        "M0,0 L{r:.1},0 Q0,0 0,{r:.1} Z M{x:.1},0 L{w:.1},0 Q{w:.1},0 {w:.1},{r:.1} Z M{w:.1},{y:.1} Q{w:.1},{h:.1} {x:.1},{h:.1} L{w:.1},{h:.1} Z M0,{y:.1} Q0,{h:.1} {r:.1},{h:.1} L0,{h:.1} Z",
        r = r,
        x = w - r,
        w = w,
        y = h - r,
        h = h
    )
}
fn square_tabs_path(w: f64, h: f64) -> String {
    scale_normalized_path_about_center(
        "M 0.000000,0.000000 L 0.100000,0.000000 0.100000,0.100000 0.000000,0.100000 Z M 0.900000,0.000000 L 1.000000,0.000000 1.000000,0.100000 0.900000,0.100000 Z M 0.900000,0.900000 L 1.000000,0.900000 1.000000,1.000000 0.900000,1.000000 Z M 0.000000,0.900000 L 0.100000,0.900000 0.100000,1.000000 0.000000,1.000000 Z",
        w,
        h,
        1.030,
        1.035,
    )
}

// Ribbons
fn ellipse_ribbon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn ellipse_ribbon2_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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

// Circular arrows
fn left_circular_arrow_path(w: f64, h: f64) -> String {
    let thickness = w.min(h) * 0.16;
    let cx = w * 0.50;
    let cy = h * 0.50;
    let rx = w * 0.40;
    let ry = h * 0.40;
    let centerline = sample_ellipse_arc_points(cx, cy, rx, ry, 3.35, -0.30, 28);
    ribbon_path_from_centerline(&centerline, thickness, false, true)
}
fn left_right_circular_arrow_path(w: f64, h: f64) -> String {
    let thickness = w.min(h) * 0.18;
    let cx = w * 0.50;
    let cy = h * 0.62;
    let rx = w * 0.41;
    let ry = h * 0.49;
    let centerline = sample_ellipse_arc_points(cx, cy, rx, ry, 3.50, 5.92, 28);
    ribbon_path_from_centerline(&centerline, thickness, true, true)
}

// Misc shapes
fn chord_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(2700000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(16200000.0);
    let (rx, ry) = (w / 2.0, h / 2.0);
    let (cx, cy) = (rx, ry);
    let start_angle = std::f64::consts::PI * 5.0 / 6.0
        - (adj1 - 2_700_000.0) / 21_600_000.0 * std::f64::consts::TAU;
    let end_angle =
        std::f64::consts::PI / 6.0 + (adj2 - 16_200_000.0) / 21_600_000.0 * std::f64::consts::TAU;
    let (x1, y1) = ellipse_point(cx, cy, rx, ry, start_angle);
    let (x2, y2) = ellipse_point(cx, cy, rx, ry, end_angle);
    format!(
        "M{x1:.1},{y1:.1} A{rx:.1},{ry:.1} 0 1,1 {x2:.1},{y2:.1} L{x1:.1},{y1:.1} Z",
        x1 = x1,
        y1 = y1,
        rx = rx,
        ry = ry,
        x2 = x2,
        y2 = y2
    )
}
fn line_inv_path(w: f64, h: f64) -> String {
    format!("M0,{h:.1} L{w:.1},0", w = w, h = h)
}
fn non_isosceles_trapezoid_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn swoosh_arrow_path(w: f64, h: f64) -> String {
    scale_normalized_path(
        "M 0.036738,0.972701 L 0.028315,0.959195 0.065376,0.834943 0.139498,0.672874 0.245627,0.518908 0.350072,0.410862 0.589283,0.248793 0.862186,0.146149 0.862186,0.027299 0.971685,0.213678 0.895878,0.508103 0.884086,0.402759 0.858817,0.394655 0.676882,0.432471 0.447778,0.524310 0.323118,0.605345 0.191720,0.729598 Z",
        w,
        h,
    )
}
fn left_right_ribbon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(16667.0) / 100_000.0;
    let (f, bt, bb) = (w * a3, h * (1.0 - a2), h * a2);
    let _ = a1;
    let m = (bt + bb) / 2.0;
    format!(
        "M0,{bt:.1} L{f:.1},{bt:.1} L{f:.1},{m1:.1} L0,{m:.1} L{f:.1},{m2:.1} L{f:.1},{bb:.1} L0,{bb:.1} L{f:.1},{h:.1} L{x:.1},{h:.1} L{x:.1},{bb:.1} L{w:.1},{bb:.1} L{x2:.1},{m:.1} L{w:.1},{bt:.1} L{x:.1},{bt:.1} L{x:.1},0 L{f:.1},0 Z",
        f = f,
        bt = bt,
        bb = bb,
        m = m,
        m1 = m - h * 0.05,
        m2 = m + h * 0.05,
        h = h,
        x = w - f,
        x2 = w - f * 0.3,
        w = w
    )
}
fn flowchart_offline_storage_path(w: f64, h: f64) -> String {
    let i = w * 0.15;
    format!(
        "M0,0 L{w:.1},0 L{x:.1},{h:.1} L{i:.1},{h:.1} Z",
        w = w,
        x = w - i,
        i = i,
        h = h
    )
}
fn curved_connector2_path(w: f64, h: f64) -> String {
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
fn curved_connector3_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn curved_connector4_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn curved_connector5_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn bent_connector2_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},0 L{w:.1},{h:.1}", w = w, h = h)
}
fn bent_connector3_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let cx = w * adj1 / 100000.0;
    format!(
        "M0,0 L{cx:.1},0 L{cx:.1},{h:.1} L{w:.1},{h:.1}",
        cx = cx,
        w = w,
        h = h
    )
}
fn bent_connector4_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
fn bent_connector5_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_shape_svg_returns_none_for_unknown() {
        let adj = HashMap::new();
        assert!(preset_shape_svg("unknownShape", 100.0, 100.0, &adj).is_none());
    }

    #[test]
    fn test_total_supported_shapes_at_least_187() {
        let adj = HashMap::new();
        let all = [
            "rect",
            "roundRect",
            "ellipse",
            "triangle",
            "isosTriangle",
            "rtTriangle",
            "diamond",
            "parallelogram",
            "trapezoid",
            "pentagon",
            "hexagon",
            "octagon",
            "snip1Rect",
            "snip2SameRect",
            "snip2DiagRect",
            "snipRoundRect",
            "round1Rect",
            "round2SameRect",
            "round2DiagRect",
            "foldCorner",
            "diagStripe",
            "corner",
            "plaque",
            "bracePair",
            "bracketPair",
            "halfFrame",
            "line",
            "rightArrow",
            "leftArrow",
            "upArrow",
            "downArrow",
            "leftRightArrow",
            "upDownArrow",
            "bentArrow",
            "chevron",
            "notchedRightArrow",
            "stripedRightArrow",
            "curvedRightArrow",
            "curvedLeftArrow",
            "curvedUpArrow",
            "curvedDownArrow",
            "circularArrow",
            "bentUpArrow",
            "uturnArrow",
            "leftRightUpArrow",
            "quadArrow",
            "leftUpArrow",
            "homePlate",
            "wedgeRoundRectCallout",
            "wedgeEllipseCallout",
            "cloudCallout",
            "callout1",
            "callout2",
            "callout3",
            "borderCallout1",
            "borderCallout2",
            "borderCallout3",
            "accentCallout1",
            "accentCallout2",
            "accentCallout3",
            "accentBorderCallout1",
            "accentBorderCallout2",
            "accentBorderCallout3",
            "wedgeRectCallout",
            "flowChartProcess",
            "flowChartDecision",
            "flowChartTerminator",
            "flowChartDocument",
            "flowChartPredefinedProcess",
            "flowChartAlternateProcess",
            "flowChartManualInput",
            "flowChartConnector",
            "flowChartInputOutput",
            "flowChartInternalStorage",
            "flowChartMultidocument",
            "flowChartPreparation",
            "flowChartManualOperation",
            "flowChartOffpageConnector",
            "flowChartPunchedCard",
            "flowChartPunchedTape",
            "flowChartSummingJunction",
            "flowChartOr",
            "flowChartCollate",
            "flowChartSort",
            "flowChartExtract",
            "flowChartMerge",
            "flowChartOnlineStorage",
            "flowChartDelay",
            "flowChartMagneticTape",
            "flowChartMagneticDisk",
            "flowChartMagneticDrum",
            "flowChartDisplay",
            "actionButtonBlank",
            "actionButtonHome",
            "actionButtonHelp",
            "actionButtonInformation",
            "actionButtonBackPrevious",
            "actionButtonForwardNext",
            "actionButtonBeginning",
            "actionButtonEnd",
            "actionButtonReturn",
            "actionButtonDocument",
            "actionButtonSound",
            "actionButtonMovie",
            "star4",
            "star5",
            "star6",
            "star7",
            "star8",
            "star10",
            "star12",
            "star16",
            "star24",
            "star32",
            "irregularSeal1",
            "irregularSeal2",
            "mathPlus",
            "mathEqual",
            "mathNotEqual",
            "mathMultiply",
            "mathDivide",
            "mathMinus",
            "heart",
            "plus",
            "lightningBolt",
            "cloud",
            "frame",
            "ribbon",
            "ribbon2",
            "donut",
            "noSmoking",
            "blockArc",
            "smileyFace",
            "can",
            "cube",
            "moon",
            "sun",
            "bevel",
            "gear6",
            "gear9",
            "pie",
            "pieWedge",
            "arc",
            "wave",
            "doubleWave",
            "decagon",
            "dodecagon",
            "funnel",
            "teardrop",
            "heptagon",
            "downArrowCallout",
            "leftArrowCallout",
            "rightArrowCallout",
            "upArrowCallout",
            "quadArrowCallout",
            "leftRightArrowCallout",
            "upDownArrowCallout",
            "leftBrace",
            "rightBrace",
            "leftBracket",
            "rightBracket",
            "chartPlus",
            "chartStar",
            "chartX",
            "horizontalScroll",
            "verticalScroll",
            "cornerTabs",
            "plaqueTabs",
            "squareTabs",
            "ellipseRibbon",
            "ellipseRibbon2",
            "leftCircularArrow",
            "leftRightCircularArrow",
            "chord",
            "lineInv",
            "nonIsoscelesTrapezoid",
            "swooshArrow",
            "leftRightRibbon",
            "flowChartOfflineStorage",
            "cross",
            "curvedConnector2",
            "curvedConnector3",
            "curvedConnector4",
            "curvedConnector5",
            "bentConnector2",
            "bentConnector3",
            "bentConnector4",
        ];
        let supported: Vec<_> = all
            .iter()
            .filter(|n| preset_shape_svg(n, 100.0, 100.0, &adj).is_some())
            .collect();
        let unsupported: Vec<_> = all
            .iter()
            .filter(|n| preset_shape_svg(n, 100.0, 100.0, &adj).is_none())
            .collect();
        assert!(unsupported.is_empty(), "Unsupported: {:?}", unsupported);
        assert!(
            supported.len() >= 187,
            "Expected >= 187, got {}",
            supported.len()
        );
    }

    #[test]
    fn test_flowchart_all_28_shapes() {
        let adj = HashMap::new();
        for name in [
            "flowChartProcess",
            "flowChartAlternateProcess",
            "flowChartDecision",
            "flowChartInputOutput",
            "flowChartPredefinedProcess",
            "flowChartInternalStorage",
            "flowChartDocument",
            "flowChartMultidocument",
            "flowChartTerminator",
            "flowChartPreparation",
            "flowChartManualInput",
            "flowChartManualOperation",
            "flowChartConnector",
            "flowChartOffpageConnector",
            "flowChartPunchedCard",
            "flowChartPunchedTape",
            "flowChartSummingJunction",
            "flowChartOr",
            "flowChartCollate",
            "flowChartSort",
            "flowChartExtract",
            "flowChartMerge",
            "flowChartOnlineStorage",
            "flowChartDelay",
            "flowChartMagneticTape",
            "flowChartMagneticDisk",
            "flowChartMagneticDrum",
            "flowChartDisplay",
        ] {
            assert!(
                preset_shape_svg(name, 100.0, 100.0, &adj).is_some(),
                "Missing: {name}"
            );
        }
    }

    #[test]
    fn test_action_buttons_all_12() {
        let adj = HashMap::new();
        for name in [
            "actionButtonBlank",
            "actionButtonHome",
            "actionButtonHelp",
            "actionButtonInformation",
            "actionButtonBackPrevious",
            "actionButtonForwardNext",
            "actionButtonBeginning",
            "actionButtonEnd",
            "actionButtonReturn",
            "actionButtonDocument",
            "actionButtonSound",
            "actionButtonMovie",
        ] {
            assert!(
                preset_shape_svg(name, 100.0, 100.0, &adj).is_some(),
                "Missing: {name}"
            );
        }
    }

    #[test]
    fn test_action_button_unknown_icon_falls_back_to_blank_frame() {
        let blank = action_button_blank_path(100.0, 100.0);
        let unknown = action_button_icon_path(100.0, 100.0, "mystery");

        assert_eq!(unknown, format!("{blank} "));
    }

    #[test]
    fn test_scale_normalized_path_about_center_expands_coordinates_symmetrically() {
        let path =
            scale_normalized_path_about_center("M 0.0,0.0 L 1.0,1.0", 100.0, 100.0, 1.1, 1.1);

        assert_eq!(path, "M -5.0,-5.0 L 105.0,105.0");
    }

    #[test]
    fn test_right_arrow_default_path_uses_narrower_head_length() {
        let adj = HashMap::new();
        let path = right_arrow_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M0,25.0 L80.0,25.0 L80.0,0 L120.0,50.0 L80.0,100.0 L80.0,75.0 L0,75.0 Z"
        );
    }

    #[test]
    fn test_left_arrow_default_path_uses_narrower_head_length() {
        let adj = HashMap::new();
        let path = left_arrow_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M120.0,25.0 L40.0,25.0 L40.0,0 L0,50.0 L40.0,100.0 L40.0,75.0 L120.0,75.0 Z"
        );
    }

    #[test]
    fn test_up_arrow_default_path_widens_the_shaft() {
        let adj = HashMap::new();
        let path = up_arrow_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M15.0,100.0 L15.0,50.0 L0,50.0 L60.0,0 L120.0,50.0 L105.0,50.0 L105.0,100.0 Z"
        );
    }

    #[test]
    fn test_down_arrow_default_path_widens_the_shaft() {
        let adj = HashMap::new();
        let path = down_arrow_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M15.0,0 L105.0,0 L105.0,50.0 L120.0,50.0 L60.0,100.0 L0,50.0 L15.0,50.0 Z"
        );
    }

    #[test]
    fn test_folded_corner_alias_uses_fold_corner_geometry() {
        let adj = HashMap::new();
        let path = preset_shape_svg("foldedCorner", 120.0, 100.0, &adj).unwrap();

        assert_eq!(
            path,
            "M0,0 L120.0,0 L120.0,83.3 L103.3,100.0 L0,100.0 Z M103.3,100.0 L120.0,83.3 L103.3,83.3 Z"
        );
    }

    #[test]
    fn test_diag_stripe_default_path_spans_full_diagonal_band() {
        let adj = HashMap::new();
        let path = diag_stripe_path(120.0, 100.0, &adj);

        assert_eq!(path, "M0,100.0 L0,55.0 L42.0,0 L120.0,0 Z");
    }

    #[test]
    fn test_pie_default_path_renders_three_quarter_sector() {
        let adj = HashMap::new();
        let path = pie_path(120.0, 100.0, &adj);

        assert_eq!(path, "M60.0,50.0 L60.0,0.0 A60.0,50.0 0 1,0 120.0,50.0 Z");
    }

    #[test]
    fn test_moon_default_path_faces_right_with_left_bulge() {
        let adj = HashMap::new();
        let path = moon_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M120.0,0 A114.0,50.0 0 1,0 120.0,100.0 A66.0,47.5 0 1,1 120.0,0 Z"
        );
    }

    #[test]
    fn test_bevel_default_path_keeps_filled_face() {
        let adj = HashMap::new();
        let path = bevel_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M0,0 L120.0,0 L120.0,100.0 L0,100.0 Z M0,0 L12.5,12.5 M120.0,0 L107.5,12.5 M120.0,100.0 L107.5,87.5 M0,100.0 L12.5,87.5"
        );
    }

    #[test]
    fn test_brace_pair_default_path_keeps_center_fill_between_braces() {
        let adj = HashMap::new();
        let path = brace_pair_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M13.3,0 L106.7,0 Q120.0,0 120.0,13.3 L120.0,41.7 Q120.0,50.0 111.7,50.0 Q120.0,50.0 120.0,58.3 L120.0,86.7 Q120.0,100.0 106.7,100.0 L13.3,100.0 Q0,100.0 0,86.7 L0,58.3 Q0,50.0 8.3,50.0 Q0,50.0 0,41.7 L0,13.3 Q0,0 13.3,0 Z"
        );
    }

    #[test]
    fn test_vertical_scroll_default_path_keeps_filled_body_and_rolls() {
        let adj = HashMap::new();
        let path = vertical_scroll_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M10.0,15.0 L109.4,15.0 Q115.0,15.0 115.0,20.6 L115.0,91.9 Q115.0,97.5 109.4,97.5 L10.0,97.5 Q0,97.5 0,91.9 L0,20.6 Q0,15.0 10.0,15.0 Z M10.0,0 L114.4,0 Q120.0,0 120.0,5.6 Q120.0,13.8 114.4,13.8 L10.0,13.8 Q4.4,13.8 4.4,5.6 Q4.4,0 10.0,0 Z M10.0,5.6 A5.6,5.6 0 1,1 10.0,8.1 A5.6,5.6 0 1,1 10.0,5.6 Z M5.6,86.2 A5.6,5.6 0 1,1 5.6,97.5 A5.6,5.6 0 1,1 5.6,86.2 Z"
        );
    }

    #[test]
    fn test_left_circular_arrow_default_path_tracks_u_shape_reference() {
        let path = left_circular_arrow_path(100.0, 140.0);

        assert_eq!(
            path,
            "M2.9,57.6 L2.1,65.2 L2.1,73.2 L2.7,81.2 L4.2,89.1 L6.3,96.6 L9.1,103.8 L12.7,110.4 L16.9,116.5 L21.7,121.8 L27.0,126.3 L32.9,129.8 L39.1,132.4 L45.7,133.7 L52.3,133.9 L58.9,132.9 L65.3,130.7 L71.3,127.4 L76.8,123.2 L81.8,118.1 L86.1,112.3 L89.9,105.8 L92.9,98.8 L95.3,91.3 L96.9,83.6 L97.8,75.6 L98.0,67.6 L97.4,59.6 L96.1,52.1 L84.6,32.2 L80.3,54.8 L81.5,61.5 L82.0,68.0 L81.9,74.6 L81.1,81.0 L79.8,87.3 L77.9,93.2 L75.6,98.6 L72.8,103.5 L69.6,107.7 L66.2,111.3 L62.6,114.0 L58.8,116.1 L55.1,117.4 L51.3,118.0 L47.6,117.9 L43.8,117.1 L40.0,115.5 L36.4,113.3 L32.8,110.3 L29.4,106.5 L26.3,102.1 L23.7,97.0 L21.4,91.4 L19.7,85.4 L18.6,79.1 L18.0,72.6 L18.1,66.1 L18.8,59.2 Z"
        );
    }

    #[test]
    fn test_left_right_circular_arrow_default_path_tracks_arch_reference() {
        let path = left_right_circular_arrow_path(100.0, 140.0);

        assert_eq!(
            path,
            "M5.6,86.3 L20.3,65.0 L21.6,59.7 L23.1,55.0 L24.9,50.6 L26.8,46.5 L28.9,42.7 L31.1,39.3 L33.4,36.3 L35.8,33.8 L38.3,31.7 L40.7,30.0 L43.1,28.8 L45.4,27.9 L47.7,27.4 L49.9,27.2 L52.2,27.4 L54.5,27.8 L56.8,28.7 L59.2,29.9 L61.6,31.6 L64.0,33.7 L66.5,36.2 L68.8,39.1 L71.0,42.5 L73.1,46.3 L75.0,50.4 L76.8,54.8 L78.3,59.5 L79.6,64.7 L94.4,86.0 L97.0,60.2 L95.6,54.5 L93.7,48.7 L91.6,43.3 L89.1,38.0 L86.4,33.1 L83.3,28.6 L80.0,24.4 L76.4,20.6 L72.6,17.3 L68.4,14.5 L64.0,12.2 L59.4,10.5 L54.7,9.5 L49.9,9.2 L45.0,9.6 L40.3,10.6 L35.7,12.3 L31.4,14.6 L27.2,17.5 L23.4,20.8 L19.8,24.6 L16.5,28.8 L13.5,33.4 L10.7,38.3 L8.3,43.5 L6.2,49.1 L4.3,54.8 L2.9,60.5 Z"
        );
    }

    #[test]
    fn test_circular_arrow_default_path_tracks_office_arc_span() {
        let adj = HashMap::new();
        let path = circular_arrow_path(160.0, 100.0, &adj);

        assert_eq!(
            path,
            "M15.3,69.3 L16.9,62.6 L18.9,56.4 L21.5,50.5 L24.7,44.9 L28.4,39.6 L32.5,34.7 L37.1,30.3 L42.1,26.3 L47.5,22.8 L53.1,19.9 L59.0,17.5 L65.1,15.7 L71.4,14.6 L77.7,14.0 L84.0,14.1 L90.3,14.8 L96.5,16.1 L102.5,18.1 L108.4,20.6 L114.0,23.7 L119.2,27.3 L124.1,31.4 L128.6,36.0 L132.6,41.0 L136.2,46.4 L139.4,52.4 L158.2,69.2 L154.9,44.3 L151.3,37.4 L146.8,30.7 L141.7,24.3 L136.0,18.5 L129.8,13.3 L123.1,8.8 L116.1,4.9 L108.7,1.7 L101.0,-0.8 L93.1,-2.4 L85.1,-3.3 L77.0,-3.4 L69.0,-2.8 L61.1,-1.3 L53.3,1.0 L45.8,4.0 L38.7,7.7 L31.9,12.1 L25.6,17.1 L19.8,22.8 L14.5,28.9 L9.9,35.6 L5.9,42.7 L2.6,50.1 L0.0,57.9 L-1.7,65.4 Z"
        );
    }

    #[test]
    fn test_curved_right_arrow_default_path_tracks_reference_c_shape() {
        let adj = HashMap::new();
        let path = curved_right_arrow_path(100.0, 140.0, &adj);

        assert_eq!(
            path,
            "M 0.0,54.3 L 0.0,54.3 C 0.0,63.9 4.7,73.3 13.4,81.5 22.2,89.8 34.8,96.6 50.0,101.4 57.8,103.9 66.3,105.7 75.0,107.0 L 75.0,96.0 100.0,119.7 75.0,140.0 75.0,129.0 75.0,129.0 C 66.2,127.8 57.8,125.9 50.0,123.4 34.8,118.7 22.2,111.8 13.4,103.5 4.6,95.3 0.0,85.9 0.0,76.4 L 0.0,54.3 0.0,54.3 C 0.0,44.8 4.7,35.4 13.4,27.2 22.2,18.9 34.8,12.1 50.0,7.3 65.2,2.5 82.4,0.0 100.0,0.0 L 100.0,22.0 100.0,22.0 C 82.4,22.0 65.2,24.5 50.0,29.3 34.8,34.1 22.2,41.0 13.4,49.2 8.1,54.2 4.3,59.7 2.1,65.4"
        );
    }

    #[test]
    fn test_star4_default_path_matches_office_body_width() {
        let adj = HashMap::new();
        let path = star4_path(100.0, 100.0, &adj);

        assert_eq!(
            path,
            "M50.0,0 L59.0,41.0 L100.0,50.0 L59.0,59.0 L50.0,100.0 L41.0,59.0 L0,50.0 L41.0,41.0 Z"
        );
    }

    #[test]
    fn test_star5_default_path_matches_office_body_width() {
        let adj = HashMap::new();
        let path = star5_path(100.0, 100.0, &adj);

        assert_eq!(
            path,
            "M50.0,0.0 L64.7,29.8 L97.6,34.5 L73.8,57.7 L79.4,90.5 L50.0,75.0 L20.6,90.5 L26.2,57.7 L2.4,34.5 L35.3,29.8 Z"
        );
    }

    #[test]
    fn test_star_variants() {
        let adj = HashMap::new();
        for name in [
            "star4", "star5", "star6", "star7", "star8", "star10", "star12", "star16", "star24",
            "star32",
        ] {
            let path = preset_shape_svg(name, 100.0, 100.0, &adj).unwrap();
            assert!(path.ends_with('Z'), "Star {name} not closed");
        }
    }

    #[test]
    fn test_math_shapes() {
        let adj = HashMap::new();
        for name in [
            "mathPlus",
            "mathMinus",
            "mathEqual",
            "mathNotEqual",
            "mathMultiply",
            "mathDivide",
        ] {
            assert!(
                preset_shape_svg(name, 100.0, 100.0, &adj).is_some(),
                "Missing: {name}"
            );
        }
    }

    #[test]
    fn test_pie_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 5400000.0);
        custom_adj.insert("adj2".to_string(), 10800000.0);

        let default_path = preset_shape_svg("pie", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("pie", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "pie adj values should change the path"
        );
    }

    #[test]
    fn test_arc_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 5400000.0);
        custom_adj.insert("adj2".to_string(), 10800000.0);

        let default_path = preset_shape_svg("arc", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("arc", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "arc adj values should change the path"
        );
    }

    #[test]
    fn test_block_arc_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 5400000.0);
        custom_adj.insert("adj2".to_string(), 16200000.0);
        custom_adj.insert("adj3".to_string(), 40000.0);

        let default_path = preset_shape_svg("blockArc", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("blockArc", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "blockArc adj values should change the path"
        );
    }

    #[test]
    fn test_block_arc_default_path_matches_upper_band_silhouette() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("blockArc", 120.0, 100.0, &default_adj).unwrap();

        let ys: Vec<f64> = path
            .split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
            .filter(|token| !token.is_empty())
            .skip(1)
            .step_by(2)
            .map(|token| token.parse::<f64>().unwrap())
            .collect();

        assert!(
            path.contains('C'),
            "blockArc default should preserve the extracted curved band outline"
        );
        assert!(
            ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) <= 50.1,
            "blockArc default should stay in the upper half of its box: {path}"
        );
        assert!(
            ys.iter().copied().fold(f64::INFINITY, f64::min) <= 0.1,
            "blockArc default should still reach the top edge: {path}"
        );
    }

    #[test]
    fn test_funnel_default_path_matches_extracted_body_curve() {
        let path = funnel_path(120.0, 100.0);

        let ys: Vec<f64> = path
            .split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
            .filter(|token| !token.is_empty())
            .skip(1)
            .step_by(2)
            .map(|token| token.parse::<f64>().unwrap())
            .collect();

        assert!(
            path.contains('C'),
            "funnel default should use the extracted curved body instead of a hexagon"
        );
        assert!(
            ys.iter().copied().fold(f64::INFINITY, f64::min) <= 0.1,
            "funnel mouth should still reach the top edge: {path}"
        );
        assert!(
            ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) >= 94.0,
            "funnel tail should extend near the bottom edge: {path}"
        );
    }

    #[test]
    fn test_circular_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 20000.0);
        custom_adj.insert("adj5".to_string(), 25000.0);

        let default_path = preset_shape_svg("circularArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("circularArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "circularArrow adj values should change the path"
        );
    }

    #[test]
    fn test_circular_arrow_negative_adjust_flips_large_arc_flag() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), -40_000.0);

        let default_path = preset_shape_svg("circularArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("circularArrow", 120.0, 100.0, &custom_adj).unwrap();

        let parse_start = |path: &str| {
            let start = path
                .strip_prefix('M')
                .and_then(|rest| rest.split_once(' '))
                .map(|(coords, _)| coords)
                .unwrap();
            let (x, y) = start.split_once(',').unwrap();
            (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap())
        };

        let (default_x, default_y) = parse_start(&default_path);
        let (custom_x, custom_y) = parse_start(&custom_path);

        assert!(
            default_path.ends_with('Z') && custom_path.ends_with('Z'),
            "circularArrow should remain a closed polygon ribbon"
        );
        assert!(
            custom_x > default_x && custom_y > default_y,
            "negative adj1 should tighten the sweep and move the start point inward: default={default_path} custom={custom_path}"
        );
    }

    #[test]
    fn test_curved_right_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 10000.0);
        custom_adj.insert("adj2".to_string(), 80000.0);

        let default_path =
            preset_shape_svg("curvedRightArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("curvedRightArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "curvedRightArrow adj1/adj2 should change the path"
        );
    }

    #[test]
    fn test_curved_left_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 10000.0);
        custom_adj.insert("adj2".to_string(), 80000.0);

        let default_path = preset_shape_svg("curvedLeftArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("curvedLeftArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "curvedLeftArrow adj1/adj2 should change the path"
        );
    }

    #[test]
    fn test_curved_up_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 10000.0);
        custom_adj.insert("adj2".to_string(), 80000.0);

        let default_path = preset_shape_svg("curvedUpArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("curvedUpArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "curvedUpArrow adj1/adj2 should change the path"
        );
    }

    #[test]
    fn test_curved_down_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 10000.0);
        custom_adj.insert("adj2".to_string(), 80000.0);

        let default_path = preset_shape_svg("curvedDownArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("curvedDownArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "curvedDownArrow adj1/adj2 should change the path"
        );
    }

    #[test]
    fn test_wave_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj2".to_string(), 40000.0);

        let default_path = preset_shape_svg("wave", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("wave", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "wave adj2 should change the path"
        );
    }

    #[test]
    fn test_double_wave_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj2".to_string(), 40000.0);

        let default_path = preset_shape_svg("doubleWave", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("doubleWave", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "doubleWave adj2 should change the path"
        );
    }

    #[test]
    fn test_chord_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 5400000.0);
        custom_adj.insert("adj2".to_string(), 10800000.0);

        let default_path = preset_shape_svg("chord", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("chord", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "chord adj values should change the path"
        );
    }

    #[test]
    fn test_bent_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj4".to_string(), 70000.0);

        let default_path = preset_shape_svg("bentArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("bentArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "bentArrow adj4 should change the path"
        );
    }

    #[test]
    fn test_bent_up_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj3".to_string(), 70000.0);

        let default_path = preset_shape_svg("bentUpArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("bentUpArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "bentUpArrow adj3 should change the path"
        );
    }

    #[test]
    fn test_left_right_up_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj2".to_string(), 60000.0);
        custom_adj.insert("adj3".to_string(), 70000.0);

        let default_path =
            preset_shape_svg("leftRightUpArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("leftRightUpArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "leftRightUpArrow adj2/adj3 should change the path"
        );
    }

    #[test]
    fn test_left_right_up_arrow_default_path_matches_extracted_polygon() {
        let adj = HashMap::new();
        let path = left_right_up_arrow_path(120.0, 100.0, &adj);

        assert_eq!(
            path,
            "M0.0,75.0 L18.6,50.0 L18.6,62.5 L50.7,62.5 L50.7,25.0 L41.3,25.0 L60.0,0.0 L78.6,25.0 L69.3,25.0 L69.3,62.5 L101.3,62.5 L101.3,50.0 L120.0,75.0 L101.3,100.0 L101.3,87.5 L18.6,87.5 L18.6,100.0 Z"
        );
    }

    #[test]
    fn test_left_up_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj3".to_string(), 70000.0);

        let default_path = preset_shape_svg("leftUpArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("leftUpArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "leftUpArrow adj3 should change the path"
        );
    }

    #[test]
    fn test_uturn_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj3".to_string(), 45000.0);
        custom_adj.insert("adj4".to_string(), 70000.0);
        custom_adj.insert("adj5".to_string(), 85000.0);

        let default_path = preset_shape_svg("uturnArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("uturnArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "uturnArrow adj3/adj4/adj5 should change the path"
        );
    }

    #[test]
    fn test_uturn_arrow_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("uturnArrow", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 31.9,97.2 L 3.4,96.2 3.4,41.8 5.3,32.2 14.9,17.4 30.0,7.2 47.3,2.8 66.5,4.0 82.3,10.4 93.1,19.4 101.3,33.4 102.7,49.0 116.6,50.2 87.6,73.6 59.5,50.2 73.4,48.6 73.0,40.6 69.1,33.8 57.8,27.6 43.4,29.2 37.0,33.8 33.1,41.0 Z"
        );
    }

    #[test]
    fn test_swoosh_arrow_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("swooshArrow", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 4.4,97.3 L 3.4,95.9 7.8,83.5 16.7,67.3 29.5,51.9 42.0,41.1 70.7,24.9 103.5,14.6 103.5,2.7 116.6,21.4 107.5,50.8 106.1,40.3 103.1,39.5 81.2,43.2 53.7,52.4 38.8,60.5 23.0,73.0 Z"
        );
    }

    #[test]
    fn test_cloud_callout_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), -5000.0);
        custom_adj.insert("adj2".to_string(), 45000.0);

        let default_path = preset_shape_svg("cloudCallout", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("cloudCallout", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "cloudCallout adj1/adj2 should change the path"
        );
    }

    #[test]
    fn test_math_not_equal_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj2".to_string(), 9600000.0);

        let default_path = preset_shape_svg("mathNotEqual", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("mathNotEqual", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "mathNotEqual adj2 should change the path"
        );
    }

    #[test]
    fn test_math_not_equal_default_path_matches_extracted_office_polygon() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("mathNotEqual", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 4.0,20.9 L 58.3,20.9 65.4,1.9 86.3,9.3 82.0,20.9 115.9,20.9 115.9,42.5 73.9,42.5 69.8,53.4 115.9,53.4 115.9,75.0 61.7,75.0 54.6,94.0 33.7,86.6 38.0,75.0 4.0,75.0 4.0,53.4 46.1,53.4 50.2,42.5 4.0,42.5 4.0,20.9 Z"
        );
    }

    #[test]
    fn test_math_divide_default_path_matches_extracted_office_geometry() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("mathDivide", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(path.matches('M').count(), 3);
        assert!(path.contains("60.0,1.6"));
        assert!(path.contains("113.7,33.9"));
        assert!(path.contains("6.2,33.9"));
    }

    #[test]
    fn test_math_equal_default_path_matches_extracted_office_geometry() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("mathEqual", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 13.5,19.7 L 106.0,19.7 106.0,49.1 13.5,49.1 13.5,19.7 Z M 13.5,54.9 L 106.0,54.9 106.0,84.4 13.5,84.4 13.5,54.9 Z"
        );
    }

    #[test]
    fn test_math_plus_default_path_matches_extracted_office_geometry() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("mathPlus", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 42.3,11.1 L 77.7,11.1 77.7,40.8 108.0,40.8 108.0,59.2 77.7,59.2 77.7,88.9 42.3,88.9 42.3,59.2 12.0,59.2 12.0,40.8 42.3,40.8 42.3,11.1 Z"
        );
    }

    #[test]
    fn test_plus_default_path_matches_benchmark_cross_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("plus", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 88.0,100.2 L 30.5,99.8 28.7,97.9 28.7,77.2 27.5,76.1 4.0,76.1 -0.2,73.9 -0.2,26.1 1.0,23.9 28.7,23.2 28.7,2.5 30.0,-0.2 89.0,-0.2 90.8,2.1 91.3,23.2 118.0,23.9 119.8,25.7 120.2,73.4 119.0,75.3 115.5,76.1 92.0,76.1 90.8,78.0 90.8,98.3 Z"
        );
    }

    #[test]
    fn test_cross_alias_default_path_matches_benchmark_cross_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("cross", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 88.0,100.2 L 30.5,99.8 28.7,97.9 28.7,77.2 27.5,76.1 4.0,76.1 -0.2,73.9 -0.2,26.1 1.0,23.9 28.7,23.2 28.7,2.5 30.0,-0.2 89.0,-0.2 90.8,2.1 91.3,23.2 118.0,23.9 119.8,25.7 120.2,73.4 119.0,75.3 115.5,76.1 92.0,76.1 90.8,78.0 90.8,98.3 Z"
        );
    }

    #[test]
    fn test_math_minus_default_path_matches_extracted_office_geometry() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("mathMinus", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 12.0,36.3 L 108.0,36.3 108.0,63.7 12.0,63.7 12.0,36.3 Z"
        );
    }

    #[test]
    fn test_curved_right_arrow_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("curvedRightArrow", 120.0, 100.0, &default_adj).unwrap();

        assert!(path.contains("120.0,85.5"));
        assert!(path.contains("90.0,100.0"));
        assert!(path.contains("0.0,54.5"));
    }

    #[test]
    fn test_curved_left_arrow_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("curvedLeftArrow", 120.0, 100.0, &default_adj).unwrap();

        assert!(path.contains("120.0,56.5"));
        assert!(path.contains("0.0,75.3"));
        assert!(path.contains("18.6,100.0"));
    }

    #[test]
    fn test_curved_up_arrow_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("curvedUpArrow", 120.0, 100.0, &default_adj).unwrap();

        assert!(path.contains("90.9,0.0"));
        assert!(path.contains("120.0,25.0"));
        assert!(path.contains("37.9,100.0"));
    }

    #[test]
    fn test_curved_down_arrow_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("curvedDownArrow", 120.0, 100.0, &default_adj).unwrap();

        assert!(path.contains("90.3,100.0"));
        assert!(path.contains("120.0,84.4"));
        assert!(path.contains("52.7,8.4"));
    }

    #[test]
    fn test_bent_connector5_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 20_000.0);
        custom_adj.insert("adj2".to_string(), 35_000.0);
        custom_adj.insert("adj3".to_string(), 80_000.0);

        let default_path = preset_shape_svg("bentConnector5", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("bentConnector5", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "bentConnector5 adj1/adj2/adj3 should change the path"
        );
    }

    #[test]
    fn test_lightning_bolt_default_path_matches_extracted_office_polygon() {
        let path = lightning_bolt_path(120.0, 100.0);

        assert_eq!(
            path,
            "M 47.8,1.4 L 70.7,27.9 61.3,31.0 90.2,53.7 80.7,57.5 116.4,95.5 55.8,66.4 67.4,62.3 29.8,43.7 43.3,37.9 3.6,18.4 47.8,1.4 Z"
        );
    }

    #[test]
    fn test_math_multiply_default_path_matches_extracted_office_polygon() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("mathMultiply", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 6.3,13.8 L 36.1,2.0 60.0,26.6 83.8,2.0 113.6,13.8 80.6,47.9 113.6,82.0 83.8,93.8 60.0,69.2 36.1,93.8 6.3,82.0 39.4,47.9 6.3,13.8 Z"
        );
    }

    #[test]
    fn test_bent_up_arrow_default_path_matches_extracted_office_polygon() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("bentUpArrow", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 3.0,71.0 L 90.4,71.0 90.4,24.9 81.5,24.9 99.2,1.9 117.0,24.9 108.1,24.9 108.1,94.0 3.0,94.0 3.0,71.0 Z"
        );
    }

    #[test]
    fn test_left_up_arrow_default_path_matches_extracted_office_polygon() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("leftUpArrow", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 4.7,81.4 L 32.3,66.7 32.3,74.0 73.8,74.0 73.8,16.0 60.0,16.0 87.6,1.2 115.3,16.0 101.4,16.0 101.4,88.8 32.3,88.8 32.3,96.2 4.7,81.4 Z"
        );
    }

    #[test]
    fn test_right_brace_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("rightBrace", 120.0, 100.0, &default_adj).unwrap();

        assert!(path.contains("M 3.6,1.4"));
        assert!(path.contains("116.4,48.4"));
        assert!(path.contains("3.6,95.5"));
    }

    #[test]
    fn test_left_brace_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("leftBrace", 120.0, 100.0, &default_adj).unwrap();

        assert!(path.contains("M 116.4,95.5"));
        assert!(path.contains("3.6,48.4"));
        assert!(path.contains("116.4,1.5"));
    }

    #[test]
    fn test_bracket_pair_default_path_matches_extracted_office_outline() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("bracketPair", 120.0, 100.0, &default_adj).unwrap();

        assert!(path.contains("M 3.0,17.3"));
        assert!(path.contains("105.2,1.9"));
        assert!(path.contains("14.8,94.0"));
    }

    #[test]
    fn test_uturn_arrow_extreme_adj_keeps_arc_radii_non_negative() {
        let mut extreme_adj = HashMap::new();
        extreme_adj.insert("adj1".to_string(), 90000.0);
        extreme_adj.insert("adj2".to_string(), 90000.0);

        let path = preset_shape_svg("uturnArrow", 120.0, 100.0, &extreme_adj).unwrap();

        assert!(
            !path.contains("A-"),
            "uturnArrow should not emit negative arc radii under extreme adj values: {path}"
        );
        assert!(
            !path.contains("NaN") && !path.contains("inf"),
            "uturnArrow should keep SVG path numeric under extreme adj values: {path}"
        );
    }
}

// Custom geometry (custGeom) rendering

use crate::model::{CustomGeometry, GeometryPath, PathCommand, PathFill};
use std::fmt::Write;

pub struct CustomGeomSvg {
    pub paths: Vec<CustomGeomPathSvg>,
}
pub struct CustomGeomPathSvg {
    pub d: String,
    pub fill: PathFill,
}

pub fn custom_geometry_svg(
    geom: &CustomGeometry,
    shape_w: f64,
    shape_h: f64,
) -> Option<CustomGeomSvg> {
    if geom.paths.is_empty() {
        return None;
    }
    let mut result_paths = Vec::with_capacity(geom.paths.len());
    for path in &geom.paths {
        let d = geometry_path_to_svg(path, shape_w, shape_h);
        result_paths.push(CustomGeomPathSvg {
            d,
            fill: path.fill.clone(),
        });
    }
    Some(CustomGeomSvg {
        paths: result_paths,
    })
}

fn geometry_path_to_svg(path: &GeometryPath, shape_w: f64, shape_h: f64) -> String {
    let path_w = if path.width > 0.0 {
        path.width
    } else {
        shape_w
    };
    let path_h = if path.height > 0.0 {
        path.height
    } else {
        shape_h
    };
    let sx = shape_w / path_w;
    let sy = shape_h / path_h;
    let mut d = String::with_capacity(256);
    let mut cur_x = 0.0_f64;
    let mut cur_y = 0.0_f64;
    for cmd in &path.commands {
        match cmd {
            PathCommand::MoveTo { x, y } => {
                let px = x * sx;
                let py = y * sy;
                let _ = write!(d, "M{px:.2},{py:.2} ");
                cur_x = px;
                cur_y = py;
            }
            PathCommand::LineTo { x, y } => {
                let px = x * sx;
                let py = y * sy;
                let _ = write!(d, "L{px:.2},{py:.2} ");
                cur_x = px;
                cur_y = py;
            }
            PathCommand::CubicBezTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                let _ = write!(
                    d,
                    "C{:.2},{:.2} {:.2},{:.2} {:.2},{:.2} ",
                    x1 * sx,
                    y1 * sy,
                    x2 * sx,
                    y2 * sy,
                    x * sx,
                    y * sy
                );
                cur_x = x * sx;
                cur_y = y * sy;
            }
            PathCommand::QuadBezTo { x1, y1, x, y } => {
                let _ = write!(
                    d,
                    "Q{:.2},{:.2} {:.2},{:.2} ",
                    x1 * sx,
                    y1 * sy,
                    x * sx,
                    y * sy
                );
                cur_x = x * sx;
                cur_y = y * sy;
            }
            PathCommand::ArcTo {
                wr,
                hr,
                start_angle,
                swing_angle,
            } => {
                let rx = wr * sx;
                let ry = hr * sy;
                if rx.abs() < 0.001 || ry.abs() < 0.001 {
                    continue;
                }
                let st_deg = start_angle / 60000.0;
                let sw_deg = swing_angle / 60000.0;
                if sw_deg.abs() < 0.001 {
                    continue;
                }
                let st_rad = st_deg.to_radians();
                let end_rad = (st_deg + sw_deg).to_radians();
                let end_x = cur_x + rx * (end_rad.cos() - st_rad.cos());
                let end_y = cur_y + ry * (end_rad.sin() - st_rad.sin());
                let large_arc = if sw_deg.abs() > 180.0 { 1 } else { 0 };
                let sweep = if sw_deg > 0.0 { 1 } else { 0 };
                let _ = write!(
                    d,
                    "A{rx:.2},{ry:.2} 0 {large_arc},{sweep} {end_x:.2},{end_y:.2} "
                );
                cur_x = end_x;
                cur_y = end_y;
            }
            PathCommand::Close => {
                d.push_str("Z ");
            }
        }
    }
    d.trim_end().to_string()
}
