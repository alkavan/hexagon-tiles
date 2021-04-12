use crate::hexagon::{
    Hex, FractionalHex, OffsetCoord, DoubledCoord,
    HexMath, HexRotation, HexRound, HexUtility,
};
use crate::point::Point;
use crate::tools::{HexOffset, HexDoubled, HexDirection, HEX_EVEN, HEX_ODD};
use crate::layout::{Layout, LayoutTool, LAYOUT_ORIENTATION_FLAT, LAYOUT_ORIENTATION_POINTY};

#[test]
fn test_s_component() {
    let hex = Hex::new(1, 4);
    assert_eq!(-5, hex.s());

    let fractional_hex = FractionalHex::new(1.0, 4.0);
    assert_eq!(-5.0, fractional_hex.s());
}

#[test]
fn test_hex_arithmetic() {
    let add_expected = Hex::new(4, -10);
    let add_actual = Hex::new(1, -3).add(Hex::new(3, -7));
    assert_eq!(add_expected, add_actual);

    let sub_expected = Hex::new(-2, 4);
    let sub_actual = Hex::new(1, -3).sub(Hex::new(3, -7));
    assert_eq!(sub_expected, sub_actual);

    let scale_expected = Hex::new(4, -8);
    let scale_actual = Hex::new(1, -2).scale(4);
    assert_eq!(scale_expected, scale_actual);
}

#[test]
fn test_hex_direction() {
    let expected_direction = Hex::new(0, -1);
    assert_eq!(expected_direction, HexDirection::direction(2));
}

#[test]
fn test_hex_neighbor() {
    let expected_neighbor = Hex::new(1, -3);
    assert_eq!(
        expected_neighbor,
        HexDirection::neighbor(Hex::new(1, -2), 2)
    );
}

#[test]
fn test_hex_diagonal() {
    let expected_neighbor = Hex::new(-1, -1);
    assert_eq!(
        expected_neighbor,
        HexDirection::diagonal_neighbor(Hex::new(1, -2), 3)
    );
}

#[test]
fn test_hex_distance() {
    let hex_from = Hex::new(3, -7);
    let hex_to = Hex::new(0, 0);
    let expected_distance = 7;

    assert_eq!(expected_distance, hex_from.distance(hex_to));
}

#[test]
fn test_hex_rotate_right() {
    let hex = Hex::new(1, -3);
    let hex_expected = Hex::new(3, -2);

    assert_eq!(hex_expected, hex.rotate_right());
}

#[test]
fn test_hex_rotate_left() {
    let hex = Hex::new(1, -3);
    let hex_expected = Hex::new(-2, -1);

    assert_eq!(hex_expected, hex.rotate_left());
}

#[test]
fn test_hex_round() {
    let a: FractionalHex = FractionalHex::new(0.0, 0.0);
    let b: FractionalHex = FractionalHex::new(1.0, -1.0);
    let c: FractionalHex = FractionalHex::new(0.0, -1.0);

    let actual_round_1 = a
        .linear_interpolation(FractionalHex::new(10.0, -20.0), 0.5)
        .round();

    let expected_round_1 = Hex::new(5, -10);

    assert_eq!(expected_round_1, actual_round_1);

    assert_eq!(a.round(), a.linear_interpolation(b, 0.499).round());
    assert_eq!(b.round(), a.linear_interpolation(b, 0.501).round());

    let expected_round_4 = a.round();
    let actual_round_4 = FractionalHex::new(
        a.q() * 0.4 + b.q() * 0.3 + c.q() * 0.3,
        a.r() * 0.4 + b.r() * 0.3 + c.r() * 0.3,
    )
    .round();
    assert_eq!(expected_round_4, actual_round_4);

    let expected_round_5 = c.round();
    let actual_round_5 = FractionalHex::new(
        a.q() * 0.3 + b.q() * 0.3 + c.q() * 0.4,
        a.r() * 0.3 + b.r() * 0.3 + c.r() * 0.4,
    )
    .round();
    assert_eq!(expected_round_5, actual_round_5);
}

