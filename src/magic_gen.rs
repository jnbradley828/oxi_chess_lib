// PSEUDO-CODE
//
// rooks:
//
// mut attacks = [[0u64; 2^12]; 64]; // indexes like attacks[sqi][magic_result] = attack_mask
// mut blockers_attacks = Vector, [(blockers, attack_mask)] -> initialize empty
// mut magic_nums = [0u64; 64];
// for sq_index in 0..=63:
//     generate all blockers
//     generate all attack masks via ray casting
//     store in blockers_attackers
//
//     mut magic_num
//     while true:
//         success = true
//         reset attacks[sq_index][*all*] to 0s
//         magic_num = rand(64) & rand(64) & rand(64)
//         for (blockers, attack_mask) in blockers_attacks:
//             magic_result = blockers * magic_num >> (64-12);
//             if attacks[sq_index][magic_result] == 0:
//                 // it's not initialized yet. Add it
//                 attacks[sq_index][magic_result] = attack_mask
//             else if attacks[sq_index][magic_result] == attack_mask:
//                 // already correct. continue
//                 continue
//             else:
//                 // this magic number does not work. restart.
//                 success = false
//                 break
//
//         // if you made it through the for loop, you found the magic number! store your number and break the while loop
//         if success:
//             magic_nums[sq_index] = magic_num
//             break

// rook gen

use crate::{
    board::ChessBoard,
    moves::{bishop_attacks, rook_attacks, RAYS},
};
use rand::Rng;

fn clear_lowest_bit(int: u64) -> u64 {
    if int == 0 {
        0
    } else {
        int & !(1 << int.trailing_zeros())
    }
}
fn clear_highest_bit(int: u64) -> u64 {
    if int == 0 {
        0
    } else {
        int & !(1 << (63 - int.leading_zeros()))
    }
}

fn flip_bit(int: u64, idx: u8) -> u64 {
    int ^ 1u64 << idx
}

fn get_bit_indices(int: u64) -> Vec<u8> {
    let mut int_left = int;
    let mut indices: Vec<u8> = Vec::new();
    while int_left != 0 {
        let idx = int_left.trailing_zeros();
        indices.push(idx as u8);
        int_left = clear_lowest_bit(int_left);
    }
    return indices;
}

pub fn rook_relevant_mask(sq_i: u8) -> u64 {
    let rays = RAYS[sq_i as usize];
    let ray_n = clear_highest_bit(rays[0]);
    let ray_e = clear_highest_bit(rays[2]);
    let ray_s = clear_lowest_bit(rays[4]);
    let ray_w = clear_lowest_bit(rays[6]);

    ray_n | ray_e | ray_s | ray_w
}

fn generate_rook_blockers(sq_i: u8) -> Vec<u64> {
    let bmask = rook_relevant_mask(sq_i);
    let mut sub = bmask;
    let mut bmasks: Vec<u64> = vec![];
    bmasks.push(bmask);

    // Carry-Ripler trick for bitboard enumeration
    while sub != 0 {
        sub = (sub - 1) & bmask;
        bmasks.push(sub);
    }

    return bmasks;
}

fn generate_rook_blk_attacks(sq_i: u8) -> Vec<(u64, u64)> {
    let blockers = generate_rook_blockers(sq_i);
    let mut blockers_attacks: Vec<(u64, u64)> = Vec::new();
    for block_mask in blockers {
        // for the sake of reusing rook_attacks(), we make a board instance to pass in, where all blockers are assumed to be opposing pieces.
        let board: ChessBoard = ChessBoard {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,
            white_pieces: 0,
            black_pieces: block_mask,
            side_to_move: true,
            en_passant: 0,
            castling_rights: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
            zobrist_hash: 0,
        };
        let attack_mask = rook_attacks(true, 1 << sq_i, &board);
        blockers_attacks.push((block_mask, attack_mask));
    }

    blockers_attacks
}

pub fn generate_magic_rook_table() -> ([u64; 64], Vec<[u64; 4096]>) {
    let mut rook_attacks: Vec<[u64; 4096]> = vec![[0u64; 4096]; 64];
    let mut magic_nums = [0u64; 64];

    for sq_i in 0..64 {
        let sqi = sq_i as usize;
        let blockers_attackers = generate_rook_blk_attacks(sq_i);
        let mut magic_num: u64;

        loop {
            let mut success = true;
            rook_attacks[sqi] = [0u64; 4096];
            magic_num = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();

            for (block_mask, attack_mask) in &blockers_attackers {
                let magic_result = (block_mask.wrapping_mul(magic_num) >> (64 - 12)) as usize;
                let attack_mask = *attack_mask;
                if rook_attacks[sqi][magic_result] == 0 {
                    rook_attacks[sqi][magic_result] = attack_mask;
                } else if rook_attacks[sqi][magic_result] == attack_mask {
                    continue;
                } else {
                    success = false;
                    break;
                }
            }

            if success {
                magic_nums[sqi] = magic_num;
                break;
            }
        }
    }

    (magic_nums, rook_attacks)
}

