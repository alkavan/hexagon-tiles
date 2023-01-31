use std::ops::{Add, Div, Mul, Neg, Sub};
use float_eq::{FloatEq, FloatEqUlpsTol, UlpsTol};
use num::{Float, NumCast, PrimInt};
use crate::hex::{Hex, hex};
use crate::traits::HexRound;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractionalHex<F> {
    q: F,
    r: F,
    s: F,
}

pub const fn frac_hex<F>(q: F, r: F, s: F) -> FractionalHex<F> {
    FractionalHex { q, r, s }
}

impl<F: Float> FractionalHex<F> {
    pub fn new(q: F, r: F) -> Self {
        let s = -q - r;

        Self { q, r, s }
    }

    pub fn q(&self) -> F {
        self.q
    }

    pub fn r(&self) -> F {
        self.r
    }

    pub fn s(&self) -> F {
        self.s
    }
}

impl<F: Float> Add for FractionalHex<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let q = self.q + other.q;
        let r = self.r + other.r;
        let s = self.s + other.s;

        Self { q, r, s }
    }
}

impl<F: Float> Sub for FractionalHex<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let q = self.q - other.q;
        let r = self.r - other.r;
        let s = self.s - other.s;

        Self { q, r, s }
    }
}

impl<F: Float> Mul<F> for FractionalHex<F> {
    type Output = Self;

    fn mul(self, k: F) -> Self::Output {
        let q = self.q * k;
        let r = self.r * k;
        let s = self.s * k;

        Self { q, r, s }
    }
}

impl<F: Float> Div<F> for FractionalHex<F> {
    type Output = Self;

    fn div(self, k: F) -> Self::Output {
        let q = self.q / k;
        let r = self.r / k;
        let s = self.s / k;

        Self { q, r, s }
    }
}

impl<F: Float + Neg<Output = F>> Neg for FractionalHex<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let q = -self.q;
        let r = -self.r;
        let s = -self.s;

        Self { q, r, s }
    }
}

impl<F: Float> HexRound<F> for FractionalHex<F> {
    fn round<I: PrimInt + Neg<Output = I>>(self) -> Hex<I> {
        let qf = self.q.round();
        let rf = self.r.round();
        let sf = self.s.round();

        let q_diff = (qf - self.q).abs();
        let r_diff = (rf - self.r).abs();
        let s_diff = (sf - self.s).abs();

        let mut q: I = NumCast::from(qf).unwrap();
        let mut r: I = NumCast::from(rf).unwrap();
        let mut s: I = NumCast::from(sf).unwrap();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            s = -q - r;
        }

        hex(q, r, s)
    }

    fn lerp(self, other: Self, t: F) -> Self {
        Self {
            q: self.q * (F::one() - t) + other.q * t,
            r: self.r * (F::one() - t) + other.r * t,
            s: self.s * (F::one() - t) + other.s * t,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FractionalHexUlps<F: Float + FloatEqUlpsTol>
    where
        UlpsTol<F>: Copy
{
    q: UlpsTol<F>,
    r: UlpsTol<F>,
    s: UlpsTol<F>,
}

impl FloatEqUlpsTol for FractionalHex<f32>
    where
        UlpsTol<f32>: Copy
{
    type UlpsTol = FractionalHexUlps<f32>;
}

impl FloatEq for FractionalHex<f32>
    where
        UlpsTol<f32>: Copy
{
    type Tol = Self;

    fn eq_abs(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_abs(&other.q, &tol.q)
            && self.r.eq_abs(&other.r, &tol.r)
            && self.s.eq_abs(&other.s, &tol.s)
    }

    fn eq_rmax(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_rmax(&other.q, &tol.q)
            && self.r.eq_rmax(&other.r, &tol.r)
            && self.s.eq_rmax(&other.s, &tol.s)
    }

    fn eq_rmin(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_rmin(&other.q, &tol.q)
            && self.r.eq_rmin(&other.r, &tol.r)
            && self.s.eq_rmin(&other.s, &tol.s)
    }

    fn eq_r1st(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_r1st(&other.q, &tol.q)
            && self.r.eq_r1st(&other.r, &tol.r)
            && self.s.eq_r1st(&other.s, &tol.s)
    }

    fn eq_r2nd(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_r2nd(&other.q, &tol.q)
            && self.r.eq_r2nd(&other.r, &tol.r)
            && self.s.eq_r2nd(&other.s, &tol.s)
    }

    fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Self>) -> bool {
        self.q.eq_ulps(&other.q, &tol.q)
            && self.r.eq_ulps(&other.r, &tol.r)
            && self.s.eq_ulps(&other.s, &tol.s)
    }
}

impl FloatEqUlpsTol for FractionalHex<f64>
    where
        UlpsTol<f64>: Copy
{
    type UlpsTol = FractionalHexUlps<f64>;
}

impl FloatEq for FractionalHex<f64>
    where
        UlpsTol<f64>: Copy
{
    type Tol = Self;

    fn eq_abs(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_abs(&other.q, &tol.q)
            && self.r.eq_abs(&other.r, &tol.r)
            && self.s.eq_abs(&other.s, &tol.s)
    }

    fn eq_rmax(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_rmax(&other.q, &tol.q)
            && self.r.eq_rmax(&other.r, &tol.r)
            && self.s.eq_rmax(&other.s, &tol.s)
    }

    fn eq_rmin(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_rmin(&other.q, &tol.q)
            && self.r.eq_rmin(&other.r, &tol.r)
            && self.s.eq_rmin(&other.s, &tol.s)
    }

    fn eq_r1st(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_r1st(&other.q, &tol.q)
            && self.r.eq_r1st(&other.r, &tol.r)
            && self.s.eq_r1st(&other.s, &tol.s)
    }

    fn eq_r2nd(&self, other: &Self, tol: &Self) -> bool {
        self.q.eq_r2nd(&other.q, &tol.q)
            && self.r.eq_r2nd(&other.r, &tol.r)
            && self.s.eq_r2nd(&other.s, &tol.s)
    }

    fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Self>) -> bool {
        self.q.eq_ulps(&other.q, &tol.q)
            && self.r.eq_ulps(&other.r, &tol.r)
            && self.s.eq_ulps(&other.s, &tol.s)
    }
}