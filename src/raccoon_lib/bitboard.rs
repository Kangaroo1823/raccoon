use crate::bitboard::Square::*;

use std::mem::transmute;
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

pub enum Color {
    White = 0,
    Black = 1,
}

pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

pub const ALL_SQUARES: [Square; 64] = [
    A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8, E1, E2, E3, E4, E5, E6, E7, E8, F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8, H1, H2, H3, H4, H5, H6, H7, H8,
];

pub type Bitboard = u64;

pub const EMPTY_BOARD: Bitboard = 0;

pub fn set_bit(square: Square, bitboard: Bitboard) -> Bitboard {
    let bit = 1 << square as u8;
    bitboard | bit
}

pub fn clear_bit(square: Square, bitboard: Bitboard) -> Bitboard {
    let bit = 1 << square as u8;
    bitboard & !bit
}

pub fn is_bit_set(square: Square, bitboard: Bitboard) -> bool {
    let bit = 1 << square as u8;
    bitboard & bit != 0
}

pub fn rank_and_file_to_square(rank: u8, file: u8) -> Square {
    let square = rank * 8 + file;
    unsafe { transmute(square) }
}

pub fn print_bitboard(bitboard: Bitboard) {
    for rank in 0..8 {
        print!("    ");
        for file in 0..8 {
            if is_bit_set(rank_and_file_to_square(rank, file), bitboard) {
                print!(" 1 ");
            } else {
                print!(" . ");
            }
        }
        println!("\n");
    }
}


#[test]
fn test_bitboard() {
    let bitboard = set_bit(A1, 0);
    assert_eq!(bitboard, 1);
    
    print_bitboard(bitboard);
}
