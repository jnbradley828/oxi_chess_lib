// this file produces attack masks for each piece on a given board.

use crate::board;
use crate::board::ChessBoard;
use crate::rules::is_check;
use crate::utils::{self, encode_move};

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

// returns squares attacked by one pawn.
pub fn pawn_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    let mut pawn_attacks: u64;
    if *color {
        pawn_attacks = WHITE_PAWN_ATTACKS[square.trailing_zeros() as usize];
        pawn_attacks &= !board.white_pieces;
    } else {
        pawn_attacks = BLACK_PAWN_ATTACKS[square.trailing_zeros() as usize];
        pawn_attacks &= !board.black_pieces;
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

// returns squares attacked by one knight.
pub fn knight_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    let mut knight_attacks = KNIGHT_ATTACKS[square.trailing_zeros() as usize];

    if *color {
        knight_attacks = knight_attacks & !(board.white_pieces);
    } else {
        knight_attacks = knight_attacks & !(board.black_pieces);
    }
    knight_attacks
}

pub const fn generate_north_ray(square: u64) -> u64 {
    let mut ray: u64 = 0;

    let mut n_square = square.wrapping_shl(8);
    while n_square > square {
        ray |= n_square;
        n_square = n_square.wrapping_shl(8);
    }
    ray
}

pub const fn generate_south_ray(square: u64) -> u64 {
    let mut ray: u64 = 0;

    let mut n_square = square.wrapping_shr(8);
    while n_square != 0 {
        ray |= n_square;
        n_square = n_square.wrapping_shr(8);
    }
    ray
}

pub const fn generate_east_ray(square: u64) -> u64 {
    if utils::on_h_file(square) {
        0
    } else {
        let mut ray: u64 = 0;

        let mut e_square = square;
        while !utils::on_h_file(e_square) {
            e_square = e_square << 1;
            ray |= e_square;
        }
        ray
    }
}

pub const fn generate_west_ray(square: u64) -> u64 {
    if utils::on_a_file(square) {
        0
    } else {
        let mut ray: u64 = 0;

        let mut w_square = square;
        while !utils::on_a_file(w_square) {
            w_square = w_square >> 1;
            ray |= w_square;
        }
        ray
    }
}

pub const fn generate_ne_ray(square: u64) -> u64 {
    if utils::on_h_file(square) || utils::on_rank_8(square) {
        0
    } else {
        let mut ray: u64 = 0;

        let mut ne_square = square;
        while !(utils::on_h_file(ne_square) || utils::on_rank_8(ne_square)) {
            ne_square = ne_square << 9;
            ray |= ne_square;
        }
        ray
    }
}

pub const fn generate_nw_ray(square: u64) -> u64 {
    if utils::on_a_file(square) || utils::on_rank_8(square) {
        0
    } else {
        let mut ray: u64 = 0;

        let mut nw_square = square;
        while !(utils::on_a_file(nw_square) || utils::on_rank_8(nw_square)) {
            nw_square = nw_square << 7;
            ray |= nw_square;
        }
        ray
    }
}

pub const fn generate_sw_ray(square: u64) -> u64 {
    if utils::on_a_file(square) || utils::on_rank_1(square) {
        0
    } else {
        let mut ray: u64 = 0;

        let mut sw_square = square;
        while !(utils::on_a_file(sw_square) || utils::on_rank_1(sw_square)) {
            sw_square = sw_square >> 9;
            ray |= sw_square;
        }
        ray
    }
}

pub const fn generate_se_ray(square: u64) -> u64 {
    if utils::on_h_file(square) || utils::on_rank_1(square) {
        0
    } else {
        let mut ray: u64 = 0;

        let mut se_square = square;
        while !(utils::on_h_file(se_square) || utils::on_rank_1(se_square)) {
            se_square = se_square >> 7;
            ray |= se_square;
        }
        ray
    }
}

pub const fn generate_rays() -> [[u64; 8]; 64] {
    // indices of inner array defined as follows (0-7): [n, ne, e, se, s, sw, w, nw]
    let mut rays: [[u64; 8]; 64] = [[0; 8]; 64];

    let mut i = 0;
    while i < 64 {
        let piece_bb: u64 = 1 << i;

        rays[i][0] = generate_north_ray(piece_bb);
        rays[i][1] = generate_ne_ray(piece_bb);
        rays[i][2] = generate_east_ray(piece_bb);
        rays[i][3] = generate_se_ray(piece_bb);
        rays[i][4] = generate_south_ray(piece_bb);
        rays[i][5] = generate_sw_ray(piece_bb);
        rays[i][6] = generate_west_ray(piece_bb);
        rays[i][7] = generate_nw_ray(piece_bb);

        i += 1;
    }
    rays
}

