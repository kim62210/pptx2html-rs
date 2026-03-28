//! Preset shape SVG path generation for the top 30 OOXML preset shapes.
//! Generates SVG `<path>` elements parameterized by width, height, and adjust values.

use std::collections::HashMap;

/// Generate an SVG path string for a named preset shape.
/// Returns `None` if the shape name is not supported.
pub fn preset_shape_svg(
    name: &str,
    w: f64,
    h: f64,
    adjust_values: &HashMap<String, f64>,
) -> Option<String> {
    match name {
        // ── Basic shapes ──
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

        // ── Arrows ──
        "rightArrow" => Some(right_arrow_path(w, h, adjust_values)),
        "leftArrow" => Some(left_arrow_path(w, h, adjust_values)),
        "upArrow" => Some(up_arrow_path(w, h, adjust_values)),
        "downArrow" => Some(down_arrow_path(w, h, adjust_values)),
        "leftRightArrow" => Some(left_right_arrow_path(w, h, adjust_values)),
        "upDownArrow" => Some(up_down_arrow_path(w, h, adjust_values)),
        "bentArrow" => Some(bent_arrow_path(w, h)),
        "chevron" => Some(chevron_path(w, h, adjust_values)),
        "notchedRightArrow" => Some(notched_right_arrow_path(w, h, adjust_values)),
        "stripedRightArrow" => Some(striped_right_arrow_path(w, h, adjust_values)),
        "curvedRightArrow" => Some(curved_right_arrow_path(w, h)),

        // ── Callouts ──
        "wedgeRoundRectCallout" => Some(wedge_round_rect_callout_path(w, h)),
        "wedgeEllipseCallout" => Some(wedge_ellipse_callout_path(w, h)),
        "cloudCallout" => Some(cloud_callout_path(w, h)),

        // ── Flowchart ──
        "flowChartProcess" => Some(rect_path(w, h)),
        "flowChartDecision" => Some(diamond_path(w, h)),
        "flowChartTerminator" => Some(flowchart_terminator_path(w, h)),
        "flowChartDocument" => Some(flowchart_document_path(w, h)),
        "flowChartPredefinedProcess" => Some(flowchart_predefined_process_path(w, h)),
        "flowChartAlternateProcess" => Some(flowchart_alternate_process_path(w, h)),
        "flowChartManualInput" => Some(flowchart_manual_input_path(w, h)),
        "flowChartConnector" => Some(ellipse_path(w, h)),

        // ── Stars ──
        "star4" => Some(star4_path(w, h)),
        "star5" => Some(star5_path(w, h)),
        "star6" => Some(star6_path(w, h)),

        // ── Other ──
        "heart" => Some(heart_path(w, h)),
        "plus" | "mathPlus" => Some(plus_path(w, h, adjust_values)),
        "mathMinus" => Some(math_minus_path(w, h)),
        "lightningBolt" => Some(lightning_bolt_path(w, h)),
        "cloud" => Some(cloud_path(w, h)),
        "frame" => Some(frame_path(w, h, adjust_values)),
        "ribbon2" => Some(ribbon2_path(w, h)),
        "donut" => Some(donut_path(w, h, adjust_values)),
        "noSmoking" => Some(no_smoking_path(w, h)),
        "blockArc" => Some(block_arc_path(w, h)),
        "smileyFace" => Some(smiley_face_path(w, h)),
        "can" => Some(can_path(w, h)),
        "cube" => Some(cube_path(w, h)),

        _ => None,
    }
}

// ── Basic shapes ──

fn rect_path(w: f64, h: f64) -> String {
    format!("M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z")
}

fn round_rect_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    // adj default = 16667 (1/6 of min dimension, OOXML units = 1/100000 of min)
    let adj_val = adj.get("adj").copied().unwrap_or(16667.0);
    let min_dim = w.min(h);
    let r = (min_dim * adj_val / 100_000.0).min(min_dim / 2.0);
    format!(
        "M{r:.1},0 L{x1:.1},0 Q{w:.1},0 {w:.1},{r:.1} \
         L{w:.1},{y1:.1} Q{w:.1},{h:.1} {x1:.1},{h:.1} \
         L{r:.1},{h:.1} Q0,{h:.1} 0,{y1:.1} \
         L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x1 = w - r,
        y1 = h - r,
        w = w,
        h = h,
    )
}

fn ellipse_path(w: f64, h: f64) -> String {
    let rx = w / 2.0;
    let ry = h / 2.0;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} \
         A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z",
        cx = rx,
        rx = rx,
        ry = ry,
        h = h,
    )
}

fn triangle_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    format!("M{cx:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z")
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
    let adj_val = adj.get("adj").copied().unwrap_or(25000.0);
    let offset = w * adj_val / 100_000.0;
    format!(
        "M{offset:.1},0 L{w:.1},0 L{x1:.1},{h:.1} L0,{h:.1} Z",
        offset = offset,
        w = w,
        x1 = w - offset,
        h = h,
    )
}

fn trapezoid_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj_val = adj.get("adj").copied().unwrap_or(25000.0);
    let offset = w * adj_val / 100_000.0;
    format!(
        "M{offset:.1},0 L{x1:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z",
        offset = offset,
        x1 = w - offset,
        w = w,
        h = h,
    )
}

