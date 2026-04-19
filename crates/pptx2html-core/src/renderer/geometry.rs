//! Preset shape SVG path generation for all 187 OOXML preset geometries.
//! Generates SVG `<path>` elements parameterized by width, height, and adjust values.
//! Covers flowchart, action buttons, stars, callouts, math shapes, arrow callouts,
//! brackets/braces, chart shapes, scrolls, tabs, ribbons, circular arrows, and more
//! per ECMA-376 Part 1 section 20.1.10.

use std::collections::HashMap;

type Point = (f64, f64);
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
            | "funnel"
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

pub fn action_button_information_icon_paths(w: f64, h: f64) -> (String, String) {
    let cx = w * 0.5;
    let cy = h * 0.5;
    let ring_r = w.min(h) * 0.36;
    let dot_r = w.min(h) * 0.075;
    let ring = format!(
        "M{cx:.1},{y1:.1} A{r:.1},{r:.1} 0 1,1 {cx:.1},{y2:.1} A{r:.1},{r:.1} 0 1,1 {cx:.1},{y1:.1} Z",
        cx = cx,
        y1 = cy - ring_r,
        y2 = cy + ring_r,
        r = ring_r
    );
    let mark = format!(
        "M{dot_cx:.1},{dot_y1:.1} A{dot_r:.1},{dot_r:.1} 0 1,1 {dot_cx:.1},{dot_y2:.1} A{dot_r:.1},{dot_r:.1} 0 1,1 {dot_cx:.1},{dot_y1:.1} Z \
         M{top_left:.1},{top_y1:.1} L{top_right:.1},{top_y1:.1} L{top_right:.1},{top_y2:.1} L{stem_right:.1},{top_y2:.1} L{stem_right:.1},{stem_y2:.1} L{base_right:.1},{stem_y2:.1} L{base_right:.1},{base_y2:.1} L{base_left:.1},{base_y2:.1} L{base_left:.1},{stem_y2:.1} L{stem_left:.1},{stem_y2:.1} L{stem_left:.1},{top_y2:.1} L{top_left:.1},{top_y2:.1} Z",
        dot_cx = cx,
        dot_y1 = cy - h * 0.33,
        dot_y2 = cy - h * 0.18,
        dot_r = dot_r,
        top_left = cx - w * 0.13,
        top_right = cx + w * 0.02,
        top_y1 = cy - h * 0.02,
        top_y2 = cy + h * 0.03,
        stem_left = cx - w * 0.04,
        stem_right = cx + w * 0.04,
        stem_y2 = cy + h * 0.29,
        base_left = cx - w * 0.11,
        base_right = cx + w * 0.12,
        base_y2 = cy + h * 0.35
    );
    (ring, mark)
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
const CURVED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000112,0.344994 L -0.000112,0.344994 C -0.000112,0.405512 0.046007,0.465129 0.133971,0.517548 0.221710,0.569966 0.347919,0.613386 0.500000,0.643757 0.598988,0.663555 0.707199,0.677278 0.819910,0.684252 L 0.819685,0.494376 0.999888,0.749944 0.819685,0.994263 0.819685,0.804387 0.819685,0.804387 C 0.706974,0.797188 0.598763,0.783465 0.499775,0.763892 0.347694,0.733521 0.221485,0.689876 0.133746,0.637458 0.045782,0.585039 -0.000337,0.525647 -0.000337,0.465129 L -0.000112,0.344994 Z"#;
const CURVED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000112,0.349944 L -0.000112,0.349944 C -0.000112,0.411361 0.046007,0.471654 0.133971,0.524972 0.221710,0.578065 0.347919,0.622385 0.500000,0.652981 0.525872,0.658380 0.552643,0.663105 0.579865,0.667604 L 0.579865,0.667604 0.999888,0.850056 0.579865,0.967717 0.579865,0.967717 0.579865,0.967717 C 0.552643,0.963217 0.525872,0.958493 0.499775,0.953093 0.347919,0.922497 0.221710,0.878178 0.133746,0.825084 0.046007,0.771766 -0.000112,0.711474 -0.000112,0.650056 L -0.000112,0.349944 Z"#;
const CURVED_LEFT_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.000112,0.749944 L 0.180090,0.494376 0.180090,0.684477 0.180090,0.684477 C 0.292801,0.677278 0.401012,0.663555 0.500000,0.643982 0.652081,0.613611 0.778290,0.569966 0.866029,0.517548 0.924297,0.482677 0.964567,0.444657 0.984814,0.405062 L 0.984814,0.405062 C 0.994938,0.424859 1.000112,0.445107 1.000112,0.465129 1.000112,0.525647 0.953993,0.585264 0.866029,0.637683 0.778290,0.690101 0.652081,0.733521 0.500000,0.763892 0.401012,0.783690 0.292801,0.797413 0.180090,0.804387 L 0.180090,0.994263 0.000112,0.749944 Z"#;
const CURVED_LEFT_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000112,0.850056 L 0.419685,0.667604 0.419685,0.667604 0.419685,0.667604 C 0.446907,0.663105 0.473678,0.658380 0.499775,0.652981 0.651631,0.622385 0.777840,0.578065 0.865804,0.524972 0.879078,0.516873 0.891676,0.508549 0.903150,0.500000 L 0.903150,0.500000 C 0.966817,0.546794 0.999663,0.598088 0.999663,0.650056 0.999663,0.711474 0.953543,0.771766 0.865804,0.824859 0.777840,0.878178 0.651631,0.922272 0.499775,0.953093 0.473678,0.958268 0.446907,0.963217 0.419685,0.967717 L 0.419685,0.967717 -0.000112,0.850056 Z"#;
const CURVED_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.749719,-0.000112 L 0.994038,0.179865 0.804162,0.179865 0.804162,0.179865 C 0.796963,0.292576 0.783240,0.400787 0.763667,0.499775 0.733296,0.651856 0.689651,0.778065 0.637233,0.865804 0.584814,0.953768 0.525422,0.999888 0.464904,0.999888 0.444657,0.999888 0.424634,0.994713 0.404837,0.984589 L 0.404837,0.984589 C 0.444432,0.964342 0.482452,0.924072 0.517323,0.865804 0.569741,0.778065 0.613386,0.651856 0.643532,0.499775 0.663330,0.400787 0.677053,0.292576 0.684252,0.179865 L 0.494151,0.179865 0.749719,-0.000112 Z"#;
const CURVED_UP_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M 0.849831,-0.000112 L 0.970191,0.419685 1.030259,0.419685 1.030259,0.419685 C 1.026209,0.446907 1.021710,0.473678 1.016985,0.499775 0.988864,0.651631 0.948594,0.777840 0.899775,0.865804 0.851181,0.953543 0.796063,0.999663 0.739820,0.999663 0.683577,0.999663 0.628459,0.953543 0.579865,0.865804 0.562092,0.833633 0.545444,0.796288 0.529921,0.754218 L 0.529921,0.754218 C 0.556693,0.681552 0.579190,0.595613 0.596963,0.499775 0.601687,0.473678 0.606187,0.446907 0.610236,0.419685 L 0.670079,0.419685 0.849831,-0.000112 Z"#;
const CURVED_DOWN_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.749944,0.999888 L 0.494376,0.819685 0.684477,0.819685 0.684477,0.819685 C 0.677278,0.706974 0.663555,0.598763 0.643982,0.499775 0.613611,0.347694 0.569966,0.221485 0.517548,0.133746 0.465129,0.045782 0.405737,-0.000337 0.345219,-0.000337 L 0.464904,-0.000112 0.464904,-0.000112 C 0.525422,-0.000112 0.585039,0.046007 0.637458,0.133971 0.689876,0.221710 0.733296,0.347919 0.763667,0.499775 0.783465,0.598988 0.797188,0.707199 0.804162,0.819910 L 0.994263,0.819685 0.749944,0.999888 Z"#;
const CURVED_DOWN_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M 0.849831,0.999888 L 0.670079,0.579865 0.610236,0.579865 0.610236,0.579865 C 0.606187,0.552643 0.601687,0.525872 0.596963,0.499775 0.568841,0.347919 0.528571,0.221710 0.479753,0.133746 0.431159,0.046007 0.376040,-0.000112 0.319798,-0.000112 L 0.739820,-0.000112 0.739820,-0.000112 C 0.796063,-0.000112 0.851181,0.046007 0.899775,0.133971 0.948369,0.221710 0.988864,0.347919 1.016985,0.499775 1.021710,0.525872 1.026209,0.552643 1.030259,0.579865 L 0.970191,0.579865 0.849831,0.999888 Z"#;

fn curved_arrow_adjust_profile(adj: &HashMap<String, f64>) -> f64 {
    let adj1 = adj.get("adj1").copied().unwrap_or(25_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50_000.0);
    let adj3 = adj.get("adj3").copied().unwrap_or(25_000.0);
    let t1 = ((adj1 - 12_000.0) / 30_000.0).clamp(0.0, 1.0);
    let t2 = ((70_000.0 - adj2) / 40_000.0).clamp(0.0, 1.0);
    let t3 = ((adj3 - 18_000.0) / 24_000.0).clamp(0.0, 1.0);
    (t1 + t2 + t3) / 3.0
}

fn interpolate_normalized_paths(
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

fn curved_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.000342,0.388195 L 0.000342,0.388195 C 0.000342,0.456269 0.046543,0.523266 0.134155,0.582292 0.222108,0.641318 0.348049,0.690220 0.500342,0.724257 0.578371,0.741922 0.662560,0.755278 0.750171,0.764110 L 0.749829,0.685480 1.000000,0.855019 0.749829,1.000000 0.749829,0.921370 0.749829,0.921370 C 0.662218,0.912538 0.578371,0.899181 0.500000,0.881732 0.348049,0.847695 0.221766,0.798578 0.134155,0.739552 0.046201,0.680526 0.000000,0.613744 0.000000,0.545455 L 0.000342,0.388195 0.000342,0.388195 C 0.000342,0.320121 0.046543,0.253124 0.134155,0.194097 0.222108,0.135071 0.348049,0.086170 0.500342,0.052133 0.652293,0.017880 0.824435,0.000000 1.000000,0.000000 L 1.000000,0.157475 1.000000,0.157475 C 0.824435,0.157475 0.652293,0.175355 0.500342,0.209393 0.348049,0.243645 0.222108,0.292546 0.134155,0.351573 0.080767,0.387333 0.042779,0.426325 0.021218,0.467040",
            w,
            h,
        );
    }

    interpolate_normalized_paths(
        CURVED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        CURVED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        curved_arrow_adjust_profile(adj),
        w,
        h,
    )
}
fn curved_left_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 1.000000,0.564799 L 1.000000,0.564799 1.000000,0.564799 C 1.000000,0.509797 0.953820,0.455483 0.865929,0.408044 0.778251,0.360261 0.652054,0.320729 0.499894,0.292884 0.347946,0.265383 0.175569,0.250945 0.000000,0.250945 L 0.000000,0.000000 0.000000,0.000000 C 0.175569,0.000000 0.347946,0.014438 0.499894,0.041939 0.652054,0.069440 0.778251,0.109316 0.865929,0.156755 0.953820,0.204538 1.000000,0.258852 1.000000,0.313854 L 1.000000,0.313854 1.000000,0.564799 1.000000,0.564799 C 1.000000,0.619801 0.953820,0.674115 0.865929,0.721554 0.778251,0.769337 0.652054,0.808869 0.500106,0.836714 0.393701,0.855964 0.276867,0.868683 0.155352,0.874871 L 0.155352,1.000000 0.000000,0.752836 0.155352,0.498109 0.155352,0.623582 0.155352,0.623582 C 0.276655,0.617738 0.393488,0.604675 0.499894,0.585425 0.652054,0.557924 0.778251,0.518047 0.865929,0.470608 0.884656,0.460296 0.901468,0.449983 0.916365,0.438982",
            w,
            h,
        );
    }

    interpolate_normalized_paths(
        CURVED_LEFT_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        CURVED_LEFT_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        curved_arrow_adjust_profile(adj),
        w,
        h,
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

    interpolate_normalized_paths(
        CURVED_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        CURVED_UP_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        curved_arrow_adjust_profile(adj),
        w,
        h,
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

    interpolate_normalized_paths(
        CURVED_DOWN_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        CURVED_DOWN_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        curved_arrow_adjust_profile(adj),
        w,
        h,
    )
}
const CIRCULAR_ARROW_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.332552,0.225485 L 0.246128,0.271726 L 0.184911,0.329528 L 0.141698,0.398890 L 0.123693,0.462472 L 0.120092,0.508714 L 0.037269,0.508714 L 0.037269,0.445132 L 0.058875,0.352649 L 0.087683,0.289067 L 0.130895,0.225485 L 0.174107,0.179243 L 0.260532,0.115661 L 0.354159,0.075200 L 0.433381,0.057859 L 0.570220,0.057859 L 0.627836,0.069420 L 0.707059,0.098321 L 0.815089,0.167683 L 0.876307,0.231265 L 0.912317,0.294847 L 0.941125,0.294847 L 0.930322,0.491373 L 0.923120,0.508714 L 0.897913,0.479813 L 0.793483,0.323748 L 0.786281,0.323748 L 0.775478,0.289067 L 0.685453,0.231265 L 0.559417,0.196584 L 0.444184,0.196584 Z"#;
const CIRCULAR_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M 0.099880,0.499914 L 0.099880,0.499914 C 0.099880,0.429733 0.118367,0.360750 0.153458,0.299983 0.188548,0.239045 0.239045,0.188548 0.299812,0.153458 0.360750,0.118367 0.429733,0.099880 0.499914,0.099880 0.570096,0.099880 0.639079,0.118367 0.699846,0.153458 0.760613,0.188548 0.811109,0.239045 0.846371,0.299983 0.859038,0.322064 0.869651,0.345344 0.878038,0.369480 L 0.978004,0.369308 0.899777,0.499743 0.777901,0.369308 0.877867,0.369308 0.877867,0.369308 C 0.869480,0.345173 0.858867,0.321893 0.846200,0.299812 0.811109,0.239045 0.760613,0.188548 0.699675,0.153458 0.638908,0.118196 0.569925,0.099709 0.499743,0.099709 0.429562,0.099709 0.360579,0.118196 0.299812,0.153458 0.238874,0.188548 0.188377,0.239045 0.153287,0.299812 0.118196,0.360579 0.099709,0.429562 0.099709,0.499743 L 0.099880,0.499914 Z"#;
const CIRCULAR_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M 0.124872,0.499914 L 0.124872,0.499914 C 0.124872,0.434012 0.142160,0.369480 0.175197,0.312479 0.208062,0.255478 0.255306,0.208062 0.312307,0.175197 0.369308,0.142160 0.434012,0.124872 0.499914,0.124872 0.565645,0.124872 0.630349,0.142160 0.687350,0.175197 0.744351,0.208062 0.791767,0.255478 0.824632,0.312479 0.843632,0.345344 0.857669,0.381119 0.865885,0.418435 L 0.986220,0.418264 0.749829,0.499743 0.486220,0.418264 0.405084,0.418264 0.405084,0.418264 C 0.399949,0.424255 0.395498,0.430589 0.391561,0.437265 0.380606,0.456265 0.374786,0.477833 0.374786,0.499743 L 0.124872,0.499914 Z"#;
const CIRCULAR_ARROW_ADJ_SWEEP_NORMALIZED_PATH: &str = r#"M -0.000086,0.499914 L -0.000086,0.499914 C -0.000086,0.412102 0.023023,0.326001 0.066844,0.250000 0.110835,0.173827 0.173827,0.110835 0.249829,0.066844 0.326001,0.023023 0.412102,-0.000086 0.499914,-0.000086 0.587727,-0.000086 0.673827,0.023023 0.749829,0.066844 0.826001,0.110835 0.888993,0.173827 0.932985,0.249829 0.957463,0.292280 0.975608,0.338155 0.986734,0.385741 L 0.986563,0.385741 0.849795,0.499743 0.664071,0.385741 0.664071,0.385741 0.664071,0.385741 C 0.647295,0.361606 0.625385,0.341407 0.599880,0.326686 0.569411,0.309226 0.535005,0.299983 0.499914,0.299983 0.464824,0.299983 0.430246,0.309226 0.399777,0.326686 0.369480,0.344317 0.344146,0.369480 0.326686,0.399949 0.309055,0.430246 0.299812,0.464824 0.299812,0.499914 L -0.000086,0.499914 Z"#;
const CIRCULAR_ARROW_ADJ_THICK_NORMALIZED_PATH: &str = r#"M 0.187350,0.499914 L 0.187350,0.499914 C 0.187350,0.445139 0.201729,0.391219 0.229288,0.343632 0.256676,0.296217 0.296046,0.256676 0.343632,0.229288 0.391048,0.201900 0.444967,0.187350 0.499914,0.187350 0.554690,0.187350 0.608610,0.201900 0.656025,0.229288 0.703612,0.256676 0.742982,0.296217 0.770541,0.343632 0.784064,0.367083 0.794505,0.392246 0.801523,0.418435 L 0.986220,0.418264 0.749829,0.499743 0.486220,0.418264 0.330965,0.418264 0.330965,0.418264 C 0.318641,0.443769 0.312307,0.471499 0.312307,0.499743 L 0.187350,0.499914 Z"#;

fn circular_arrow_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(12_500.0);
    let adj5 = adj.get("adj5").copied().unwrap_or(12_500.0);
    let anchors = [
        (
            -20_000.0,
            10_000.0,
            CIRCULAR_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (25_000.0, 35_000.0, CIRCULAR_ARROW_ADJ_WIDE_NORMALIZED_PATH),
        (45_000.0, 15_000.0, CIRCULAR_ARROW_ADJ_SWEEP_NORMALIZED_PATH),
        (12_500.0, 45_000.0, CIRCULAR_ARROW_ADJ_THICK_NORMALIZED_PATH),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a5x, _), (a1y, a5y, _)| {
            let dx = (adj1 - *a1x) / 65_000.0;
            let dy = (adj5 - *a5x) / 35_000.0;
            let dxy = (adj1 - *a1y) / 65_000.0;
            let dyy = (adj5 - *a5y) / 35_000.0;
            (dx * dx + dy * dy)
                .partial_cmp(&(dxy * dxy + dyy * dyy))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, path)| path)
        .unwrap_or(CIRCULAR_ARROW_ADJ_WIDE_NORMALIZED_PATH)
}

