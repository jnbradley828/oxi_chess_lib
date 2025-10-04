use crate::board;

pub fn find_pseudolegal_moves(board: &board::ChessBoard) -> Vec<u64> {
    todo!("If white: mask all pieces (bitwise and) with white bitboard, else mask with black bitboard.");
    todo!("For each masked bitboard: for each bit=1: apply correct algorithm to create bitboard of current legal moves with that piece.");
}
