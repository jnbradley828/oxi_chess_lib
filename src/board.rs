use crate::{moves, utils};

/// Struct representing a chess board.
/// We will let the least significant bit represent the a1 square.
#[derive(Debug, PartialEq)]
pub struct ChessBoard {
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,
    pub white_pieces: u64,
    pub black_pieces: u64,
    pub side_to_move: bool,   // let true = white to move
    pub en_passant: u64, // let u64 = en passant target (location of capture square). Let value = 0 if no en passant is possible.
    pub castling_rights: u8, // uses 4 least significant bits (from most sig to least sig: white kingside, white queenside, black kingside, black queenside)
    pub halfmove_clock: u8,  // tracks half moves since last capture or pawn move.
    pub fullmove_number: u16, // tracks full moves since start of game.

    pub white_attacks: u64,
    pub black_attacks: u64,
}

#[derive(Debug, PartialEq)]
pub struct UndoInfo {
    halfmove_clock: u8,
    castling_rights: u8,
    en_passant_square: Option<u8>,
    captured_type: Option<u8>, // let (pawn, knight, bishop, rook, queen) = (0, 1, 2, 3, 4)
}

/// Creates a new chess board with the standard starting position.
impl ChessBoard {
    pub fn initialize() -> Self {
        let mut board = ChessBoard {
            pawns: 0x00FF00000000FF00,   // pawns at ranks 2 & 7.
            knights: 0x4200000000000042, // knights at b1, g1, b8, & g8.
            bishops: 0x2400000000000024, // bishops at c1, f1, c8, & f8.
            rooks: 0x8100000000000081,   // rooks at a1, h1, a8, & h8.
            queens: 0x0800000000000008,  // queens at d1 & d8.
            kings: 0x1000000000000010,   // kings at e1 & e8.

            white_pieces: 0x000000000000FFFF,
            black_pieces: 0xFFFF000000000000,

            side_to_move: true,
            en_passant: 0,
            castling_rights: 0b1111,
            halfmove_clock: 0,
            fullmove_number: 1,

            white_attacks: 0,
            black_attacks: 0,
        };

        board.white_attacks = moves::board_attacks(&board, true);
        board.black_attacks = moves::board_attacks(&board, false);
        board
    }

    /// Creates a new empty chess board. White to move by default.
    pub fn empty() -> Self {
        Self {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,

            white_pieces: 0,
            black_pieces: 0,

            side_to_move: true, // white to move by default.
            en_passant: 0,
            castling_rights: 0,
            halfmove_clock: 0,
            fullmove_number: 1,

            white_attacks: 0,
            black_attacks: 0,
        }
    }

