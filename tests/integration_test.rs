use oxi_chess_lib::board::ChessBoard;
use oxi_chess_lib::game::ChessGame;
use oxi_chess_lib::moves;

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

#[test]
fn test_perft() {
    let mut board = ChessBoard::initialize();
    assert_eq!(perft(&mut board, 0), 1);
    assert_eq!(perft(&mut board, 1), 20);
    assert_eq!(perft(&mut board, 2), 400);
    assert_eq!(perft(&mut board, 3), 8902);
    assert_eq!(perft(&mut board, 4), 197281);
    assert_eq!(perft(&mut board, 5), 4865609);

    let mut board = ChessBoard::initialize_from_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    )
    .unwrap();
    assert_eq!(perft(&mut board, 1), 48);
    assert_eq!(perft(&mut board, 2), 2039);
    assert_eq!(perft(&mut board, 3), 97862);
    assert_eq!(perft(&mut board, 4), 4085603);
    //assert_eq!(perft(&mut board, 5), 193690690);
}