fn circular_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(CIRCULAR_ARROW_DEFAULT_NORMALIZED_PATH, w, h);
    }

    scale_normalized_path(circular_arrow_adjust_anchor(adj), w, h)
}
const BENT_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH: &str = r#"M -0.000086,0.849795 L 0.774820,0.849795 0.774820,0.149863 0.699846,0.149863 0.849795,-0.000086 0.999914,0.149863 0.924769,0.149863 0.924769,0.999914 -0.000086,0.999914 -0.000086,0.849795 Z"#;
const BENT_UP_ARROW_ADJ_WIDE_NORMALIZED_PATH: &str = r#"M -0.000086,0.649863 L 0.474752,0.649863 0.474752,0.399777 0.299812,0.399777 0.649863,-0.000086 0.999914,0.399777 0.824803,0.399777 0.824803,0.999914 -0.000086,0.999914 -0.000086,0.649863 Z"#;
const BENT_UP_ARROW_ADJ_TALL_NORMALIZED_PATH: &str = r#"M -0.000086,0.749829 L 0.754793,0.749829 0.754793,0.499914 0.759757,0.499914 0.879750,-0.000086 0.999914,0.499914 1.004878,0.499914 1.004878,0.999914 -0.000086,0.999914 -0.000086,0.749829 Z"#;
const BENT_UP_ARROW_ADJ_DEEP_NORMALIZED_PATH: &str = r#"M -0.000086,0.799812 L 0.449760,0.799812 0.449760,0.199846 0.099880,0.199846 0.549897,-0.000086 0.999914,0.199846 0.649863,0.199846 0.649863,0.999914 -0.000086,0.999914 -0.000086,0.799812 Z"#;