    /// Creates a new chess board from a FEN string.
    pub fn initialize_from_fen(fen: &str) -> Result<Self, String> {
        if !verify_fen(&fen) {
            return Err("Invalid FEN string.".to_string());
        } else {
            let fen_components: Vec<&str> = fen.split_whitespace().collect();

            // parse fen_components[0] (piece locations).
            let ranks: Vec<&str> = fen_components[0].split('/').collect();
            let mut square = 0;

            let mut pawns1: u64 = 0;
            let mut knights1: u64 = 0;
            let mut bishops1: u64 = 0;
            let mut rooks1: u64 = 0;
            let mut queens1: u64 = 0;
            let mut kings1: u64 = 0;

            let mut white_pieces1: u64 = 0;
            let mut black_pieces1: u64 = 0;

            for rank in ranks {
                for ch in rank.chars().rev() {
                    // increment by 1 for alphabetical values and set piece_type[bit] = 1,
                    // increment by numerical for numberical values
                    if ch.is_ascii_alphabetic() {
                        match ch {
                            'k' | 'K' => kings1 |= 1 << (63 - square),
                            'q' | 'Q' => queens1 |= 1 << (63 - square),
                            'r' | 'R' => rooks1 |= 1 << (63 - square),
                            'b' | 'B' => bishops1 |= 1 << (63 - square),
                            'n' | 'N' => knights1 |= 1 << (63 - square),
                            'p' | 'P' => pawns1 |= 1 << (63 - square),
                            _ => (),
                        }
                        match ch.is_uppercase() {
                            true => white_pieces1 |= 1 << (63 - square),
                            false => black_pieces1 |= 1 << (63 - square),
                        }
                        square += 1;
                    } else {
                        square += ch.to_digit(10).unwrap();
                    }
                }
            }

            // parse fen_components[2] (castling rights)
            let mut castling_rights1: u8 = 0;
            for ch in fen_components[2].chars() {
                match ch {
                    'K' => castling_rights1 |= 1 << 3,
                    'Q' => castling_rights1 |= 1 << 2,
                    'k' => castling_rights1 |= 1 << 1,
                    'q' => castling_rights1 |= 1 << 0,
                    _ => (),
                }
            }

            // parse fen_components[3] (en passant target square)
            let ep_square: u64;
            if fen_components[3].len() == 2 {
                ep_square = utils::square_to_bb(&fen_components[3]).unwrap();
            } else if fen_components[3] == '-'.to_string() {
                ep_square = 0;
            } else {
                return Err("Invalid en passant square".to_string());
            }

            let mut board = ChessBoard {
                pawns: pawns1,
                knights: knights1,
                bishops: bishops1,
                rooks: rooks1,
                queens: queens1,
                kings: kings1,

                white_pieces: white_pieces1,
                black_pieces: black_pieces1,

                // parse fen_components[1] (color to move)
                side_to_move: match fen_components[1] {
                    "w" => true,
                    _ => false,
                },
                en_passant: ep_square,
                castling_rights: castling_rights1,
                halfmove_clock: fen_components[4].parse::<u8>().unwrap(),
                fullmove_number: fen_components[5].parse::<u16>().unwrap(),
                white_attacks: 0,
                black_attacks: 0,
            };
            board.white_attacks = moves::board_attacks(&board, true);
            board.black_attacks = moves::board_attacks(&board, false);

            return Ok(board);
        }
    }

    // let (pawn, knight, bishop, rook, queen, king) = (0, 1, 2, 3, 4, 5)
    #[rustfmt::skip]
    pub fn piece_type_at(&self, sq_i: u8) -> Option<u8> {
        let sq_bb: u64 = 1 << sq_i;

        if sq_bb & self.pawns != 0 { return Some(0) };
        if sq_bb & self.knights != 0 { return Some(1) };
        if sq_bb & self.bishops != 0 { return Some(2) };
        if sq_bb & self.rooks != 0 { return Some(3) };
        if sq_bb & self.queens != 0 { return Some(4) };
        if sq_bb & self.kings != 0 { return Some(5) };
        return None;
    }