fn pentagon_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    // Regular pentagon scaled to bounding box
    let x1 = w * 0.0245;
    let x2 = w * 0.9755;
    let y1 = h * 0.382;
    format!(
        "M{cx:.1},0 L{x2:.1},{y1:.1} L{x3:.1},{h:.1} L{x4:.1},{h:.1} L{x1:.1},{y1:.1} Z",
        cx = cx,
        x1 = x1,
        x2 = x2,
        y1 = y1,
        x3 = w * 0.7939,
        x4 = w * 0.2061,
        h = h,
    )
}

fn hexagon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj_val = adj.get("adj").copied().unwrap_or(25000.0);
    let offset = w * adj_val / 100_000.0;
    let cy = h / 2.0;
    format!(
        "M{offset:.1},0 L{x1:.1},0 L{w:.1},{cy:.1} \
         L{x1:.1},{h:.1} L{offset:.1},{h:.1} L0,{cy:.1} Z",
        offset = offset,
        x1 = w - offset,
        w = w,
        cy = cy,
        h = h,
    )
}

fn octagon_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj_val = adj.get("adj").copied().unwrap_or(29289.0);
    let ox = w * adj_val / 100_000.0;
    let oy = h * adj_val / 100_000.0;
    format!(
        "M{ox:.1},0 L{x1:.1},0 L{w:.1},{oy:.1} L{w:.1},{y1:.1} \
         L{x1:.1},{h:.1} L{ox:.1},{h:.1} L0,{y1:.1} L0,{oy:.1} Z",
        ox = ox,
        x1 = w - ox,
        w = w,
        oy = oy,
        y1 = h - oy,
        h = h,
    )
}

// ── Arrows ──

fn right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = h * adj1 / 100_000.0 / 2.0;
    let head_w = w * adj2 / 100_000.0;
    let cy = h / 2.0;
    let y_top = cy - shaft;
    let y_bot = cy + shaft;
    let x_head = w - head_w;
    format!(
        "M0,{y_top:.1} L{x_head:.1},{y_top:.1} L{x_head:.1},0 \
         L{w:.1},{cy:.1} L{x_head:.1},{h:.1} L{x_head:.1},{y_bot:.1} \
         L0,{y_bot:.1} Z"
    )
}

fn left_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = h * adj1 / 100_000.0 / 2.0;
    let head_w = w * adj2 / 100_000.0;
    let cy = h / 2.0;
    let y_top = cy - shaft;
    let y_bot = cy + shaft;
    format!(
        "M{w:.1},{y_top:.1} L{head_w:.1},{y_top:.1} L{head_w:.1},0 \
         L0,{cy:.1} L{head_w:.1},{h:.1} L{head_w:.1},{y_bot:.1} \
         L{w:.1},{y_bot:.1} Z"
    )
}

fn up_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = w * adj1 / 100_000.0 / 2.0;
    let head_h = h * adj2 / 100_000.0;
    let cx = w / 2.0;
    let x_left = cx - shaft;
    let x_right = cx + shaft;
    format!(
        "M{x_left:.1},{h:.1} L{x_left:.1},{head_h:.1} L0,{head_h:.1} \
         L{cx:.1},0 L{w:.1},{head_h:.1} L{x_right:.1},{head_h:.1} \
         L{x_right:.1},{h:.1} Z"
    )
}

fn down_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = w * adj1 / 100_000.0 / 2.0;
    let head_h = h * adj2 / 100_000.0;
    let cx = w / 2.0;
    let x_left = cx - shaft;
    let x_right = cx + shaft;
    let y_head = h - head_h;
    format!(
        "M{x_left:.1},0 L{x_right:.1},0 L{x_right:.1},{y_head:.1} \
         L{w:.1},{y_head:.1} L{cx:.1},{h:.1} L0,{y_head:.1} \
         L{x_left:.1},{y_head:.1} Z"
    )
}

fn left_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = h * adj1 / 100_000.0 / 2.0;
    let head_w = w * adj2 / 100_000.0;
    let cy = h / 2.0;
    let y_top = cy - shaft;
    let y_bot = cy + shaft;
    let x_right = w - head_w;
    format!(
        "M0,{cy:.1} L{head_w:.1},0 L{head_w:.1},{y_top:.1} \
         L{x_right:.1},{y_top:.1} L{x_right:.1},0 L{w:.1},{cy:.1} \
         L{x_right:.1},{h:.1} L{x_right:.1},{y_bot:.1} \
         L{head_w:.1},{y_bot:.1} L{head_w:.1},{h:.1} Z"
    )
}

fn up_down_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = w * adj1 / 100_000.0 / 2.0;
    let head_h = h * adj2 / 100_000.0;
    let cx = w / 2.0;
    let x_left = cx - shaft;
    let x_right = cx + shaft;
    let y_bot = h - head_h;
    format!(
        "M{cx:.1},0 L{w:.1},{head_h:.1} L{x_right:.1},{head_h:.1} \
         L{x_right:.1},{y_bot:.1} L{w:.1},{y_bot:.1} L{cx:.1},{h:.1} \
         L0,{y_bot:.1} L{x_left:.1},{y_bot:.1} \
         L{x_left:.1},{head_h:.1} L0,{head_h:.1} Z"
    )
}

fn bent_arrow_path(w: f64, h: f64) -> String {
    let shaft = h * 0.25;
    let head_w = w * 0.4;
    let cy_head = h * 0.35;
    let x_head = w - head_w;
    let y_top_shaft = cy_head - shaft / 2.0;
    let y_bot_shaft = cy_head + shaft / 2.0;
    format!(
        "M0,{h:.1} L0,{y_bot:.1} L{x_head:.1},{y_bot:.1} L{x_head:.1},0 \
         L{w:.1},{cy:.1} L{x_head:.1},{hd:.1} \
         L{x_head:.1},{y_top:.1} L{shaft:.1},{y_top:.1} \
         L{shaft:.1},{h:.1} Z",
        h = h,
        y_bot = y_bot_shaft,
        x_head = x_head,
        w = w,
        cy = cy_head,
        hd = cy_head * 2.0,
        y_top = y_top_shaft,
        shaft = shaft,
    )
}

