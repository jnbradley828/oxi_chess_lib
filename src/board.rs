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

impl ChessBoard {
    /// Creates a new chess board with standard starting positions by default.
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
}