fn bent_up_arrow_adjust_anchor(adj: &HashMap<String, f64>) -> &'static str {
    let adj1 = adj.get("adj1").copied().unwrap_or(25_000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(25_000.0);
    let adj3 = adj.get("adj3").copied().unwrap_or(25_000.0);
    let anchors = [
        (
            15_000.0,
            15_000.0,
            15_000.0,
            BENT_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
        ),
        (
            35_000.0,
            35_000.0,
            40_000.0,
            BENT_UP_ARROW_ADJ_WIDE_NORMALIZED_PATH,
        ),
        (
            25_000.0,
            15_000.0,
            50_000.0,
            BENT_UP_ARROW_ADJ_TALL_NORMALIZED_PATH,
        ),
        (
            20_000.0,
            45_000.0,
            20_000.0,
            BENT_UP_ARROW_ADJ_DEEP_NORMALIZED_PATH,
        ),
    ];

    anchors
        .into_iter()
        .min_by(|(a1x, a2x, a3x, _), (a1y, a2y, a3y, _)| {
            let dx1 = (adj1 - *a1x) / 20_000.0;
            let dx2 = (adj2 - *a2x) / 30_000.0;
            let dx3 = (adj3 - *a3x) / 35_000.0;
            let dy1 = (adj1 - *a1y) / 20_000.0;
            let dy2 = (adj2 - *a2y) / 30_000.0;
            let dy3 = (adj3 - *a3y) / 35_000.0;
            (dx1 * dx1 + dx2 * dx2 + dx3 * dx3)
                .partial_cmp(&(dy1 * dy1 + dy2 * dy2 + dy3 * dy3))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, _, _, path)| path)
        .unwrap_or(BENT_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH)
}