pub const RAYS: [[u64; 8]; 64] = generate_rays();
// relevant ray = RAYS[square_bb.trailing_zeros()][i in 0-7 in order n, ne, e, se, s, sw, w, nw]

pub fn check_along_ray(piece: u64, ray: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    // define nw, n, ne, e as dir = true, else dir = false
    let dir = ray > piece;
    let mut ray_left = ray;
    let mut updated_ray = 0;

    if dir {
        let mut next_sqi = (1 as u64).wrapping_shl(ray_left.trailing_zeros());

        while ray_left != 0 {
            if next_sqi & friendly_pieces != 0 {
                return updated_ray;
            } else if next_sqi & enemy_pieces != 0 {
                updated_ray = updated_ray | next_sqi;
                return updated_ray;
            } else {
                updated_ray = updated_ray | next_sqi;
                ray_left = ray_left & !(next_sqi);
                next_sqi = (1 as u64).wrapping_shl(ray_left.trailing_zeros());
            }
        }
        return updated_ray;
    } else {
        let mut next_sqi = (0x8000000000000000 as u64).wrapping_shr(ray_left.leading_zeros());

        while ray_left != 0 {
            if next_sqi & friendly_pieces != 0 {
                return updated_ray;
            } else if next_sqi & enemy_pieces != 0 {
                updated_ray = updated_ray | next_sqi;
                return updated_ray;
            } else {
                updated_ray = updated_ray | next_sqi;
                ray_left = ray_left & !(next_sqi);
                next_sqi = (0x8000000000000000 as u64).wrapping_shr(ray_left.leading_zeros());
            }
        }
        return updated_ray;
    }
}

// returns squares attacked by one bishop.
pub fn bishop_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    let mut bishop_attacks: u64 = 0;

    let friendly_pieces;
    let enemy_pieces;
    if *color {
        friendly_pieces = board.white_pieces;
        enemy_pieces = board.black_pieces;
    } else {
        friendly_pieces = board.black_pieces;
        enemy_pieces = board.white_pieces;
    }

    let sq_trzs = square.trailing_zeros() as usize;
    let bishop_rays: [u64; 4] = [
        RAYS[sq_trzs][1], //ne
        RAYS[sq_trzs][3], //se
        RAYS[sq_trzs][5], //sw
        RAYS[sq_trzs][7], //nw
    ];

    for ray in bishop_rays {
        bishop_attacks =
            bishop_attacks | check_along_ray(*square, ray, friendly_pieces, enemy_pieces);
    }

    bishop_attacks
}

// returns squares attacked by one rook
pub fn rook_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    let mut rook_attacks: u64 = 0;

    let friendly_pieces;
    let enemy_pieces;
    if *color {
        friendly_pieces = board.white_pieces;
        enemy_pieces = board.black_pieces;
    } else {
        friendly_pieces = board.black_pieces;
        enemy_pieces = board.white_pieces;
    }

    let sq_trzs = square.trailing_zeros() as usize;
    let rook_rays: [u64; 4] = [
        RAYS[sq_trzs][0], //n
        RAYS[sq_trzs][2], //e
        RAYS[sq_trzs][4], //s
        RAYS[sq_trzs][6], //w
    ];

    for ray in rook_rays {
        rook_attacks = rook_attacks | check_along_ray(*square, ray, friendly_pieces, enemy_pieces);
    }

    rook_attacks
}

// returns squares attacked by one queen.
pub fn queen_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    let mut queen_attacks: u64 = 0;
    queen_attacks = queen_attacks | rook_attacks(color, square, board);
    queen_attacks = queen_attacks | bishop_attacks(color, square, board);
    queen_attacks
}

pub const fn generate_one_king_attacks(square: u64) -> u64 {
    let mut king_attacks: u64 = 0;

    if !utils::on_a_file(square) {
        king_attacks |= square >> 1;
        if !utils::on_rank_8(square) {
            king_attacks |= square << 7;
        }
        if !utils::on_rank_1(square) {
            king_attacks |= square >> 9;
        }
    }
    if !utils::on_h_file(square) {
        king_attacks |= square << 1;
        if !utils::on_rank_8(square) {
            king_attacks |= square << 9;
        }
        if !utils::on_rank_1(square) {
            king_attacks |= square >> 7;
        }
    }
    if !utils::on_rank_1(square) {
        king_attacks |= square >> 8;
    }
    if !utils::on_rank_8(square) {
        king_attacks |= square << 8;
    }

    king_attacks
}

pub const fn generate_king_attacks() -> [u64; 64] {
    let mut king_attacks = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let i_square: u64 = 1 << i;
        king_attacks[i] = generate_one_king_attacks(i_square);
        i += 1;
    }

    king_attacks
}