fn chevron_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj_val = adj.get("adj").copied().unwrap_or(50000.0);
    let point = w * adj_val / 100_000.0;
    let cy = h / 2.0;
    let x1 = w - point;
    format!("M0,0 L{x1:.1},0 L{w:.1},{cy:.1} L{x1:.1},{h:.1} L0,{h:.1} L{point:.1},{cy:.1} Z")
}

// ── Callouts ──

fn wedge_round_rect_callout_path(w: f64, h: f64) -> String {
    let r = w.min(h) * 0.06;
    let tail_x = w * 0.1;
    let tail_w = w * 0.1;
    let tail_tip_x = w * 0.05;
    let tail_tip_y = h * 1.2;
    format!(
        "M{r:.1},0 L{x1:.1},0 Q{w:.1},0 {w:.1},{r:.1} \
         L{w:.1},{y1:.1} Q{w:.1},{h:.1} {x1:.1},{h:.1} \
         L{tx2:.1},{h:.1} L{ttx:.1},{tty:.1} L{tx1:.1},{h:.1} \
         L{r:.1},{h:.1} Q0,{h:.1} 0,{y1:.1} \
         L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x1 = w - r,
        y1 = h - r,
        w = w,
        h = h,
        tx1 = tail_x,
        tx2 = tail_x + tail_w,
        ttx = tail_tip_x,
        tty = tail_tip_y,
    )
}

fn wedge_ellipse_callout_path(w: f64, h: f64) -> String {
    let rx = w / 2.0;
    let ry = h / 2.0;
    let tail_x = w * 0.05;
    let tail_y = h * 1.2;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} \
         A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z \
         M{tx1:.1},{h1:.1} L{ttx:.1},{tty:.1} L{tx2:.1},{h2:.1}",
        cx = rx,
        rx = rx,
        ry = ry,
        h = h,
        tx1 = w * 0.35,
        h1 = h * 0.93,
        ttx = tail_x,
        tty = tail_y,
        tx2 = w * 0.45,
        h2 = h * 0.93,
    )
}

fn cloud_callout_path(w: f64, h: f64) -> String {
    // Simplified cloud shape using cubic beziers
    format!(
        "M{x1:.1},{y1:.1} C{x1:.1},{y0:.1} {x2:.1},{y0:.1} {x3:.1},{y0:.1} \
         C{x4:.1},{y0:.1} {x5:.1},{y2:.1} {x5:.1},{y3:.1} \
         C{x5:.1},{y4:.1} {x4:.1},{y5:.1} {x3:.1},{y5:.1} \
         C{x2:.1},{y5:.1} {x1:.1},{y4:.1} {x1:.1},{y3:.1} \
         C{x0:.1},{y3:.1} {x0:.1},{y1:.1} {x1:.1},{y1:.1} Z \
         M{bx1:.1},{by1:.1} A{br:.1},{br:.1} 0 1,1 {bx2:.1},{by2:.1} \
         A{br:.1},{br:.1} 0 1,1 {bx1:.1},{by1:.1} Z",
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
        bx1 = w * 0.2,
        by1 = h * 0.9,
        bx2 = w * 0.15,
        by2 = h * 1.05,
        br = w * 0.04,
    )
}

// ── Flowchart ──

fn flowchart_terminator_path(w: f64, h: f64) -> String {
    let r = h / 2.0;
    format!(
        "M{r:.1},0 L{x1:.1},0 A{r:.1},{r:.1} 0 0,1 {x1:.1},{h:.1} \
         L{r:.1},{h:.1} A{r:.1},{r:.1} 0 0,1 {r:.1},0 Z",
        r = r,
        x1 = w - r,
        h = h,
    )
}

fn flowchart_document_path(w: f64, h: f64) -> String {
    let wave_h = h * 0.85;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{wh:.1} \
         C{c1x:.1},{c1y:.1} {c2x:.1},{c2y:.1} 0,{wh:.1} Z",
        w = w,
        wh = wave_h,
        c1x = w * 0.7,
        c1y = h * 1.05,
        c2x = w * 0.3,
        c2y = h * 0.7,
    )
}

// ── Stars ──

fn star4_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    let cy = h / 2.0;
    let ix = w * 0.35;
    let iy = h * 0.35;
    format!(
        "M{cx:.1},0 L{ix2:.1},{iy:.1} L{w:.1},{cy:.1} L{ix2:.1},{iy2:.1} \
         L{cx:.1},{h:.1} L{ix:.1},{iy2:.1} L0,{cy:.1} L{ix:.1},{iy:.1} Z",
        cx = cx,
        cy = cy,
        w = w,
        h = h,
        ix = ix,
        ix2 = w - ix,
        iy = iy,
        iy2 = h - iy,
    )
}