pub fn bishop_relevant_mask(sq_i: u8) -> u64 {
    let rays = RAYS[sq_i as usize];
    let ray_ne = clear_highest_bit(rays[1]);
    let ray_se = clear_lowest_bit(rays[3]);
    let ray_sw = clear_lowest_bit(rays[5]);
    let ray_nw = clear_highest_bit(rays[7]);

    ray_ne | ray_se | ray_sw | ray_nw
}

fn generate_bishop_blockers(sq_i: u8) -> Vec<u64> {
    let bmask = bishop_relevant_mask(sq_i);
    let mut sub = bmask;
    let mut bmasks: Vec<u64> = vec![];
    bmasks.push(bmask);

    // Carry-Ripler trick for bitboard enumeration
    while sub != 0 {
        sub = (sub - 1) & bmask;
        bmasks.push(sub);
    }

    return bmasks;
}

fn generate_bishop_blk_attacks(sq_i: u8) -> Vec<(u64, u64)> {
    let blockers = generate_bishop_blockers(sq_i);
    let mut blockers_attacks: Vec<(u64, u64)> = Vec::new();
    for block_mask in blockers {
        let board: ChessBoard = ChessBoard {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,
            white_pieces: 0,
            black_pieces: block_mask,
            side_to_move: true,
            en_passant: 0,
            castling_rights: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
            zobrist_hash: 0,
        };
        let attack_mask = bishop_attacks(true, 1 << sq_i, &board);
        blockers_attacks.push((block_mask, attack_mask));
    }

    blockers_attacks
}

pub fn generate_magic_bishop_table() -> ([u64; 64], Vec<[u64; 4096]>) {
    let mut bishop_attacks: Vec<[u64; 4096]> = vec![[0u64; 4096]; 64];
    let mut magic_nums = [0u64; 64];

    for sq_i in 0..64 {
        let sqi = sq_i as usize;
        let blockers_attackers = generate_bishop_blk_attacks(sq_i);
        let mut magic_num: u64;

        loop {
            let mut success = true;
            bishop_attacks[sqi] = [0u64; 4096];
            magic_num = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();

            for (block_mask, attack_mask) in &blockers_attackers {
                let magic_result = (block_mask.wrapping_mul(magic_num) >> (64 - 12)) as usize;
                let attack_mask = *attack_mask;
                if bishop_attacks[sqi][magic_result] == 0 {
                    bishop_attacks[sqi][magic_result] = attack_mask;
                } else if bishop_attacks[sqi][magic_result] == attack_mask {
                    continue;
                } else {
                    success = false;
                    break;
                }
            }

            if success {
                magic_nums[sqi] = magic_num;
                break;
            }
        }
    }

    (magic_nums, bishop_attacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_indices() {
        let int = 0x800000000000000F;
        let indices_correct: Vec<u8> = vec![0, 1, 2, 3, 63];
        let indices_calculated = get_bit_indices(int);
        assert_eq!(indices_calculated, indices_correct);
    }

    #[test]
    fn test_magic_rook_table() {
        let (magic_nums, magic_rook_table) = generate_magic_rook_table();
        let sq_i: u8 = 0;
        let block_pattern: u64 = 0x0000000000000140;
        let attacks: u64 = 0x000000000000017E;

        let magic_num = magic_nums[sq_i as usize];
        let magic_result = block_pattern.wrapping_mul(magic_num) >> (64 - 12);
        let magic_attack_mask = magic_rook_table[sq_i as usize][magic_result as usize];

        assert_eq!(magic_attack_mask, attacks);
    }

    #[test]
    fn test_magic_bishop_table() {
        let (magic_nums, magic_bishop_table) = generate_magic_bishop_table();
        let sq_i: u8 = 0;
        let block_pattern: u64 = 0x0000000008000000; // blocker on d4
        let attacks: u64 = 0x0000000008040200; // b2, c3, d4 (ne ray from a1, stopped at d4)

        let magic_num = magic_nums[sq_i as usize];
        let magic_result = block_pattern.wrapping_mul(magic_num) >> (64 - 12);
        let magic_attack_mask = magic_bishop_table[sq_i as usize][magic_result as usize];

        assert_eq!(magic_attack_mask, attacks);
    }
}