pub const KING_ATTACKS: [u64; 64] = generate_king_attacks();

// returns squares attacked by one king.
pub fn king_attacks(color: &bool, square: &u64, board: &board::ChessBoard) -> u64 {
    let mut king_attacks = KING_ATTACKS[square.trailing_zeros() as usize];

    if *color {
        king_attacks = king_attacks & !(board.white_pieces);
    } else {
        king_attacks = king_attacks & !(board.black_pieces);
    }
    king_attacks
}

pub fn board_attacks(board: &board::ChessBoard, color: bool) -> u64 {
    // The idea is to return a bitboard of all squares attacked BY specified color.
    let mut attacks: u64 = 0;

    let color_mask;
    if color {
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
                "pawns" => pawn_attacks(&color, &from_square, board),
                "knights" => knight_attacks(&color, &from_square, board),
                "bishops" => bishop_attacks(&color, &from_square, board),
                "rooks" => rook_attacks(&color, &from_square, board),
                "queens" => queen_attacks(&color, &from_square, board),
                "kings" => king_attacks(&color, &from_square, board),
                &_ => 0,
            };

            colored_bb = colored_bb & !(1 << colored_bb.trailing_zeros()); // erase current from_piece from colored_bb.
            attacks |= attack_squares;
        }
    }
    attacks
}

pub fn get_pawn_plmoves(board: &board::ChessBoard) -> Vec<u16> {
    let mut friendly_pawns: u64; // bitboard of pieces to check
    let to_move = &board.side_to_move;
    if board.side_to_move {
        friendly_pawns = board.white_pieces & board.pawns;
    } else {
        friendly_pawns = board.black_pieces & board.pawns;
    }

    let mut moves: Vec<u16> = Vec::new();

    // while there are still pawns to check, add moves for the next pawn
    while friendly_pawns != 0 {
        let this_piece_i: u8 = friendly_pawns.trailing_zeros() as u8; // next piece to check
        let this_piece_bb: u64 = 1 << this_piece_i;

        // Move Logic:
        // from attack squares
        let mut attack_sqs = pawn_attacks(to_move, &this_piece_bb, board);
        while attack_sqs != 0 {
            // while there are still attack squares to check for this pawn
            let this_target_i: u8 = attack_sqs.trailing_zeros() as u8;
            let this_target_bb: u64 = 1 << attack_sqs.trailing_zeros();

            // if white pawn
            if *to_move {
                let ep = board.en_passant;
                if this_target_bb == board.en_passant {
                    // if en passant
                    moves.push(utils::encode_move(this_piece_i, this_target_i, 3));
                } else if this_target_bb & board.black_pieces != 0 {
                    // if there is a capturable target
                    if utils::on_rank_7(this_piece_bb) {
                        // if promotion with capture
                        for flag in 8..=11 {
                            // one move for each promotion piece choice
                            moves.push(utils::encode_move(this_piece_i, this_target_i, flag));
                        }
                    } else {
                        // if normal capture
                        moves.push(utils::encode_move(this_piece_i, this_target_i, 1));
                    }
                }
            } else {
                // if black pawn
                if this_target_bb == board.en_passant {
                    // if en passant
                    moves.push(utils::encode_move(this_piece_i, this_target_i, 3));
                } else if this_target_bb & board.white_pieces != 0 {
                    if utils::on_rank_2(this_piece_bb) {
                        // if promotion with capture
                        for flag in 8..=11 {
                            // one move for each promotion piece choice
                            moves.push(utils::encode_move(this_piece_i, this_target_i, flag));
                        }
                    } else {
                        // if normal capture
                        moves.push(utils::encode_move(this_piece_i, this_target_i, 1));
                    }
                }
            }

            attack_sqs &= !this_target_bb;
        }

        if *to_move {
            // if white pawn
            // forward moves
            if board.piece_type_at(this_piece_i + 8).is_none() {
                if utils::on_rank_7(this_piece_bb) {
                    // promotion w/out capture
                    for flag in 4..=7 {
                        // one move for each promotion piece choice
                        moves.push(utils::encode_move(this_piece_i, this_piece_i + 8, flag));
                    }
                } else {
                    moves.push(utils::encode_move(this_piece_i, this_piece_i + 8, 0)); // forward 1 square
                    if utils::on_rank_2(this_piece_bb) {
                        // if pawn is on starting square
                        if board.piece_type_at(this_piece_i + 16).is_none() {
                            moves.push(utils::encode_move(this_piece_i, this_piece_i + 16, 0));
                            // forward 2 squares
                        }
                    }
                }
            }
        } else {
            // if black pawn
            // forward moves
            if board.piece_type_at(this_piece_i - 8).is_none() {
                if utils::on_rank_2(this_piece_bb) {
                    // promotion w/out capture
                    for flag in 4..=7 {
                        // one move for each promotion piece choice
                        moves.push(utils::encode_move(this_piece_i, this_piece_i - 8, flag));
                    }
                } else {
                    moves.push(utils::encode_move(this_piece_i, this_piece_i - 8, 0)); // forward 1 square
                    if utils::on_rank_7(this_piece_bb) {
                        // if pawn is on starting square
                        if board.piece_type_at(this_piece_i - 16).is_none() {
                            moves.push(utils::encode_move(this_piece_i, this_piece_i - 16, 0));
                            // forward 2 squares
                        }
                    }
                }
            }
        }

        friendly_pawns &= !this_piece_bb; // remove checked piece
    }

    return moves;
}