    // move: u16, most significant 6 digits = from square, next 6 = to_square, least sig 4 digits = move type flag
    // move type flags: normal = 0, capture = 1, castle = 2, en passant = 3, promotion (n,b,r,q) = (4,5,6,7) respectively, promo w/ capture (n,b,r,q) = (8,9,10,11) respectively
    pub fn make_move(&mut self, move_int: u16) -> Result<UndoInfo, String> {
        let from_sqi = (move_int >> 10) as u8;
        let to_sqi = ((move_int >> 4) & 0b111111) as u8;
        let flag = (move_int & 0b1111) as u8;

        // collect unmake move data
        let prev_halfmove_clock = self.halfmove_clock;
        let prev_castling_rights = self.castling_rights;
        let prev_en_passant: Option<u8>;
        let capture_type: Option<u8>;

        if matches!(flag, 1 | 8 | 9 | 10 | 11) {
            // if move flag is a capture
            capture_type = self.piece_type_at(to_sqi);
        } else if flag == 3 {
            // if move flag is en passant
            capture_type = Some(self.en_passant.trailing_zeros() as u8);
        } else {
            capture_type = None;
        }

        if self.en_passant == 0 {
            prev_en_passant = None;
        } else {
            prev_en_passant = Some(self.en_passant.trailing_zeros() as u8);
        }

        let undo_info = UndoInfo {
            halfmove_clock: prev_halfmove_clock,
            castling_rights: prev_castling_rights,
            en_passant_square: prev_en_passant,
            captured_type: capture_type,
        };

        // check if there is a piece of the color to move at the given square.
        let from_sq_bb: u64 = 1 << from_sqi;
        let to_sq_bb: u64 = 1 << to_sqi;
        let piece_from_type = self.piece_type_at(from_sqi);

        if piece_from_type.is_none() {
            return Err("No piece at given square.".to_string());
        } else {
            if self.side_to_move {
                if self.white_pieces & from_sq_bb == 0 {
                    return Err("Black piece cannot move on white's turn.".to_string());
                }
            } else {
                if self.black_pieces & from_sq_bb == 0 {
                    return Err("White piece cannot move on black's turn.".to_string());
                }
            }
        }

        // update board state: piece locations, en_passant, castling rights, and halfmove clock.
        if piece_from_type == Some(0) {
            // pawn
            self.pawns &= !from_sq_bb;
            self.pawns |= to_sq_bb;

            self.halfmove_clock = 0;
            if ((from_sqi as i16) - (to_sqi as i16)).abs() == 16 {
                if self.side_to_move {
                    self.en_passant = to_sq_bb >> 8;
                } else {
                    self.en_passant = to_sq_bb << 8;
                }
            }
        } else if piece_from_type == Some(1) {
            // knight
            self.knights &= !from_sq_bb;
            self.knights |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
        } else if piece_from_type == Some(2) {
            self.bishops &= !from_sq_bb;
            self.bishops |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
        } else if piece_from_type == Some(3) {
            self.rooks &= !from_sq_bb;
            self.rooks |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
        } else if piece_from_type == Some(4) {
            self.queens &= !from_sq_bb;
            self.queens |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
        } else {
            self.kings &= !from_sq_bb;
            self.kings |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
            if self.side_to_move {
                self.castling_rights &= !0b1100;
            } else {
                self.castling_rights &= !0b0011;
            }
        }
        if capture_type.is_some() {
            self.halfmove_clock = 0;
            match capture_type {
                Some(0) => self.pawns &= !to_sq_bb,
                Some(1) => self.knights &= !to_sq_bb,
                Some(2) => self.bishops &= !to_sq_bb,
                Some(3) => self.rooks &= !to_sq_bb,
                Some(4) => self.queens &= !to_sq_bb,
                _ => {}
            }
            if self.side_to_move {
                self.black_pieces &= !to_sq_bb;
            } else {
                self.white_pieces &= !to_sq_bb;
            }
        }

        if self.side_to_move {
            self.white_pieces &= !from_sq_bb;
            self.white_pieces |= to_sq_bb;
            self.side_to_move = false;
        } else {
            self.black_pieces &= !from_sq_bb;
            self.black_pieces |= to_sq_bb;
            self.side_to_move = true;
            self.fullmove_number += 1;
        }

        self.white_attacks = moves::board_attacks(&self, true);
        self.black_attacks = moves::board_attacks(&self, false);

        return Ok(undo_info);

        todo!("implement en passant, castling, and promotion board state changes. Implement castling rights being taken away from rook moves & castling. Test these changes.")
    }

    pub fn unmake_move(&mut self, move_int: &u16) {
        todo!("this fn is to unmake a move made with make_move() using the unmake move data.")
    }
}