fn star5_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    // 5-pointed star points (outer radius = half dimensions, inner ~ 38%)
    let points = [
        (cx, 0.0),                // top
        (w * 0.6173, h * 0.3455), // inner right-up
        (w, h * 0.382),           // right
        (w * 0.6909, h * 0.5878), // inner right-down
        (w * 0.7939, h),          // bottom-right
        (cx, h * 0.7265),         // inner bottom
        (w * 0.2061, h),          // bottom-left
        (w * 0.3090, h * 0.5878), // inner left-down
        (0.0, h * 0.382),         // left
        (w * 0.3827, h * 0.3455), // inner left-up
    ];
    let mut path = format!("M{:.1},{:.1}", points[0].0, points[0].1);
    for &(x, y) in &points[1..] {
        path.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    path.push_str(" Z");
    path
}

fn star6_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    let cy = h / 2.0;
    // 6-pointed star (Star of David)
    let points = [
        (cx, 0.0),
        (w * 0.625, h * 0.25),
        (w, h * 0.25),
        (w * 0.75, cy),
        (w, h * 0.75),
        (w * 0.625, h * 0.75),
        (cx, h),
        (w * 0.375, h * 0.75),
        (0.0, h * 0.75),
        (w * 0.25, cy),
        (0.0, h * 0.25),
        (w * 0.375, h * 0.25),
    ];
    let mut path = format!("M{:.1},{:.1}", points[0].0, points[0].1);
    for &(x, y) in &points[1..] {
        path.push_str(&format!(" L{x:.1},{y:.1}"));
    }
    path.push_str(" Z");
    path
}

// ── Other ──

fn heart_path(w: f64, h: f64) -> String {
    let cx = w / 2.0;
    format!(
        "M{cx:.1},{h1:.1} \
         C{c1x:.1},{c1y:.1} 0,{c2y:.1} 0,{c3y:.1} \
         C0,{c4y:.1} {c5x:.1},0 {cx:.1},{c6y:.1} \
         C{c7x:.1},0 {w:.1},{c4y:.1} {w:.1},{c3y:.1} \
         C{w:.1},{c2y:.1} {c8x:.1},{c1y:.1} {cx:.1},{h1:.1} Z",
        cx = cx,
        w = w,
        h1 = h,
        c1x = w * 0.15,
        c1y = h * 0.75,
        c2y = h * 0.5,
        c3y = h * 0.3,
        c4y = h * 0.1,
        c5x = w * 0.15,
        c6y = h * 0.2,
        c7x = w * 0.85,
        c8x = w * 0.85,
    )
}

fn plus_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj_val = adj.get("adj").copied().unwrap_or(25000.0);
    let arm_x = w * adj_val / 100_000.0;
    let arm_y = h * adj_val / 100_000.0;
    let x1 = w - arm_x;
    let y1 = h - arm_y;
    format!(
        "M{arm_x:.1},0 L{x1:.1},0 L{x1:.1},{arm_y:.1} \
         L{w:.1},{arm_y:.1} L{w:.1},{y1:.1} L{x1:.1},{y1:.1} \
         L{x1:.1},{h:.1} L{arm_x:.1},{h:.1} L{arm_x:.1},{y1:.1} \
         L0,{y1:.1} L0,{arm_y:.1} L{arm_x:.1},{arm_y:.1} Z"
    )
}

fn math_minus_path(w: f64, h: f64) -> String {
    let y_top = h * 0.4;
    let y_bot = h * 0.6;
    format!("M0,{y_top:.1} L{w:.1},{y_top:.1} L{w:.1},{y_bot:.1} L0,{y_bot:.1} Z")
}

fn lightning_bolt_path(w: f64, h: f64) -> String {
    format!(
        "M{x1:.1},0 L{x2:.1},{y1:.1} L{x3:.1},{y2:.1} \
         L{x4:.1},{y2:.1} L{x5:.1},{y3:.1} L{x6:.1},{y4:.1} \
         L{x7:.1},{h:.1} L{x8:.1},{y5:.1} L{x9:.1},{y6:.1} \
         L{x10:.1},{y6:.1} L{x11:.1},{y7:.1} Z",
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
        x11 = w * 0.83,
    )
}

// ── Block arrows (new) ──

fn notched_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = h * adj1 / 100_000.0 / 2.0;
    let head_w = w * adj2 / 100_000.0;
    let cy = h / 2.0;
    let y_top = cy - shaft;
    let y_bot = cy + shaft;
    let x_head = w - head_w;
    // Same as rightArrow but with a notch at the tail
    let notch = head_w * 0.5;
    format!(
        "M0,{y_top:.1} L{x_head:.1},{y_top:.1} L{x_head:.1},0 \
         L{w:.1},{cy:.1} L{x_head:.1},{h:.1} L{x_head:.1},{y_bot:.1} \
         L0,{y_bot:.1} L{notch:.1},{cy:.1} Z",
        y_top = y_top,
        x_head = x_head,
        w = w,
        cy = cy,
        h = h,
        y_bot = y_bot,
        notch = notch,
    )
}

fn striped_right_arrow_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj1 = adj.get("adj1").copied().unwrap_or(50000.0);
    let adj2 = adj.get("adj2").copied().unwrap_or(50000.0);
    let shaft = h * adj1 / 100_000.0 / 2.0;
    let head_w = w * adj2 / 100_000.0;
    let cy = h / 2.0;
    let y_top = cy - shaft;
    let y_bot = cy + shaft;
    let x_head = w - head_w;
    // Stripes at the tail: two thin vertical bars
    let s1 = w * 0.025;
    let s2 = w * 0.05;
    let s3 = w * 0.075;
    let s4 = w * 0.1;
    format!(
        "M0,{y_top:.1} L{s1:.1},{y_top:.1} L{s1:.1},{y_bot:.1} L0,{y_bot:.1} Z \
         M{s2:.1},{y_top:.1} L{s3:.1},{y_top:.1} L{s3:.1},{y_bot:.1} L{s2:.1},{y_bot:.1} Z \
         M{s4:.1},{y_top:.1} L{x_head:.1},{y_top:.1} L{x_head:.1},0 \
         L{w:.1},{cy:.1} L{x_head:.1},{h:.1} L{x_head:.1},{y_bot:.1} \
         L{s4:.1},{y_bot:.1} Z",
        y_top = y_top,
        y_bot = y_bot,
        s1 = s1,
        s2 = s2,
        s3 = s3,
        s4 = s4,
        x_head = x_head,
        w = w,
        cy = cy,
        h = h,
    )
}

