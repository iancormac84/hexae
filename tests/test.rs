use hexae::*;
use nalgebra::{Point2, Vector2};

const EVEN: i64 = 1;
const ODD: i64 = -1;

#[test]
fn test_hex_add() {
    let a = Hex::new(1, -3, 2);
    let b = Hex::new(3, -7, 4);
    let c = Hex::new(4, -10, 6);

    assert_eq!(a + b, c);
}

#[test]
fn test_hex_subtract() {
    let a = Hex::new(1, -3, 2);
    let b = Hex::new(3, -7, 4);
    let c = Hex::new(-2, 4, -2);

    assert_eq!(a - b, c);
}

#[test]
fn test_hex_direction() {
    assert_eq!(Hex::direction(2), Hex::new(0, -1, 1));
}

#[test]
fn test_hex_neighbor() {
    let a = Hex::new(1, -2, 1);
    let b = Hex::new(1, -3, 2);

    assert_eq!(a.neighbor(2), b);
}

#[test]
fn test_hex_diagonal_neighbor() {
    let a = Hex::new(1, -2, 1);
    let b = Hex::new(-1, -1, 2);

    assert_eq!(a.diagonal_neighbor(3), b);
}

#[test]
fn test_hex_distance() {
    let a = Hex::new(3, -7, 4);
    let b = Hex::new(0, 0, 0);

    assert_eq!(a.distance(b), 7);
}

#[test]
fn test_hex_rotate_right() {
    let a = Hex::new(1, -3, 2);
    let b = Hex::new(3, -2, -1);

    assert_eq!(a.rotate_right(), b);
}

#[test]
fn test_hex_rotate_left() {
    let a = Hex::new(1, -3, 2);
    let b = Hex::new(-2, -1, 3);

    assert_eq!(a.rotate_left(), b);
}

#[test]
fn test_hex_round() {
    let a = FractionalHex::new2(0.0, 0.0, 0.0);
    let b = FractionalHex::new2(1.0, -1.0, 0.0);
    let c = FractionalHex::new2(0.0, -1.0, 1.0);
    assert_eq!(
        Hex::new(5, -10, 5),
        hex_round(a.lerp(FractionalHex::new2(10.0, -20.0, 10.0), 0.5))
    );
    assert_eq!(hex_round(a), hex_round(a.lerp(b, 0.499)));
    assert_eq!(hex_round(b), hex_round(a.lerp(b, 0.501)));
    assert_eq!(
        hex_round(a),
        hex_round(FractionalHex::new2(
            a.q * 0.4 + b.q * 0.3 + c.q * 0.3,
            a.r * 0.4 + b.r * 0.3 + c.r * 0.3,
            a.s * 0.4 + b.s * 0.3 + c.s * 0.3
        ))
    );
    assert_eq!(
        hex_round(c),
        hex_round(FractionalHex::new2(
            a.q * 0.3 + b.q * 0.3 + c.q * 0.4,
            a.r * 0.3 + b.r * 0.3 + c.r * 0.4,
            a.s * 0.3 + b.s * 0.3 + c.s * 0.4
        ))
    );
}

#[test]
fn test_hex_linedraw() {
    assert_eq!(
        vec![Hex::new(0, 0, 0), Hex::new(0, -1, 1), Hex::new(0, -2, 2), Hex::new(1, -3, 2), Hex::new(1, -4, 3), Hex::new(1, -5, 4)],
        hex_linedraw(Hex::new(0, 0, 0), Hex::new(1, -5, 4))
    );
}

#[test]
fn test_layout() {
    let h = Hex::new(3, 4, -7);
    let flat = Layout::new(
        *LAYOUT_FLAT,
        Vector2::new(10.0, 15.0),
        Point2::new(35.0, 71.0),
    );
    assert_eq!(h, hex_round(pixel_to_hex(flat, hex_to_pixel(flat, h))));
    let pointy = Layout::new(
        *LAYOUT_POINTY,
        Vector2::new(10.0, 15.0),
        Point2::new(35.0, 71.0),
    );
    assert_eq!(h, hex_round(pixel_to_hex(pointy, hex_to_pixel(pointy, h))));
}

#[test]
fn test_offset_roundtrip() {
    let a = Hex::new(3, 4, -7);
    let b = OffsetCoord::new(1, -3);

    assert_eq!(a, qoffset_to_cube(EVEN, qoffset_from_cube(EVEN, a)));
    assert_eq!(b, qoffset_from_cube(EVEN, qoffset_to_cube(EVEN, b)));
    assert_eq!(a, qoffset_to_cube(ODD, qoffset_from_cube(ODD, a)));
    assert_eq!(b, qoffset_from_cube(ODD, qoffset_to_cube(ODD, b)));
    assert_eq!(a, roffset_to_cube(EVEN, roffset_from_cube(EVEN, a)));
    assert_eq!(b, roffset_from_cube(EVEN, roffset_to_cube(EVEN, b)));
    assert_eq!(a, roffset_to_cube(ODD, roffset_from_cube(ODD, a)));
    assert_eq!(b, roffset_from_cube(ODD, roffset_to_cube(ODD, b)));
}

#[test]
fn test_offset_from_cube() {
    assert_eq!(
        OffsetCoord::new(1, 3),
        qoffset_from_cube(EVEN, Hex::new(1, 2, -3))
    );
    assert_eq!(
        OffsetCoord::new(1, 2),
        qoffset_from_cube(ODD, Hex::new(1, 2, -3))
    );
}

#[test]
fn test_offset_to_cube() {
    assert_eq!(
        Hex::new(1, 2, -3),
        qoffset_to_cube(EVEN, OffsetCoord::new(1, 3))
    );
    assert_eq!(
        Hex::new(1, 2, -3),
        qoffset_to_cube(ODD, OffsetCoord::new(1, 2))
    );
}

#[test]
fn test_doubled_roundtrip() {
    let a = Hex::new(3, 4, -7);
    let b = DoubledCoord::new(1, -3);

    assert_eq!(a, qdoubled_to_cube(qdoubled_from_cube(a)));
    assert_eq!(b, qdoubled_from_cube(qdoubled_to_cube(b)));
    assert_eq!(a, rdoubled_to_cube(rdoubled_from_cube(a)));
    assert_eq!(b, rdoubled_from_cube(rdoubled_to_cube(b)));
}

#[test]
fn test_doubled_from_cube() {
    assert_eq!(
        DoubledCoord::new(1, 5),
        qdoubled_from_cube(Hex::new(1, 2, -3))
    );
    assert_eq!(
        DoubledCoord::new(4, 2),
        rdoubled_from_cube(Hex::new(1, 2, -3))
    );
}

#[test]
fn test_doubled_to_cube() {
    assert_eq!(
        Hex::new(1, 2, -3),
        qdoubled_to_cube(DoubledCoord::new(1, 5))
    );
    assert_eq!(
        Hex::new(1, 2, -3),
        rdoubled_to_cube(DoubledCoord::new(4, 2))
    );
}
