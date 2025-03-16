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
