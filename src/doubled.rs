use std::ops::{Add, Mul, Neg, Sub};
use num::PrimInt;
use crate::hex::Hex;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DoubledCoord<I> {
    pub col: I,
    pub row: I,
}

impl<I: PrimInt + Neg<Output = I>> DoubledCoord<I> {
    pub fn q_from_cube(hex: Hex<I>) -> Self {
        let col = hex.q();
        let row = (I::one() + I::one()) * hex.r() + hex.q();

        Self { col, row }
    }

    pub fn q_to_cube(self) -> Hex<I> {
        let q = self.col;
        let r = (self.row - self.col) / (I::one() + I::one());

        Hex::new(q, r)
    }

    pub fn r_from_cube(hex: Hex<I>) -> Self {
        let col = (I::one() + I::one()) * hex.q() + hex.r();
        let row = hex.r();

        Self { col, row }
    }

    pub fn r_to_cube(self) -> Hex<I> {
        let q = (self.col - self.row) / (I::one() + I::one());
        let r = self.row;

        Hex::new(q, r)
    }
}

impl<I: PrimInt> Add for DoubledCoord<I> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let col = self.col + other.col;
        let row = self.row + other.row;

        Self { col, row }
    }
}

impl<I: PrimInt> Sub for DoubledCoord<I> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let col = self.col - other.col;
        let row = self.row - other.row;

        Self { col, row }
    }
}

impl<I: PrimInt> Mul<I> for DoubledCoord<I> {
    type Output = Self;

    fn mul(self, k: I) -> Self::Output {
        let col = self.col * k;
        let row = self.row * k;

        Self { col, row }
    }
}

impl<I: PrimInt + Neg<Output = I>> Neg for DoubledCoord<I> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let col = -self.col;
        let row = -self.row;

        Self { col, row }
    }
}

// note by Madeline Sparkles: idk how division works so we left it out.