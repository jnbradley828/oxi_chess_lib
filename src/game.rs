use crate::board::ChessBoard;
use crate::moves;
use crate::{board, rules, utils};
use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct ChessGame {
    pub board: board::ChessBoard,
    pub time_control: (u32, u32), // starting_ms, increment_ms, ** CLOCK CONTROL NOT IMPLEMENTED YET **
    pub moves: Vec<(u16, board::UndoInfo)>,
    pub positions_count: FxHashMap<u64, u8>,
    pub legal_moves: Vec<u16>,
    pub result: GameResult,
}
impl ChessGame {
    pub fn initialize(time_control: (u32, u32), fen: Option<&str>) -> Self {
        let mut game_board = match fen {
            None => ChessBoard::initialize(),
            Some(f) => ChessBoard::initialize_from_fen(f).unwrap(),
        };

        let legal_moves = moves::get_legal_moves(&mut game_board);
        let mut positions_count = FxHashMap::default();
        positions_count.insert(game_board.zobrist_hash, 1);
        let mut game = ChessGame {
            board: game_board,
            time_control: time_control,
            moves: Vec::new(),
            positions_count: positions_count,
            legal_moves: legal_moves,
            result: GameResult::InProgress,
        };
        game.result = game.check_result();
        return game;
    }

    pub fn make_move(&mut self, movei: u16) -> Result<GameResult, String> {
        if self.result != GameResult::InProgress {
            return Err("Game over".to_string());
        }

        if self.legal_moves.contains(&movei) {
            // move will not be in the legal moves list if it is not valid, thus this is sufficient validation.
            let undo_info = self.board.make_move(movei)?; // returns error from board.make_move() if there is one.
            self.legal_moves = moves::get_legal_moves(&mut self.board);
            self.moves.push((movei, undo_info));
            *self
                .positions_count
                .entry(self.board.zobrist_hash)
                .or_insert(0) += 1;
            self.result = self.check_result();
            return Ok(self.result);
        } else {
            return Err("Invalid move".to_string());
        }
    }

    pub fn make_move_from_uci(&mut self, uci_move: &str) -> Result<GameResult, String> {
        let movei = utils::encode_from_uci(uci_move)?;
        if (movei << 12) == 0 {
            // no flag is given
            for lmovei in &self.legal_moves {
                if lmovei >> 4 == movei >> 4 {
                    return self.make_move(*lmovei);
                }
            }
        } else {
            if self.legal_moves.contains(&movei) {
                // promotion
                return self.make_move(movei);
            } else if self.legal_moves.contains(&(movei + 4)) {
                // promotion w capture
                return self.make_move(movei + 4);
            }
        }

        return Err("invalid move".to_string());
    }

