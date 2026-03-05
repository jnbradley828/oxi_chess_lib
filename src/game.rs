use crate::board;
use crate::moves;
use rustc_hash::FxHashMap;

pub struct ChessGame {
    pub board: board::ChessBoard,
    pub time_control: (u32, u32), // starting_ms, increment_ms
    pub moves: Vec<(u16, board::UndoInfo)>,
    pub positions_count: FxHashMap<u64, u8>,
    pub legal_moves: Vec<u16>,
    pub result: GameResult,
}

pub enum GameResult {
    InProgress,
    WhiteWins(WinReason),
    BlackWins(WinReason),
    Draw(DrawReason),
}

pub enum WinReason {
    Checkmate,
    Resignation,
    Timeout,
}

pub enum DrawReason {
    Stalemate,
    FiftyMoveRule,
    ThreefoldRepitition,
    InsufficientMaterial,
}