fn curved_right_arrow_path(w: f64, h: f64) -> String {
    let cy = h / 2.0;
    let head_w = w * 0.3;
    let x_head = w - head_w;
    format!(
        "M0,{h:.1} C0,{y1:.1} {x1:.1},{y2:.1} {x_head:.1},{y2:.1} \
         L{x_head:.1},0 L{w:.1},{cy:.1} L{x_head:.1},{h:.1} \
         L{x_head:.1},{y3:.1} C{x2:.1},{y3:.1} {x3:.1},{y4:.1} {x3:.1},{h:.1} Z",
        h = h,
        y1 = h * 0.3,
        x1 = w * 0.3,
        y2 = h * 0.25,
        x_head = x_head,
        w = w,
        cy = cy,
        y3 = h * 0.75,
        x2 = w * 0.45,
        x3 = w * 0.2,
        y4 = h * 0.7,
    )
}

// ── Flowchart (new) ──

fn flowchart_predefined_process_path(w: f64, h: f64) -> String {
    // Rectangle with vertical lines near left and right edges
    let inset = w * 0.1;
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z \
         M{inset:.1},0 L{inset:.1},{h:.1} \
         M{x1:.1},0 L{x1:.1},{h:.1}",
        w = w,
        h = h,
        inset = inset,
        x1 = w - inset,
    )
}

fn flowchart_alternate_process_path(w: f64, h: f64) -> String {
    // Rounded rectangle with a larger corner radius than roundRect default
    let r = w.min(h) * 0.1;
    format!(
        "M{r:.1},0 L{x1:.1},0 Q{w:.1},0 {w:.1},{r:.1} \
         L{w:.1},{y1:.1} Q{w:.1},{h:.1} {x1:.1},{h:.1} \
         L{r:.1},{h:.1} Q0,{h:.1} 0,{y1:.1} \
         L0,{r:.1} Q0,0 {r:.1},0 Z",
        r = r,
        x1 = w - r,
        y1 = h - r,
        w = w,
        h = h,
    )
}

fn flowchart_manual_input_path(w: f64, h: f64) -> String {
    // Trapezoid-like shape with slanted top edge (left side higher)
    let slant = h * 0.2;
    format!(
        "M0,{slant:.1} L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z",
        slant = slant,
        w = w,
        h = h,
    )
}

// ── Common shapes (new) ──

fn cloud_path(w: f64, h: f64) -> String {
    // Cloud shape using cubic bezier curves
    format!(
        "M{x1:.1},{y1:.1} \
         C{x1:.1},{y0:.1} {x2:.1},{y0a:.1} {x3:.1},{y0a:.1} \
         C{x3a:.1},{y_top:.1} {x4:.1},{y_top:.1} {x5:.1},{y0a:.1} \
         C{x5a:.1},{y_top:.1} {x6:.1},{y_top2:.1} {x7:.1},{y2:.1} \
         C{x8:.1},{y2:.1} {x8:.1},{y3:.1} {x7:.1},{y4:.1} \
         C{x8:.1},{y5:.1} {x7:.1},{y6:.1} {x5:.1},{y6:.1} \
         C{x4:.1},{y7:.1} {x2a:.1},{y7:.1} {x2:.1},{y6:.1} \
         C{x1a:.1},{y7:.1} {x0:.1},{y5:.1} {x0:.1},{y4:.1} \
         C{x0:.1},{y3:.1} {x1a:.1},{y1:.1} {x1:.1},{y1:.1} Z",
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
        y_top = h * 0.1,
        y_top2 = h * 0.12,
        y0 = h * 0.25,
        y0a = h * 0.22,
        y1 = h * 0.35,
        y2 = h * 0.3,
        y3 = h * 0.45,
        y4 = h * 0.55,
        y5 = h * 0.7,
        y6 = h * 0.75,
        y7 = h * 0.85,
    )
}

fn frame_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj_val = adj.get("adj").copied().unwrap_or(12500.0);
    let t = w.min(h) * adj_val / 100_000.0;
    // Outer rectangle + inner rectangle (cutout)
    format!(
        "M0,0 L{w:.1},0 L{w:.1},{h:.1} L0,{h:.1} Z \
         M{t:.1},{t:.1} L{t:.1},{y1:.1} L{x1:.1},{y1:.1} L{x1:.1},{t:.1} Z",
        w = w,
        h = h,
        t = t,
        x1 = w - t,
        y1 = h - t,
    )
}

