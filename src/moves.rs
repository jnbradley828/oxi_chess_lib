use crate::board;
use crate::utils;

pub const A_FILE: u64 = 0x0101010101010101;
pub const H_FILE: u64 = 0x8080808080808080;

const fn generate_one_pawn_attacks(color: &bool, square: u64) -> u64 {
    // generates pawn attack squares for one square. no pruning based on occupied squares.
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

const fn generate_white_pawn_attacks() -> [u64; 64] {
    let mut white_pawn_attacks = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let i_square: u64 = 1 << i;
        white_pawn_attacks[i] = generate_one_pawn_attacks(&true, i_square);
        i += 1;
    }

    white_pawn_attacks
}

const fn generate_black_pawn_attacks() -> [u64; 64] {
    let mut black_pawn_attacks = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let i_square: u64 = 1 << i;
        black_pawn_attacks[i] = generate_one_pawn_attacks(&false, i_square);
        i += 1;
    }

    black_pawn_attacks
}

pub const WHITE_PAWN_ATTACKS: [u64; 64] = generate_white_pawn_attacks();
pub const BLACK_PAWN_ATTACKS: [u64; 64] = generate_black_pawn_attacks();

pub fn pawn_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    let mut pawn_attacks: u64 = 0;
    if *color {
        pawn_attacks = WHITE_PAWN_ATTACKS[square.trailing_zeros() as usize];
        pawn_attacks = pawn_attacks & !(board.white_pieces);
    } else {
        pawn_attacks = BLACK_PAWN_ATTACKS[square.trailing_zeros() as usize];
        pawn_attacks = pawn_attacks & !(board.black_pieces);
    }

    pawn_attacks
}

const fn generate_one_knight_attacks(square: u64) -> u64 {
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

const fn generate_knight_attacks() -> [u64; 64] {
    let mut knight_attacks = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let i_square: u64 = 1 << i;
        knight_attacks[i] = generate_one_knight_attacks(i_square);
        i += 1;
    }

    knight_attacks
}

pub const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();

pub fn knight_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    // prune knight_attacks if that square is taken by a same color piece.
    let mut knight_attacks = KNIGHT_ATTACKS[square.trailing_zeros() as usize];

    if *color {
        knight_attacks = knight_attacks & !(board.white_pieces);
    } else {
        knight_attacks = knight_attacks & !(board.black_pieces);
    }
    knight_attacks
}

pub fn bishop_attacks(square: &u64) -> u64 {
    let mut bishop_attacks: u64 = 0;

    let square_name = utils::bb_to_square(&square).unwrap();
    let file = (square_name.chars().nth(0).unwrap() as u8) - b'a' + 1;
    let rank = (square_name.chars().nth(1).unwrap() as u8) - b'0';

    let sum = file + rank;
    let diff: i8 = (rank as i8) - (file as i8);

    // include all squares with same (file + rank) sum. ('a' = 1, 'b' = 2, etc.)
    for i in 1.max(sum.saturating_sub(8))..sum.min(9) {
        let ifile = (b'a' + (i - 1)) as char;
        let irank = (b'0' + (sum - i)) as char;

        let mut sq_name = String::with_capacity(2);
        sq_name.push(ifile);
        sq_name.push(irank);

        let sq = utils::square_to_bb(&sq_name).unwrap();
        bishop_attacks = bishop_attacks | sq;
    }

    // include all squares with same (rank - file) difference.
    for j in 1.max(1 + diff)..9.min(9 + diff) {
        let jrank = ((b'0' as u8) + (j as u8)) as char;
        let jfile = ((b'a' as i8) + (j as i8 - diff - 1)) as u8 as char;

        let mut sq_name = String::with_capacity(2);
        sq_name.push(jfile);
        sq_name.push(jrank);

        let sq = utils::square_to_bb(&sq_name).unwrap();
        bishop_attacks = bishop_attacks | sq;
    }

    bishop_attacks = bishop_attacks ^ square;
    bishop_attacks
}

pub fn rook_attacks(square: &u64) -> u64 {
    let mut rook_attacks: u64 = 0;

    let square_name = utils::bb_to_square(&square).unwrap();
    let file = square_name.chars().nth(0).unwrap();
    let rank = square_name.chars().nth(1).unwrap();

    let file_modifier = (file as u64) - (b'a' as u64);
    for r in 0..8 {
        let sq_modifier: u64 = (8 * r) + file_modifier;
        rook_attacks = rook_attacks | (1 << sq_modifier);
    }

    let rank_val = (rank as u64) - (b'0' as u64);
    let left_sq = (rank_val - 1) * 8;
    for f in left_sq..(left_sq + 8) {
        rook_attacks = rook_attacks | (1 << f);
    }

    rook_attacks = rook_attacks ^ square;
    rook_attacks
}

