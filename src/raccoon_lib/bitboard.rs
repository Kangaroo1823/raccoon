use crate::bitboard::Square::*;
use std::arch::x86_64::{_blsi_u64, _pdep_u64, _pext_u64, _popcnt64, _tzcnt_u64};
use std::fmt::UpperHex;
use std::mem::transmute;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
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

#[inline(always)]
pub fn set_bit(square: Square, bitboard: Bitboard) -> Bitboard {
    let bit = 1 << square as u8;
    bitboard | bit
}

#[inline(always)]
pub fn clear_bit(square: Square, bitboard: Bitboard) -> Bitboard {
    let bit = 1 << square as u8;
    bitboard & !bit
}

#[inline(always)]
pub fn is_bit_set(square: Square, bitboard: Bitboard) -> bool {
    let bit = 1 << square as u8;
    bitboard & bit != 0
}

#[inline(always)]
pub fn rank_and_file_to_square(rank: usize, file: usize) -> Square {
    let square = rank * 8 + file;
    unsafe { transmute(square as u8) }
}

#[inline(always)]
pub fn bit_count(bitboard: Bitboard) -> usize {
    unsafe { _popcnt64(bitboard as i64) as usize }
}

#[inline(always)]
pub fn bitboard_square_of(bitboard: Bitboard) -> Bitboard {
    unsafe { _blsi_u64(bitboard) }
}

#[inline(always)]
pub fn square_of(bitboard: Bitboard) -> Square {
    unsafe { transmute(_tzcnt_u64(bitboard) as u8) }
}

#[inline(always)]
pub fn extract_mask(bitboard: Bitboard, mask: Bitboard) -> Bitboard {
    unsafe { _pext_u64(bitboard, mask) }
}

#[inline(always)]
pub fn create_occupancy_from_mask(index: Bitboard, mask: Bitboard) -> Bitboard {
    unsafe { _pdep_u64(index, mask) }
}

macro_rules! bit_loop {
    ($a:expr, $b:tt, $code:block) => {
        let mut $b = $a;
        while $b != 0 {
            $code;
            $b = unsafe { _blsr_u64($b) };
        }
    };
}

pub fn print_bitboard(bitboard: Bitboard) {
    print!("/*\n");
    for rank in 0..8 {
        print!("  {}  ", rank + 1);
        for file in 0..8 {
            if is_bit_set(rank_and_file_to_square(7 - rank, file), bitboard) {
                print!(" 1 ");
            } else {
                print!(" . ");
            }
        }
        print!("\n");
    }
    print!("\n      A  B  C  D  E  F  G  H \n*/\n");
    print!("0x{:016x},\n", bitboard);
}

pub const NOT_A_FILE: Bitboard = /*
        A  B  C  D  E  F  G  H
    1   0  1  1  1  1  1  1  1
    2   0  1  1  1  1  1  1  1
    3   0  1  1  1  1  1  1  1
    4   0  1  1  1  1  1  1  1
    5   0  1  1  1  1  1  1  1
    6   0  1  1  1  1  1  1  1
    7   0  1  1  1  1  1  1  1
    8   0  1  1  1  1  1  1  1

       bitboard as 64 bit integer: */
    18374403900871474942;

pub const NOT_AB_FILE: Bitboard = /*
        A  B  C  D  E  F  G  H
    1   0  0  1  1  1  1  1  1
    2   0  0  1  1  1  1  1  1
    3   0  0  1  1  1  1  1  1
    4   0  0  1  1  1  1  1  1
    5   0  0  1  1  1  1  1  1
    6   0  0  1  1  1  1  1  1
    7   0  0  1  1  1  1  1  1
    8   0  0  1  1  1  1  1  1

       bitboard as 64 bit integer: */
    18229723555195321596;

pub const NOT_H_FILE: Bitboard = /*
        A  B  C  D  E  F  G  H
    1   1  1  1  1  1  1  1  0
    2   1  1  1  1  1  1  1  0
    3   1  1  1  1  1  1  1  0
    4   1  1  1  1  1  1  1  0
    5   1  1  1  1  1  1  1  0
    6   1  1  1  1  1  1  1  0
    7   1  1  1  1  1  1  1  0
    8   1  1  1  1  1  1  1  0

       bitboard as 64 bit integer: */
    9187201950435737471;

pub const NOT_GH_FILE: Bitboard = /*
        A  B  C  D  E  F  G  H
    1   1  1  1  1  1  1  0  0
    2   1  1  1  1  1  1  0  0
    3   1  1  1  1  1  1  0  0
    4   1  1  1  1  1  1  0  0
    5   1  1  1  1  1  1  0  0
    6   1  1  1  1  1  1  0  0
    7   1  1  1  1  1  1  0  0
    8   1  1  1  1  1  1  0  0

       bitboard as 64 bit integer: */
    4557430888798830399;

#[inline(always)]
pub fn get_promotion_rank<const is_white: bool>() -> Bitboard {
    if is_white == true {
        0x00FF000000000000
    } else {
        0x000000000000FF00
    }
}

pub fn bitboard_from_hex(hex: &str) -> Bitboard {
    let hex = hex.trim_start_matches("0x");
    u64::from_str_radix(hex, 16).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::arch::x86_64::_blsr_u64;
    #[test]
    fn test_bitboard() {
        let bitboard = set_bit(A1, 0);
        assert_eq!(bitboard, 1);

        print_bitboard(bitboard);
    }

    #[test]
    fn test_square_of() {
        {
            let bitboard: Bitboard = 1;
            assert_eq!(square_of(bitboard), A1);
            assert_eq!(bitboard_square_of(bitboard), bitboard);
        }
        {
            let bitboard: Bitboard = 1 << 7;
            assert_eq!(square_of(bitboard), H1);
            assert_eq!(bitboard_square_of(bitboard), bitboard);
        }
    }

    #[test]
    fn test_bit_count() {
        {
            let bitboard: Bitboard = 1;
            assert_eq!(bit_count(bitboard), 1);
        }
        {
            let bitboard: Bitboard = 7;
            assert_eq!(bit_count(bitboard), 3);
        }
    }

    #[test]
    fn test_bit_loop() {
        {
            let bitboard: Bitboard = 7;
            let mut u = 0;
            let mut u2 = 0;
            bit_loop!(bitboard, k, {
                u = u + 1;
                u2 = u2 + k;
            });
            assert_eq!(u, 3);
            assert_eq!(u2, 17);
        }
    }
}
