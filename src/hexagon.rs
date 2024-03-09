use float_eq::derive_float_eq;
use std::cmp::max;
use std::ops::{Neg, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Hex {
    q: i32,
    r: i32,
    s: i32,
}

impl Hex {
    pub fn new(q: i32, r: i32) -> Hex {
        let s = q.neg().sub(r);
        return Hex { q, r, s };
    }

    pub fn q(&self) -> i32 {
        self.q
    }

    pub fn r(&self) -> i32 {
        self.r
    }

    pub fn s(&self) -> i32 {
        self.s
    }
}

#[derive_float_eq(
    ulps_tol = "HexUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "HexUlpsDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq",
    all_tol = "f64"
)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractionalHex {
    q: f64,
    r: f64,
    s: f64,
}

impl FractionalHex {
    pub fn new(q: f64, r: f64) -> FractionalHex {
        let s = q.neg().sub(r);
        return FractionalHex { q, r, s };
    }

    pub fn q(&self) -> f64 {
        self.q
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn s(&self) -> f64 {
        self.s
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OffsetCoord {
    pub col: i32,
    pub row: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DoubledCoord {
    pub col: i32,
    pub row: i32,
}

pub trait HexMath {
    fn add(&self, other: Hex) -> Hex;
    fn sub(&self, other: Hex) -> Hex;
    fn scale(&self, k: i32) -> Hex;
}

impl HexMath for Hex {
    fn add(&self, other: Hex) -> Hex {
        let q = self.q + other.q;
        let r = self.r + other.r;
        let s = self.s + other.s;

        Hex { q, r, s }
    }

    fn sub(&self, other: Hex) -> Hex {
        let q = self.q - other.q;
        let r = self.r - other.r;
        let s = self.s - other.s;

        Hex { q, r, s }
    }

    fn scale(&self, k: i32) -> Hex {
        let q = self.q * k;
        let r = self.r * k;
        let s = self.s * k;

        Hex { q, r, s }
    }
}

pub trait HexRotation {
    fn rotate_left(&self) -> Hex;
    fn rotate_right(&self) -> Hex;
}

impl HexRotation for Hex {
    fn rotate_left(&self) -> Hex {
        Hex {
            q: -self.s,
            r: -self.q,
            s: -self.r,
        }
    }

    fn rotate_right(&self) -> Hex {
        Hex {
            q: -self.r,
            r: -self.s,
            s: -self.q,
        }
    }
}

pub trait HexUtility {
    fn length(&self) -> i32;
    fn distance(&self, other: Hex) -> i32;
    fn line(&self, b: Hex) -> Vec<Hex>;
}

impl HexUtility for Hex {
    fn length(&self) -> i32 {
        return (self.q.abs() + self.r.abs() + self.s.abs()) / 2 as i32;
    }

    fn distance(&self, to: Hex) -> i32 {
        let len = self.sub(to);
        return (len.q.abs() + len.r.abs() + len.s.abs()) / 2 as i32;
    }

    fn line(&self, to: Hex) -> Vec<Hex> {
        let n: i32 = self.distance(to);

        let a_nudge: FractionalHex = FractionalHex {
            q: self.q as f64 + 1e-06,
            r: self.r as f64 + 1e-06,
            s: self.s as f64 - 2e-06,
        };

        let b_nudge: FractionalHex = FractionalHex {
            q: to.q as f64 + 1e-06,
            r: to.r as f64 + 1e-06,
            s: to.s as f64 - 2e-06,
        };

        let mut results: Vec<Hex> = vec![];
        let step: f64 = 1.0 / max(n, 1) as f64;

        for i in 0..=n {
            let hex = a_nudge
                .linear_interpolation(b_nudge, step * i as f64)
                .round();
            results.push(hex);
        }

        return results;
    }
}

pub trait HexRound {
    fn round(&self) -> Hex;
    fn linear_interpolation(&self, other: FractionalHex, t: f64) -> FractionalHex;
}

impl HexRound for FractionalHex {
    fn round(&self) -> Hex {
        let mut qi: i32 = self.q.round() as i32;
        let mut ri: i32 = self.r.round() as i32;
        let mut si: i32 = self.s.round() as i32;

        let q_diff: f64 = (qi as f64 - self.q).abs();
        let r_diff: f64 = (ri as f64 - self.r).abs();
        let s_diff: f64 = (si as f64 - self.s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            qi = -ri - si;
        } else if r_diff > s_diff {
            ri = -qi - si;
        } else {
            si = -qi - ri;
        }

        return Hex {
            q: qi,
            r: ri,
            s: si,
        };
    }

    fn linear_interpolation(&self, other: FractionalHex, t: f64) -> FractionalHex {
        return FractionalHex {
            q: self.q * (1.0 - t) + other.q * t,
            r: self.r * (1.0 - t) + other.r * t,
            s: self.s * (1.0 - t) + other.s * t,
        };
    }
}

pub static HEX_DIRECTIONS: [Hex; 6] = [
    Hex { q: 1, r: 0, s: -1 },
    Hex { q: 1, r: -1, s: 0 },
    Hex { q: 0, r: -1, s: 1 },
    Hex { q: -1, r: 0, s: 1 },
    Hex { q: -1, r: 1, s: 0 },
    Hex { q: 0, r: 1, s: -1 },
];

pub static HEX_DIAGONALS: [Hex; 6] = [
    Hex { q: 2, r: -1, s: -1 },
    Hex { q: 1, r: -2, s: 1 },
    Hex { q: -1, r: -1, s: 2 },
    Hex { q: -2, r: 1, s: 1 },
    Hex { q: -1, r: 2, s: -1 },
    Hex { q: 1, r: 1, s: -2 },
];
