/// Struct representing a chess board.
/// We will let the least significant bit represent the a1 square.
pub struct ChessBoard {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
    white_pieces: u64,
    black_pieces: u64,
    side_to_move: bool,   // let true = white to move
    en_passant: u64, // let u64 = en passant target (location of capture square). Let value = 0 if no en passant is possible.
    castling_rights: u8, // uses first 4 bits (white kingside, white queenside, black kingside, black queenside)
    halfmove_clock: u8,  // tracks half moves since last capture or pawn move.
    fullmove_number: u16, // tracks full moves since start of game.
}

/// Creates a new chess board with the standard starting position.
impl ChessBoard {
    pub fn initialize() -> Self {
        Self {
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
        }
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
        }
    }

    /// Creates a new chess board from a FEN string.
    pub fn initialize_from_fen(fen: &str) -> Result<Self, String> {
        if !verify_fen(&fen) {
            return Err("Invalid FEN string.".to_string());
        } else {
            todo!("Implement initialize_from_fen.")
        }
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
}