pub fn get_nonpk_plmoves(board: &board::ChessBoard) -> Vec<u16> {
    // generates pseudolegal moves for all non pawn/king pieces
    let mut moves: Vec<u16> = Vec::new();

    let to_move = board.side_to_move;
    let color_mask: u64;
    if board.side_to_move {
        color_mask = board.white_pieces;
    } else {
        color_mask = board.black_pieces;
    }

    for (i, piece_mask) in [board.knights, board.bishops, board.rooks, board.queens]
        .iter()
        .enumerate()
    {
        // for piece type
        let mut pieces: u64 = *piece_mask & color_mask;
        while pieces != 0 {
            // while there are pieces of this type left to check
            let this_piece_i: u8 = pieces.trailing_zeros() as u8; // next piece to check
            let this_piece_bb: u64 = 1 << this_piece_i;

            // create attack bb for this piece.
            let mut piece_attacks: u64;
            match i {
                0 => piece_attacks = knight_attacks(&to_move, &this_piece_bb, &board),
                1 => piece_attacks = bishop_attacks(&to_move, &this_piece_bb, &board),
                2 => piece_attacks = rook_attacks(&to_move, &this_piece_bb, &board),
                3 => piece_attacks = queen_attacks(&to_move, &this_piece_bb, &board),
                _ => piece_attacks = 0,
            }

            while piece_attacks != 0 {
                // while there are attack squares left to encode
                let this_target_i: u8 = piece_attacks.trailing_zeros() as u8;
                let this_target_bb: u64 = 1 << piece_attacks.trailing_zeros();

                if board.piece_type_at(this_target_i).is_some() {
                    // if capture
                    moves.push(utils::encode_move(this_piece_i, this_target_i, 1));
                } else {
                    // if normal move
                    moves.push(utils::encode_move(this_piece_i, this_target_i, 0));
                }

                piece_attacks &= !this_target_bb;
            }

            pieces &= !this_piece_bb; // remove checked piece
        }
    }

    return moves;
}

pub fn get_king_plmoves(board: &board::ChessBoard) -> Vec<u16> {
    let mut moves: Vec<u16> = Vec::new();
    let king_bb: u64;
    if board.side_to_move {
        king_bb = board.kings & board.white_pieces;
    } else {
        king_bb = board.kings & board.black_pieces;
    }
    let king_i: u8 = king_bb.trailing_zeros() as u8;

    // add moves based on attack squares
    let mut king_attacks = king_attacks(&board.side_to_move, &king_bb, &board);
    while king_attacks != 0 {
        // while there are attack squares left to encode
        let this_target_i: u8 = king_attacks.trailing_zeros() as u8;
        let this_target_bb: u64 = 1 << king_attacks.trailing_zeros();

        if board.piece_type_at(this_target_i).is_some() {
            // if capture
            moves.push(utils::encode_move(king_i, this_target_i, 1));
        } else {
            // if normal move
            moves.push(utils::encode_move(king_i, this_target_i, 0));
        }

        king_attacks &= !this_target_bb;
    }

    // add castling moves
    if board.side_to_move {
        if board.castling_rights & 0b1000 != 0 {
            // white kingside castle
            if board.piece_type_at(5).is_none() && board.piece_type_at(6).is_none() {
                moves.push(utils::encode_move(4, 6, 2));
            }
        }
        if board.castling_rights & 0b0100 != 0 {
            // white queenside castle
            if board.piece_type_at(1).is_none()
                && board.piece_type_at(2).is_none()
                && board.piece_type_at(3).is_none()
            {
                moves.push(utils::encode_move(4, 2, 2));
            }
        }
    } else {
        if board.castling_rights & 0b0010 != 0 {
            // black kingside castle
            if board.piece_type_at(61).is_none() && board.piece_type_at(62).is_none() {
                moves.push(utils::encode_move(60, 62, 2));
            }
        }
        if board.castling_rights & 0b0001 != 0 {
            // black queenside castle
            if board.piece_type_at(57).is_none()
                && board.piece_type_at(58).is_none()
                && board.piece_type_at(59).is_none()
            {
                moves.push(utils::encode_move(60, 58, 2));
            }
        }
    }

    return moves;
}

