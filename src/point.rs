use float_eq::derive_float_eq;

pub fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

#[derive_float_eq(
    ulps_tol = "PointUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "PointDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq"
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}