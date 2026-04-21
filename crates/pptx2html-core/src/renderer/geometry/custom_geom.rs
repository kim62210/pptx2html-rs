// Auto-split from renderer/geometry.rs (mechanical move, no logic edits).
// Family: custom_geom

use super::shared::{CustomGeomPathSvg, CustomGeomSvg};
use crate::model::{CustomGeometry, GeometryPath, PathCommand};
use std::fmt::Write;

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
            stroke: true,
        });
    }
    Some(CustomGeomSvg {
        paths: result_paths,
    })
}

pub(super) fn geometry_path_to_svg(path: &GeometryPath, shape_w: f64, shape_h: f64) -> String {
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
