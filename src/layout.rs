use crate::orientation::Orientation;
use nalgebra::{Point2, Vector2};

#[derive(Copy, Clone, Debug)]
pub struct Layout {
    pub orientation: Orientation,
    pub size: Vector2<f64>,
    pub origin: Point2<f64>,
}

impl Layout {
    pub fn new(orientation: Orientation, size: Vector2<f64>, origin: Point2<f64>) -> Layout {
        Layout {
            orientation,
            size,
            origin,
        }
    }
}