fn bent_up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.000000,0.736994 L 0.755396,0.736994 0.757194,0.260116 0.681655,0.242775 0.832734,0.000000 0.996403,0.228324 0.937050,0.312139 0.937050,0.994220 0.000000,0.994220 0.000000,0.736994 Z",
            w,
            h,
        );
    }

    scale_normalized_path(bent_up_arrow_adjust_anchor(adj), w, h)
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
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.806338,0.991573 L 0.764085,0.974719 L 0.517606,0.783708 L 0.485915,0.783708 L 0.246479,0.969101 L 0.193662,0.991573 L 0.301056,0.629213 L 0.005282,0.382022 L 0.380282,0.373596 L 0.496479,0.008427 L 0.508803,0.016854 L 0.619718,0.373596 L 0.985915,0.373596 L 0.994718,0.382022 L 0.985915,0.401685 L 0.698944,0.629213 L 0.806338,0.991573 Z",
            w,
            h,
        );
    }

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
    scale_normalized_path(
        "M 0.399293,0.991573 L 0.383392,0.966292 L 0.353357,0.750000 L 0.226148,0.817416 L 0.250883,0.665730 L 0.008834,0.668539 L 0.153710,0.544944 L 0.005300,0.398876 L 0.196113,0.348315 L 0.024735,0.132022 L 0.024735,0.109551 L 0.332155,0.283708 L 0.392226,0.109551 L 0.498233,0.261236 L 0.671378,0.008427 L 0.660777,0.238764 L 0.844523,0.205056 L 0.787986,0.334270 L 0.973498,0.382022 L 0.835689,0.488764 L 0.989399,0.598315 L 0.992933,0.620787 L 0.789753,0.623596 L 0.837456,0.834270 L 0.650177,0.699438 L 0.607774,0.912921 L 0.501767,0.727528 L 0.484099,0.727528 L 0.399293,0.991573 Z",
        w,
        h,
    )
}
fn irregular_seal2_path(w: f64, h: f64) -> String {
    scale_normalized_path(
        "M 0.238372,0.969203 L 0.235465,0.844203 L 0.209302,0.835145 L 0.078488,0.818841 L 0.165698,0.717391 L 0.165698,0.706522 L 0.020349,0.601449 L 0.186047,0.545290 L 0.194767,0.532609 L 0.072674,0.387681 L 0.258721,0.365942 L 0.229651,0.221014 L 0.232558,0.193841 L 0.369186,0.289855 L 0.401163,0.302536 L 0.415698,0.278986 L 0.450581,0.123188 L 0.462209,0.119565 L 0.514535,0.199275 L 0.529070,0.208333 L 0.540698,0.204710 L 0.668605,0.038043 L 0.659884,0.268116 L 0.680233,0.273551 L 0.796512,0.186594 L 0.747093,0.304348 L 0.755814,0.309783 L 0.968023,0.315217 L 0.776163,0.431159 L 0.828488,0.521739 L 0.755814,0.559783 L 0.747093,0.572464 L 0.845930,0.702899 L 0.674419,0.657609 L 0.665698,0.681159 L 0.677326,0.782609 L 0.563953,0.730072 L 0.549419,0.742754 L 0.531977,0.851449 L 0.479651,0.807971 L 0.453488,0.798913 L 0.438953,0.811594 L 0.401163,0.893116 L 0.363372,0.840580 L 0.337209,0.835145 L 0.238372,0.969203 Z",
        w,
        h,
    )
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
            "M 0.450237,0.991620 L 0.274882,0.913408 L 0.267773,0.882682 L 0.289100,0.801676 L 0.047393,0.801676 L 0.011848,0.787709 L 0.014218,0.544693 L 0.374408,0.544693 L 0.395735,0.463687 L 0.014218,0.449721 L 0.007109,0.217877 L 0.018957,0.203911 L 0.478673,0.203911 L 0.545024,0.008380 L 0.732227,0.083799 L 0.706161,0.203911 L 0.985782,0.203911 L 0.985782,0.449721 L 0.644550,0.455307 L 0.618483,0.474860 L 0.599526,0.541899 L 0.985782,0.544693 L 0.992891,0.782123 L 0.962085,0.801676 L 0.530806,0.801676 L 0.509479,0.826816 L 0.466825,0.977654 L 0.450237,0.991620 Z",
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
const MATH_MULTIPLY_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.301724,0.827586 L 0.155172,0.685345 L 0.155172,0.672414 L 0.323276,0.508621 L 0.323276,0.495690 L 0.155172,0.331897 L 0.155172,0.318966 L 0.318966,0.155172 L 0.331897,0.155172 L 0.495690,0.323276 L 0.508621,0.323276 L 0.672414,0.155172 L 0.685345,0.155172 L 0.849138,0.318966 L 0.849138,0.331897 L 0.676724,0.504310 L 0.849138,0.672414 L 0.849138,0.685345 L 0.685345,0.849138 L 0.672414,0.849138 L 0.504310,0.676724 L 0.331897,0.849138 L 0.318966,0.849138 Z"#;

fn math_multiply_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(MATH_MULTIPLY_DEFAULT_NORMALIZED_PATH, w, h);
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
const MATH_DIVIDE_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.493506,0.112069 L 0.536797,0.116379 L 0.580087,0.137931 L 0.606061,0.163793 L 0.619048,0.189655 L 0.619048,0.202586 L 0.623377,0.202586 L 0.623377,0.219828 L 0.627706,0.219828 L 0.627706,0.254310 L 0.623377,0.254310 L 0.623377,0.271552 L 0.619048,0.271552 L 0.619048,0.284483 L 0.606061,0.310345 L 0.567100,0.344828 L 0.536797,0.353448 L 0.536797,0.357759 L 0.515152,0.357759 L 0.515152,0.362069 L 0.467532,0.357759 L 0.424242,0.336207 L 0.393939,0.301724 L 0.385281,0.271552 L 0.380952,0.271552 L 0.380952,0.241379 L 0.376623,0.241379 L 0.380952,0.202586 L 0.402597,0.159483 L 0.437229,0.129310 L 0.467532,0.120690 L 0.467532,0.116379 L 0.493506,0.116379 Z M 0.874459,0.379310 L 0.878788,0.383621 L 0.878788,0.620690 L 0.874459,0.625000 L 0.129870,0.625000 L 0.125541,0.620690 L 0.125541,0.383621 L 0.129870,0.379310 Z M 0.437229,0.659483 L 0.467532,0.650862 L 0.467532,0.646552 L 0.489177,0.646552 L 0.489177,0.642241 L 0.536797,0.646552 L 0.536797,0.650862 L 0.549784,0.650862 L 0.575758,0.663793 L 0.606061,0.693966 L 0.619048,0.719828 L 0.619048,0.732759 L 0.623377,0.732759 L 0.623377,0.750000 L 0.627706,0.750000 L 0.627706,0.784483 L 0.623377,0.784483 L 0.623377,0.801724 L 0.619048,0.801724 L 0.619048,0.814655 L 0.606061,0.840517 L 0.567100,0.875000 L 0.536797,0.883621 L 0.536797,0.887931 L 0.510823,0.887931 L 0.510823,0.892241 L 0.467532,0.887931 L 0.424242,0.866379 L 0.393939,0.831897 L 0.385281,0.801724 L 0.380952,0.801724 L 0.380952,0.771552 L 0.376623,0.771552 L 0.380952,0.732759 L 0.402597,0.689655 Z"#;

fn math_divide_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(MATH_DIVIDE_DEFAULT_NORMALIZED_PATH, w, h);
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
    scale_normalized_path(
        "M 0.529070,0.998188 L 0.482558,0.994565 L 0.427326,0.960145 L 0.392442,0.909420 L 0.377907,0.864130 L 0.351744,0.865942 L 0.369186,0.909420 L 0.313953,0.936594 L 0.267442,0.936594 L 0.244186,0.929348 L 0.200581,0.902174 L 0.165698,0.862319 L 0.148256,0.829710 L 0.165698,0.807971 L 0.151163,0.802536 L 0.122093,0.817029 L 0.104651,0.813406 L 0.072674,0.793478 L 0.037791,0.735507 L 0.031977,0.663043 L 0.049419,0.608696 L 0.058140,0.603261 L 0.069767,0.610507 L 0.110465,0.614130 L 0.113372,0.601449 L 0.081395,0.596014 L 0.052326,0.581522 L 0.031977,0.557971 L 0.008721,0.489130 L 0.014535,0.423913 L 0.037791,0.376812 L 0.063953,0.353261 L 0.075581,0.346014 L 0.084302,0.365942 L 0.107558,0.365942 L 0.101744,0.257246 L 0.113372,0.213768 L 0.148256,0.148551 L 0.209302,0.103261 L 0.238372,0.096014 L 0.279070,0.103261 L 0.311047,0.119565 L 0.348837,0.153986 L 0.363372,0.148551 L 0.363372,0.141304 L 0.340116,0.119565 L 0.340116,0.108696 L 0.363372,0.072464 L 0.406977,0.041667 L 0.453488,0.038043 L 0.470930,0.045290 L 0.508721,0.079710 L 0.502907,0.105072 L 0.523256,0.110507 L 0.549419,0.043478 L 0.587209,0.012681 L 0.622093,0.009058 L 0.654070,0.025362 L 0.677326,0.050725 L 0.659884,0.086957 L 0.686047,0.092391 L 0.694767,0.065217 L 0.723837,0.032609 L 0.755814,0.012681 L 0.790698,0.009058 L 0.828488,0.028986 L 0.851744,0.054348 L 0.875000,0.108696 L 0.880814,0.152174 L 0.901163,0.153986 L 0.906977,0.146739 L 0.921512,0.159420 L 0.962209,0.231884 L 0.968023,0.322464 L 0.950581,0.376812 L 0.927326,0.416667 L 0.950581,0.420290 L 0.970930,0.385870 L 0.991279,0.456522 L 0.985465,0.550725 L 0.950581,0.630435 L 0.921512,0.663043 L 0.883721,0.686594 L 0.875000,0.637681 L 0.845930,0.568841 L 0.802326,0.527174 L 0.781977,0.532609 L 0.822674,0.576087 L 0.851744,0.644928 L 0.851744,0.750000 L 0.822674,0.818841 L 0.773256,0.864130 L 0.726744,0.875000 L 0.709302,0.871377 L 0.677326,0.851449 L 0.683140,0.804348 L 0.656977,0.802536 L 0.648256,0.869565 L 0.619186,0.931159 L 0.584302,0.971014 L 0.529070,0.998188 Z",
        w,
        h,
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
    let cy = ry;
    let t = w.min(h) * adj.get("adj").copied().unwrap_or(18750.0) / 100_000.0;
    let inner_rx = (rx - t).max(0.1);
    let inner_ry = (ry - t).max(0.1);
    let a = std::f64::consts::FRAC_PI_4;
    let (ca, sa) = (a.cos(), a.sin());
    let (bx1, by1) = (cx - rx * ca, cy - ry * sa);
    let (bx2, by2) = (cx + rx * ca, cy + ry * sa);
    let (dx, dy) = (t / 2.0 * sa, t / 2.0 * ca);
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z          M{cx:.1},{iy:.1} A{irx:.1},{iry:.1} 0 1,0 {cx:.1},{y2:.1} A{irx:.1},{iry:.1} 0 1,0 {cx:.1},{iy:.1} Z          M{a:.1},{b:.1} L{c:.1},{d:.1} L{e:.1},{f:.1} L{g:.1},{hh:.1} Z",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        iy = cy - inner_ry,
        y2 = cy + inner_ry,
        irx = inner_rx,
        iry = inner_ry,
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
        "M0,{bt:.1} L0,{bb:.1} A{rx:.1},{ry:.1} 0 0,0 {w:.1},{bb:.1} L{w:.1},{bt:.1} A{rx:.1},{ry:.1} 0 0,0 0,{bt:.1} Z M0,{bt:.1} A{rx:.1},{ry:.1} 0 0,0 {w:.1},{bt:.1} A{rx:.1},{ry:.1} 0 0,0 0,{bt:.1} Z",
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
    if adj.is_empty() {
        return scale_normalized_path(
            "M 0.936975,0.993776 L 0.718487,0.977178 L 0.504202,0.931535 L 0.340336,0.873444 L 0.163866,0.769710 L 0.065126,0.672199 L 0.031513,0.618257 L 0.006303,0.539419 L 0.010504,0.435685 L 0.044118,0.356846 L 0.073529,0.315353 L 0.151261,0.238589 L 0.235294,0.180498 L 0.415966,0.097510 L 0.592437,0.047718 L 0.705882,0.026971 L 0.987395,0.006224 L 0.991597,0.022822 L 0.819328,0.101660 L 0.676471,0.197095 L 0.598739,0.273859 L 0.556723,0.331950 L 0.523109,0.402490 L 0.510504,0.456432 L 0.518908,0.580913 L 0.552521,0.659751 L 0.586134,0.709544 L 0.668067,0.794606 L 0.777311,0.873444 L 0.894958,0.935685 L 0.993697,0.975104 L 0.987395,0.989627 L 0.936975,0.993776 Z",
            w,
            h,
        );
    }

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
const GEAR_6_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.500000,0.000000 L 0.521788,0.000000 L 0.543268,0.002215 L 0.561755,0.026892 L 0.578237,0.051185 L 0.591594,0.080519 L 0.607628,0.090279 L 0.621861,0.103565 L 0.637551,0.109820 L 0.652427,0.117189 L 0.666759,0.124757 L 0.680252,0.133099 L 0.690623,0.146268 L 0.702693,0.154885 L 0.711779,0.167355 L 0.722260,0.176472 L 0.732510,0.185129 L 0.745225,0.190013 L 0.757907,0.194808 L 0.770584,0.199522 L 0.785569,0.201773 L 0.804260,0.200636 L 0.818536,0.204256 L 0.838462,0.203072 L 0.859302,0.201824 L 0.885425,0.197145 L 0.906756,0.197142 L 0.918445,0.204597 L 0.924667,0.215629 L 0.930571,0.226429 L 0.941582,0.233752 L 0.952253,0.241234 L 0.956536,0.252151 L 0.960957,0.262625 L 0.960838,0.274995 L 0.966821,0.284024 L 0.972338,0.293087 L 0.981997,0.300265 L 0.979849,0.312113 L 0.978310,0.323273 L 0.944014,0.345435 L 0.931371,0.358782 L 0.892532,0.379409 L 0.886626,0.388811 L 0.863933,0.402303 L 0.866172,0.408550 L 0.868119,0.414800 L 0.868339,0.421355 L 0.868244,0.427858 L 0.868327,0.434217 L 0.870075,0.440212 L 0.872050,0.446144 L 0.874261,0.452035 L 0.874716,0.458126 L 0.874900,0.464198 L 0.874815,0.470248 L 0.874462,0.476270 L 0.874351,0.482236 L 0.874486,0.488167 L 0.874364,0.494089 L 0.874492,0.500000 L 0.874364,0.505911 L 0.874486,0.511833 L 0.874351,0.517764 L 0.874462,0.523730 L 0.874815,0.529752 L 0.874900,0.535802 L 0.874716,0.541874 L 0.874261,0.547965 L 0.872050,0.553856 L 0.870075,0.559788 L 0.868327,0.565783 L 0.868244,0.572142 L 0.868339,0.578645 L 0.868119,0.585200 L 0.866172,0.591450 L 0.863933,0.597697 L 0.886626,0.611189 L 0.892532,0.620591 L 0.931371,0.641218 L 0.944014,0.654565 L 0.978310,0.676727 L 0.979849,0.687887 L 0.981997,0.699735 L 0.977682,0.709254 L 0.972061,0.718401 L 0.965972,0.727512 L 0.960957,0.737375 L 0.956536,0.747849 L 0.952253,0.758766 L 0.941582,0.766248 L 0.930571,0.773571 L 0.924667,0.784371 L 0.918445,0.795403 L 0.906756,0.802858 L 0.883879,0.801640 L 0.861990,0.800407 L 0.841053,0.799200 L 0.822411,0.799342 L 0.804260,0.799364 L 0.785569,0.798227 L 0.770584,0.800478 L 0.754908,0.801643 L 0.746319,0.811370 L 0.736230,0.819909 L 0.728288,0.832302 L 0.717060,0.840940 L 0.705407,0.849736 L 0.693136,0.858394 L 0.680252,0.866901 L 0.666759,0.875243 L 0.652427,0.882811 L 0.637551,0.890180 L 0.621861,0.896435 L 0.607628,0.909721 L 0.591594,0.919481 L 0.578237,0.948815 L 0.561755,0.973108 L 0.543268,0.997785 L 0.521788,1.000000 L 0.500000,1.000000 L 0.478212,1.000000 L 0.457477,0.989218 L 0.437767,0.976773 L 0.422666,0.943635 L 0.407411,0.924037 L 0.394359,0.902156 L 0.377491,0.898544 L 0.362132,0.891079 L 0.348996,0.879236 L 0.336923,0.866957 L 0.323212,0.859851 L 0.308278,0.855772 L 0.293237,0.852047 L 0.282211,0.842084 L 0.270740,0.833718 L 0.260877,0.823827 L 0.250619,0.815241 L 0.241170,0.806283 L 0.228446,0.801555 L 0.213415,0.799288 L 0.195740,0.799364 L 0.181464,0.795744 L 0.162977,0.795665 L 0.142192,0.796937 L 0.116121,0.801640 L 0.093244,0.802858 L 0.076275,0.799130 L 0.064453,0.791657 L 0.058236,0.780683 L 0.052309,0.769931 L 0.046272,0.759609 L 0.041954,0.748669 L 0.037883,0.737972 L 0.034028,0.727512 L 0.027939,0.718401 L 0.021907,0.709435 L 0.017585,0.699908 L 0.013759,0.690390 L 0.015191,0.679129 L 0.055546,0.654718 L 0.070416,0.640633 L 0.109732,0.619896 L 0.107412,0.612904 L 0.134210,0.598195 L 0.131950,0.591919 L 0.131881,0.585200 L 0.131661,0.578645 L 0.129823,0.572521 L 0.127775,0.566479 L 0.125998,0.560423 L 0.125974,0.554142 L 0.125739,0.547965 L 0.125284,0.541874 L 0.125100,0.535802 L 0.125185,0.529752 L 0.125538,0.523730 L 0.125649,0.517764 L 0.125514,0.511833 L 0.125636,0.505911 L 0.125508,0.500000 L 0.125636,0.494089 L 0.125514,0.488167 L 0.125649,0.482236 L 0.125538,0.476270 L 0.125185,0.470248 L 0.125100,0.464198 L 0.125284,0.458126 L 0.125739,0.452035 L 0.125974,0.445858 L 0.125998,0.439577 L 0.127775,0.433521 L 0.129823,0.427479 L 0.131661,0.421355 L 0.131881,0.414800 L 0.131950,0.408081 L 0.131889,0.401182 L 0.105119,0.386437 L 0.111543,0.380661 L 0.074435,0.360683 L 0.059506,0.346661 L 0.015191,0.320871 L 0.013759,0.309610 L 0.017585,0.300092 L 0.021907,0.290565 L 0.027939,0.281599 L 0.034028,0.272488 L 0.037883,0.262028 L 0.041954,0.251331 L 0.046272,0.240391 L 0.052309,0.230069 L 0.058236,0.219317 L 0.064453,0.208343 L 0.076275,0.200870 L 0.093244,0.197142 L 0.116121,0.198360 L 0.142192,0.203063 L 0.162977,0.204335 L 0.181464,0.204256 L 0.195740,0.200636 L 0.214431,0.201773 L 0.229416,0.199522 L 0.242093,0.194808 L 0.250837,0.185035 L 0.261084,0.176453 L 0.270934,0.166566 L 0.282211,0.157916 L 0.293915,0.149109 L 0.308906,0.145393 L 0.323789,0.141324 L 0.336923,0.133043 L 0.348996,0.120764 L 0.362132,0.108921 L 0.377861,0.102661 L 0.394677,0.099054 L 0.407677,0.077178 L 0.422666,0.056365 L 0.437767,0.023227 L 0.457477,0.010782 L 0.478212,0.000000 Z
"#;

const GEAR_9_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.500000,0.000000 L 0.512975,0.000000 L 0.525960,0.000000 L 0.537277,0.020124 L 0.547400,0.043087 L 0.555186,0.075302 L 0.564335,0.088463 L 0.573105,0.100373 L 0.582955,0.104594 L 0.593311,0.106219 L 0.602146,0.113771 L 0.612090,0.116612 L 0.622068,0.119369 L 0.633547,0.117913 L 0.643632,0.120904 L 0.651306,0.129908 L 0.660817,0.134048 L 0.670217,0.138438 L 0.681793,0.138517 L 0.691242,0.143141 L 0.698232,0.152134 L 0.711488,0.150343 L 0.723200,0.151780 L 0.753371,0.126493 L 0.777353,0.113233 L 0.803722,0.098974 L 0.813525,0.107734 L 0.823318,0.116462 L 0.833117,0.125168 L 0.837969,0.139169 L 0.834167,0.161436 L 0.830762,0.181988 L 0.823584,0.204810 L 0.820676,0.222513 L 0.816241,0.240543 L 0.818830,0.252134 L 0.825708,0.260245 L 0.831494,0.269173 L 0.834621,0.279835 L 0.840502,0.288594 L 0.846160,0.297515 L 0.855395,0.304497 L 0.860967,0.313658 L 0.866635,0.322823 L 0.870428,0.332905 L 0.873618,0.343206 L 0.876531,0.353562 L 0.880865,0.363354 L 0.893872,0.370339 L 0.920262,0.373875 L 0.954738,0.376550 L 0.980572,0.383107 L 0.991544,0.394144 L 0.994465,0.407150 L 0.995626,0.420468 L 0.998584,0.433497 L 1.000000,0.446742 L 0.995941,0.460455 L 0.966531,0.475228 L 0.936358,0.488423 L 0.910534,0.500000 L 0.908229,0.510831 L 0.906729,0.521596 L 0.906751,0.532433 L 0.906850,0.543324 L 0.906664,0.554243 L 0.900848,0.564323 L 0.899403,0.574999 L 0.893094,0.584654 L 0.895683,0.596245 L 0.888531,0.605476 L 0.889131,0.616783 L 0.886324,0.627176 L 0.887666,0.639086 L 0.880572,0.648010 L 0.876284,0.657913 L 0.882292,0.672447 L 0.906684,0.696531 L 0.931360,0.722681 L 0.944164,0.744335 L 0.941760,0.758406 L 0.934625,0.769843 L 0.927202,0.781078 L 0.918604,0.791484 L 0.898671,0.793463 L 0.870060,0.787693 L 0.839511,0.778548 L 0.819030,0.776063 L 0.804282,0.777582 L 0.793342,0.782034 L 0.783149,0.786874 L 0.775888,0.794551 L 0.770839,0.804755 L 0.765683,0.815168 L 0.757449,0.822107 L 0.749478,0.829404 L 0.738337,0.832359 L 0.732325,0.842482 L 0.723200,0.848220 L 0.716765,0.858383 L 0.705087,0.859894 L 0.701226,0.875489 L 0.695386,0.888513 L 0.698172,0.920943 L 0.694124,0.941747 L 0.689685,0.963966 L 0.680218,0.975662 L 0.669312,0.984413 L 0.657184,0.990128 L 0.641188,0.982914 L 0.622874,0.964605 L 0.605606,0.945670 L 0.589780,0.927940 L 0.577049,0.921190 L 0.565181,0.916952 L 0.554386,0.918538 L 0.543440,0.918746 L 0.532635,0.920119 L 0.521807,0.921571 L 0.510935,0.923101 L 0.500000,0.923246 L 0.489065,0.923101 L 0.478193,0.921571 L 0.467365,0.920119 L 0.456560,0.918746 L 0.445755,0.917450 L 0.435665,0.911537 L 0.423411,0.918674 L 0.408195,0.937592 L 0.390436,0.962373 L 0.372645,0.981551 L 0.357070,0.988872 L 0.341813,0.993256 L 0.330688,0.984413 L 0.318101,0.980098 L 0.307416,0.971057 L 0.302061,0.950429 L 0.299809,0.925231 L 0.303795,0.890141 L 0.298946,0.875168 L 0.295094,0.859578 L 0.283423,0.858071 L 0.276800,0.848220 L 0.267675,0.842482 L 0.261663,0.832359 L 0.250522,0.829404 L 0.242551,0.822107 L 0.231505,0.818503 L 0.223368,0.811273 L 0.215421,0.803830 L 0.208178,0.795662 L 0.200901,0.787569 L 0.192769,0.780272 L 0.180970,0.776063 L 0.171703,0.769347 L 0.150261,0.771896 L 0.122050,0.778210 L 0.090613,0.785066 L 0.072798,0.781078 L 0.065375,0.769843 L 0.059802,0.757493 L 0.052983,0.745904 L 0.064461,0.724838 L 0.084850,0.700622 L 0.108151,0.676758 L 0.115383,0.661410 L 0.115050,0.649713 L 0.113694,0.638598 L 0.110931,0.628079 L 0.107410,0.617821 L 0.108682,0.606233 L 0.107123,0.595562 L 0.104789,0.585110 L 0.099178,0.575265 L 0.096302,0.564781 L 0.094767,0.554052 L 0.093150,0.543324 L 0.093249,0.532433 L 0.093271,0.521596 L 0.092131,0.510821 L 0.094877,0.500000 L 0.086361,0.489026 L 0.063010,0.476797 L 0.028514,0.462405 L 0.007044,0.447507 L 0.001416,0.433497 L 0.008650,0.421154 L 0.009791,0.407950 L 0.012691,0.395056 L 0.019428,0.383107 L 0.045262,0.376550 L 0.084235,0.375224 L 0.110588,0.371808 L 0.127296,0.366282 L 0.127174,0.355002 L 0.131381,0.345304 L 0.132209,0.334094 L 0.136622,0.324397 L 0.140962,0.314654 L 0.149678,0.307288 L 0.155089,0.298246 L 0.160728,0.289358 L 0.166589,0.280631 L 0.172668,0.272071 L 0.182756,0.266476 L 0.185463,0.255472 L 0.189646,0.245374 L 0.194137,0.235331 L 0.194110,0.220951 L 0.179705,0.192052 L 0.163793,0.159369 L 0.157064,0.133866 L 0.166883,0.125168 L 0.176682,0.116462 L 0.189880,0.111995 L 0.207259,0.113472 L 0.232189,0.126539 L 0.251738,0.134023 L 0.272085,0.144423 L 0.287570,0.148785 L 0.301768,0.152134 L 0.311168,0.147638 L 0.318207,0.138517 L 0.329783,0.138438 L 0.339183,0.134048 L 0.348694,0.129908 L 0.356368,0.120904 L 0.366453,0.117913 L 0.377932,0.119369 L 0.387910,0.116612 L 0.397854,0.113771 L 0.406689,0.106219 L 0.417045,0.104594 L 0.427158,0.101811 L 0.435891,0.089907 L 0.445190,0.078201 L 0.453505,0.051811 L 0.463403,0.028869 L 0.474417,0.005430 L 0.487025,0.000000 Z
"#;

fn gear_path(w: f64, h: f64, teeth: u32) -> String {
    match teeth {
        6 => scale_normalized_path(GEAR_6_DEFAULT_NORMALIZED_PATH, w, h),
        9 => scale_normalized_path(GEAR_9_DEFAULT_NORMALIZED_PATH, w, h),
        _ => {
            let (cx, cy) = (w / 2.0, h / 2.0);
            let (ro, ri) = (w.min(h) * 0.48, w.min(h) * 0.38);
            let t = teeth * 2;
            let mut pts: Vec<(f64, f64)> = Vec::with_capacity(t as usize);
            for i in 0..t {
                let a = 2.0 * std::f64::consts::PI * (i as f64) / (t as f64)
                    - std::f64::consts::FRAC_PI_2;
                let r = if i % 2 == 0 { ro } else { ri };
                pts.push((cx + r * a.cos(), cy + r * a.sin()));
            }
            let mut p = format!("M{:.1},{:.1}", pts[0].0, pts[0].1);
            for &(x, y) in &pts[1..] {
                p.push_str(&format!(" L{x:.1},{y:.1}"));
            }
            p.push_str(" Z");
            p
        }
    }
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
        "M0,{h:.1} L{w:.1},{h:.1} L{w:.1},0 A{w:.1},{h:.1} 0 0,0 0,{h:.1} Z",
        w = w,
        h = h
    )
}
const ARC_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.664260,0.023256 L 0.707581,0.034884 L 0.707581,0.040698 L 0.732852,0.046512 L 0.750903,0.063953 L 0.772563,0.069767 L 0.779783,0.081395 L 0.790614,0.081395 L 0.848375,0.127907 L 0.851986,0.139535 L 0.859206,0.139535 L 0.862816,0.151163 L 0.891697,0.174419 L 0.898917,0.191860 L 0.906137,0.191860 L 0.953069,0.267442 L 0.953069,0.279070 L 0.963899,0.290698 L 0.974729,0.331395 L 0.981949,0.337209 L 0.989170,0.377907 L 0.996390,0.389535 L 0.996390,0.406977 L 1.000000,0.406977 L 1.000000,0.430233 L 1.000000,0.430233 L 1.000000,0.500000 L 0.996390,0.505814 L 0.501805,0.505814 L 0.501805,0.000000 L 0.595668,0.000000 L 0.595668,0.005814 L 0.624549,0.005814 L 0.624549,0.011628 L 0.646209,0.011628 L 0.646209,0.017442 L 0.664260,0.017442 Z"#;

fn arc_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(ARC_DEFAULT_NORMALIZED_PATH, w, h);
    }

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
    let outer = format!(
        "M{sx:.1},{sy:.1} C{c1x:.1},{c1y:.1} {c2x:.1},{c2y:.1} {x1:.1},{y1:.1} C{c3x:.1},{c3y:.1} {c4x:.1},{c4y:.1} {x2:.1},{y2:.1} C{c5x:.1},{c5y:.1} {c6x:.1},{c6y:.1} {x3:.1},{y3:.1} C{c7x:.1},{c7y:.1} {c8x:.1},{c8y:.1} {x4:.1},{y4:.1} C{c9x:.1},{c9y:.1} {c10x:.1},{c10y:.1} {x5:.1},{y5:.1} C{c11x:.1},{c11y:.1} {c12x:.1},{c12y:.1} {x6:.1},{y6:.1} C{c13x:.1},{c13y:.1} {c14x:.1},{c14y:.1} {x7:.1},{y7:.1} C{c15x:.1},{c15y:.1} {c16x:.1},{c16y:.1} {x8:.1},{y8:.1} C{c17x:.1},{c17y:.1} {c18x:.1},{c18y:.1} {x9:.1},{y9:.1} C{c19x:.1},{c19y:.1} {c20x:.1},{c20y:.1} {x10:.1},{y10:.1} C{c21x:.1},{c21y:.1} {c22x:.1},{c22y:.1} {x11:.1},{y11:.1} C{c23x:.1},{c23y:.1} {c24x:.1},{c24y:.1} {x12:.1},{y12:.1} C{c25x:.1},{c25y:.1} {c26x:.1},{c26y:.1} {x13:.1},{y13:.1} C{c27x:.1},{c27y:.1} {c28x:.1},{c28y:.1} {x14:.1},{y14:.1} L{sx:.1},{sy:.1} Z"
    );
    let hole_cx = w * 0.49955;
    let hole_cy = h * 0.26229;
    let hole_rx = w * 0.45225;
    let hole_ry = h * 0.17101;
    let hole = format!(
        "M{left:.1},{cy:.1} A{rx:.1},{ry:.1} 0 1,0 {right:.1},{cy:.1} A{rx:.1},{ry:.1} 0 1,0 {left:.1},{cy:.1} Z",
        left = hole_cx - hole_rx,
        right = hole_cx + hole_rx,
        cy = hole_cy,
        rx = hole_rx,
        ry = hole_ry
    );
    format!("{outer} {hole}")
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
const QUAD_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.976608,0.525362 L 0.824561,0.623188 L 0.812865,0.619565 L 0.812865,0.565217 L 0.754386,0.565217 L 0.754386,0.746377 L 0.602339,0.750000 L 0.602339,0.884058 L 0.695906,0.884058 L 0.695906,0.894928 L 0.520468,1.000000 L 0.485380,1.000000 L 0.309942,0.894928 L 0.309942,0.884058 L 0.403509,0.884058 L 0.403509,0.750000 L 0.251462,0.746377 L 0.251462,0.565217 L 0.192982,0.565217 L 0.192982,0.619565 L 0.175439,0.619565 L 0.000000,0.510870 L 0.000000,0.492754 L 0.175439,0.384058 L 0.192982,0.384058 L 0.192982,0.438406 L 0.251462,0.438406 L 0.251462,0.257246 L 0.403509,0.253623 L 0.403509,0.119565 L 0.309942,0.119565 L 0.309942,0.108696 L 0.485380,0.000000 L 0.520468,0.000000 L 0.695906,0.108696 L 0.695906,0.119565 L 0.602339,0.119565 L 0.602339,0.253623 L 0.754386,0.257246 L 0.754386,0.438406 L 0.812865,0.438406 L 0.812865,0.384058 L 0.824561,0.380435 L 1.000000,0.492754 L 1.000000,0.510870 Z"#;

