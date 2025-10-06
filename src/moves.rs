use crate::board;

pub const A_FILE: u64 = 0x0101010101010101;
pub const H_FILE: u64 = 0x8080808080808080;

pub fn pawn_attacks(color: &bool, square: &u64) -> u64 {
    let mut pawn_attacks: u64 = 0;
    if *color {
        pawn_attacks = pawn_attacks | ((square & !A_FILE) << 7);
        pawn_attacks = pawn_attacks | ((square & !H_FILE) << 9);
    } else {
        pawn_attacks = pawn_attacks | ((square & !A_FILE) >> 9);
        pawn_attacks = pawn_attacks | ((square & !H_FILE) >> 7);
    }
    return pawn_attacks;
}

pub fn knight_attacks(board: &board::ChessBoard, color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn bishop_attacks(board: &board::ChessBoard, color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn rook_attacks(board: &board::ChessBoard, color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn queen_attacks(board: &board::ChessBoard, color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn king_attacks(board: &board::ChessBoard, color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn find_pseudolegal_moves(board: &board::ChessBoard) -> Vec<(u64, u64)> {
    // The idea is to return a vector containing types (u64, u64).
    // The first u64 denotes just one bit that shows the location of the from-piece.
    // The second u64 is a mask where every square the from-piece attacks has bit value 1.
    todo!("Implement this after creating attack mask functions.")
}

#[test]
fn test_pawn_attacks() {
    // white pawn on b5
    let square1: u64 = 0x0000000200000000; // b5 bit = 1.
    let square1_pawn_attacks = pawn_attacks(&true, &square1);
    assert_eq!(square1_pawn_attacks, 0x0000050000000000); // a6 and c6 bit = 1.

    // black pawn on b5
    let square2: u64 = 0x0000000200000000; // b5 bit = 1.
    let square2_pawn_attacks = pawn_attacks(&false, &square2);
    assert_eq!(square2_pawn_attacks, 0x0000000005000000); // a4 and c4 bit = 1.

    // white pawn on a1
    let square3: u64 = 0x0000000000000001; // a1 bit = 1.
    let square3_pawn_attacks = pawn_attacks(&true, &square3);
    assert_eq!(square3_pawn_attacks, 0x0000000000000200); // b2 bit = 1.

    // black pawn on a8
    let square4: u64 = 0x0100000000000000; // a8 bit = 1.
    let square4_pawn_attacks = pawn_attacks(&false, &square4);
    assert_eq!(square4_pawn_attacks, 0x0002000000000000); // b7 bit = 1.

    // white pawn on h1
    let square5: u64 = 0x0000000000000080; // h1 bit = 1.
    let square5_pawn_attacks = pawn_attacks(&true, &square5);
    assert_eq!(square5_pawn_attacks, 0x0000000000004000); // g2 bit = 1.

    // black pawn on h8
    let square6: u64 = 0x8000000000000000; // h8 bit = 1.
    let square6_pawn_attacks = pawn_attacks(&false, &square6);
    assert_eq!(square6_pawn_attacks, 0x0040000000000000); // g7 bit = 1.
}
