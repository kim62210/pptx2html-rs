//! Preset shape SVG path generation for all 187 OOXML preset geometries.
//! Generates SVG `<path>` elements parameterized by width, height, and adjust values.
//! Covers flowchart, action buttons, stars, callouts, math shapes, arrow callouts,
//! brackets/braces, chart shapes, scrolls, tabs, ribbons, circular arrows, and more
//! per ECMA-376 Part 1 section 20.1.10.

use std::collections::HashMap;

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
        "foldCorner" => Some(fold_corner_path(w, h, adjust_values)),
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
        "plus" | "mathPlus" => Some(plus_path(w, h, adjust_values)),
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
        "cross" => Some(plus_path(w, h, adjust_values)),
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
    format!("M0,0 L{:.1},0 L0,{:.1} Z", w * ratio, h * ratio)
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
    let i = r * 2.0;
    format!(
        "M{i:.1},0 Q0,0 0,{r:.1} L0,{y1:.1} Q0,{cy:.1} {n:.1},{cy:.1} Q0,{cy:.1} 0,{y2:.1} L0,{y3:.1} Q0,{h:.1} {i:.1},{h:.1} M{x1:.1},0 Q{w:.1},0 {w:.1},{r:.1} L{w:.1},{y1:.1} Q{w:.1},{cy:.1} {x2:.1},{cy:.1} Q{w:.1},{cy:.1} {w:.1},{y2:.1} L{w:.1},{y3:.1} Q{w:.1},{h:.1} {x1:.1},{h:.1}",
        i = i,
        r = r,
        cy = cy,
        y1 = cy - r,
        n = -r * 0.5,
        y2 = cy + r,
        y3 = h - r,
        h = h,
        x1 = w - i,
        w = w,
        x2 = w + r * 0.5
    )
}
fn bracket_pair_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
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
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let s = h * a1 / 100_000.0 / 2.0;
    let hw = w * a2 / 100_000.0;
    let cy = h / 2.0;
    let (yt, yb) = (cy - s, cy + s);
    format!(
        "M{w:.1},{yt:.1} L{hw:.1},{yt:.1} L{hw:.1},0 L0,{cy:.1} L{hw:.1},{h:.1} L{hw:.1},{yb:.1} L{w:.1},{yb:.1} Z"
    )
}
fn up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
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
    let a1 = adj.get("adj1").copied().unwrap_or(50000.0);
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
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
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
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0);
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
    let a1 = adj.get("adj1").copied().unwrap_or(25000.0) / 100_000.0;
    let a2 = adj.get("adj2").copied().unwrap_or(50000.0) / 100_000.0;
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let body_top = h * a1.clamp(0.08, 0.42);
    let body_bottom = h - body_top;
    let ctrl_y = h * (0.1 + a2.clamp(0.0, 1.0) * 0.4);
    let tail_ctrl_y = h * (0.6 + a2.clamp(0.0, 1.0) * 0.2);
    let ctrl_front_x = w * (0.15 + a2.clamp(0.0, 1.0) * 0.3);
    let ctrl_back_x = w * (0.25 + a2.clamp(0.0, 1.0) * 0.4);
    let tail_x = w * (0.05 + a2.clamp(0.0, 1.0) * 0.3);
    let cy = h / 2.0;
    let xh = w * (1.0 - a3);
    format!(
        "M0,{h:.1} C0,{ctrl_y:.1} {ctrl_front_x:.1},{body_top:.1} {xh:.1},{body_top:.1} L{xh:.1},0 L{w:.1},{cy:.1} L{xh:.1},{h:.1} L{xh:.1},{body_bottom:.1} C{ctrl_back_x:.1},{body_bottom:.1} {tail_x:.1},{tail_ctrl_y:.1} {tail_x:.1},{h:.1} Z",
        h = h,
        ctrl_y = ctrl_y,
        ctrl_front_x = ctrl_front_x,
        body_top = body_top,
        xh = xh,
        w = w,
        cy = cy,
        body_bottom = body_bottom,
        ctrl_back_x = ctrl_back_x,
        tail_x = tail_x,
        tail_ctrl_y = tail_ctrl_y
    )
}
fn curved_left_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
    let rx = w / 2.0;
    let ry = h / 2.0;
    let cx = rx;
    let cy = ry;
    let sweep = (std::f64::consts::PI * (1.25 + a1)).clamp(
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::TAU - 0.2,
    );
    let start_angle = -std::f64::consts::FRAC_PI_2;
    let end_angle = start_angle + sweep;
    let t = (w.min(h) * (0.08 + a5 * 0.2)).min(w.min(h) * 0.35);
    let (sx, sy) = ellipse_point(cx, cy, rx, ry, start_angle);
    let (ax, ay) = ellipse_point(cx, cy, rx, ry, end_angle);
    let (isx, isy) = ellipse_point(cx, cy, (rx - t).max(0.1), (ry - t).max(0.1), start_angle);
    let (iax, iay) = ellipse_point(cx, cy, (rx - t).max(0.1), (ry - t).max(0.1), end_angle);
    let tx = -end_angle.sin();
    let ty = end_angle.cos();
    let head_len = t * 1.6;
    let head_half = t * 0.8;
    let tip_x = ax + end_angle.cos() * head_len;
    let tip_y = ay + end_angle.sin() * head_len;
    format!(
        "M{sx:.1},{sy:.1} A{rx:.1},{ry:.1} 0 {large_arc},1 {ax:.1},{ay:.1} L{b1x:.1},{b1y:.1} L{tip_x:.1},{tip_y:.1} L{b2x:.1},{b2y:.1} L{iax:.1},{iay:.1} A{rxi:.1},{ryi:.1} 0 {large_arc},0 {isx:.1},{isy:.1} Z",
        sx = sx,
        sy = sy,
        rx = rx,
        ry = ry,
        large_arc = if sweep > std::f64::consts::PI { 1 } else { 0 },
        ax = ax,
        ay = ay,
        b1x = ax + tx * head_half,
        b1y = ay + ty * head_half,
        tip_x = tip_x,
        tip_y = tip_y,
        b2x = ax - tx * head_half,
        b2y = ay - ty * head_half,
        iax = iax,
        iay = iay,
        rxi = (rx - t).max(0.1),
        ryi = (ry - t).max(0.1),
        isx = isx,
        isy = isy
    )
}
fn bent_up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
    let ratio = adj.get("adj").copied().unwrap_or(12500.0) / 100_000.0;
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
    let ratio = adj.get("adj").copied().unwrap_or(10530.0) / 100_000.0;
    let (cx, cy) = (w / 2.0, h / 2.0);
    let (ro_x, ro_y) = (cx, cy);
    let (ri_x, ri_y) = (cx * ratio * 2.0, cy * ratio * 2.0);
    let n = 5;
    let t = n * 2;
    let st = -std::f64::consts::FRAC_PI_2;
    let mut pts: Vec<(f64, f64)> = Vec::with_capacity(t as usize);
    for i in 0..t {
        let a = st + 2.0 * std::f64::consts::PI * (i as f64) / (t as f64);
        let (rx, ry) = if i % 2 == 0 { (ro_x, ro_y) } else { (ri_x, ri_y) };
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
        let (rx, ry) = if i % 2 == 0 { (ro_x, ro_y) } else { (ri_x, ri_y) };
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
    let a = adj.get("adj").copied().unwrap_or(25000.0);
    let ax = w * a / 100_000.0;
    let ay = h * a / 100_000.0;
    let (x1, y1) = (w - ax, h - ay);
    format!(
        "M{ax:.1},0 L{x1:.1},0 L{x1:.1},{ay:.1} L{w:.1},{ay:.1} L{w:.1},{y1:.1} L{x1:.1},{y1:.1} L{x1:.1},{h:.1} L{ax:.1},{h:.1} L{ax:.1},{y1:.1} L0,{y1:.1} L0,{ay:.1} L{ax:.1},{ay:.1} Z"
    )
}
fn math_minus_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
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
    format!(
        "M{x1:.1},0 L{x2:.1},{y1:.1} L{x3:.1},{y2:.1} L{x4:.1},{y2:.1} L{x5:.1},{y3:.1} L{x6:.1},{y4:.1} L{x7:.1},{h:.1} L{x8:.1},{y5:.1} L{x9:.1},{y6:.1} L{x10:.1},{y6:.1} L{x11:.1},{y7:.1} Z",
        x1 = w * 0.55,
        x2 = w * 0.3,
        y1 = h * 0.35,
        x3 = w * 0.52,
        y2 = h * 0.35,
        x4 = w * 0.25,
        y3 = h * 0.6,
        x5 = w * 0.47,
        y4 = h * 0.6,
        x6 = w * 0.17,
        x7 = w * 0.45,
        h = h,
        x8 = w * 0.7,
        y5 = h * 0.65,
        x9 = w * 0.48,
        y6 = h * 0.65,
        x10 = w * 0.75,
        y7 = h * 0.4,
        x11 = w * 0.83
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
    let adj1 = adj.get("adj1").copied().unwrap_or(10800000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(0.0);
    let a3 = adj.get("adj3").copied().unwrap_or(25000.0) / 100_000.0;
    let (ro, ryo) = (w / 2.0, h / 2.0);
    let t = w.min(h) * a3;
    let (cx, cy) = (ro, ryo);
    let start_angle = -std::f64::consts::FRAC_PI_2 + adj2 / 21_600_000.0 * std::f64::consts::TAU;
    let end_angle = std::f64::consts::PI
        + (adj1 - 10_800_000.0) / 21_600_000.0 * std::f64::consts::TAU;
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
    let (rx, ry) = (w / 2.0, h / 2.0);
    let x = w * (1.0 - ratio);
    format!(
        "M{x:.1},0 A{rx:.1},{ry:.1} 0 1,1 {x:.1},{h:.1} A{rx2:.1},{ry2:.1} 0 1,0 {x:.1},0 Z",
        x = x,
        rx = rx,
        ry = ry,
        h = h,
        rx2 = rx * ratio,
        ry2 = ry * 0.85
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
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z M{t:.1},{t:.1} L{x:.1},{t:.1} L{x:.1},{y:.1} L{t:.1},{y:.1} Z M0,0 L{t:.1},{t:.1} M{w:.1},0 L{x:.1},{t:.1} M{w:.1},{h:.1} L{x:.1},{y:.1} M0,{h:.1} L{t:.1},{y:.1}",
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
    let end_angle = (adj2 - 16_200_000.0) / 21_600_000.0 * std::f64::consts::TAU;
    let (sx, sy) = ellipse_point(cx, cy, rx, ry, start_angle);
    let (ex, ey) = ellipse_point(cx, cy, rx, ry, end_angle);
    format!(
        "M{cx:.1},{cy:.1} L{sx:.1},{sy:.1} A{rx:.1},{ry:.1} 0 1,1 {ex:.1},{ey:.1} Z",
        cx = cx,
        cy = cy,
        sx = sx,
        sy = sy,
        rx = rx,
        ry = ry,
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

fn ellipse_point(cx: f64, cy: f64, rx: f64, ry: f64, angle: f64) -> (f64, f64) {
    (cx + rx * angle.cos(), cy + ry * angle.sin())
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
    let cx = w / 2.0;
    let nw = w * 0.2;
    let ny = h * 0.6;
    format!(
        "M0,0 L{w:.1},0 L{x2:.1},{ny:.1} L{x2:.1},{h:.1} L{x1:.1},{h:.1} L{x1:.1},{ny:.1} Z",
        w = w,
        x1 = cx - nw / 2.0,
        x2 = cx + nw / 2.0,
        ny = ny,
        h = h
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
    let r2 = r / 2.0;
    let (y, yh, x1, x2) = (h - r, h - r2, w - r, w - r2);
    format!(
        "M{r:.1},{r:.1} L{r:.1},0 A{r2:.1},{r2:.1} 0 0,1 {r2:.1},0 A{r2:.1},{r2:.1} 0 0,1 {r:.1},{r:.1} L{r:.1},{y:.1} A{r2:.1},{r2:.1} 0 0,1 {r:.1},{yh:.1} A{r2:.1},{r2:.1} 0 0,1 0,{y:.1} L0,{r:.1} L{x1:.1},{r:.1} L{x1:.1},0 A{r2:.1},{r2:.1} 0 0,1 {x2:.1},0 A{r2:.1},{r2:.1} 0 0,1 {x1:.1},{r:.1} L{x1:.1},{y:.1} A{r2:.1},{r2:.1} 0 0,1 {x1:.1},{yh:.1} A{r2:.1},{r2:.1} 0 0,1 {w:.1},{y:.1} L{w:.1},{r:.1} Z",
        r = r,
        r2 = r2,
        y = y,
        yh = yh,
        x1 = x1,
        x2 = x2,
        w = w
    )
}

// Tabs
fn corner_tabs_path(w: f64, h: f64) -> String {
    let s = w.min(h) * 0.12;
    format!(
        "M0,0 L{s:.1},0 L0,{s:.1} Z M{x:.1},0 L{w:.1},0 L{w:.1},{s:.1} Z M{w:.1},{y:.1} L{w:.1},{h:.1} L{x:.1},{h:.1} Z M0,{y:.1} L{s:.1},{h:.1} L0,{h:.1} Z",
        s = s,
        x = w - s,
        w = w,
        y = h - s,
        h = h
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
    let s = w.min(h) * 0.1;
    format!(
        "M0,0 L{s:.1},0 L{s:.1},{s:.1} L0,{s:.1} Z M{x:.1},0 L{w:.1},0 L{w:.1},{s:.1} L{x:.1},{s:.1} Z M{x:.1},{y:.1} L{w:.1},{y:.1} L{w:.1},{h:.1} L{x:.1},{h:.1} Z M0,{y:.1} L{s:.1},{y:.1} L{s:.1},{h:.1} L0,{h:.1} Z",
        s = s,
        x = w - s,
        w = w,
        y = h - s,
        h = h
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
    let (rx, ry) = (w / 2.0, h / 2.0);
    let cx = rx;
    let t = w.min(h) * 0.1;
    let ax = cx - rx * 0.866;
    let ay = ry + ry * 0.5;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,0 {ax:.1},{ay:.1} L{tx1:.1},{ty1:.1} L{tx2:.1},{ty2:.1} L{tx3:.1},{ty3:.1} A{rxi:.1},{ryi:.1} 0 1,1 {cx:.1},{t:.1} Z",
        cx = cx,
        rx = rx,
        ry = ry,
        ax = ax,
        ay = ay,
        tx1 = ax - t,
        ty1 = ay + t,
        tx2 = ax - t * 1.5,
        ty2 = ay - t * 0.5,
        tx3 = ax,
        ty3 = ay,
        rxi = (rx - t).max(0.1),
        ryi = (ry - t).max(0.1),
        t = t
    )
}
fn left_right_circular_arrow_path(w: f64, h: f64) -> String {
    let (rx, ry) = (w / 2.0, h / 2.0);
    let cx = rx;
    let t = w.min(h) * 0.1;
    let (ax1, ay1) = (cx + rx * 0.866, ry - ry * 0.5);
    let (ax2, ay2) = (cx - rx * 0.866, ry + ry * 0.5);
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 0,1 {ax1:.1},{ay1:.1} L{tx1:.1},{ty1:.1} L{tx2:.1},{ty2:.1} L{ax1:.1},{ay1:.1} A{rxi:.1},{ryi:.1} 0 0,0 {cx:.1},{t:.1} Z M{cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 0,1 {ax2:.1},{ay2:.1} L{bx1:.1},{by1:.1} L{bx2:.1},{by2:.1} L{ax2:.1},{ay2:.1} A{rxi:.1},{ryi:.1} 0 0,0 {cx:.1},{yt:.1} Z",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        t = t,
        ax1 = ax1,
        ay1 = ay1,
        tx1 = ax1 + t,
        ty1 = ay1 - t,
        tx2 = ax1 + t * 1.5,
        ty2 = ay1 + t * 0.5,
        rxi = (rx - t).max(0.1),
        ryi = (ry - t).max(0.1),
        ax2 = ax2,
        ay2 = ay2,
        bx1 = ax2 - t,
        by1 = ay2 + t,
        bx2 = ax2 - t * 1.5,
        by2 = ay2 - t * 0.5,
        yt = h - t
    )
}

// Misc shapes
fn chord_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(2700000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(16200000.0);
    let (rx, ry) = (w / 2.0, h / 2.0);
    let (cx, cy) = (rx, ry);
    let start_angle = std::f64::consts::PI * 5.0 / 6.0
        - (adj1 - 2_700_000.0) / 21_600_000.0 * std::f64::consts::TAU;
    let end_angle = std::f64::consts::PI / 6.0
        + (adj2 - 16_200_000.0) / 21_600_000.0 * std::f64::consts::TAU;
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
    let cy = h * 0.5;
    format!(
        "M0,{h:.1} C{c1:.1},{c1y:.1} {c2:.1},{c2y:.1} {w:.1},0 L{w:.1},{cy:.1} C{c3:.1},{c3y:.1} {c4:.1},{c4y:.1} 0,{h:.1} Z",
        h = h,
        c1 = w * 0.1,
        c1y = h * 0.7,
        c2 = w * 0.5,
        c2y = h * 0.1,
        w = w,
        cy = cy,
        c3 = w * 0.6,
        c3y = h * 0.4,
        c4 = w * 0.2,
        c4y = h * 0.9
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

        assert_ne!(default_path, custom_path, "pie adj values should change the path");
    }

    #[test]
    fn test_arc_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 5400000.0);
        custom_adj.insert("adj2".to_string(), 10800000.0);

        let default_path = preset_shape_svg("arc", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("arc", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(default_path, custom_path, "arc adj values should change the path");
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
    fn test_curved_right_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj1".to_string(), 10000.0);
        custom_adj.insert("adj2".to_string(), 80000.0);

        let default_path = preset_shape_svg("curvedRightArrow", 120.0, 100.0, &default_adj).unwrap();
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

        assert_ne!(default_path, custom_path, "wave adj2 should change the path");
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

        assert_ne!(default_path, custom_path, "chord adj values should change the path");
    }

    #[test]
    fn test_bent_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj4".to_string(), 70000.0);

        let default_path = preset_shape_svg("bentArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("bentArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(default_path, custom_path, "bentArrow adj4 should change the path");
    }

    #[test]
    fn test_bent_up_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj3".to_string(), 70000.0);

        let default_path = preset_shape_svg("bentUpArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("bentUpArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(default_path, custom_path, "bentUpArrow adj3 should change the path");
    }

    #[test]
    fn test_left_right_up_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj2".to_string(), 60000.0);
        custom_adj.insert("adj3".to_string(), 70000.0);

        let default_path =
            preset_shape_svg("leftRightUpArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path =
            preset_shape_svg("leftRightUpArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(
            default_path, custom_path,
            "leftRightUpArrow adj2/adj3 should change the path"
        );
    }

    #[test]
    fn test_left_up_arrow_adjust_values_change_path() {
        let default_adj = HashMap::new();
        let mut custom_adj = HashMap::new();
        custom_adj.insert("adj3".to_string(), 70000.0);

        let default_path = preset_shape_svg("leftUpArrow", 120.0, 100.0, &default_adj).unwrap();
        let custom_path = preset_shape_svg("leftUpArrow", 120.0, 100.0, &custom_adj).unwrap();

        assert_ne!(default_path, custom_path, "leftUpArrow adj3 should change the path");
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

        assert_ne!(default_path, custom_path, "uturnArrow adj3/adj4/adj5 should change the path");
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