    pub fn check_result(&mut self) -> GameResult {
        if self.legal_moves.is_empty() {
            // if no legal moves remain
            if rules::is_check(&self.board, self.board.side_to_move) {
                // if side to move is in check == checkmate
                match self.board.side_to_move {
                    true => return GameResult::BlackWins(WinReason::Checkmate),
                    false => return GameResult::WhiteWins(WinReason::Checkmate),
                }
            } else {
                // stalemate
                return GameResult::Draw(DrawReason::Stalemate);
            }
        } else {
            if self.positions_count[&self.board.zobrist_hash] >= 3 {
                // threefold repitition
                return GameResult::Draw(DrawReason::ThreefoldRepitition);
            }
            if self.board.halfmove_clock >= 100 {
                // 50 move rule
                return GameResult::Draw(DrawReason::FiftyMoveRule);
            }
            if rules::is_insuf_material(&self.board) {
                // insufficient material
                return GameResult::Draw(DrawReason::InsufficientMaterial);
            }
        }

        return GameResult::InProgress;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    InProgress,
    WhiteWins(WinReason),
    BlackWins(WinReason),
    Draw(DrawReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinReason {
    Checkmate,
    Resignation,
    Timeout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawReason {
    Stalemate,
    FiftyMoveRule,
    ThreefoldRepitition,
    InsufficientMaterial,
    Agreement,
}

#[cfg(test)]
mod tests {
    use crate::utils::encode_move;

    use super::*;

    #[test]
    fn test_initialize_game() {
        // from standard starting position
        let game = ChessGame::initialize((1, 1), None);

        let mut starting_board = ChessBoard::initialize();
        let map: FxHashMap<u64, u8> = [(starting_board.zobrist_hash, 1)].into_iter().collect();
        assert_eq!(
            game,
            ChessGame {
                board: starting_board.clone(),
                time_control: (1, 1),
                moves: (Vec::new()),
                positions_count: map,
                legal_moves: moves::get_legal_moves(&mut starting_board),
                result: GameResult::InProgress
            }
        );

        // from fen position
        let game = ChessGame::initialize((1, 1), Some("k7/8/8/8/2b5/b2b4/8/K7 w - - 0 1"));

        let mut starting_board =
            ChessBoard::initialize_from_fen("k7/8/8/8/2b5/b2b4/8/K7 w - - 0 1").unwrap();
        let map: FxHashMap<u64, u8> = [(starting_board.zobrist_hash, 1)].into_iter().collect();
        assert_eq!(
            game,
            ChessGame {
                board: starting_board.clone(),
                time_control: (1, 1),
                moves: (Vec::new()),
                positions_count: map,
                legal_moves: moves::get_legal_moves(&mut starting_board),
                result: GameResult::Draw(DrawReason::Stalemate)
            }
        );
    }

    #[test]
    fn test_check_result() {
        let mut game = ChessGame::initialize((1, 1), None); // starting position
        assert_eq!(game.check_result(), GameResult::InProgress);

        let mut game = ChessGame::initialize((1, 1), Some("k7/8/8/8/2b5/b2b4/8/K7 w - - 0 1")); // white is in stalemate
        assert_eq!(game.check_result(), GameResult::Draw(DrawReason::Stalemate));

        let mut game = ChessGame::initialize((1, 1), Some("k5Q1/7R/8/8/8/8/8/K7 b - - 0 1")); // black is in checkmate
        assert_eq!(
            game.check_result(),
            GameResult::WhiteWins(WinReason::Checkmate)
        );

        let mut game = ChessGame::initialize((1, 1), Some("8/3k4/8/8/8/8/6R1/K7 b - - 100 50")); // 50 move rule
        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::FiftyMoveRule)
        );

        let mut game = ChessGame::initialize((1, 1), None); // 3 fold repetition
        game.make_move(encode_move(1, 18, 0));
        game.make_move(encode_move(57, 42, 0));
        game.make_move(encode_move(18, 1, 0));
        game.make_move(encode_move(42, 57, 0));
        game.make_move(encode_move(1, 18, 0));
        game.make_move(encode_move(57, 42, 0));
        game.make_move(encode_move(18, 1, 0));
        game.make_move(encode_move(42, 57, 0));

        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::ThreefoldRepitition)
        );

        // insufficient material cases

        let mut game = ChessGame::initialize((1, 1), Some("8/8/8/8/6K1/2k5/8/8 w - - 0 1")); // K vs K
        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::InsufficientMaterial)
        );
        let mut game = ChessGame::initialize((1, 1), Some("8/8/8/2b5/6K1/2k5/8/8 b - - 0 1")); // K vs kb
        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::InsufficientMaterial)
        );
        let mut game = ChessGame::initialize((1, 1), Some("8/8/7n/8/6K1/2k5/8/8 w - - 0 1")); // K vs kn
        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::InsufficientMaterial)
        );
        let mut game = ChessGame::initialize((1, 1), Some("8/2K5/6k1/8/6B1/8/8/8 b - - 0 1")); // KB vs k
        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::InsufficientMaterial)
        );
        let mut game = ChessGame::initialize((1, 1), Some("8/2K5/6k1/8/8/8/8/7N w - - 0 1")); // KN vs k
        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::InsufficientMaterial)
        );
        let mut game = ChessGame::initialize((1, 1), Some("5b2/2K5/3B2k1/8/8/8/8/8 w - - 0 1")); // KB vs kb (same color)
        assert_eq!(
            game.check_result(),
            GameResult::Draw(DrawReason::InsufficientMaterial)
        );
        let mut game = ChessGame::initialize((1, 1), Some("6b1/2K5/3B2k1/8/8/8/8/8 b - - 0 1")); // KB vs kb (opp_color=InProgress)
        assert_eq!(game.check_result(), GameResult::InProgress);
    }

    #[test]
    fn test_make_move() {
        let mut game = ChessGame::initialize((1, 1), None); // starting position e2-e4
        let movei = encode_move(12, 28, 0);
        let move_result = game.make_move(movei);
        assert!(move_result.is_ok());

        let mut game = ChessGame::initialize((1, 1), None); // illegal first move
        let movei = encode_move(12, 36, 0);
        let move_result = game.make_move(movei);
        assert!(move_result.is_err());

        let mut game = ChessGame::initialize(
            (1, 1),
            Some("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 1"),
        );
        let movei = encode_move(59, 31, 0); // checkmating move
        let move_result = game.make_move(movei);
        assert_eq!(
            move_result.unwrap(),
            GameResult::BlackWins(WinReason::Checkmate)
        );
    }
}