pub fn queen_attacks(square: &u64) -> u64 {
    let mut queen_attacks: u64 = 0;
    queen_attacks = queen_attacks | rook_attacks(square);
    queen_attacks = queen_attacks | bishop_attacks(square);
    queen_attacks
}

pub fn king_attacks(square: &u64) -> u64 {
    let mut king_attacks: u64 = 0;

    let square_name = utils::bb_to_square(&square).unwrap();
    let file = square_name.chars().nth(0).unwrap();
    let rank = square_name.chars().nth(1).unwrap();

    // using 0 to 2 instead of -1 to 1 to avoid i8/u8 type conversions. adjust by -1.
    for fv in 0..=2 {
        for rv in 0..=2 {
            let sq_file = ((file as u8) + (fv as u8) - 1) as char;
            let sq_rank = ((rank as u8) + (rv as u8) - 1) as char;

            if utils::FILES.contains(&sq_file) && utils::RANKS.contains(&sq_rank) {
                let mut sq_name = String::with_capacity(2);
                sq_name.push(sq_file);
                sq_name.push(sq_rank);

                king_attacks = king_attacks | utils::square_to_bb(&sq_name).unwrap();
            }
        }
    }
    king_attacks = king_attacks ^ square;
    king_attacks
}

pub fn board_attacks(board: &board::ChessBoard) -> Vec<(u64, u64)> {
    // The idea is to return a vector containing type (u64, u64).
    // The first u64 denotes just one bit that shows the location of the from-piece.
    // The second u64 is a mask where every square the from-piece attacks has bit value 1.
    let mut attack_list: Vec<(u64, u64)> = Vec::new();

    let mut color_mask: u64 = 0;
    if board.side_to_move {
        color_mask = board.white_pieces;
    } else {
        color_mask = board.black_pieces;
    }

    for piece_bb in [
        ("pawns", board.pawns),
        ("knights", board.knights),
        ("bishops", board.bishops),
        ("rooks", board.rooks),
        ("queens", board.queens),
        ("kings", board.kings),
    ] {
        let mut colored_bb = piece_bb.1 & color_mask;

        while colored_bb.trailing_zeros() != 64 {
            let from_square: u64 = 1 << colored_bb.trailing_zeros();

            let attack_squares: u64 = match piece_bb.0 {
                "pawns" => pawn_attacks(&board.side_to_move, &from_square, board),
                "knights" => knight_attacks(&board.side_to_move, &from_square, board),
                "bishops" => bishop_attacks(&from_square),
                "rooks" => rook_attacks(&from_square),
                "queens" => queen_attacks(&from_square),
                "kings" => king_attacks(&from_square),
                &_ => 0,
            };

            colored_bb = colored_bb & !(1 << colored_bb.trailing_zeros()); // erase current from_piece from colored_bb.
            attack_list.push((from_square, attack_squares));
        }
    }
    attack_list
}

#[test]
fn test_pawn_attacks() {
    let empty_board = board::ChessBoard::empty();

    // white pawn on b5
    let square1: u64 = utils::square_to_bb("b5").unwrap(); // b5 bit = 1.
    let square1_pawn_attacks = pawn_attacks(&true, &square1, &empty_board);
    assert_eq!(square1_pawn_attacks, 0x0000050000000000); // a6 and c6 bit = 1.

    // black pawn on b5
    let square2: u64 = utils::square_to_bb("b5").unwrap(); // b5 bit = 1.
    let square2_pawn_attacks = pawn_attacks(&false, &square2, &empty_board);
    assert_eq!(square2_pawn_attacks, 0x0000000005000000); // a4 and c4 bit = 1.

    // white pawn on a1
    let square3: u64 = utils::square_to_bb("a1").unwrap(); // a1 bit = 1.
    let square3_pawn_attacks = pawn_attacks(&true, &square3, &empty_board);
    assert_eq!(square3_pawn_attacks, 0x0000000000000200); // b2 bit = 1.

    // black pawn on a8
    let square4: u64 = utils::square_to_bb("a8").unwrap(); // a8 bit = 1.
    let square4_pawn_attacks = pawn_attacks(&false, &square4, &empty_board);
    assert_eq!(square4_pawn_attacks, 0x0002000000000000); // b7 bit = 1.

    // white pawn on h1
    let square5: u64 = utils::square_to_bb("h1").unwrap(); // h1 bit = 1.
    let square5_pawn_attacks = pawn_attacks(&true, &square5, &empty_board);
    assert_eq!(square5_pawn_attacks, 0x0000000000004000); // g2 bit = 1.

    // black pawn on h8
    let square6: u64 = utils::square_to_bb("h8").unwrap(); // h8 bit = 1.
    let square6_pawn_attacks = pawn_attacks(&false, &square6, &empty_board);
    assert_eq!(square6_pawn_attacks, 0x0040000000000000); // g7 bit = 1.

    let non_empty_board =
        board::ChessBoard::initialize_from_fen("k7/1p6/b7/8/8/B7/1P6/K7 b KQkq - 0 1").unwrap();

    // white pawn blocked by its own piece.
    let square7: u64 = utils::square_to_bb("b2").unwrap();
    let square7_pawn_attacks = pawn_attacks(&true, &square7, &non_empty_board);
    assert_eq!(square7_pawn_attacks, 0x0000000000040000);
    // black pawn blocked by its own piece.
    let square8: u64 = utils::square_to_bb("b7").unwrap();
    let square8_pawn_attacks = pawn_attacks(&false, &square8, &non_empty_board);
    assert_eq!(square8_pawn_attacks, 0x0000040000000000);
}