/// Verifies if a string is a valid FEN string.
pub fn verify_fen(fen: &str) -> bool {
    // Split whitespace and create vector of fen components.
    let fen_components: Vec<&str> = fen.split_whitespace().collect();
    // fen_components indices:
    //      [0] = board position
    //      [1] = color to move
    //      [2] = castling rights
    //      [3] = en passant target square
    //      [4] = halfmove clock
    //      [5] = fullmove number
    // Steps to verify:
    // Piece locations:
    //      Split by '/' characters. If len != 8, return false.

    let ranks: Vec<&str> = fen_components[0].split('/').collect();
    if ranks.len() != 8 {
        return false;
    }

    //      For each line, take count of alphabetical characters and add value of digits. If for any line, sum != 8, return false.
    //      For efficiency, only check if characters are in "PNBRQKpnbrqk12345678".
    //      Check for 1 white and 1 black king while we're at it.
    let mut white_king_count: u8 = 0;
    let mut black_king_count: u8 = 0;

    for rank in &ranks {
        let mut square_count = 0;

        for ch in rank.chars() {
            if "kqrbnpKQRBNP".contains(ch) {
                square_count += 1;
                if ch == 'K' {
                    white_king_count += 1;
                } else if ch == 'k' {
                    black_king_count += 1;
                }
            } else if ch >= '1' && ch <= '8' {
                square_count += ch.to_digit(10).unwrap();
            } else {
                return false;
            }
        }
        if square_count != 8 {
            return false;
        }
    }

    if white_king_count != 1 {
        return false;
    }
    if black_king_count != 1 {
        return false;
    }

    // Color to move:
    //      If not in "wb" and length 1, return false.
    if !"wb".contains(fen_components[1]) || fen_components[1].len() != 1 {
        return false;
    }

    // Castling rights:
    //      If not in "KQkq-" or not len 1..4 return false.
    if !"KQkq-".contains(fen_components[2]) || fen_components[2].len() > 4 {
        return false;
    }
    //      Castling rights cannot be both empty and not empty.
    if fen_components[2].contains("-") && fen_components[2] != "-" {
        return false;
    }

    // En passant target square:
    //      If len > 2, return false.
    if fen_components[3].len() > 2 {
        return false;
    }
    //      If not '-':
    //          If first char not "a".."h" or second char not '6' or '3':
    //              return false.
    if fen_components[3] != "-" {
        let mut en_passant_chars = fen_components[3].chars();
        let en_passant_char1 = en_passant_chars.next().unwrap(); // safe to unwrap because the component cannot be empty (white space erased, each component has len > 1.)
        if en_passant_char1 < 'a' || en_passant_char1 > 'h' {
            return false;
        }
        // Use Some() in case there is no second char, it returns false.
        if let Some(en_passant_char2) = en_passant_chars.next() {
            if en_passant_char2 != '6' && en_passant_char2 != '3' {
                return false;
            }
        } else {
            return false;
        }
    }

    // Halfmove clock:
    //      If not a non-negative integer, return false.
    if !fen_components[4].parse::<u16>().is_ok() {
        return false;
    }
    // Fullmove number:
    //      If not a positive integer, return false.
    if !fen_components[5].parse::<u16>().is_ok() || fen_components[5] == "0" {
        return false;
    }

    // Finally: return true.
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_board() {
        let board = ChessBoard::initialize();
        assert_eq!(board.pawns, 0x00FF00000000FF00);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.kings, 0x1000000000000010);
        assert_eq!(board.white_pieces, 0x000000000000FFFF);
        assert_eq!(board.black_pieces, 0xFFFF000000000000);
        assert_eq!(board.side_to_move, true);
        assert_eq!(board.en_passant, 0);
        assert_eq!(board.castling_rights, 0b1111);
        assert_eq!(board.halfmove_clock, 0);
        assert_eq!(board.fullmove_number, 1);
    }

    #[test]
    fn test_empty_board() {
        let board = ChessBoard::empty();
        assert_eq!(board.pawns, 0);
        assert_eq!(board.knights, 0);
        assert_eq!(board.bishops, 0);
        assert_eq!(board.rooks, 0);
        assert_eq!(board.queens, 0);
        assert_eq!(board.kings, 0);
        assert_eq!(board.white_pieces, 0);
        assert_eq!(board.black_pieces, 0);
        assert_eq!(board.side_to_move, true);
        assert_eq!(board.en_passant, 0);
        assert_eq!(board.castling_rights, 0);
        assert_eq!(board.halfmove_clock, 0);
        assert_eq!(board.fullmove_number, 1);
    }

    #[test]
    fn test_verify_fen() {
        let starting_fen = verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"); // Starting position.
        assert_eq!(starting_fen, true);

        let empty_fen = verify_fen("8/8/8/8/8/8/8/8 w - - 0 1"); // No kings.
        assert_eq!(empty_fen, false);

        let random_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R1BQR1K1 w - - 1 18"); // Valid FEN of a middlegame.
        assert_eq!(random_fen, true);

        let too_many_ranks_false_fen = verify_fen(
            "1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R1BQR1K1/1rbq1rk1 w - - 1 18", // 9 ranks != 8.
        );
        assert_eq!(too_many_ranks_false_fen, false);

        let too_few_ranks_false_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1 w - - 1 18"); // 7 ranks != 7.
        assert_eq!(too_few_ranks_false_fen, false);

        let too_many_squares_false_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R2BQR1K1 w - - 1 18"); // 1st rank has too many squares.
        assert_eq!(too_many_squares_false_fen, false);

        let too_few_squares_false_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/RBQR1K1 w - - 1 18"); // 1st rank has too few squares.
        assert_eq!(too_few_squares_false_fen, false);

        let wrong_characters_false_fen =
            verify_fen("1rbq1ak1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R1BQR1K1 w - - 1 18"); // 8th rank contains the char 'a'.
        assert_eq!(wrong_characters_false_fen, false);

        let invalid_color_to_move_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR v KQkq - 0 1"); // Starting position with invalid color to move - 'v'.
        assert_eq!(invalid_color_to_move_false_fen, false);

        let color_to_move_len_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR ww KQkq - 0 1"); // Starting position with invalid color to move - 'ww'.
        assert_eq!(color_to_move_len_false_fen, false);

        let castle_rights_len_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkqK - 0 1"); // Castling rights too long.
        assert_eq!(castle_rights_len_false_fen, false);

        let castle_invalid_char_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQka - 0 1"); // Invalid character in castling rights.
        assert_eq!(castle_invalid_char_false_fen, false);

        let castling_contradiction_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w -K - 0 1");
        assert_eq!(castling_contradiction_false_fen, false);

        let invalid_len_en_passant_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a33 0 1");
        assert_eq!(invalid_len_en_passant_false_fen, false);

        let invalid_char_en_passant_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq i3 0 1");
        assert_eq!(invalid_char_en_passant_false_fen, false);

        let invalid_num_en_passant_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a2 0 1");
        assert_eq!(invalid_num_en_passant_false_fen, false);

        let invalid_halfmove_negative_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1");
        assert_eq!(invalid_halfmove_negative_false_fen, false);

        let invalid_halfmove_char_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - a 1");
        assert_eq!(invalid_halfmove_char_false_fen, false);

        let invalid_fullmove_negative_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 -1");
        assert_eq!(invalid_fullmove_negative_false_fen, false);

        let invalid_fullmove_zero_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0");
        assert_eq!(invalid_fullmove_zero_false_fen, false);

        let invalid_fullmove_char_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 a");
        assert_eq!(invalid_fullmove_char_false_fen, false);

        let random_fen2 =
            verify_fen("rnbqk2r/pp2nppp/2pbp3/3p4/3P4/1P1BPN2/PBP2PPP/RN1QK2R b KQkq - 2 6");
        assert_eq!(random_fen2, true);
    }

    #[test]
    fn test_intialize_from_fen() {
        let starting_board = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(starting_board.pawns, 0x00FF00000000FF00);
        assert_eq!(starting_board.knights, 0x4200000000000042);
        assert_eq!(starting_board.bishops, 0x2400000000000024);
        assert_eq!(starting_board.rooks, 0x8100000000000081);
        assert_eq!(starting_board.queens, 0x0800000000000008);
        assert_eq!(starting_board.kings, 0x1000000000000010);
        assert_eq!(starting_board.white_pieces, 0x000000000000FFFF);
        assert_eq!(starting_board.black_pieces, 0xFFFF000000000000);
        assert_eq!(starting_board.side_to_move, true);
        assert_eq!(starting_board.en_passant, 0);
        assert_eq!(starting_board.castling_rights, 0b1111);
        assert_eq!(starting_board.halfmove_clock, 0);
        assert_eq!(starting_board.fullmove_number, 1);

        let board1 = ChessBoard::initialize_from_fen(
            "r2qkbnr/1p2pppp/p1n5/3p4/3P1B2/3Q1N2/PPP2PPP/RN2K1R1 b Qkq - 2 8",
        )
        .unwrap();
        assert_eq!(
            board1.pawns,
            0b0000000011110010000000010000100000001000000000001110011100000000
        );
        assert_eq!(
            board1.knights,
            0b0100000000000000000001000000000000000000001000000000000000000010
        );
        assert_eq!(
            board1.bishops,
            0b0010000000000000000000000000000000100000000000000000000000000000
        );
        assert_eq!(
            board1.rooks,
            0b1000000100000000000000000000000000000000000000000000000001000001
        );
        assert_eq!(
            board1.queens,
            0b0000100000000000000000000000000000000000000010000000000000000000
        );
        assert_eq!(
            board1.kings,
            0b0001000000000000000000000000000000000000000000000000000000010000
        );
        assert_eq!(
            board1.white_pieces,
            0b0000000000000000000000000000000000101000001010001110011101010011
        );
        assert_eq!(
            board1.black_pieces,
            0b1111100111110010000001010000100000000000000000000000000000000000
        );
        assert_eq!(board1.side_to_move, false);
        assert_eq!(board1.en_passant, 0);
        assert_eq!(board1.castling_rights, 0b0111);
        assert_eq!(board1.halfmove_clock, 2);
        assert_eq!(board1.fullmove_number, 8);

        let board2 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/6pP/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1",
        )
        .unwrap();
        assert_eq!(
            board2.pawns,
            0b0000000011111111000000000000000011000000000000000111111100000000
        );
        assert_eq!(board2.knights, 0x4200000000000042);
        assert_eq!(board2.bishops, 0x2400000000000024);
        assert_eq!(board2.rooks, 0x8100000000000081);
        assert_eq!(board2.queens, 0x0800000000000008);
        assert_eq!(board2.kings, 0x1000000000000010);
        assert_eq!(
            board2.white_pieces,
            0b0000000000000000000000000000000010000000000000000111111111111111
        );
        assert_eq!(
            board2.black_pieces,
            0b1111111111111111000000000000000001000000000000000000000000000000
        );
        assert_eq!(board2.side_to_move, false);
        assert_eq!(
            board2.en_passant,
            0b0000000000000000000000000000000000000000100000000000000000000000
        );
        assert_eq!(board2.castling_rights, 0b1111);
        assert_eq!(board2.halfmove_clock, 0);
        assert_eq!(board2.fullmove_number, 1);
    }
}