fn quad_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(QUAD_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH, w, h);
    }

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
const LEFT_RIGHT_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.256318,1.000000 L 0.256318,0.639535 L 0.162455,0.633721 L 0.162455,0.755814 L 0.151625,0.761628 L 0.000000,0.517442 L 0.000000,0.488372 L 0.151625,0.244186 L 0.162455,0.250000 L 0.162455,0.372093 L 0.256318,0.366279 L 0.256318,0.000000 L 0.747292,0.000000 L 0.747292,0.372093 L 0.841155,0.372093 L 0.841155,0.250000 L 0.851986,0.244186 L 1.000000,0.488372 L 1.000000,0.517442 L 0.920578,0.651163 L 0.913357,0.651163 L 0.848375,0.761628 L 0.841155,0.755814 L 0.841155,0.633721 L 0.747292,0.633721 L 0.747292,1.000000 Z"#;

fn left_right_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(LEFT_RIGHT_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH, w, h);
    }

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
const UP_DOWN_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.340580,0.255814 L 0.344203,0.238372 L 0.492754,0.000000 L 0.510870,0.000000 L 0.663043,0.244186 L 0.663043,0.255814 L 1.000000,0.255814 L 1.000000,0.750000 L 0.663043,0.750000 L 0.663043,0.761628 L 0.510870,1.000000 L 0.492754,1.000000 L 0.344203,0.767442 L 0.340580,0.750000 L 0.000000,0.750000 L 0.000000,0.255814 Z"#;

