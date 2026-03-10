// todo!("Functions needed before finishing moves.rs and game.rs: is_check, is_checkmate, is_stalemate, is_fifty_move_rule, is_threefold_repetition, etc.");

use crate::{board, moves, utils};

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

pub fn is_insuf_material(board: &board::ChessBoard) -> bool {
    let white_material = (board.white_pieces & !board.kings).count_ones();
    let black_material = (board.black_pieces & !board.kings).count_ones();

    if white_material == 0 {
        if black_material == 0 {
            return true;
        } else if black_material == 1 {
            if board.black_pieces & board.knights != 0 {
                return true;
            } else if board.black_pieces & board.bishops != 0 {
                return true;
            }
        }
    } else if black_material == 0 {
        if white_material == 1 {
            if board.white_pieces & board.knights != 0 {
                return true;
            } else if board.white_pieces & board.bishops != 0 {
                return true;
            }
        }
    } else if white_material == 1 {
        if black_material == 1 {
            let white_bishop = board.white_pieces & board.bishops;
            let black_bishop = board.black_pieces & board.bishops;
            if (white_bishop != 0) && (black_bishop != 0) {
                return utils::square_color(white_bishop.trailing_zeros() as u8)
                    == utils::square_color(black_bishop.trailing_zeros() as u8);
                // return true if bishops are of same color
            }
        }
    }
    return false;
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

    #[test]
    fn test_is_insuf_material() {
        let board1 = board::ChessBoard::initialize(); // starting position
        assert_eq!(is_insuf_material(&board1), false);

        let board2 = board::ChessBoard::initialize_from_fen("k7/8/8/8/8/8/8/K7 w - - 0 1").unwrap(); // K vs k
        assert_eq!(is_insuf_material(&board2), true);

        let board3 =
            board::ChessBoard::initialize_from_fen("k7/8/8/8/2b5/8/8/K7 w - - 0 1").unwrap(); // K vs k & b
        assert_eq!(is_insuf_material(&board3), true);

        let board4 =
            board::ChessBoard::initialize_from_fen("k7/8/8/8/8/4n3/8/K7 w - - 0 1").unwrap(); // K vs k & n
        assert_eq!(is_insuf_material(&board4), true);

        let board5 =
            board::ChessBoard::initialize_from_fen("k7/8/8/8/8/8/8/K6B b - - 0 1").unwrap(); // K & B vs k
        assert_eq!(is_insuf_material(&board5), true);

        let board6 =
            board::ChessBoard::initialize_from_fen("k7/4N3/8/8/8/8/8/K7 b - - 0 1").unwrap(); // K & N vs k
        assert_eq!(is_insuf_material(&board6), true);

        let board7 =
            board::ChessBoard::initialize_from_fen("k7/4b3/8/4B3/8/8/8/K7 w - - 0 1").unwrap(); // K & B vs k & b (same color)
        assert_eq!(is_insuf_material(&board7), true);

        let board8 =
            board::ChessBoard::initialize_from_fen("k7/8/4b3/4B3/8/8/8/K7 w - - 0 1").unwrap(); // K & B vs k & b (diff color)
        assert_eq!(is_insuf_material(&board8), false);
    }
}
