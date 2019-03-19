use crate::hex::Hex;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OffsetCoord {
    pub col: i64,
    pub row: i64,
}

impl OffsetCoord {
    pub fn new(col: i64, row: i64) -> OffsetCoord {
        OffsetCoord { col, row }
    }
}

pub fn qoffset_from_cube(offset: i64, h: Hex) -> OffsetCoord {
    let col = h.q;
    let row = (h.r as f64 + (h.q as f64 + offset as f64 * (h.q & 1) as f64) / 2.0) as i64;
    OffsetCoord::new(col, row)
}
pub fn roffset_from_cube(offset: i64, h: Hex) -> OffsetCoord {
    let col = h.q + ((h.r as f64 + offset as f64 * (h.r & 1) as f64) / 2.0) as i64;
    let row = h.r;
    OffsetCoord::new(col, row)
}
pub fn qoffset_to_cube(offset: i64, offset_coord: OffsetCoord) -> Hex {
    let q = offset_coord.col;
    let r = offset_coord.row
        - ((offset_coord.col as f64 + offset as f64 * (offset_coord.col & 1) as f64) / 2.0) as i64;
    let s = -q - r;
    Hex::new(q, r, s)
}
pub fn roffset_to_cube(offset: i64, offset_coord: OffsetCoord) -> Hex {
    let q = offset_coord.col
        - ((offset_coord.row as f64 + offset as f64 * (offset_coord.row & 1) as f64) / 2.0) as i64;
    let r = offset_coord.row as i64;
    let s = -q - r;
    Hex::new(q, r, s)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DoubledCoord {
    pub col: i64,
    pub row: i64,
}

impl DoubledCoord {
    pub fn new(col: i64, row: i64) -> DoubledCoord {
        DoubledCoord { col, row }
    }
}

pub fn qdoubled_from_cube(h: Hex) -> DoubledCoord {
    let col = h.q;
    let row = 2 * h.r + h.q;
    DoubledCoord::new(col, row)
}
pub fn rdoubled_from_cube(h: Hex) -> DoubledCoord {
    let col = 2 * h.q + h.r;
    let row = h.r;
    DoubledCoord::new(col, row)
}
pub fn qdoubled_to_cube(doubled_coord: DoubledCoord) -> Hex {
    let q = doubled_coord.col;
    let r = ((doubled_coord.row - doubled_coord.col) as f64 / 2.0) as i64;
    let s = -q - r;
    Hex::new(q, r, s)
}

pub fn rdoubled_to_cube(doubled_coord: DoubledCoord) -> Hex {
    let q = ((doubled_coord.col - doubled_coord.row) / 2) as i64;
    let r = doubled_coord.row;
    let s = -q - r;
    Hex::new(q, r, s)
}
