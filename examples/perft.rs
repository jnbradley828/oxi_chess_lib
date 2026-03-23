use oxi_chess_lib::board::ChessBoard;
use oxi_chess_lib::perft;

fn main() {
    let mut board = ChessBoard::initialize_from_fen(
        "r3k2r/p1pNqpb1/bn2pnp1/3P4/1p2P3/2N2Q2/PPPBBPpP/R3K2R w KQkq - 0 1",
    )
    .unwrap();
    perft::perft_divide(&mut board, 1);
}