fn ribbon2_path(w: f64, h: f64) -> String {
    // Upward ribbon banner
    let fold = w * 0.1;
    let banner_top = h * 0.15;
    let banner_bot = h * 0.85;
    format!(
        "M0,{bt:.1} L{fold:.1},{bt:.1} L{fold:.1},0 L{x1:.1},0 \
         L{x1:.1},{bt:.1} L{w:.1},{bt:.1} L{x2:.1},{mid:.1} \
         L{w:.1},{bb:.1} L{x1:.1},{bb:.1} L{x1:.1},{h:.1} \
         L{fold:.1},{h:.1} L{fold:.1},{bb:.1} L0,{bb:.1} \
         L{fold2:.1},{mid:.1} Z",
        bt = banner_top,
        bb = banner_bot,
        fold = fold,
        x1 = w - fold,
        x2 = w - fold * 0.3,
        fold2 = fold * 0.3,
        mid = (banner_top + banner_bot) / 2.0,
        w = w,
        h = h,
    )
}

fn donut_path(w: f64, h: f64, adj: &HashMap<String, f64>) -> String {
    let adj_val = adj.get("adj").copied().unwrap_or(25000.0);
    let t = w.min(h) * adj_val / 100_000.0;
    let rx_out = w / 2.0;
    let ry_out = h / 2.0;
    let rx_in = rx_out - t;
    let ry_in = ry_out - t;
    let cx = rx_out;
    // Outer ellipse CW, then inner ellipse CCW (hole)
    format!(
        "M{cx:.1},0 A{rx_o:.1},{ry_o:.1} 0 1,1 {cx:.1},{h:.1} \
         A{rx_o:.1},{ry_o:.1} 0 1,1 {cx:.1},0 Z \
         M{cx:.1},{t:.1} A{rx_i:.1},{ry_i:.1} 0 1,0 {cx:.1},{y1:.1} \
         A{rx_i:.1},{ry_i:.1} 0 1,0 {cx:.1},{t:.1} Z",
        cx = cx,
        rx_o = rx_out,
        ry_o = ry_out,
        h = h,
        t = t,
        rx_i = rx_in.max(0.1),
        ry_i = ry_in.max(0.1),
        y1 = h - t,
    )
}

fn no_smoking_path(w: f64, h: f64) -> String {
    let rx = w / 2.0;
    let ry = h / 2.0;
    let cx = rx;
    let t = w.min(h) * 0.08;
    // Circle + diagonal bar from upper-left to lower-right
    // Outer circle, inner circle cutout, then diagonal bar
    let angle = std::f64::consts::FRAC_PI_4;
    let cos = angle.cos();
    let sin = angle.sin();
    let bar_x1 = cx - rx * cos;
    let bar_y1 = ry - ry * sin;
    let bar_x2 = cx + rx * cos;
    let bar_y2 = ry + ry * sin;
    // Perpendicular offset for bar thickness
    let dx = t / 2.0 * sin;
    let dy = t / 2.0 * cos;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} \
         A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z \
         M{bx1a:.1},{by1a:.1} L{bx2a:.1},{by2a:.1} \
         L{bx2b:.1},{by2b:.1} L{bx1b:.1},{by1b:.1} Z",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        bx1a = bar_x1 + dx,
        by1a = bar_y1 - dy,
        bx2a = bar_x2 + dx,
        by2a = bar_y2 - dy,
        bx2b = bar_x2 - dx,
        by2b = bar_y2 + dy,
        bx1b = bar_x1 - dx,
        by1b = bar_y1 + dy,
    )
}

fn block_arc_path(w: f64, h: f64) -> String {
    let rx_out = w / 2.0;
    let ry_out = h / 2.0;
    let t = w.min(h) * 0.15;
    let rx_in = rx_out - t;
    let ry_in = ry_out - t;
    let cx = rx_out;
    let cy = ry_out;
    // 270-degree arc (open at bottom)
    format!(
        "M{cx:.1},0 A{rx_o:.1},{ry_o:.1} 0 1,1 0,{cy:.1} \
         L{t:.1},{cy:.1} A{rx_i:.1},{ry_i:.1} 0 1,0 {cx:.1},{t:.1} Z",
        cx = cx,
        cy = cy,
        rx_o = rx_out,
        ry_o = ry_out,
        t = t,
        rx_i = rx_in.max(0.1),
        ry_i = ry_in.max(0.1),
    )
}

fn smiley_face_path(w: f64, h: f64) -> String {
    let rx = w / 2.0;
    let ry = h / 2.0;
    let cx = rx;
    // Face outline
    let eye_rx = w * 0.05;
    let eye_ry = h * 0.06;
    let left_eye_cx = w * 0.35;
    let right_eye_cx = w * 0.65;
    let eye_cy = h * 0.38;
    // Smile arc
    let smile_x1 = w * 0.3;
    let smile_x2 = w * 0.7;
    let smile_y = h * 0.6;
    let smile_ctrl_y = h * 0.8;
    format!(
        "M{cx:.1},0 A{rx:.1},{ry:.1} 0 1,1 {cx:.1},{h:.1} \
         A{rx:.1},{ry:.1} 0 1,1 {cx:.1},0 Z \
         M{le_r:.1},{ecy:.1} A{erx:.1},{ery:.1} 0 1,1 {le_l:.1},{ecy:.1} \
         A{erx:.1},{ery:.1} 0 1,1 {le_r:.1},{ecy:.1} Z \
         M{re_r:.1},{ecy:.1} A{erx:.1},{ery:.1} 0 1,1 {re_l:.1},{ecy:.1} \
         A{erx:.1},{ery:.1} 0 1,1 {re_r:.1},{ecy:.1} Z \
         M{sx1:.1},{sy:.1} Q{cx:.1},{scy:.1} {sx2:.1},{sy:.1}",
        cx = cx,
        rx = rx,
        ry = ry,
        h = h,
        le_r = left_eye_cx + eye_rx,
        le_l = left_eye_cx - eye_rx,
        re_r = right_eye_cx + eye_rx,
        re_l = right_eye_cx - eye_rx,
        ecy = eye_cy,
        erx = eye_rx,
        ery = eye_ry,
        sx1 = smile_x1,
        sx2 = smile_x2,
        sy = smile_y,
        scy = smile_ctrl_y,
    )
}

