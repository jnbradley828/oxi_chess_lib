use oxi_chess_lib::board::ChessBoard;
use oxi_chess_lib::perft::perft;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use thousands::Separable;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("src/bin/perft_results.txt")
        .unwrap();

    for depth in 1..=5 {
        let mut total_elapsed: Duration = Duration::ZERO;
        let mut total_nodes = 0;

        let rounds = 5;
        for i in 1..=rounds {
            let start = Instant::now();
            let mut board = ChessBoard::initialize();
            let nodes = perft(&mut board, depth);
            let elapsed = start.elapsed();
            total_elapsed += elapsed;
            total_nodes += nodes;
        }
        let avg_elapsed = total_elapsed / rounds;
        let avg_nodes = total_nodes / (rounds as u64);

        writeln!(
            file,
            "depth {}: {} nodes in {:?}\n  nodes per second = {}",
            depth,
            avg_nodes,
            avg_elapsed,
            (((avg_nodes as f64) / avg_elapsed.as_secs_f64()) as u64).separate_with_commas()
        )
        .unwrap();
    }
}
