use std::f64::consts::PI;
use std::ops::Neg;
use num::PrimInt;
use crate::fractional::FractionalHex;

use crate::hex::Hex;
use crate::point::{Point, point};

const SQRT_3: f64 = 1.73205080756888;

pub const LAYOUT_ORIENTATION_POINTY: Orientation = Orientation {
    f0: SQRT_3,
    f1: SQRT_3 / 2.0,
    f2: 0.0,
    f3: 3.0 / 2.0,
    b0: SQRT_3 / 3.0,
    b1: -1.0 / 3.0,
    b2: 0.0,
    b3: 2.0 / 3.0,
    start_angle: 0.5,
};

pub const LAYOUT_ORIENTATION_FLAT: Orientation = Orientation {
    f0: 3.0 / 2.0,
    f1: 0.0,
    f2: SQRT_3 / 2.0,
    f3: SQRT_3,
    b0: 2.0 / 3.0,
    b1: 0.0,
    b2: -1.0 / 3.0,
    b3: SQRT_3 / 3.0,
    start_angle: 0.0,
};

#[derive(Clone, Copy, Debug)]
pub struct Orientation {
    pub f0: f64,
    pub f1: f64,
    pub f2: f64,
    pub f3: f64,
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub b3: f64,
    pub start_angle: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub orientation: Orientation,
    pub size: Point,
    pub origin: Point,
}

pub fn hex_to_pixel<I: PrimInt + Neg<Output = I> + Into<f64>>(layout: Layout, hex: Hex<I>) -> Point {
    let orientation = layout.orientation;
    let size = layout.size;
    let origin = layout.origin;

    let x = (orientation.f0 * hex.q().into() + orientation.f1 * hex.r().into()) * size.x;
    let y = (orientation.f2 * hex.q().into() + orientation.f3 * hex.r().into()) * size.y;

    point(x + origin.x, y + origin.y)
}

//TODO make these generic too
pub fn pixel_to_hex(layout: Layout, point: Point) -> FractionalHex<f64> {
    let orientation = layout.orientation;
    let size = layout.size;
    let origin = layout.origin;

    let pt = Point {
        x: (point.x - origin.x) / size.x,
        y: (point.y - origin.y) / size.y,
    };

    let q = orientation.b0 * pt.x + orientation.b1 * pt.y;
    let r = orientation.b2 * pt.x + orientation.b3 * pt.y;

    FractionalHex::new(q, r)
}

pub fn corner_offset(layout: Layout, corner: i32) -> Point {
    let orientation = layout.orientation;
    let size = layout.size;
    let angle = 2.0 * PI * (orientation.start_angle - corner as f64) / 6.0;

    point(size.x * angle.cos(), size.y * angle.sin())
}

pub fn polygon_corners<I: PrimInt + Neg<Output = I> + Into<f64>>(layout: Layout, hex: Hex<I>) -> Vec<Point> {
    let center = hex_to_pixel(layout, hex);

    (0..(6))
        .map(|i| {
            let offset = corner_offset(layout, i);

            point(center.x + offset.x, center.y + offset.y)
        })
        .collect()
}