#[test]
fn test_hex_line() {
    let hex = Hex::new(0, 0);

    let expected_line = vec![
        Hex::new(0, 0),
        Hex::new(0, -1),
        Hex::new(0, -2),
        Hex::new(1, -3),
        Hex::new(1, -4),
        Hex::new(1, -5),
    ];

    let actual_line = hex.line(Hex::new(1, -5));

    assert_eq!(expected_line, actual_line);
}

#[test]
fn test_hex_layout() {
    let expected_hex: Hex = Hex::new(3, 4);

    let flat: Layout = Layout {
        orientation: LAYOUT_ORIENTATION_FLAT,
        size: Point { x: 10.0, y: 15.0 },
        origin: Point { x: 35.0, y: 71.0 },
    };

    let point_1 = LayoutTool::hex_to_pixel(flat, expected_hex);
    let actual_1 = LayoutTool::pixel_to_hex(flat, point_1).round();
    assert_eq!(expected_hex, actual_1);

    let pointy: Layout = Layout {
        orientation: LAYOUT_ORIENTATION_POINTY,
        size: Point { x: 10.0, y: 15.0 },
        origin: Point { x: 35.0, y: 71.0 },
    };

    let point_2 = LayoutTool::hex_to_pixel(pointy, expected_hex);
    let actual_2 = LayoutTool::pixel_to_hex(pointy, point_2).round();
    assert_eq!(expected_hex, actual_2);

    let expected_corner_offset_1 = Point {
        x: 5.000000000000001,
        y: -12.990381056766578,
    };
    let actual_corner_offset_1 = LayoutTool::corner_offset(flat, 1);
    assert_eq!(expected_corner_offset_1, actual_corner_offset_1);

    let expected_corner_offset_2 = Point {
        x: 8.660254037844387,
        y: -7.499999999999999,
    };
    let actual_corner_offset_2 = LayoutTool::corner_offset(pointy, 1);
    assert_eq!(expected_corner_offset_2, actual_corner_offset_2);

    let expected_polygon_corners_1 = vec![
        Point {
            x: 90.0,
            y: 213.8941916244326,
        },
        Point {
            x: 85.0,
            y: 200.90381056766603,
        },
        Point {
            x: 75.0,
            y: 200.90381056766603,
        },
        Point {
            x: 70.0,
            y: 213.8941916244326,
        },
        Point {
            x: 75.0,
            y: 226.8845726811992,
        },
        Point {
            x: 85.0,
            y: 226.8845726811992,
        },
    ];

    let actual_polygon_corners_1 = LayoutTool::polygon_corners(flat, expected_hex);
    assert_eq!(expected_polygon_corners_1, actual_polygon_corners_1);

    let expected_polygon_corners_2 = vec![
        Point {
            x: 130.2627944162884,
            y: 168.5,
        },
        Point {
            x: 130.2627944162884,
            y: 153.5,
        },
        Point {
            x: 121.60254037844399,
            y: 146.0,
        },
        Point {
            x: 112.94228634059961,
            y: 153.5,
        },
        Point {
            x: 112.9422863405996,
            y: 168.5,
        },
        Point {
            x: 121.60254037844399,
            y: 176.0,
        },
    ];

    let actual_polygon_corners_2 = LayoutTool::polygon_corners(pointy, expected_hex);
    assert_eq!(expected_polygon_corners_2, actual_polygon_corners_2);
}

