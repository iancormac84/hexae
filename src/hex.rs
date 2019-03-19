use std::ops::{Add, Sub};

pub const HEX_DIRECTIONS: [Hex; 6] = [
    Hex::new(1, 0, -1),
    Hex::new(1, -1, 0),
    Hex::new(0, -1, 1),
    Hex::new(-1, 0, 1),
    Hex::new(-1, 1, 0),
    Hex::new(0, 1, -1),
];
pub const HEX_DIAGONALS: [Hex; 6] = [
    Hex::new(2, -1, -1),
    Hex::new(1, -2, 1),
    Hex::new(-1, -1, 2),
    Hex::new(-2, 1, 1),
    Hex::new(-1, 2, -1),
    Hex::new(1, 1, -2),
];

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hex {
    pub q: i64,
    pub r: i64,
    pub s: i64,
}

impl Hex {
    pub const fn new(q: i64, r: i64, s: i64) -> Hex {
        Hex { q, r, s }
    }
    pub const fn direction(direction: usize) -> Hex {
        HEX_DIRECTIONS[direction]
    }
    pub fn scale(&self, n: i64) -> Hex {
        Hex {
            q: self.q * n,
            r: self.r * n,
            s: self.s * n,
        }
    }
    pub fn rotate_left(&self) -> Hex {
        Hex {
            q: -self.s,
            r: -self.q,
            s: -self.r,
        }
    }
    pub fn rotate_right(&self) -> Hex {
        Hex {
            q: -self.r,
            r: -self.s,
            s: -self.q,
        }
    }
    pub fn neighbor(&self, direction: usize) -> Hex {
        *self + Hex::direction(direction)
    }
    pub fn diagonal_neighbor(&self, direction: usize) -> Hex {
        *self + HEX_DIAGONALS[direction]
    }
    pub fn length(&self) -> u64 {
        ((self.q.abs() + self.r.abs() + self.s.abs()) / 2) as u64
    }
    pub fn distance(&self, other: Hex) -> u64 {
        hex_length(*self - other)
    }
}

pub fn hex_length(hex: Hex) -> u64 {
    ((hex.q.abs() + hex.r.abs() + hex.s.abs()) / 2) as u64
}

impl Add for Hex {
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        Hex {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl Sub for Hex {
    type Output = Hex;

    fn sub(self, other: Hex) -> Hex {
        Hex {
            q: self.q - other.q,
            r: self.r - other.r,
            s: self.s - other.s,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FractionalHex {
    pub q: f64,
    pub r: f64,
    pub s: f64,
}

impl FractionalHex {
    pub fn new(q: f64, r: f64) -> FractionalHex {
        FractionalHex { q, r, s: -q - r }
    }
    pub fn new2(q: f64, r: f64, s: f64) -> FractionalHex {
        FractionalHex { q, r, s }
    }
    pub fn lerp(&self, other: FractionalHex, t: f64) -> FractionalHex {
        FractionalHex {
            q: self.q * (1.0 - t) + other.q * t,
            r: self.r * (1.0 - t) + other.r * t,
            s: self.s * (1.0 - t) + other.s * t,
        }
    }
}

pub fn hex_round(h: FractionalHex) -> Hex {
    let mut qi = h.q.round();
    let mut ri = h.r.round();
    let mut si = h.s.round();
    let q_diff = (qi - h.q).abs();
    let r_diff = (ri - h.r).abs();
    let s_diff = (si - h.s).abs();
    if q_diff > r_diff && q_diff > s_diff {
        qi = -ri - si;
    } else if r_diff > s_diff {
        ri = -qi - si;
    } else {
        si = -qi - ri;
    }
    Hex::new(qi as i64, ri as i64, si as i64)
}

pub fn hex_linedraw(a: Hex, b: Hex) -> Vec<Hex> {
    let n = a.distance(b);
    let a_nudge = FractionalHex::new2(
        a.q as f64 + 0.000001,
        a.r as f64 + 0.000001,
        a.s as f64 - 0.000002,
    );
    let b_nudge = FractionalHex::new2(
        b.q as f64 + 0.000001,
        b.r as f64 + 0.000001,
        b.s as f64 - 0.000002,
    );
    let mut results = vec![];
    let step = 1.0 / (n as f64).max(1.0);
    for i in 0..=n {
        results.push(hex_round(a_nudge.lerp(b_nudge, step * i as f64)));
    }
    results
}