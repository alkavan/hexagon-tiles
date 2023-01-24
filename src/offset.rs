use std::ops::{Add, Div, Mul, Neg, Sub};
use num::PrimInt;
use crate::hex::Hex;
use crate::util::Offset;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct OffsetCoord<I> {
    pub col: I,
    pub row: I,
}

impl<I: PrimInt + Neg<Output = I>> OffsetCoord<I> {
    pub fn q_from_cube(hex: Hex<I>, offset: Offset) -> Self {
        let col = hex.q();
        let row = hex.r() + (hex.q() + offset.into::<I>() * (hex.q() & I::one())) / (I::one() + I::one());

        Self { col, row }
    }

    pub fn q_to_cube(self, offset: Offset) -> Hex<I> {
        let q = self.col;
        let r = self.row - (self.col + offset.into::<I>() * (self.col & I::one())) / (I::one() + I::one());

        Hex::new(q, r)
    }

    pub fn r_from_cube(hex: Hex<I>, offset: Offset) -> Self {
        let col = hex.q() + (hex.r() + offset.into::<I>() * (hex.r() & I::one())) / (I::one() + I::one());
        let row = hex.r();

        Self { col, row }
    }

    pub fn r_to_cube(self, offset: Offset) -> Hex<I> {
        let q = self.col - (self.row + offset.into::<I>() * (self.row & I::one())) / (I::one() + I::one());
        let r = self.row;

        Hex::new(q, r)
    }
}

impl<I: PrimInt> Add for OffsetCoord<I> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let col = self.col + other.col;
        let row = self.row + other.row;

        Self { col, row }
    }
}

impl<I: PrimInt> Sub for OffsetCoord<I> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let col = self.col - other.col;
        let row = self.row - other.row;

        Self { col, row }
    }
}

impl<I: PrimInt> Mul<I> for OffsetCoord<I> {
    type Output = Self;

    fn mul(self, k: I) -> Self::Output {
        let col = self.col * k;
        let row = self.row * k;

        Self { col, row }
    }
}

impl<I: PrimInt> Div<I> for OffsetCoord<I> {
    type Output = Self;

    fn div(self, k: I) -> Self::Output {
        let col = self.col / k;
        let row = self.row / k;

        Self { col, row }
    }
}