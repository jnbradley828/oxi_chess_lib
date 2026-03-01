// todo!("Functions needed before finishing moves.rs and game.rs: is_check, is_checkmate, is_stalemate, is_fifty_move_rule, is_threefold_repetition, etc.");

use crate::{board, moves};

// checks if the player to move is in check.
pub fn is_check(board: &board::ChessBoard, side_in_check: bool) -> bool {
    let king_sq: u64;
    if side_in_check {
        king_sq = board.kings & board.white_pieces;
    } else {
        king_sq = board.kings & board.black_pieces;
    }

    let attacked_sqs = moves::board_attacks(&board, !side_in_check);
    if king_sq & attacked_sqs != 0 {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_check() {
        let board1 = board::ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppp1Bpp/8/4pp2/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(is_check(&board1, false), true);

        let board2 = board::ChessBoard::initialize();
        assert_eq!(is_check(&board2, true), false);

        let board3 = board::ChessBoard::initialize_from_fen(
            "rnbqkb1r/pppppppp/8/8/8/5nP1/PPPPPP2/RNBQKBNR w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(is_check(&board3, true), true);
    }
}
