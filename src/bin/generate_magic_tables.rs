use oxi_chess_lib::magic_gen::{
    bishop_relevant_mask, generate_magic_bishop_table, generate_magic_rook_table,
    rook_relevant_mask,
};
use std::fs::File;
use std::io::Write;

fn write_masks(file: &mut File, piece_name: &str, relevant_mask: impl Fn(u8) -> u64) {
    writeln!(file, "pub const {piece_name}_MASKS: [u64; 64] = [").unwrap();
    for sq_i in 0..64u8 {
        writeln!(file, "    {},", relevant_mask(sq_i)).unwrap();
    }
    writeln!(file, "];\n").unwrap();
}

fn write_table(file: &mut File, piece_name: &str, magic_nums: &[u64; 64], table: &[[u64; 4096]]) {
    // write magic nums table!
    writeln!(file, "pub const {piece_name}_MAGIC_NUMS: [u64; 64] = [").unwrap();
    for n in magic_nums {
        writeln!(file, "    {n},").unwrap();
    }
    writeln!(file, "];\n").unwrap();

    // write attack table!
    writeln!(
        file,
        "pub static {piece_name}_ATTACKS: [[u64; 4096]; 64] = ["
    )
    .unwrap();
    for row in table {
        write!(file, "    [").unwrap();
        for n in row {
            write!(file, "{n},").unwrap();
        }
        writeln!(file, "],").unwrap();
    }
    writeln!(file, "];").unwrap();
}

fn main() {
    let (rook_magic_nums, rook_magic_table) = generate_magic_rook_table();
    let (bishop_magic_nums, bishop_magic_table) = generate_magic_bishop_table();

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/magic_tables.rs");
    let mut file = File::create(path).unwrap();

    write_masks(&mut file, "ROOK", rook_relevant_mask);
    write_masks(&mut file, "BISHOP", bishop_relevant_mask);
    write_table(&mut file, "ROOK", &rook_magic_nums, &rook_magic_table);
    write_table(&mut file, "BISHOP", &bishop_magic_nums, &bishop_magic_table);
}
