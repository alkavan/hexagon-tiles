use crate::hexagon::{DoubledCoord, Hex, OffsetCoord, HexMath, HEX_DIRECTIONS, HEX_DIAGONALS};

pub const HEX_EVEN: i32 = 1;
pub const HEX_ODD: i32 = -1;

pub struct HexDirection;

impl HexDirection {
    pub fn direction(direction: i32) -> Hex {
        HEX_DIRECTIONS[direction as usize]
    }

    pub fn neighbor(hex: Hex, direction: i32) -> Hex {
        hex.add(Self::direction(direction))
    }

    pub fn diagonal_neighbor(hex: Hex, direction: i32) -> Hex {
        hex.add(HEX_DIAGONALS[direction as usize])
    }
}

pub struct HexOffset;

impl HexOffset {
    pub fn q_from_cube(offset: i32, hex: Hex) -> OffsetCoord {
        let col: i32 = hex.q();
        let row: i32 = hex.r() + (hex.q() + offset * (hex.q() & 1)) / 2 as i32;
        if offset != HEX_EVEN && offset != HEX_ODD {
            panic!("offset must be EVEN (+1) or ODD (-1)");
        }
        return OffsetCoord { col, row };
    }

    pub fn q_to_cube(offset: i32, coord: OffsetCoord) -> Hex {
        let q: i32 = coord.col;
        let r: i32 = coord.row - (coord.col + offset * (coord.col & 1)) / 2 as i32;

        if offset != HEX_EVEN && offset != HEX_ODD {
            panic!("offset must be EVEN (+1) or ODD (-1)");
        }

        return Hex::new(q, r);
    }

    pub fn r_from_cube(offset: i32, hex: Hex) -> OffsetCoord {
        let col: i32 = hex.q() + (hex.r() + offset * (hex.r() & 1)) / 2 as i32;
        let row: i32 = hex.r();

        if offset != HEX_EVEN && offset != HEX_ODD {
            panic!("offset must be EVEN (+1) or ODD (-1)");
        }

        return OffsetCoord { col, row };
    }

    pub fn r_to_cube(offset: i32, coord: OffsetCoord) -> Hex {
        let q: i32 = coord.col - (coord.row + offset * (coord.row & 1)) / 2 as i32;
        let r: i32 = coord.row;

        if offset != HEX_EVEN && offset != HEX_ODD {
            panic!("offset must be EVEN (+1) or ODD (-1)");
        }

        return Hex::new(q, r);
    }
}

pub struct HexDoubled;

impl HexDoubled {
    pub fn q_from_cube(h: Hex) -> DoubledCoord {
        let col: i32 = h.q();
        let row: i32 = 2 * h.r() + h.q();
        return DoubledCoord { col, row };
    }

    pub fn q_to_cube(h: DoubledCoord) -> Hex {
        let q: i32 = h.col;
        let r: i32 = (h.row - h.col) / 2 as i32;
        return Hex::new(q, r);
    }

    pub fn r_from_cube(h: Hex) -> DoubledCoord {
        let col: i32 = 2 * h.q() + h.r();
        let row: i32 = h.r();
        return DoubledCoord { col, row };
    }

    pub fn r_to_cube(h: DoubledCoord) -> Hex {
        let q: i32 = (h.col - h.row) / 2 as i32;
        let r: i32 = h.row;
        return Hex::new(q, r);
    }
}
