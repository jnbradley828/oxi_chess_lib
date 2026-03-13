use oxi_chess_lib::game::{ChessGame, GameResult};
use oxi_chess_lib::utils::{encode_from_uci, encode_move, render_board};
use std::io;

fn main() {
    let mut game = ChessGame::initialize((1, 1), None);
    render_board(&game.board);
    while game.result == GameResult::InProgress {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        game.make_move_from_uci(input);
        render_board(&game.board);
    }
}