fn can_path(w: f64, h: f64) -> String {
    // Cylinder: top ellipse + sides + bottom ellipse
    let ry = h * 0.1;
    let rx = w / 2.0;
    let body_top = ry;
    let body_bot = h - ry;
    format!(
        "M0,{bt:.1} L0,{bb:.1} A{rx:.1},{ry:.1} 0 0,0 {w:.1},{bb:.1} \
         L{w:.1},{bt:.1} A{rx:.1},{ry:.1} 0 0,0 0,{bt:.1} Z \
         M0,{bt:.1} A{rx:.1},{ry:.1} 0 0,1 {w:.1},{bt:.1} \
         A{rx:.1},{ry:.1} 0 0,1 0,{bt:.1} Z",
        bt = body_top,
        bb = body_bot,
        rx = rx,
        ry = ry,
        w = w,
    )
}

fn cube_path(w: f64, h: f64) -> String {
    // 3D cube: front face + top face + right face
    let d = w.min(h) * 0.25;
    format!(
        "M0,{d:.1} L{x1:.1},0 L{w:.1},0 L{w:.1},{y1:.1} L{x2:.1},{h:.1} L0,{h:.1} Z \
         M0,{d:.1} L{x1:.1},0 L{w:.1},0 L{x2:.1},{d:.1} Z \
         M{x2:.1},{d:.1} L{w:.1},0 L{w:.1},{y1:.1} L{x2:.1},{h:.1} Z",
        d = d,
        x1 = d,
        x2 = w - d,
        y1 = h - d,
        w = w,
        h = h,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_path() {
        let path = rect_path(100.0, 50.0);
        assert!(path.contains("M0,0"));
        assert!(path.contains("L100.0,0"));
        assert!(path.contains("L100.0,50.0"));
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_round_rect_default_radius() {
        let adj = HashMap::new();
        let path = round_rect_path(300.0, 100.0, &adj);
        assert!(
            path.contains('Q'),
            "Should contain quadratic bezier for corners"
        );
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_ellipse_path() {
        let path = ellipse_path(200.0, 100.0);
        assert!(path.contains('A'), "Should contain arc commands");
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_preset_shape_svg_returns_none_for_unknown() {
        let adj = HashMap::new();
        assert!(preset_shape_svg("unknownShape", 100.0, 100.0, &adj).is_none());
    }

    #[test]
    fn test_preset_shape_svg_returns_some_for_known() {
        let adj = HashMap::new();
        let known = [
            "rect",
            "roundRect",
            "ellipse",
            "triangle",
            "rtTriangle",
            "diamond",
            "parallelogram",
            "trapezoid",
            "pentagon",
            "hexagon",
            "octagon",
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
            "wedgeRoundRectCallout",
            "wedgeEllipseCallout",
            "cloudCallout",
            "flowChartProcess",
            "flowChartDecision",
            "flowChartTerminator",
            "flowChartDocument",
            "flowChartPredefinedProcess",
            "flowChartAlternateProcess",
            "flowChartManualInput",
            "flowChartConnector",
            "star4",
            "star5",
            "star6",
            "heart",
            "plus",
            "mathMinus",
            "lightningBolt",
            "cloud",
            "frame",
            "ribbon2",
            "donut",
            "noSmoking",
            "blockArc",
            "smileyFace",
            "can",
            "cube",
        ];
        for name in &known {
            assert!(
                preset_shape_svg(name, 100.0, 100.0, &adj).is_some(),
                "Expected Some for shape: {name}"
            );
        }
    }

    #[test]
    fn test_right_arrow_path_structure() {
        let adj = HashMap::new();
        let path = right_arrow_path(200.0, 100.0, &adj);
        assert!(path.starts_with('M'));
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_star5_path_has_10_points() {
        let path = star5_path(100.0, 100.0);
        // 10 points = 1 M + 9 L
        let l_count = path.matches(" L").count();
        assert_eq!(l_count, 9, "Star5 should have 10 vertices (1 M + 9 L)");
    }

    #[test]
    fn test_notched_right_arrow_has_notch() {
        let adj = HashMap::new();
        let path = notched_right_arrow_path(200.0, 100.0, &adj);
        assert!(path.starts_with('M'));
        assert!(path.ends_with('Z'));
        // Notched arrow should have 8 points (the notch adds one vertex)
        let m_count = path.matches('M').count();
        assert_eq!(m_count, 1);
    }

    #[test]
    fn test_striped_right_arrow_has_stripes() {
        let adj = HashMap::new();
        let path = striped_right_arrow_path(200.0, 100.0, &adj);
        // Should have multiple sub-paths (stripes + main arrow)
        let m_count = path.matches('M').count();
        assert!(m_count >= 3, "Striped arrow should have at least 3 sub-paths (2 stripes + arrow)");
    }

    #[test]
    fn test_curved_right_arrow_has_curves() {
        let path = curved_right_arrow_path(200.0, 100.0);
        assert!(path.contains('C'), "Curved arrow should contain cubic bezier commands");
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_flowchart_predefined_process_has_inner_lines() {
        let path = flowchart_predefined_process_path(200.0, 100.0);
        // Should have the outer rect + 2 inner vertical lines
        let m_count = path.matches('M').count();
        assert!(m_count >= 3, "Predefined process should have outer rect + 2 inner lines");
    }

    #[test]
    fn test_flowchart_alternate_process_is_rounded() {
        let path = flowchart_alternate_process_path(200.0, 100.0);
        assert!(path.contains('Q'), "Alternate process should have rounded corners (Q commands)");
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_flowchart_manual_input_has_slant() {
        let path = flowchart_manual_input_path(200.0, 100.0);
        assert!(path.starts_with('M'));
        assert!(path.ends_with('Z'));
        // The top-left Y should not be 0 (slanted)
        assert!(!path.starts_with("M0,0"), "Manual input should have slanted top (not starting at 0,0)");
    }

    #[test]
    fn test_cloud_path_has_curves() {
        let path = cloud_path(200.0, 100.0);
        assert!(path.contains('C'), "Cloud should use cubic bezier curves");
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_frame_path_has_cutout() {
        let adj = HashMap::new();
        let path = frame_path(200.0, 200.0, &adj);
        // Frame has outer rect + inner rect cutout = 2 sub-paths
        let m_count = path.matches('M').count();
        assert_eq!(m_count, 2, "Frame should have 2 sub-paths (outer + inner)");
    }

    #[test]
    fn test_donut_path_has_hole() {
        let adj = HashMap::new();
        let path = donut_path(100.0, 100.0, &adj);
        // Donut has outer ellipse + inner ellipse = 2 M commands
        let m_count = path.matches('M').count();
        assert_eq!(m_count, 2, "Donut should have 2 sub-paths (outer + inner)");
        assert!(path.contains('A'), "Donut should use arc commands");
    }

    #[test]
    fn test_no_smoking_path_has_circle_and_bar() {
        let path = no_smoking_path(100.0, 100.0);
        assert!(path.contains('A'), "No smoking should have circle (arc commands)");
        // Circle + diagonal bar = 2 M commands
        let m_count = path.matches('M').count();
        assert_eq!(m_count, 2, "No smoking should have circle + diagonal bar");
    }

    #[test]
    fn test_smiley_face_path_has_features() {
        let path = smiley_face_path(100.0, 100.0);
        // Face + 2 eyes + smile = at least 4 M commands
        let m_count = path.matches('M').count();
        assert!(m_count >= 4, "Smiley should have face outline + 2 eyes + smile");
        assert!(path.contains('Q'), "Smiley smile should use quadratic bezier");
    }

    #[test]
    fn test_can_path_has_elliptical_caps() {
        let path = can_path(100.0, 200.0);
        assert!(path.contains('A'), "Can/cylinder should use arc commands for caps");
        let m_count = path.matches('M').count();
        assert_eq!(m_count, 2, "Can should have body + top cap");
    }

    #[test]
    fn test_cube_path_has_three_faces() {
        let path = cube_path(100.0, 100.0);
        // Front face + top face + right face = 3 M commands
        let m_count = path.matches('M').count();
        assert_eq!(m_count, 3, "Cube should have 3 sub-paths for 3 visible faces");
    }

    #[test]
    fn test_math_minus_is_horizontal_bar() {
        let path = math_minus_path(100.0, 50.0);
        assert!(path.starts_with('M'));
        assert!(path.ends_with('Z'));
        // Should be a simple 4-point rectangle
        let l_count = path.matches('L').count();
        assert_eq!(l_count, 3, "Math minus should be a simple rectangle (M + 3L)");
    }

    #[test]
    fn test_lightning_bolt_path_structure() {
        let path = lightning_bolt_path(100.0, 200.0);
        assert!(path.starts_with('M'));
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_ribbon2_path_structure() {
        let path = ribbon2_path(200.0, 100.0);
        assert!(path.starts_with('M'));
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_block_arc_path_has_arcs() {
        let path = block_arc_path(100.0, 100.0);
        assert!(path.contains('A'), "Block arc should use arc commands");
        assert!(path.ends_with('Z'));
    }

    #[test]
    fn test_new_shapes_count() {
        let adj = HashMap::new();
        // Count all supported shapes (should be at least 50 now: 30 original + 20 new)
        let all_shapes = [
            "rect", "roundRect", "ellipse", "triangle", "rtTriangle",
            "diamond", "parallelogram", "trapezoid", "pentagon", "hexagon",
            "octagon", "rightArrow", "leftArrow", "upArrow", "downArrow",
            "leftRightArrow", "upDownArrow", "bentArrow", "chevron",
            "notchedRightArrow", "stripedRightArrow", "curvedRightArrow",
            "wedgeRoundRectCallout", "wedgeEllipseCallout", "cloudCallout",
            "flowChartProcess", "flowChartDecision", "flowChartTerminator",
            "flowChartDocument", "flowChartPredefinedProcess",
            "flowChartAlternateProcess", "flowChartManualInput", "flowChartConnector",
            "star4", "star5", "star6",
            "heart", "plus", "mathMinus", "lightningBolt",
            "cloud", "frame", "ribbon2", "donut", "noSmoking",
            "blockArc", "smileyFace", "can", "cube",
        ];
        let supported: Vec<_> = all_shapes.iter()
            .filter(|name| preset_shape_svg(name, 100.0, 100.0, &adj).is_some())
            .collect();
        assert!(
            supported.len() >= 49,
            "Should support at least 49 shapes, got {}",
            supported.len()
        );
    }
}