pub fn get_pseudolegal_moves(board: &board::ChessBoard) -> Vec<u16> {
    let pl_moves: Vec<u16> = [
        get_pawn_plmoves(board),
        get_nonpk_plmoves(board),
        get_king_plmoves(board),
    ]
    .concat();
    return pl_moves;
}

pub fn test_plmove_legality(board: &mut board::ChessBoard, move_i: u16) -> bool {
    // assumes the move given is ALREADY in the pseudolegal moves for the position.
    // will only check if a move results in check, if castling moves through check
    let to_move = board.side_to_move;

    // castling legality
    let move_decoded = utils::decode_move(move_i);
    if move_decoded[2] == 2 {
        // if flag == castle
        if is_check(board, to_move) {
            // can't castle out of check.
            return false;
        }

        let hostile_attacks = board_attacks(board, !board.side_to_move);
        let in_between_sqs: u64;
        match move_decoded[1] {
            // match the to_sq to determine which in-between squares to check (includes target sq)
            2 => in_between_sqs = 0xC,
            6 => in_between_sqs = 0x06,
            58 => in_between_sqs = 0x0C00000000000000,
            62 => in_between_sqs = 0x6000000000000000,
            _ => return false,
        }
        if hostile_attacks & in_between_sqs != 0 {
            return false;
        }
    }

    // regular legality
    let undo_info = board.make_move(move_i).unwrap();
    if is_check(board, to_move) {
        let _ = board.unmake_move(move_i, &undo_info);
        return false;
    } else {
        let _ = board.unmake_move(move_i, &undo_info);
        return true;
    }
}