#[test]
fn test_knight_attacks() {
    // test squares: a1, a2, b1, b2, g1, g2, h1, h2, a7, a8, b7, b8, g7, g8, h7, h8, d4
    let empty_board = board::ChessBoard::empty();

    // a1:
    let square1 = utils::square_to_bb("a1").unwrap();
    let sq1_knight_attacks = knight_attacks(&true, &square1, &empty_board);
    assert_eq!(sq1_knight_attacks, 0x0000000000020400);

    // a2:
    let square2 = utils::square_to_bb("a2").unwrap();
    let sq2_knight_attacks = knight_attacks(&true, &square2, &empty_board);
    assert_eq!(sq2_knight_attacks, 0x0000000002040004);

    // b1:
    let square3 = utils::square_to_bb("b1").unwrap();
    let sq3_knight_attacks = knight_attacks(&true, &square3, &empty_board);
    assert_eq!(sq3_knight_attacks, 0x0000000000050800);

    // b2:
    let square4 = utils::square_to_bb("b2").unwrap();
    let sq4_knight_attacks = knight_attacks(&true, &square4, &empty_board);
    assert_eq!(sq4_knight_attacks, 0x0000000005080008);

    // g1:
    let square5 = utils::square_to_bb("g1").unwrap();
    let sq5_knight_attacks = knight_attacks(&true, &square5, &empty_board);
    assert_eq!(sq5_knight_attacks, 0x0000000000A01000);

    // g2:
    let square6 = utils::square_to_bb("g2").unwrap();
    let sq6_knight_attacks = knight_attacks(&true, &square6, &empty_board);
    assert_eq!(sq6_knight_attacks, 0x00000000A0100010);

    // h1:
    let square7 = utils::square_to_bb("h1").unwrap();
    let sq7_knight_attacks = knight_attacks(&true, &square7, &empty_board);
    assert_eq!(sq7_knight_attacks, 0x0000000000402000);

    // h2:
    let square8 = utils::square_to_bb("h2").unwrap();
    let sq8_knight_attacks = knight_attacks(&true, &square8, &empty_board);
    assert_eq!(sq8_knight_attacks, 0x0000000040200020);

    // a7:
    let square9 = utils::square_to_bb("a7").unwrap();
    let sq9_knight_attacks = knight_attacks(&true, &square9, &empty_board);
    assert_eq!(sq9_knight_attacks, 0x0400040200000000);

    // a8:
    let square10 = utils::square_to_bb("a8").unwrap();
    let sq10_knight_attacks = knight_attacks(&true, &square10, &empty_board);
    assert_eq!(sq10_knight_attacks, 0x0004020000000000);

    // b7:
    let square11 = utils::square_to_bb("b7").unwrap();
    let sq11_knight_attacks = knight_attacks(&true, &square11, &empty_board);
    assert_eq!(sq11_knight_attacks, 0x0800080500000000);

    let starting_board = board::ChessBoard::initialize();

    // b1 from starting position (exclude d2)
    let square12 = utils::square_to_bb("b1").unwrap();
    let sq12_knight_attacks = knight_attacks(&true, &square12, &starting_board);
    assert_eq!(sq12_knight_attacks, 0x0000000000050000);

    // b8 from starting position (exclude d7)
    let square13 = utils::square_to_bb("b8").unwrap();
    let sq13_knight_attacks = knight_attacks(&false, &square13, &starting_board);
    assert_eq!(sq13_knight_attacks, 0x0000050000000000);
}