#[test]
fn test_piece_type_at() {
    let board1 = ChessBoard::initialize();
    let mut type_array: [Option<u8>; 64] = [None; 64];
    for i in 0..64 {
        let piece_type = board1.piece_type_at(i);
        type_array[i as usize] = piece_type;
    }
    let correct_type_array: [Option<u8>; 64] = [
        Some(3),
        Some(1),
        Some(2),
        Some(4),
        Some(5),
        Some(2),
        Some(1),
        Some(3),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(3),
        Some(1),
        Some(2),
        Some(4),
        Some(5),
        Some(2),
        Some(1),
        Some(3),
    ];

    assert_eq!(type_array, correct_type_array);
}

#[test]
pub fn test_make_move() {
    let mut board1 = ChessBoard::initialize();

    let move1_int = 0b0011000111000000; // e2-e4
    let move1_undo = board1.make_move(move1_int);

    let correct_resulting_board1 = ChessBoard::initialize_from_fen(
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
    )
    .unwrap();
    let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
        halfmove_clock: 0,
        castling_rights: 0b1111,
        en_passant_square: None,
        captured_type: None,
    });

    assert_eq!(board1, correct_resulting_board1);
    assert_eq!(move1_undo, correct_undo1);

    let move2_int = 0b1111101011010000; // g8-f6
    let move2_undo = board1.make_move(move2_int);

    let correct_resulting_board2 = ChessBoard::initialize_from_fen(
        "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2",
    )
    .unwrap();
    let correct_undo2: Result<UndoInfo, String> = Ok(UndoInfo {
        halfmove_clock: 0,
        castling_rights: 0b1111,
        en_passant_square: Some(20),
        captured_type: None,
    });

    assert_eq!(board1, correct_resulting_board2);
    assert_eq!(move2_undo, correct_undo2);

    let move3_int = 0b0111001001000000; // e4-e5
    board1.make_move(move3_int);
    let move4_int = 0b1101111011110000; // h7-h6
    board1.make_move(move4_int);

    let move5_int = 0b1001001011010001; // e5 x f6;
    let move5_undo = board1.make_move(move5_int);
    let correct_resulting_board5 = ChessBoard::initialize_from_fen(
        "rnbqkb1r/ppppppp1/5P1p/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3",
    )
    .unwrap();

    let correct_undo5: Result<UndoInfo, String> = Ok(UndoInfo {
        halfmove_clock: 0,
        castling_rights: 0b1111,
        en_passant_square: None,
        captured_type: Some(1),
    });

    assert_eq!(board1, correct_resulting_board5);
    assert_eq!(move5_undo, correct_undo5);
}
