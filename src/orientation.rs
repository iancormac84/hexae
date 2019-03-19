use lazy_static::lazy_static;
use nalgebra::Matrix2;

#[derive(Copy, Clone, Debug)]
pub struct Orientation {
    pub fwd_matrix: Matrix2<f64>,
    pub inv_matrix: Matrix2<f64>,
    pub start_angle: f64,
}

impl Orientation {
    pub fn new(
        fwd_matrix: Matrix2<f64>,
        inv_matrix: Matrix2<f64>,
        start_angle: f64,
    ) -> Orientation {
        Orientation {
            fwd_matrix,
            inv_matrix,
            start_angle,
        }
    }
}

lazy_static! {
    pub static ref LAYOUT_POINTY: Orientation = Orientation::new(
        Matrix2::new(3f64.sqrt(), 3f64.sqrt() / 2.0, 0.0, 3.0 / 2.0),
        Matrix2::new(3f64.sqrt() / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0),
        0.5
    );
    pub static ref LAYOUT_FLAT: Orientation = Orientation::new(
        Matrix2::new(3.0 / 2.0, 0.0, 3f64.sqrt() / 2.0, 3f64.sqrt()),
        Matrix2::new(2.0 / 3.0, 0.0, -1.0 / 3.0, 3f64.sqrt() / 3.0),
        0.0
    );
}
