use crate::board;
use rustc_hash::FxHashMap;

pub struct ChessGame {
    pub board: board::ChessBoard,
    pub moves: Vec<(u16, board::UndoInfo)>,
    pub positions_count: FxHashMap<u64, u8>,
}
