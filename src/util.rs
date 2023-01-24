use std::ops::Neg;
use num::PrimInt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Offset {
    Even,
    Odd,
}

impl Offset {
    pub fn into<I: PrimInt + Neg<Output = I>>(self) -> I {
        match self {
            Offset::Even => I::one(),
            Offset::Odd => -I::one()
        }
    }
}