use oxi_chess_lib::board::ChessBoard;
use oxi_chess_lib::game::ChessGame;
use oxi_chess_lib::moves;
use oxi_chess_lib::moves::get_legal_moves;
use oxi_chess_lib::utils;

fn perft(board: &mut ChessBoard, depth: u32) -> u64 {
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

fn perft_divide(board: &mut ChessBoard, depth: u32) {
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

fn main() {
    let mut board = ChessBoard::initialize_from_fen(
        "r3k2r/p1pNqpb1/bn2pnp1/3P4/1p2P3/2N2Q2/PPPBBPpP/R3K2R w KQkq - 0 1",
    )
    .unwrap();
    perft_divide(&mut board, 1);
}
