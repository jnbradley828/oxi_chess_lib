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
    en_passant: u64, // let u64 = en passant target piece (location of pawn, not capture square). Let value = 0 if no en passant is possible.
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
}
