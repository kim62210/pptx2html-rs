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

        // ── Callouts ──
        "wedgeRoundRectCallout" => Some(wedge_round_rect_callout_path(w, h)),
        "wedgeEllipseCallout" => Some(wedge_ellipse_callout_path(w, h)),
        "cloudCallout" => Some(cloud_callout_path(w, h)),

        // ── Flowchart ──
        "flowChartProcess" => Some(rect_path(w, h)),
        "flowChartDecision" => Some(diamond_path(w, h)),
        "flowChartTerminator" => Some(flowchart_terminator_path(w, h)),
        "flowChartDocument" => Some(flowchart_document_path(w, h)),

        // ── Stars ──
        "star4" => Some(star4_path(w, h)),
        "star5" => Some(star5_path(w, h)),
        "star6" => Some(star6_path(w, h)),

        // ── Other ──
        "heart" => Some(heart_path(w, h)),
        "plus" | "mathPlus" => Some(plus_path(w, h, adjust_values)),

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
    format!(
        "M0,0 L{x1:.1},0 L{w:.1},{cy:.1} L{x1:.1},{h:.1} L0,{h:.1} L{point:.1},{cy:.1} Z"
    )
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
        (cx, 0.0),                                // top
        (w * 0.6173, h * 0.3455),                  // inner right-up
        (w, h * 0.382),                            // right
        (w * 0.6909, h * 0.5878),                  // inner right-down
        (w * 0.7939, h),                           // bottom-right
        (cx, h * 0.7265),                          // inner bottom
        (w * 0.2061, h),                           // bottom-left
        (w * 0.3090, h * 0.5878),                  // inner left-down
        (0.0, h * 0.382),                          // left
        (w * 0.3827, h * 0.3455),                  // inner left-up
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
        assert!(path.contains('Q'), "Should contain quadratic bezier for corners");
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
            "rect", "roundRect", "ellipse", "triangle", "rtTriangle",
            "diamond", "parallelogram", "trapezoid", "pentagon", "hexagon",
            "octagon", "rightArrow", "leftArrow", "upArrow", "downArrow",
            "leftRightArrow", "upDownArrow", "bentArrow", "chevron",
            "wedgeRoundRectCallout", "wedgeEllipseCallout", "cloudCallout",
            "flowChartProcess", "flowChartDecision", "flowChartTerminator",
            "flowChartDocument", "star4", "star5", "star6",
            "heart", "plus",
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
}