#[test]
fn test_bishop_attacks() {
    // a1
    let square1 = utils::square_to_bb("a1").unwrap();
    let sq1_bishop_attacks = bishop_attacks(&square1);
    assert_eq!(sq1_bishop_attacks, 0x8040201008040200);
    // // a8
    let square2 = utils::square_to_bb("a8").unwrap();
    let sq2_bishop_attacks = bishop_attacks(&square2);
    assert_eq!(sq2_bishop_attacks, 0x0002040810204080);
    // // h1
    let square3 = utils::square_to_bb("h1").unwrap();
    let sq3_bishop_attacks = bishop_attacks(&square3);
    assert_eq!(sq3_bishop_attacks, 0x0102040810204000);
    // // h8
    let square4 = utils::square_to_bb("h8").unwrap();
    let sq4_bishop_attacks = bishop_attacks(&square4);
    assert_eq!(sq4_bishop_attacks, 0x0040201008040201);
    // // d4
    let square5 = utils::square_to_bb("d4").unwrap();
    let sq5_bishop_attacks = bishop_attacks(&square5);
    assert_eq!(sq5_bishop_attacks, 0x8041221400142241);
}

#[test]
fn test_rook_attacks() {
    // a1
    let square1 = utils::square_to_bb("a1").unwrap();
    let sq1_rook_attacks = rook_attacks(&square1);
    assert_eq!(sq1_rook_attacks, 0x01010101010101FE);
    // a8
    let square2 = utils::square_to_bb("a8").unwrap();
    let sq2_rook_attacks = rook_attacks(&square2);
    assert_eq!(sq2_rook_attacks, 0xFE01010101010101);
    // h1
    let square3 = utils::square_to_bb("h1").unwrap();
    let sq3_rook_attacks = rook_attacks(&square3);
    assert_eq!(sq3_rook_attacks, 0x808080808080807F);
    // h8
    let square4 = utils::square_to_bb("h8").unwrap();
    let sq4_rook_attacks = rook_attacks(&square4);
    assert_eq!(sq4_rook_attacks, 0x7F80808080808080);
    // d4
    let square5 = utils::square_to_bb("d4").unwrap();
    let sq5_rook_attacks = rook_attacks(&square5);
    assert_eq!(sq5_rook_attacks, 0x08080808F7080808);
}

#[test]
fn test_queen_attacks() {
    // a1
    let square1 = utils::square_to_bb("a1").unwrap();
    let sq1_queen_attacks = queen_attacks(&square1);
    assert_eq!(sq1_queen_attacks, (0x01010101010101FE | 0x8040201008040200));
    // a8
    let square2 = utils::square_to_bb("a8").unwrap();
    let sq2_queen_attacks = queen_attacks(&square2);
    assert_eq!(sq2_queen_attacks, (0xFE01010101010101 | 0x0002040810204080));
    // h1
    let square3 = utils::square_to_bb("h1").unwrap();
    let sq3_queen_attacks = queen_attacks(&square3);
    assert_eq!(sq3_queen_attacks, (0x808080808080807F | 0x0102040810204000));
    // h8
    let square4 = utils::square_to_bb("h8").unwrap();
    let sq4_queen_attacks = queen_attacks(&square4);
    assert_eq!(sq4_queen_attacks, (0x7F80808080808080 | 0x0040201008040201));
    // d4
    let square5 = utils::square_to_bb("d4").unwrap();
    let sq5_queen_attacks = queen_attacks(&square5);
    assert_eq!(sq5_queen_attacks, (0x08080808F7080808 | 0x8041221400142241));
}

#[test]
fn test_king_attacks() {
    // a1
    let square1 = utils::square_to_bb("a1").unwrap();
    let sq1_king_attacks = king_attacks(&square1);
    assert_eq!(sq1_king_attacks, (0x0000000000000302));
    // a8
    let square2 = utils::square_to_bb("a8").unwrap();
    let sq2_king_attacks = king_attacks(&square2);
    assert_eq!(sq2_king_attacks, (0x0203000000000000));
    // h1
    let square3 = utils::square_to_bb("h1").unwrap();
    let sq3_king_attacks = king_attacks(&square3);
    assert_eq!(sq3_king_attacks, (0x000000000000C040));
    // h8
    let square4 = utils::square_to_bb("h8").unwrap();
    let sq4_king_attacks = king_attacks(&square4);
    assert_eq!(sq4_king_attacks, (0x40C0000000000000));
    // d4
    let square5 = utils::square_to_bb("d4").unwrap();
    let sq5_king_attacks = king_attacks(&square5);
    assert_eq!(sq5_king_attacks, (0x0000001C141C0000));
}