fn up_down_arrow_callout_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(UP_DOWN_ARROW_CALLOUT_DEFAULT_NORMALIZED_PATH, w, h);
    }

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
const CORNER_TABS_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.099415,0.000000 L 0.000000,0.061594 L 0.000000,0.000000 Z M 0.953216,0.032609 L 0.906433,0.000000 L 1.000000,0.000000 L 1.000000,0.061594 Z M 0.070175,0.985507 L 0.099415,1.000000 L 0.000000,1.000000 L 0.000000,0.942029 Z M 0.988304,0.952899 L 1.000000,0.942029 L 1.000000,1.000000 L 0.906433,1.000000 Z"#;

fn corner_tabs_path(w: f64, h: f64) -> String {
    scale_normalized_path(CORNER_TABS_DEFAULT_NORMALIZED_PATH, w, h)
}

const PLAQUE_TABS_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.000000,0.077586 L 0.000000,0.000000 L 0.077586,0.000000 L 0.077586,0.021552 L 0.068966,0.030172 L 0.068966,0.043103 L 0.043103,0.068966 Z M 1.000000,0.077586 L 0.987069,0.077586 L 0.987069,0.073276 L 0.974138,0.073276 L 0.974138,0.068966 L 0.956897,0.064655 L 0.931034,0.030172 L 0.926724,0.000000 L 1.000000,0.000000 Z M 0.000000,0.926724 L 0.043103,0.935345 L 0.068966,0.961207 L 0.068966,0.974138 L 0.077586,0.982759 L 0.077586,1.000000 L 0.000000,1.000000 Z M 0.987069,0.931034 L 0.987069,0.926724 L 1.000000,0.926724 L 1.000000,1.000000 L 0.926724,1.000000 L 0.931034,0.974138 L 0.935345,0.974138 L 0.939655,0.956897 L 0.956897,0.939655 L 0.965517,0.939655 L 0.974138,0.931034 Z"#;

fn plaque_tabs_path(w: f64, h: f64) -> String {
    scale_normalized_path(PLAQUE_TABS_DEFAULT_NORMALIZED_PATH, w, h)
}

const SQUARE_TABS_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.077586,0.073276 L 0.073276,0.077586 L 0.000000,0.077586 L 0.000000,0.000000 L 0.077586,0.000000 Z M 1.000000,0.077586 L 0.931034,0.077586 L 0.926724,0.073276 L 0.926724,0.000000 L 1.000000,0.000000 Z M 0.000000,0.926724 L 0.073276,0.926724 L 0.077586,0.931034 L 0.077586,1.000000 L 0.000000,1.000000 Z M 0.926724,1.000000 L 0.926724,0.931034 L 0.931034,0.926724 L 1.000000,0.926724 L 1.000000,1.000000 Z"#;