pub fn get_legal_moves(board: &mut board::ChessBoard) -> Vec<u16> {
    let mut legal_moves = get_pseudolegal_moves(board);
    legal_moves.retain(|&m| test_plmove_legality(board, m));
    return legal_moves;
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
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
        let empty_board = board::ChessBoard::empty();
        // a1
        let square1 = utils::square_to_bb("a1").unwrap();
        let sq1_bishop_attacks = bishop_attacks(&true, &square1, &empty_board);
        assert_eq!(sq1_bishop_attacks, 0x8040201008040200);
        // // a8
        let square2 = utils::square_to_bb("a8").unwrap();
        let sq2_bishop_attacks = bishop_attacks(&true, &square2, &empty_board);
        assert_eq!(sq2_bishop_attacks, 0x0002040810204080);
        // // h1
        let square3 = utils::square_to_bb("h1").unwrap();
        let sq3_bishop_attacks = bishop_attacks(&true, &square3, &empty_board);
        assert_eq!(sq3_bishop_attacks, 0x0102040810204000);
        // // h8
        let square4 = utils::square_to_bb("h8").unwrap();
        let sq4_bishop_attacks = bishop_attacks(&true, &square4, &empty_board);
        assert_eq!(sq4_bishop_attacks, 0x0040201008040201);
        // // d4
        let square5 = utils::square_to_bb("d4").unwrap();
        let sq5_bishop_attacks = bishop_attacks(&true, &square5, &empty_board);
        assert_eq!(sq5_bishop_attacks, 0x8041221400142241);
    }

    #[test]
    fn test_rook_attacks() {
        let empty_board = board::ChessBoard::empty();
        // a1
        let square1 = utils::square_to_bb("a1").unwrap();
        let sq1_rook_attacks = rook_attacks(&true, &square1, &empty_board);
        assert_eq!(sq1_rook_attacks, 0x01010101010101FE);
        // a8
        let square2 = utils::square_to_bb("a8").unwrap();
        let sq2_rook_attacks = rook_attacks(&true, &square2, &empty_board);
        assert_eq!(sq2_rook_attacks, 0xFE01010101010101);
        // h1
        let square3 = utils::square_to_bb("h1").unwrap();
        let sq3_rook_attacks = rook_attacks(&true, &square3, &empty_board);
        assert_eq!(sq3_rook_attacks, 0x808080808080807F);
        // h8
        let square4 = utils::square_to_bb("h8").unwrap();
        let sq4_rook_attacks = rook_attacks(&true, &square4, &empty_board);
        assert_eq!(sq4_rook_attacks, 0x7F80808080808080);
        // d4
        let square5 = utils::square_to_bb("d4").unwrap();
        let sq5_rook_attacks = rook_attacks(&true, &square5, &empty_board);
        assert_eq!(sq5_rook_attacks, 0x08080808F7080808);
    }

    #[test]
    fn test_queen_attacks() {
        let empty_board = board::ChessBoard::empty();

        //a1
        let square1 = utils::square_to_bb("a1").unwrap();
        let sq1_queen_attacks = queen_attacks(&true, &square1, &empty_board);
        assert_eq!(sq1_queen_attacks, (0x01010101010101FE | 0x8040201008040200));
        // a8
        let square2 = utils::square_to_bb("a8").unwrap();
        let sq2_queen_attacks = queen_attacks(&true, &square2, &empty_board);
        assert_eq!(sq2_queen_attacks, (0xFE01010101010101 | 0x0002040810204080));
        // h1
        let square3 = utils::square_to_bb("h1").unwrap();
        let sq3_queen_attacks = queen_attacks(&true, &square3, &empty_board);
        assert_eq!(sq3_queen_attacks, (0x808080808080807F | 0x0102040810204000));
        // h8
        let square4 = utils::square_to_bb("h8").unwrap();
        let sq4_queen_attacks = queen_attacks(&true, &square4, &empty_board);
        assert_eq!(sq4_queen_attacks, (0x7F80808080808080 | 0x0040201008040201));
        // d4
        let square5 = utils::square_to_bb("d4").unwrap();
        let sq5_queen_attacks = queen_attacks(&true, &square5, &empty_board);
        assert_eq!(sq5_queen_attacks, (0x08080808F7080808 | 0x8041221400142241));
    }

    #[test]
    fn test_king_attacks() {
        let board1 = board::ChessBoard::initialize();
        // white king starting position
        let square1 = utils::square_to_bb("e1").unwrap();
        let sq1_king_attacks = king_attacks(&true, &square1, &board1);
        assert_eq!(sq1_king_attacks, 0);
        // black king starting position
        let square2 = utils::square_to_bb("e8").unwrap();
        let sq2_king_attacks = king_attacks(&false, &square2, &board1);
        assert_eq!(sq2_king_attacks, 0);

        let board2 = board::ChessBoard::empty();
        // white king a1 empty board
        let square3 = utils::square_to_bb("a1").unwrap();
        let sq3_king_attacks = king_attacks(&true, &square3, &board2);
        assert_eq!(sq3_king_attacks, 0x0000000000000302);
        // black king a1 empty board
        let square3 = utils::square_to_bb("a1").unwrap();
        let sq3_king_attacks = king_attacks(&false, &square3, &board2);
        assert_eq!(sq3_king_attacks, 0x0000000000000302);
    }

    #[test]
    fn test_board_attacks() {
        // 2 pieces of each type (except king), black to move
        let board1 =
            board::ChessBoard::initialize_from_fen("rnbq1p2/rnbq1pk1/8/8/8/8/8/K7 b KQkq - 0 1")
                .unwrap();
        let board_attacks_test = board_attacks(&board1, false);
        let board_attacks_correct: u64 = 0xD090FF7FE9C98909;
        assert_eq!(board_attacks_test, board_attacks_correct);

        // 2 pieces of each type (except king), white to move
        let board2 =
            board::ChessBoard::initialize_from_fen("k7/8/8/8/8/8/RNBQ1PK1/RNBQ1P2 w KQkq - 0 1")
                .unwrap();
        let board_attacks_test = board_attacks(&board2, true);
        let board_attacks_correct: u64 = 0x0989C9E97FFF90D0;
        assert_eq!(board_attacks_test, board_attacks_correct);
    }

    #[test]
    fn test_ray_generation() {
        let square1 = utils::square_to_bb("e4").unwrap();
        let rays: [u64; 8] = [
            generate_north_ray(square1),
            generate_ne_ray(square1),
            generate_east_ray(square1),
            generate_se_ray(square1),
            generate_south_ray(square1),
            generate_sw_ray(square1),
            generate_west_ray(square1),
            generate_nw_ray(square1),
        ];

        assert_eq!(
            rays,
            [
                0x1010101000000000,
                0x0080402000000000,
                0x00000000E0000000,
                0x0000000000204080,
                0x0000000000101010,
                0x0000000000080402,
                0x000000000F000000,
                0x0102040800000000
            ]
        );
    }

    #[test]
    fn test_check_along_ray() {
        // east ray from e4, friendly piece on g4.
        let square1 = utils::square_to_bb("e4").unwrap();
        let e_ray = generate_east_ray(square1);
        let friendly_pieces = 0x0000000040000000;
        let enemy_pieces = 0;
        assert_eq!(
            check_along_ray(square1, e_ray, friendly_pieces, enemy_pieces),
            0x0000000020000000
        );

        // east ray from e4, enemy piece on g4.
        let enemy_pieces = 0x0000000040000000;
        let friendly_pieces = 0;
        assert_eq!(
            check_along_ray(square1, e_ray, friendly_pieces, enemy_pieces),
            0x0000000060000000
        );

        // west ray from e4, friendly piece on a4.
        let w_ray = generate_west_ray(square1);
        let friendly_pieces = 0x0000000001000000;
        let enemy_pieces = 0;
        assert_eq!(
            check_along_ray(square1, w_ray, friendly_pieces, enemy_pieces),
            0x000000000E000000
        );

        // west ray from e4, enemy piece on a4.
        let enemy_pieces = 0x0000000001000000;
        let friendly_pieces = 0;
        assert_eq!(
            check_along_ray(square1, w_ray, friendly_pieces, enemy_pieces),
            0x000000000F000000
        );

        // ne ray from e4, enemy piece on g6
        let ne_ray = generate_ne_ray(square1);
        let enemy_pieces = 0x0000400000000000;
        let friendly_pieces = 0;
        assert_eq!(
            check_along_ray(square1, ne_ray, friendly_pieces, enemy_pieces),
            0x0000402000000000
        );

        // nw ray from e4, friendly piece on a8;
        let nw_ray = generate_nw_ray(square1);
        let enemy_pieces = 0;
        let friendly_pieces = 0x0100000000000000;
        assert_eq!(
            check_along_ray(square1, nw_ray, friendly_pieces, enemy_pieces),
            0x0002040800000000
        );

        // se ray from e4, enemy piece on g2
        let se_ray = generate_se_ray(square1);
        let enemy_pieces = 0x0000000000004000;
        let friendly_pieces = 0;
        assert_eq!(
            check_along_ray(square1, se_ray, friendly_pieces, enemy_pieces),
            0x0000000000204000
        );

        // sw ray from e4, friendly piece on b1
        let sw_ray = generate_sw_ray(square1);
        let enemy_pieces = 0;
        let friendly_pieces = 0x0000000000000002;
        assert_eq!(
            check_along_ray(square1, sw_ray, friendly_pieces, enemy_pieces),
            0x0000000000080400
        );
    }

    #[test]
    fn test_get_pawn_plmoves() {
        let board1 = ChessBoard::initialize(); // white in starting position
        let mut correct_pawn_plmoves: Vec<u16> = Vec::new();

        for from_sqi in 8..=15 {
            correct_pawn_plmoves.push(utils::encode_move(from_sqi, from_sqi + 8, 0));
            correct_pawn_plmoves.push(utils::encode_move(from_sqi, from_sqi + 16, 0));
        }
        correct_pawn_plmoves.sort();

        let mut pawn_pl_moves = get_pawn_plmoves(&board1);
        pawn_pl_moves.sort();

        assert_eq!(pawn_pl_moves, correct_pawn_plmoves);

        let board2 = ChessBoard::initialize_from_fen("k7/8/8/8/8/8/6p1/K6N b - - 0 1").unwrap(); // black promotions test
        let mut correct_pawn_plmoves: Vec<u16> = Vec::new();

        for flag in 8..=11 {
            correct_pawn_plmoves.push(utils::encode_move(14, 7, flag));
        }
        for flag in 4..=7 {
            correct_pawn_plmoves.push(utils::encode_move(14, 6, flag));
        }
        correct_pawn_plmoves.sort();

        let mut pawn_pl_moves = get_pawn_plmoves(&board2);
        pawn_pl_moves.sort();

        assert_eq!(pawn_pl_moves, correct_pawn_plmoves);

        let board3 = ChessBoard::initialize_from_fen("k7/8/8/4pP2/8/8/8/K7 w - e6 0 2").unwrap(); // white en passant
        let mut correct_pawn_plmoves: Vec<u16> =
            vec![utils::encode_move(37, 45, 0), utils::encode_move(37, 44, 3)];
        correct_pawn_plmoves.sort();

        let mut pawn_pl_moves = get_pawn_plmoves(&board3);
        pawn_pl_moves.sort();

        assert_eq!(pawn_pl_moves, correct_pawn_plmoves);
    }

    #[test]
    fn test_get_nonpk_plmoves() {
        let board1 = ChessBoard::initialize(); // white in starting position
        let mut correct_nonpk_plmoves: Vec<u16> = vec![
            utils::encode_move(1, 16, 0),
            utils::encode_move(1, 18, 0),
            utils::encode_move(6, 23, 0),
            utils::encode_move(6, 21, 0),
        ];
        correct_nonpk_plmoves.sort();

        let mut nonpk_pl_moves = get_nonpk_plmoves(&board1);
        nonpk_pl_moves.sort();

        assert_eq!(nonpk_pl_moves, correct_nonpk_plmoves);

        let board2 =
            ChessBoard::initialize_from_fen("1q2k1r1/Ppp4b/8/5Pp1/4p3/8/8/K7 b - - 0 1").unwrap(); // black in random position
        let mut correct_nonpk_plmoves: Vec<u16> = vec![
            utils::encode_move(57, 56, 0), // 4 queen moves
            utils::encode_move(57, 58, 0),
            utils::encode_move(57, 59, 0),
            utils::encode_move(57, 48, 1),
            utils::encode_move(62, 61, 0), // 4 rook moves
            utils::encode_move(62, 63, 0),
            utils::encode_move(62, 54, 0),
            utils::encode_move(62, 46, 0),
            utils::encode_move(55, 46, 0), // 2 bishop moves
            utils::encode_move(55, 37, 1),
        ];
        correct_nonpk_plmoves.sort();

        let mut nonpk_pl_moves = get_nonpk_plmoves(&board2);
        nonpk_pl_moves.sort();

        assert_eq!(nonpk_pl_moves, correct_nonpk_plmoves);
    }

    #[test]
    fn test_get_king_plmoves() {
        let mut board1 = ChessBoard::initialize_from_fen(
            "r3kbnr/pppP1ppp/4p3/8/8/4P3/PPPp1PPP/RNBQK2R w KQkq - 0 1",
        )
        .unwrap();
        let mut correct_king_plmoves: Vec<u16> = vec![
            utils::encode_move(4, 12, 0),
            utils::encode_move(4, 5, 0),
            utils::encode_move(4, 11, 1),
            utils::encode_move(4, 6, 2),
        ];
        correct_king_plmoves.sort();

        let mut king_pl_moves = get_king_plmoves(&board1);
        king_pl_moves.sort();

        assert_eq!(king_pl_moves, correct_king_plmoves);

        board1.make_move(utils::encode_move(8, 16, 0));
        let mut correct_king_plmoves: Vec<u16> = vec![
            utils::encode_move(60, 59, 0),
            utils::encode_move(60, 52, 0),
            utils::encode_move(60, 51, 1),
            utils::encode_move(60, 58, 2),
        ];
        correct_king_plmoves.sort();

        let mut king_pl_moves = get_king_plmoves(&board1);
        king_pl_moves.sort();

        assert_eq!(king_pl_moves, correct_king_plmoves);
    }

    #[test]
    fn test_test_plmove_legality() {
        let mut board = board::ChessBoard::initialize_from_fen(
            "r3kbnr/pppP1ppp/4p3/8/8/4P3/PPPp1PPP/RNBQK2R w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(
            test_plmove_legality(&mut board, encode_move(4, 6, 2)),
            false
        );

        let mut board = board::ChessBoard::initialize_from_fen(
            "r3kbnr/pppP1ppp/4p3/8/8/4P3/PPP2PPP/RNBQK2R w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(test_plmove_legality(&mut board, encode_move(4, 6, 2)), true);

        let mut board = board::ChessBoard::initialize_from_fen(
            "r3kbnr/pppP1ppp/4p3/8/8/4P3/PPP2PPP/RNBQK2R b KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(
            test_plmove_legality(&mut board, encode_move(60, 58, 2)),
            false
        );

        let mut board = board::ChessBoard::initialize_from_fen(
            "r3kbnr/ppp2ppp/3p4/8/8/4P3/PPP2PPP/RNBQK2R b KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(
            test_plmove_legality(&mut board, encode_move(60, 58, 2)),
            true
        );

        let mut board = board::ChessBoard::initialize_from_fen(
            "rnb1kb1r/ppppqppp/8/5n2/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(
            test_plmove_legality(&mut board, encode_move(28, 37, 1)),
            false
        );
    }

    #[test]
    fn test_get_legal_moves() {
        let mut board = ChessBoard::initialize();

        let mut correct_legal_moves: Vec<u16> = Vec::new();
        for from_sqi in 8..=15 {
            correct_legal_moves.push(utils::encode_move(from_sqi, from_sqi + 8, 0));
            correct_legal_moves.push(utils::encode_move(from_sqi, from_sqi + 16, 0));
        }
        correct_legal_moves.push(utils::encode_move(1, 16, 0));
        correct_legal_moves.push(utils::encode_move(1, 18, 0));
        correct_legal_moves.push(utils::encode_move(6, 23, 0));
        correct_legal_moves.push(utils::encode_move(6, 21, 0));
        correct_legal_moves.sort();

        let mut legal_moves = get_legal_moves(&mut board);
        legal_moves.sort();

        assert_eq!(legal_moves, correct_legal_moves);
    }
}
