// todo!("Functions needed before finishing moves.rs and game.rs: is_check, is_checkmate, is_stalemate, is_fifty_move_rule, is_threefold_repetition, etc.");

use crate::{board, moves, utils};

// checks if the player to move is in check.
pub fn is_check(board: &board::ChessBoard) -> bool {
    let color_mask;
    if board.side_to_move {
        color_mask = board.white_pieces;
    } else {
        color_mask = board.black_pieces;
    }

    let king_to_check = board.kings & color_mask;
    println!("king location");
    utils::print_board_binary(&king_to_check);

    let opposing_attacks = moves::board_attacks(&board, !board.side_to_move);
    for oa_mask in opposing_attacks {
        println!("opposing piece location");
        utils::print_board_binary(&oa_mask.0);
        println!("opposing attacks");
        utils::print_board_binary(&oa_mask.1);
        if oa_mask.1 & king_to_check != 0 {
            return true;
        }
    }
    return false;
}

#[test]
fn test_is_check() {
    let board1 = board::ChessBoard::initialize_from_fen(
        "rnbqkbnr/pppp1Bpp/8/4pp2/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 1",
    )
    .unwrap();
    assert_eq!(is_check(&board1), true);

    let board2 = board::ChessBoard::initialize();
    assert_eq!(is_check(&board2), false);

    let board3 = board::ChessBoard::initialize_from_fen(
        "rnbqkb1r/pppppppp/8/8/8/5nP1/PPPPPP2/RNBQKBNR w KQkq - 0 1",
    )
    .unwrap();
    assert_eq!(is_check(&board3), true);
}
