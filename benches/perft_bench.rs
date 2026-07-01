use chrono::Local;
use oxi_chess_lib::board::ChessBoard;
use oxi_chess_lib::perft::perft;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use thousands::Separable;

fn main() {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("benches/perft_results_{}.txt", timestamp);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    for depth in 1..=6 {
        let mut total_elapsed: Duration = Duration::ZERO;
        let mut total_nodes = 0;

        let rounds = 8;
        for i in 1..=rounds {
            let start = Instant::now();
            let mut board = ChessBoard::initialize();
            let nodes = perft(&mut board, depth);
            let elapsed = start.elapsed();
            if i != 1 {
                // skip first round: avoids cold cache, branch predictor slowdowns
                total_elapsed += elapsed;
                total_nodes += nodes;
            }
        }
        let avg_elapsed = total_elapsed / (rounds - 1);
        let avg_nodes = total_nodes / ((rounds - 1) as u64);

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
