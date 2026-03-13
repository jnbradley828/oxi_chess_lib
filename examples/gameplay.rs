use oxi_chess_lib::game::{ChessGame, GameResult};
use oxi_chess_lib::utils::render_board;
use std::io;
use std::io::Write;

fn main() {
    let mut game = ChessGame::initialize((1, 1), None);
    render_board(&game.board);
    while game.result == GameResult::InProgress {
        print!("Enter move: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "resign" {
            match game.board.side_to_move {
                true => {
                    game.result = GameResult::BlackWins(oxi_chess_lib::game::WinReason::Resignation)
                }
                false => {
                    game.result = GameResult::WhiteWins(oxi_chess_lib::game::WinReason::Resignation)
                }
            }
        } else if input == "offer draw" {
            println!("Accept draw? (y/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "y" {
                game.result = GameResult::Draw(oxi_chess_lib::game::DrawReason::Agreement);
            }
        } else {
            game.make_move_from_uci(input);
            render_board(&game.board);
        }
    }
    let result = game.result;
    println!("{result:?}");
}
