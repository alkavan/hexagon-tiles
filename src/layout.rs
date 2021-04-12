use std::f64::consts::PI;

use crate::hexagon::{Hex, FractionalHex};
use crate::point::Point;

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

pub struct LayoutTool;

impl LayoutTool {
    pub fn hex_to_pixel(layout: Layout, hex: Hex) -> Point {
        let orientation: Orientation = layout.orientation;
        let size: Point = layout.size;
        let origin: Point = layout.origin;

        let x: f64 = (orientation.f0 * hex.q() as f64 + orientation.f1 * hex.r() as f64) * size.x;
        let y: f64 = (orientation.f2 * hex.q() as f64 + orientation.f3 * hex.r() as f64) * size.y;

        return Point {
            x: x + origin.x,
            y: y + origin.y,
        };
    }

    pub fn pixel_to_hex(layout: Layout, point: Point) -> FractionalHex {
        let orientation: Orientation = layout.orientation;
        let size: Point = layout.size;
        let origin: Point = layout.origin;
        let pt: Point = Point {
            x: (point.x - origin.x) / size.x,
            y: (point.y - origin.y) / size.y,
        };

        let q: f64 = orientation.b0 * pt.x + orientation.b1 * pt.y;
        let r: f64 = orientation.b2 * pt.x + orientation.b3 * pt.y;

        return FractionalHex::new(q, r);
    }

    pub fn corner_offset(layout: Layout, corner: i32) -> Point {
        let orientation: Orientation = layout.orientation;
        let size: Point = layout.size;
        let angle: f64 = 2.0 * PI * (orientation.start_angle - corner as f64) / 6.0;

        return Point {
            x: size.x * angle.cos(),
            y: size.y * angle.sin(),
        };
    }

    pub fn polygon_corners(layout: Layout, hex: Hex) -> Vec<Point> {
        let mut corners: Vec<Point> = vec![];
        let center: Point = LayoutTool::hex_to_pixel(layout, hex);

        for i in 0..(6) {
            let offset: Point = LayoutTool::corner_offset(layout, i);
            corners.push(Point {
                x: center.x + offset.x,
                y: center.y + offset.y,
            });
        }

        return corners;
    }
}
