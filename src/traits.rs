use std::ops::{Add, Neg, Sub};
use num::{Float, PrimInt};
use crate::hex::Hex;

pub trait HexRotate {
    fn rotate_left(self) -> Self;
    fn rotate_right(self) -> Self;
}

pub trait HexMath<I> : Sized {
    fn length(self) -> I;

    fn distance(self, to: Self) -> I
        where
            Self: Sub<Output = Self>,
    {
        let v = to - self;

        return v.length();
    }

    fn line<F: Float>(self, to: Self) -> Vec<Self>;
}

pub trait HexRound<F> {
    fn round<I: PrimInt + Neg<Output = I>>(self) -> Hex<I>;
    fn lerp(self, other: Self, t: F) -> Self;
}

pub trait HexDirection: Sized + Copy {
    const NEIGHBORS: [Self; 6];
    const DIAGONALS: [Self; 6];

    fn neighbor(self, direction: usize) -> Self
        where
            Self: Add<Output = Self>,
    {
        self + Self::NEIGHBORS[direction % 6]
    }

    fn diagonal(self, direction: usize) -> Self
        where
            Self: Add<Output = Self>,
    {
        self + Self::DIAGONALS[direction % 6]
    }
}