fn square_tabs_path(w: f64, h: f64) -> String {
    scale_normalized_path(SQUARE_TABS_DEFAULT_NORMALIZED_PATH, w, h)
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
const LEFT_CIRCULAR_ARROW_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.595584,0.954034 L 0.583998,0.961241 L 0.526068,0.968448 L 0.473932,0.968448 L 0.398623,0.957637 L 0.300143,0.921601 L 0.201663,0.853131 L 0.143734,0.788266 L 0.097390,0.708985 L 0.062632,0.608083 L 0.056839,0.496369 L 0.195870,0.499973 L 0.207456,0.622497 L 0.259593,0.741418 L 0.323315,0.813491 L 0.381245,0.853131 L 0.439174,0.874753 L 0.491311,0.881961 L 0.543447,0.878357 L 0.601377,0.860339 L 0.670892,0.817095 L 0.717236,0.770247 L 0.769372,0.683759 L 0.792544,0.615290 L 0.792544,0.586461 L 0.734614,0.582857 L 0.734614,0.575650 L 0.775165,0.550424 L 0.786751,0.550424 L 0.867852,0.496369 L 0.879438,0.496369 L 0.995297,0.572046 L 0.995297,0.586461 L 0.937368,0.586461 L 0.902610,0.705381 L 0.821509,0.827906 L 0.711443,0.914393 L 0.647720,0.943223 Z"#;

fn left_circular_arrow_path(w: f64, h: f64) -> String {
    scale_normalized_path(LEFT_CIRCULAR_ARROW_DEFAULT_NORMALIZED_PATH, w, h)
}

const LEFT_RIGHT_CIRCULAR_ARROW_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.126355,0.253900 L 0.137941,0.225156 L 0.195870,0.156887 L 0.248007,0.113770 L 0.323315,0.070653 L 0.398623,0.045501 L 0.468139,0.034722 L 0.583998,0.041908 L 0.699857,0.081432 L 0.780958,0.135329 L 0.838888,0.192818 L 0.879438,0.250307 L 0.919989,0.336541 L 0.937368,0.415589 L 0.995297,0.415589 L 1.000000,0.426369 L 0.879438,0.505417 L 0.862059,0.501824 L 0.740407,0.426369 L 0.740407,0.415589 L 0.798337,0.415589 L 0.798337,0.404810 L 0.769372,0.314983 L 0.699857,0.210783 L 0.659306,0.174853 L 0.595584,0.138922 L 0.543447,0.124549 L 0.491311,0.120956 L 0.439174,0.128142 L 0.375452,0.153294 L 0.294350,0.217970 L 0.242214,0.293424 L 0.201663,0.411996 L 0.259593,0.415589 L 0.265386,0.422776 L 0.132148,0.505417 L 0.120562,0.505417 L 0.004703,0.429962 L 0.000000,0.419183 L 0.062632,0.415589 L 0.080011,0.340135 Z"#;

fn left_right_circular_arrow_path(w: f64, h: f64) -> String {
    scale_normalized_path(LEFT_RIGHT_CIRCULAR_ARROW_DEFAULT_NORMALIZED_PATH, w, h)
}

// Misc shapes
const CHORD_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.121212,0.168103 L 0.194805,0.094828 L 0.203463,0.094828 L 0.233766,0.068966 L 0.251082,0.064655 L 0.255411,0.056034 L 0.290043,0.038793 L 0.303030,0.038793 L 0.346320,0.017241 L 0.419913,0.000000 L 0.510823,0.000000 L 0.510823,0.012931 L 0.523810,0.030172 L 0.523810,0.043103 L 0.545455,0.081897 L 0.562771,0.137931 L 0.575758,0.155172 L 0.584416,0.189655 L 0.619048,0.258621 L 0.636364,0.314655 L 0.670996,0.383621 L 0.688312,0.439655 L 0.701299,0.456897 L 0.709957,0.491379 L 0.744589,0.560345 L 0.761905,0.616379 L 0.796537,0.685345 L 0.813853,0.741379 L 0.848485,0.810345 L 0.861472,0.862069 L 0.826840,0.896552 L 0.818182,0.896552 L 0.809524,0.909483 L 0.800866,0.909483 L 0.792208,0.922414 L 0.761905,0.935345 L 0.757576,0.943966 L 0.683983,0.978448 L 0.584416,1.000000 L 0.419913,1.000000 L 0.376623,0.995690 L 0.255411,0.948276 L 0.251082,0.939655 L 0.233766,0.935345 L 0.229437,0.926724 L 0.194805,0.909483 L 0.181818,0.892241 L 0.173160,0.892241 L 0.112554,0.831897 L 0.112554,0.823276 L 0.095238,0.810345 L 0.095238,0.801724 L 0.082251,0.793103 L 0.069264,0.762931 L 0.060606,0.758621 L 0.025974,0.685345 L 0.000000,0.594828 L 0.000000,0.409483 L 0.008658,0.370690 L 0.030303,0.306034 L 0.060606,0.245690 L 0.069264,0.241379 L 0.073593,0.224138 L 0.095238,0.202586 L 0.095238,0.193966 Z"#;

fn chord_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(CHORD_DEFAULT_NORMALIZED_PATH, w, h);
    }

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
const LEFT_RIGHT_RIBBON_DEFAULT_NORMALIZED_PATH: &str = r#"M 0.137931,0.547414 L 0.116379,0.525862 L 0.107759,0.525862 L 0.077586,0.491379 L 0.068966,0.491379 L 0.038793,0.456897 L 0.030172,0.456897 L 0.000000,0.426724 L 0.000000,0.409483 L 0.021552,0.387931 L 0.030172,0.387931 L 0.060345,0.353448 L 0.068966,0.353448 L 0.099138,0.318966 L 0.107759,0.318966 L 0.137931,0.284483 L 0.146552,0.284483 L 0.176724,0.250000 L 0.185345,0.250000 L 0.215517,0.215517 L 0.224138,0.215517 L 0.254310,0.181034 L 0.262931,0.181034 L 0.293103,0.146552 L 0.301724,0.146552 L 0.331897,0.112069 L 0.340517,0.112069 L 0.370690,0.077586 L 0.379310,0.077586 L 0.409483,0.043103 L 0.418103,0.043103 L 0.448276,0.008621 L 0.474138,0.000000 L 0.474138,0.159483 L 0.512931,0.163793 L 0.521552,0.172414 L 0.538793,0.163793 L 0.547414,0.176724 L 0.556034,0.176724 L 0.586207,0.211207 L 0.594828,0.211207 L 0.625000,0.245690 L 0.633621,0.245690 L 0.663793,0.280172 L 0.672414,0.280172 L 0.702586,0.314655 L 0.711207,0.314655 L 0.741379,0.349138 L 0.750000,0.349138 L 0.780172,0.383621 L 0.788793,0.383621 L 0.818966,0.418103 L 0.827586,0.418103 L 0.857759,0.452586 L 0.866379,0.452586 L 0.896552,0.487069 L 0.905172,0.487069 L 0.935345,0.521552 L 0.943966,0.521552 L 0.974138,0.556034 L 0.982759,0.556034 L 1.000000,0.577586 L 1.000000,0.594828 L 0.987069,0.603448 L 0.956897,0.637931 L 0.948276,0.637931 L 0.918103,0.672414 L 0.909483,0.672414 L 0.879310,0.706897 L 0.870690,0.706897 L 0.840517,0.741379 L 0.831897,0.741379 L 0.801724,0.775862 L 0.793103,0.775862 L 0.762931,0.810345 L 0.754310,0.810345 L 0.724138,0.844828 L 0.715517,0.844828 L 0.685345,0.879310 L 0.676724,0.879310 L 0.646552,0.913793 L 0.637931,0.913793 L 0.607759,0.948276 L 0.599138,0.948276 L 0.568966,0.982759 L 0.560345,0.982759 L 0.543103,1.000000 L 0.525862,1.000000 L 0.525862,0.840517 L 0.491379,0.840517 L 0.482759,0.831897 L 0.474138,0.831897 L 0.474138,0.840517 L 0.456897,0.836207 L 0.426724,0.801724 L 0.418103,0.801724 L 0.387931,0.767241 L 0.379310,0.767241 L 0.349138,0.732759 L 0.340517,0.732759 L 0.310345,0.698276 L 0.301724,0.698276 L 0.271552,0.663793 L 0.262931,0.663793 L 0.232759,0.629310 L 0.224138,0.629310 Z"#;

