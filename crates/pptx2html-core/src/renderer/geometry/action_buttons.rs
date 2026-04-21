// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: action_buttons
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
pub(super) fn action_button_blank_path(w: f64, h: f64) -> String {
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
pub(super) fn action_button_icon_path(w: f64, h: f64, icon: &str) -> String {
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
