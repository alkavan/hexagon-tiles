use float_eq::{UlpsEpsilon, FloatEqUlpsEpsilon, FloatEq};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointUlps {
    x: UlpsEpsilon<f64>,
    y: UlpsEpsilon<f64>,
}

impl FloatEqUlpsEpsilon for Point {
    type UlpsEpsilon = PointUlps;
}

impl FloatEq for Point {
    type Epsilon = Point;

    fn eq_abs(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_abs(&other.x, &max_diff.x) && self.y.eq_abs(&other.y, &max_diff.y)
    }

    fn eq_rmax(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_rmax(&other.x, &max_diff.x) && self.y.eq_rmax(&other.y, &max_diff.y)
    }

    fn eq_rmin(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_rmin(&other.x, &max_diff.x) && self.y.eq_rmin(&other.y, &max_diff.y)
    }

    fn eq_r1st(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_r1st(&other.x, &max_diff.x) && self.y.eq_r1st(&other.y, &max_diff.y)
    }

    fn eq_r2nd(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_r2nd(&other.x, &max_diff.x) && self.y.eq_r2nd(&other.y, &max_diff.y)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<Point>) -> bool {
        self.x.eq_ulps(&other.x, &max_diff.x) && self.y.eq_ulps(&other.y, &max_diff.y)
    }
}