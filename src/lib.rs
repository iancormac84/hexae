use nalgebra::Point2;
use std::f64;

mod coord;
mod hex;
mod layout;
mod orientation;

pub use coord::*;
pub use hex::{hex_linedraw, hex_round, FractionalHex, Hex};
pub use layout::*;
pub use orientation::*;

pub fn hex_to_pixel(layout: Layout, hex: Hex) -> Point2<f64> {
    let m: &Orientation = &layout.orientation;
    let x = (m.fwd_matrix[0] * hex.q as f64 + m.fwd_matrix[1] * hex.r as f64) * layout.size.x;
    let y = (m.fwd_matrix[2] * hex.q as f64 + m.fwd_matrix[3] * hex.r as f64) * layout.size.y;
    Point2::new(x + layout.origin.x, y + layout.origin.y)
}

pub fn pixel_to_hex(layout: Layout, p: Point2<f64>) -> FractionalHex {
    let m: &Orientation = &layout.orientation;
    let pt = Point2::new(
        (p.x - layout.origin.x) / layout.size.x,
        (p.y - layout.origin.y) / layout.size.y,
    );
    let q = m.inv_matrix[0] * pt.x + m.inv_matrix[1] * pt.y;
    let r = m.inv_matrix[2] * pt.x + m.inv_matrix[3] * pt.y;
    FractionalHex::new(q, r)
}

pub fn hex_corner_offset(layout: Layout, corner: u64) -> Point2<f64> {
    let size = layout.size;
    let angle = 2.0 * f64::consts::PI * (layout.orientation.start_angle + corner as f64) / 6.0;
    Point2::new(size.x * angle.cos(), size.y * angle.sin())
}

pub fn polygon_corners(layout: Layout, h: Hex) -> Vec<Point2<f64>> {
    let mut corners = vec![];
    let center = hex_to_pixel(layout, h);

    for i in 0..6 {
        let offset = hex_corner_offset(layout, i);
        corners.push(Point2::new(center.x + offset.x, center.y + offset.y));
    }
    corners
}