#[test]
fn test_offset_roundtrip() {
    let expected_hex: Hex = Hex::new(3, 4);
    let expected_coord: OffsetCoord = OffsetCoord { col: 1, row: -3 };

    let actual_coord_1 = HexOffset::q_from_cube(HEX_EVEN, expected_hex);
    let hex_actual_1 = HexOffset::q_to_cube(HEX_EVEN, actual_coord_1);
    assert_eq!(expected_hex, hex_actual_1);

    let actual_hex_2 = HexOffset::q_to_cube(HEX_EVEN, expected_coord);
    let actual_coord_2 = HexOffset::q_from_cube(HEX_EVEN, actual_hex_2);
    assert_eq!(expected_coord, actual_coord_2);

    let actual_coord_3 = HexOffset::q_from_cube(HEX_ODD, expected_hex);
    let hex_actual_3 = HexOffset::q_to_cube(HEX_ODD, actual_coord_3);
    assert_eq!(expected_hex, hex_actual_3);

    let actual_hex_4 = HexOffset::q_to_cube(HEX_ODD, expected_coord);
    let actual_coord_4 = HexOffset::q_from_cube(HEX_ODD, actual_hex_4);
    assert_eq!(expected_coord, actual_coord_4);

    let actual_coord_5 = HexOffset::r_from_cube(HEX_EVEN, expected_hex);
    let hex_actual_5 = HexOffset::r_to_cube(HEX_EVEN, actual_coord_5);
    assert_eq!(expected_hex, hex_actual_5);

    let actual_hex_6 = HexOffset::r_to_cube(HEX_EVEN, expected_coord);
    let actual_coord_6 = HexOffset::r_from_cube(HEX_EVEN, actual_hex_6);
    assert_eq!(expected_coord, actual_coord_6);

    let actual_coord_7 = HexOffset::r_from_cube(HEX_ODD, expected_hex);
    let hex_actual_7 = HexOffset::r_to_cube(HEX_ODD, actual_coord_7);
    assert_eq!(expected_hex, hex_actual_7);

    let actual_hex_8 = HexOffset::r_to_cube(HEX_ODD, expected_coord);
    let actual_coord_8 = HexOffset::r_from_cube(HEX_ODD, actual_hex_8);
    assert_eq!(expected_coord, actual_coord_8);
}

#[test]
fn test_offset_from_cube() {
    let hex1 = Hex::new(1, 2);
    assert_eq!(
        OffsetCoord { col: 1, row: 3 },
        HexOffset::q_from_cube(HEX_EVEN, hex1)
    );

    let hex2 = Hex::new(1, 2);
    assert_eq!(
        OffsetCoord { col: 1, row: 2 },
        HexOffset::q_from_cube(HEX_ODD, hex2)
    );
}

#[test]
fn test_offset_to_cube() {
    let coord1 = OffsetCoord { col: 1, row: 3 };
    assert_eq!(Hex::new(1, 2), HexOffset::q_to_cube(HEX_EVEN, coord1));

    let coord2 = OffsetCoord { col: 1, row: 2 };
    assert_eq!(Hex::new(1, 2), HexOffset::q_to_cube(HEX_ODD, coord2));
}

#[test]
fn test_doubled_roundtrip() {
    let expected_hex: Hex = Hex::new(3, 4);
    let expected_coord: DoubledCoord = DoubledCoord { col: 1, row: -3 };

    let actual_coord_1 = HexDoubled::q_from_cube(expected_hex);
    let hex_actual_1 = HexDoubled::q_to_cube(actual_coord_1);
    assert_eq!(expected_hex, hex_actual_1);

    let actual_hex_2 = HexDoubled::q_to_cube(expected_coord);
    let actual_coord_2 = HexDoubled::q_from_cube(actual_hex_2);
    assert_eq!(expected_coord, actual_coord_2);

    let actual_coord_3 = HexDoubled::r_from_cube(expected_hex);
    let hex_actual_3 = HexDoubled::r_to_cube(actual_coord_3);
    assert_eq!(expected_hex, hex_actual_3);

    let actual_hex_4 = HexDoubled::r_to_cube(expected_coord);
    let actual_coord_4 = HexDoubled::r_from_cube(actual_hex_4);
    assert_eq!(expected_coord, actual_coord_4);
}

#[test]
pub fn test_doubled_from_cube() {
    let hex = Hex::new(1, 2);

    let expected_coord1 = DoubledCoord { col: 1, row: 5 };
    assert_eq!(expected_coord1, HexDoubled::q_from_cube(hex));

    let expected_coord2 = DoubledCoord { col: 4, row: 2 };
    assert_eq!(expected_coord2, HexDoubled::r_from_cube(hex));
}

#[test]
pub fn test_doubled_to_cube() {
    let expected_hex = Hex::new(1, 2);

    let coord1 = DoubledCoord { col: 1, row: 5 };
    assert_eq!(expected_hex, HexDoubled::q_to_cube(coord1));

    let coord2 = DoubledCoord { col: 4, row: 2 };
    assert_eq!(expected_hex, HexDoubled::r_to_cube(coord2));
}