#[test]
fn test_board_attacks() {
    // 2 pieces of each type (except king), black to move
    let board1 =
        board::ChessBoard::initialize_from_fen("rnbq1p2/rnbq1pk1/8/8/8/8/8/K7 b KQkq - 0 1")
            .unwrap();
    let psl_moves = board_attacks(&board1);

    let mut psl_moves_manual: Vec<(u64, u64)> = Vec::new();

    let p_bb1 = utils::square_to_bb("f7").unwrap();
    psl_moves_manual.push((p_bb1, pawn_attacks(&false, &p_bb1, &board1)));
    let p_bb2 = utils::square_to_bb("f8").unwrap();
    psl_moves_manual.push((p_bb2, pawn_attacks(&false, &p_bb2, &board1)));

    let n_bb1 = utils::square_to_bb("b7").unwrap();
    psl_moves_manual.push((n_bb1, knight_attacks(&false, &n_bb1, &board1)));
    let n_bb2 = utils::square_to_bb("b8").unwrap();
    psl_moves_manual.push((n_bb2, knight_attacks(&false, &n_bb2, &board1)));

    let b_bb1 = utils::square_to_bb("c7").unwrap();
    psl_moves_manual.push((b_bb1, bishop_attacks(&b_bb1)));
    let b_bb2 = utils::square_to_bb("c8").unwrap();
    psl_moves_manual.push((b_bb2, bishop_attacks(&b_bb2)));

    let r_bb1 = utils::square_to_bb("a7").unwrap();
    psl_moves_manual.push((r_bb1, rook_attacks(&r_bb1)));
    let r_bb2 = utils::square_to_bb("a8").unwrap();
    psl_moves_manual.push((r_bb2, rook_attacks(&r_bb2)));

    let q_bb1 = utils::square_to_bb("d7").unwrap();
    psl_moves_manual.push((q_bb1, queen_attacks(&q_bb1)));
    let q_bb2 = utils::square_to_bb("d8").unwrap();
    psl_moves_manual.push((q_bb2, queen_attacks(&q_bb2)));

    let k_bb1 = utils::square_to_bb("g7").unwrap();
    psl_moves_manual.push((k_bb1, king_attacks(&k_bb1)));

    assert_eq!(psl_moves, psl_moves_manual);

    // 2 pieces of each type (except king), white to move
    let board2 =
        board::ChessBoard::initialize_from_fen("k7/8/8/8/8/8/RNBQ1PK1/RNBQ1P2 w KQkq - 0 1")
            .unwrap();
    let psl_moves = board_attacks(&board2);

    let mut psl_moves_manual: Vec<(u64, u64)> = Vec::new();

    let p_bb3 = utils::square_to_bb("f1").unwrap();
    psl_moves_manual.push((p_bb3, pawn_attacks(&true, &p_bb3, &board2)));
    let p_bb4 = utils::square_to_bb("f2").unwrap();
    psl_moves_manual.push((p_bb4, pawn_attacks(&true, &p_bb4, &board2)));

    let n_bb3 = utils::square_to_bb("b1").unwrap();
    psl_moves_manual.push((n_bb3, knight_attacks(&true, &n_bb3, &board2)));
    let n_bb4 = utils::square_to_bb("b2").unwrap();
    psl_moves_manual.push((n_bb4, knight_attacks(&true, &n_bb4, &board2)));

    let b_bb3 = utils::square_to_bb("c1").unwrap();
    psl_moves_manual.push((b_bb3, bishop_attacks(&b_bb3)));
    let b_bb4 = utils::square_to_bb("c2").unwrap();
    psl_moves_manual.push((b_bb4, bishop_attacks(&b_bb4)));

    let r_bb3 = utils::square_to_bb("a1").unwrap();
    psl_moves_manual.push((r_bb3, rook_attacks(&r_bb3)));
    let r_bb4 = utils::square_to_bb("a2").unwrap();
    psl_moves_manual.push((r_bb4, rook_attacks(&r_bb4)));

    let q_bb3 = utils::square_to_bb("d1").unwrap();
    psl_moves_manual.push((q_bb3, queen_attacks(&q_bb3)));
    let q_bb4 = utils::square_to_bb("d2").unwrap();
    psl_moves_manual.push((q_bb4, queen_attacks(&q_bb4)));

    let k_bb2 = utils::square_to_bb("g2").unwrap();
    psl_moves_manual.push((k_bb2, king_attacks(&k_bb2)));

    assert_eq!(psl_moves, psl_moves_manual);
}
