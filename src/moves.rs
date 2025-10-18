use crate::board;
use crate::utils;

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
    pawn_attacks
}

pub fn knight_attacks(square: &u64) -> u64 {
    // take the piece, use rank/file data and if statements, use bitwise-or with u64 = 0 to add attack squares.
    let mut knight_attacks: u64 = 0;
    if !((utils::on_rank_7(square) || utils::on_rank_8(square)) || utils::on_a_file(square)) {
        knight_attacks = knight_attacks | (square << 15);
    }
    if !((utils::on_rank_7(square) || utils::on_rank_8(square)) || utils::on_h_file(square)) {
        knight_attacks = knight_attacks | (square << 17);
    }
    if !(utils::on_rank_8(square) || (utils::on_a_file(square) || utils::on_b_file(square))) {
        knight_attacks = knight_attacks | (square << 6);
    }
    if !(utils::on_rank_8(square) || (utils::on_g_file(square) || utils::on_h_file(square))) {
        knight_attacks = knight_attacks | (square << 10);
    }
    if !((utils::on_rank_1(square) || utils::on_rank_2(square)) || utils::on_a_file(square)) {
        knight_attacks = knight_attacks | (square >> 17);
    }
    if !((utils::on_rank_1(square) || utils::on_rank_2(square)) || utils::on_h_file(square)) {
        knight_attacks = knight_attacks | (square >> 15);
    }
    if !(utils::on_rank_1(square) || (utils::on_a_file(square) || utils::on_b_file(square))) {
        knight_attacks = knight_attacks | (square >> 10);
    }
    if !(utils::on_rank_1(square) || (utils::on_g_file(square) || utils::on_h_file(square))) {
        knight_attacks = knight_attacks | (square >> 6);
    }
    knight_attacks
}

pub fn bishop_attacks(color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn rook_attacks(color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn queen_attacks(color: &bool, square: &u64) -> u64 {
    todo!("implement");
}

pub fn king_attacks(color: &bool, square: &u64) -> u64 {
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
    let square1: u64 = utils::square_to_bb("b5").unwrap(); // b5 bit = 1.
    let square1_pawn_attacks = pawn_attacks(&true, &square1);
    assert_eq!(square1_pawn_attacks, 0x0000050000000000); // a6 and c6 bit = 1.

    // black pawn on b5
    let square2: u64 = utils::square_to_bb("b5").unwrap(); // b5 bit = 1.
    let square2_pawn_attacks = pawn_attacks(&false, &square2);
    assert_eq!(square2_pawn_attacks, 0x0000000005000000); // a4 and c4 bit = 1.

    // white pawn on a1
    let square3: u64 = utils::square_to_bb("a1").unwrap(); // a1 bit = 1.
    let square3_pawn_attacks = pawn_attacks(&true, &square3);
    assert_eq!(square3_pawn_attacks, 0x0000000000000200); // b2 bit = 1.

    // black pawn on a8
    let square4: u64 = utils::square_to_bb("a8").unwrap(); // a8 bit = 1.
    let square4_pawn_attacks = pawn_attacks(&false, &square4);
    assert_eq!(square4_pawn_attacks, 0x0002000000000000); // b7 bit = 1.

    // white pawn on h1
    let square5: u64 = utils::square_to_bb("h1").unwrap(); // h1 bit = 1.
    let square5_pawn_attacks = pawn_attacks(&true, &square5);
    assert_eq!(square5_pawn_attacks, 0x0000000000004000); // g2 bit = 1.

    // black pawn on h8
    let square6: u64 = utils::square_to_bb("h8").unwrap(); // h8 bit = 1.
    let square6_pawn_attacks = pawn_attacks(&false, &square6);
    assert_eq!(square6_pawn_attacks, 0x0040000000000000); // g7 bit = 1.
}

#[test]
fn test_knight_attacks() {
    // test squares: a1, a2, b1, b2, g1, g2, h1, h2, a7, a8, b7, b8, g7, g8, h7, h8, d4
    // a1:
    let square1 = utils::square_to_bb("a1").unwrap();
    let sq1_knight_attacks = knight_attacks(&square1);
    assert_eq!(sq1_knight_attacks, 0x0000000000020400);


    // a2:
    let square2 = utils::square_to_bb("a2").unwrap();
    let sq2_knight_attacks = knight_attacks(&square2);
    assert_eq!(sq2_knight_attacks, 0x0000000002040004);

    // b1:
    let square3 = utils::square_to_bb("b1").unwrap();
    let sq3_knight_attacks = knight_attacks(&square3);
    assert_eq!(sq3_knight_attacks, 0x0000000000050800);
    
    // b2:
    let square4 = utils::square_to_bb("b2").unwrap();
    let sq4_knight_attacks = knight_attacks(&square4);
    assert_eq!(sq4_knight_attacks, 0x0000000005080008);

    // g1:
    let square5 = utils::square_to_bb("g1").unwrap();
    let sq5_knight_attacks = knight_attacks(&square5);
    assert_eq!(sq5_knight_attacks, 0x0000000000A01000);
    
    // g2:
    let square6 = utils::square_to_bb("g2").unwrap();
    let sq6_knight_attacks = knight_attacks(&square6);
    assert_eq!(sq6_knight_attacks, 0x00000000A0100010);

    // h1:
    let square7 = utils::square_to_bb("h1").unwrap();
    let sq7_knight_attacks = knight_attacks(&square7);
    assert_eq!(sq7_knight_attacks, 0x0000000000402000);

    // h2:
    let square8 = utils::square_to_bb("h2").unwrap();
    let sq8_knight_attacks = knight_attacks(&square8);
    assert_eq!(sq8_knight_attacks, 0x0000000040200020);

    // a7:
    let square9 = utils::square_to_bb("a7").unwrap();
    let sq9_knight_attacks = knight_attacks(&square9);
    assert_eq!(sq9_knight_attacks, 0x0400040200000000);
    
    // a8:
    let square10 = utils::square_to_bb("a8").unwrap();
    let sq10_knight_attacks = knight_attacks(&square10);
    assert_eq!(sq10_knight_attacks, 0x0004020000000000);

    // b7:
    let square11 = utils::square_to_bb("b7").unwrap();
    let sq11_knight_attacks = knight_attacks(&square11);
    assert_eq!(sq11_knight_attacks, 0x0800080500000000);
}
