use crate::board::ChessBoard;
use crate::moves;
use crate::moves::get_legal_moves;
use crate::utils;

pub fn perft(board: &mut ChessBoard, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = moves::get_legal_moves(board);
    let mut nodes = 0;
    for move_i in moves {
        let undo_info = board.make_move(move_i).unwrap();
        nodes += perft(board, depth - 1);
        board.unmake_move(move_i, &undo_info).unwrap();
    }
    nodes
}

pub fn perft_divide(board: &mut ChessBoard, depth: u32) {
    let moves = get_legal_moves(board);
    let mut total = 0;
    for move_i in moves {
        let undo_info = board.make_move(move_i).unwrap();
        let nodes = perft(board, depth - 1);
        board.unmake_move(move_i, &undo_info).unwrap();
        let decoded = utils::decode_to_uci(move_i).unwrap();
        println!("{}: {}", decoded, nodes);
        total += nodes;
    }
    println!("Total: {total}");
}