fn left_right_ribbon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    if adj.is_empty() {
        return scale_normalized_path(LEFT_RIGHT_RIBBON_DEFAULT_NORMALIZED_PATH, w, h);
    }

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
    fn test_right_triangle_default_path_keeps_the_right_angle_on_the_left() {
        let adj = HashMap::new();
        let path = preset_shape_svg("rtTriangle", 120.0, 100.0, &adj).unwrap();

        assert_eq!(path, "M0,0 L120.0,100.0 L0,100.0 Z");
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
            "M 112.4,99.4 L 86.2,97.7 L 60.5,93.2 L 40.8,87.3 L 19.7,77.0 L 7.8,67.2 L 3.8,61.8 L 0.8,53.9 L 1.3,43.6 L 5.3,35.7 L 8.8,31.5 L 18.2,23.9 L 28.2,18.0 L 49.9,9.8 L 71.1,4.8 L 84.7,2.7 L 118.5,0.6 L 119.0,2.3 L 98.3,10.2 L 81.2,19.7 L 71.8,27.4 L 66.8,33.2 L 62.8,40.2 L 61.3,45.6 L 62.3,58.1 L 66.3,66.0 L 70.3,71.0 L 80.2,79.5 L 93.3,87.3 L 107.4,93.6 L 119.2,97.5 L 118.5,99.0 L 112.4,99.4 Z"
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

        assert!(path.contains("59.6,133.6"));
        assert!(path.contains("6.3,85.1"));
        assert!(path.contains("99.5,80.1"));
        assert!(path.contains("64.8,132.1"));
    }

    #[test]
    fn test_left_right_circular_arrow_default_path_tracks_arch_reference() {
        let path = left_right_circular_arrow_path(100.0, 140.0);

        assert!(path.contains("46.8,4.9"));
        assert!(path.contains("78.1,18.9"));
        assert!(path.contains("13.2,70.8"));
        assert!(path.contains("0.5,60.2"));
    }

    #[test]
    fn test_circular_arrow_default_path_tracks_office_arc_span() {
        let adj = HashMap::new();
        let path = circular_arrow_path(160.0, 100.0, &adj);

        assert!(path.contains("53.2,22.5"));
        assert!(path.contains("6.0,50.9"));
        assert!(path.contains("150.6,29.5"));
        assert!(path.contains("143.7,48.0"));
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
            "M 80.6,99.2 L 76.4,97.5 L 51.8,78.4 L 48.6,78.4 L 24.6,96.9 L 19.4,99.2 L 30.1,62.9 L 0.5,38.2 L 38.0,37.4 L 49.6,0.8 L 50.9,1.7 L 62.0,37.4 L 98.6,37.4 L 99.5,38.2 L 98.6,40.2 L 69.9,62.9 L 80.6,99.2 Z"
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
        assert!(
            path.contains("M5.7,26.2 A54.3,17.1 0 1,0 114.2,26.2"),
            "funnel default should carve the inner opening ellipse: {path}"
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
    fn test_circular_arrow_adjustment_profiles_match_benchmarked_anchors() {
        for (adj1, adj5, anchor) in [
            (
                -20_000.0,
                10_000.0,
                CIRCULAR_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
            ),
            (25_000.0, 35_000.0, CIRCULAR_ARROW_ADJ_WIDE_NORMALIZED_PATH),
            (45_000.0, 15_000.0, CIRCULAR_ARROW_ADJ_SWEEP_NORMALIZED_PATH),
            (12_500.0, 45_000.0, CIRCULAR_ARROW_ADJ_THICK_NORMALIZED_PATH),
        ] {
            let adj = HashMap::from([("adj1".to_string(), adj1), ("adj5".to_string(), adj5)]);
            let path = preset_shape_svg("circularArrow", 120.0, 100.0, &adj).unwrap();
            assert_eq!(
                path,
                scale_normalized_path(anchor, 120.0, 100.0),
                "circularArrow benchmark profile ({adj1}, {adj5}) should map to the tuned anchor path"
            );
        }
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
    fn test_curved_arrow_adjustment_profiles_match_benchmarked_anchors() {
        let make_adj = |adj1: f64, adj2: f64, adj3: f64| {
            HashMap::from([
                ("adj1".to_string(), adj1),
                ("adj2".to_string(), adj2),
                ("adj3".to_string(), adj3),
            ])
        };

        let tight_adj = make_adj(12_000.0, 70_000.0, 18_000.0);
        let wide_adj = make_adj(42_000.0, 30_000.0, 42_000.0);

        for (preset, tight_anchor, wide_anchor) in [
            (
                "curvedRightArrow",
                CURVED_RIGHT_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
                CURVED_RIGHT_ARROW_ADJ_WIDE_NORMALIZED_PATH,
            ),
            (
                "curvedLeftArrow",
                CURVED_LEFT_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
                CURVED_LEFT_ARROW_ADJ_WIDE_NORMALIZED_PATH,
            ),
            (
                "curvedUpArrow",
                CURVED_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
                CURVED_UP_ARROW_ADJ_WIDE_NORMALIZED_PATH,
            ),
            (
                "curvedDownArrow",
                CURVED_DOWN_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
                CURVED_DOWN_ARROW_ADJ_WIDE_NORMALIZED_PATH,
            ),
        ] {
            let tight_path = preset_shape_svg(preset, 120.0, 100.0, &tight_adj).unwrap();
            let wide_path = preset_shape_svg(preset, 120.0, 100.0, &wide_adj).unwrap();

            assert_eq!(
                tight_path,
                scale_normalized_path(tight_anchor, 120.0, 100.0),
                "{preset} tight benchmark profile should map to the tuned anchor path"
            );
            assert_eq!(
                wide_path,
                scale_normalized_path(wide_anchor, 120.0, 100.0),
                "{preset} wide benchmark profile should map to the tuned anchor path"
            );
        }
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
    fn test_bent_up_arrow_adjustment_profiles_match_benchmarked_anchors() {
        for (adj1, adj2, adj3, anchor) in [
            (
                15_000.0,
                15_000.0,
                15_000.0,
                BENT_UP_ARROW_ADJ_TIGHT_NORMALIZED_PATH,
            ),
            (
                35_000.0,
                35_000.0,
                40_000.0,
                BENT_UP_ARROW_ADJ_WIDE_NORMALIZED_PATH,
            ),
            (
                25_000.0,
                15_000.0,
                50_000.0,
                BENT_UP_ARROW_ADJ_TALL_NORMALIZED_PATH,
            ),
            (
                20_000.0,
                45_000.0,
                20_000.0,
                BENT_UP_ARROW_ADJ_DEEP_NORMALIZED_PATH,
            ),
        ] {
            let adj = HashMap::from([
                ("adj1".to_string(), adj1),
                ("adj2".to_string(), adj2),
                ("adj3".to_string(), adj3),
            ]);
            let path = preset_shape_svg("bentUpArrow", 120.0, 100.0, &adj).unwrap();
            assert_eq!(
                path,
                scale_normalized_path(anchor, 120.0, 100.0),
                "bentUpArrow benchmark profile ({adj1}, {adj2}, {adj3}) should map to the tuned anchor path"
            );
        }
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
            "M 54.0,99.2 L 33.0,91.3 L 32.1,88.3 L 34.7,80.2 L 5.7,80.2 L 1.4,78.8 L 1.7,54.5 L 44.9,54.5 L 47.5,46.4 L 1.7,45.0 L 0.9,21.8 L 2.3,20.4 L 57.4,20.4 L 65.4,0.8 L 87.9,8.4 L 84.7,20.4 L 118.3,20.4 L 118.3,45.0 L 77.3,45.5 L 74.2,47.5 L 71.9,54.2 L 118.3,54.5 L 119.1,78.2 L 115.5,80.2 L 63.7,80.2 L 61.1,82.7 L 56.0,97.8 L 54.0,99.2 Z"
        );
    }

    #[test]
    fn test_math_divide_default_path_matches_extracted_office_geometry() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("mathDivide", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(path.matches('M').count(), 3);
        assert!(path.contains("59.2,11.2"));
        assert!(path.contains("104.9,37.9"));
        assert!(path.contains("15.6,37.9"));
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
    fn test_gear6_default_path_matches_extracted_office_outline() {
        let path = preset_shape_svg("gear6", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("60.0,0.0"));
        assert!(path.contains("115.3,27.5"));
        assert!(path.contains("60.0,100.0"));
        assert!(
            !path.contains(" A"),
            "gear6 default silhouette should not cut a center hole"
        );
    }

    #[test]
    fn test_gear9_default_path_matches_extracted_office_outline() {
        let path = preset_shape_svg("gear9", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("60.0,0.0"));
        assert!(path.contains("120.0,44.7"));
        assert!(path.contains("82.8,96.4"));
        assert!(
            !path.contains(" A"),
            "gear9 default silhouette should not cut a center hole"
        );
    }

    #[test]
    fn test_plaque_tabs_default_path_uses_small_quarter_tabs() {
        let path = preset_shape_svg("plaqueTabs", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("M 0.0,7.8"));
        assert!(path.contains("120.0,7.8"));
        assert!(path.contains("120.0,92.7"));
        assert!(path.contains("9.3,100.0"));
    }

    #[test]
    fn test_arc_default_path_matches_quarter_sector_reference() {
        let path = preset_shape_svg("arc", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("79.7,2.3"));
        assert!(path.contains("120.0,43.0"));
        assert!(path.contains("60.2,50.6"));
        assert!(path.contains("60.2,0.0"));
    }

    #[test]
    fn test_no_smoking_default_path_carves_inner_ring_hole() {
        let path = preset_shape_svg("noSmoking", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.matches('M').count() >= 3);
        assert!(path.contains("60.0,18.8"));
        assert!(path.contains("24.2,8.0"));
        assert!(path.contains("95.8,92.0"));
    }

    #[test]
    fn test_chord_default_path_matches_office_outline() {
        let path = preset_shape_svg("chord", 120.0, 100.0, &HashMap::new()).unwrap();
        assert!(path.contains("14.5,16.8"));
        assert!(path.contains("61.3,0.0"));
        assert!(path.contains("101.8,81.0"));
        assert!(path.contains("0.0,59.5"));
    }

    #[test]
    fn test_can_default_uses_filled_top_ellipse_without_evenodd_hole() {
        let path = preset_shape_svg("can", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(!needs_evenodd_fill("can"));
        assert!(path.matches('M').count() >= 2);
        assert!(path.contains("M0,25.0"));
        assert!(path.contains("120.0,25.0 A60.0,25.0 0 0,0 0,25.0 Z"));
    }

    #[test]
    fn test_pie_wedge_default_path_matches_reference_orientation() {
        let path = preset_shape_svg("pieWedge", 120.0, 100.0, &HashMap::new()).unwrap();

        assert_eq!(
            path,
            "M0,100.0 L120.0,100.0 L120.0,0 A120.0,100.0 0 0,0 0,100.0 Z"
        );
    }

    #[test]
    fn test_left_right_arrow_callout_default_path_matches_office_outline() {
        let path =
            preset_shape_svg("leftRightArrowCallout", 120.0, 100.0, &HashMap::new()).unwrap();
        assert!(path.contains("30.8,100.0"));
        assert!(path.contains("89.7,0.0"));
        assert!(path.contains("0.0,48.8"));
    }

    #[test]
    fn test_up_down_arrow_callout_default_path_matches_office_outline() {
        let path = preset_shape_svg("upDownArrowCallout", 120.0, 100.0, &HashMap::new()).unwrap();
        assert!(path.contains("59.1,0.0"));
        assert!(path.contains("120.0,75.0"));
        assert!(path.contains("0.0,25.6"));
    }

    #[test]
    fn test_quad_arrow_callout_default_path_matches_office_outline() {
        let path = preset_shape_svg("quadArrowCallout", 120.0, 100.0, &HashMap::new()).unwrap();
        assert!(path.contains("117.2,52.5"));
        assert!(path.contains("62.5,0.0"));
        assert!(path.contains("0.0,49.3"));
    }

    #[test]
    fn test_math_divide_default_path_uses_circular_dots() {
        let path = preset_shape_svg("mathDivide", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("59.2,11.2"));
        assert!(path.contains("104.9,37.9"));
        assert!(path.contains("15.6,37.9"));
        assert!(path.contains("61.3,88.8"));
    }

    #[test]
    fn test_left_right_ribbon_default_path_matches_office_outline() {
        let path = preset_shape_svg("leftRightRibbon", 120.0, 100.0, &HashMap::new()).unwrap();
        assert!(path.contains("16.6,54.7"));
        assert!(path.contains("56.9,0.0"));
        assert!(path.contains("120.0,57.8"));
        assert!(path.contains("63.1,100.0"));
    }

    #[test]
    fn test_star5_default_path_matches_extracted_reference_outline() {
        let path = preset_shape_svg("star5", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("M 96.8,99.2"));
        assert!(path.contains("L 59.6,0.8"));
        assert!(path.contains("L 118.3,37.4"));
        assert!(path.contains("L 83.9,62.9"));
    }

    #[test]
    fn test_irregular_seal1_default_path_matches_extracted_reference_outline() {
        let path = preset_shape_svg("irregularSeal1", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("M 47.9,99.2"));
        assert!(path.contains("L 80.6,0.8"));
        assert!(path.contains("L 116.8,38.2"));
        assert!(path.contains("L 1.1,66.9"));
    }

    #[test]
    fn test_moon_default_path_matches_extracted_reference_outline() {
        let path = preset_shape_svg("moon", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("M 112.4,99.4"));
        assert!(path.contains("L 0.8,53.9"));
        assert!(path.contains("L 118.5,0.6"));
        assert!(path.contains("L 118.5,99.0"));
    }

    #[test]
    fn test_irregular_seal2_default_path_matches_extracted_reference_outline() {
        let path = preset_shape_svg("irregularSeal2", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("M 28.6,96.9"));
        assert!(path.contains("L 80.2,3.8"));
        assert!(path.contains("L 116.2,31.5"));
        assert!(path.contains("L 48.1,89.3"));
    }

    #[test]
    fn test_cloud_default_path_matches_extracted_reference_outline() {
        let path = preset_shape_svg("cloud", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("M 63.5,99.8"));
        assert!(path.contains("L 25.1,10.3"));
        assert!(path.contains("L 119.0,45.7"));
        assert!(path.contains("L 77.8,87.0"));
    }

    #[test]
    fn test_corner_tabs_default_path_matches_corner_triangles() {
        let path = preset_shape_svg("cornerTabs", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("11.9,0.0"));
        assert!(path.contains("0.0,6.2"));
        assert!(path.contains("120.0,6.2"));
        assert!(path.contains("108.8,100.0"));
    }

    #[test]
    fn test_square_tabs_default_path_matches_detached_squares() {
        let path = preset_shape_svg("squareTabs", 120.0, 100.0, &HashMap::new()).unwrap();

        assert!(path.contains("9.3,7.3"));
        assert!(path.contains("120.0,7.8"));
        assert!(path.contains("9.3,100.0"));
        assert!(path.contains("111.2,100.0"));
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

        assert!(path.contains("36.2,82.8"));
        assert!(path.contains("18.6,68.5"));
        assert!(path.contains("101.9,31.9"));
        assert!(path.contains("60.5,67.7"));
    }

    #[test]
    fn test_bent_up_arrow_default_path_matches_extracted_office_polygon() {
        let default_adj = HashMap::new();
        let path = preset_shape_svg("bentUpArrow", 120.0, 100.0, &default_adj).unwrap();

        assert_eq!(
            path,
            "M 0.0,73.7 L 90.6,73.7 90.9,26.0 81.8,24.3 99.9,0.0 119.6,22.8 112.4,31.2 112.4,99.4 0.0,99.4 0.0,73.7 Z"
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
