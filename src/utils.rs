// Ideas for this file: display game board in text, display legal moves, display move list so far, display FEN, etc.

pub fn on_a_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn on_b_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 1 {
        return true;
    } else {
        return false;
    }
}

pub fn on_c_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 2 {
        return true;
    } else {
        return false;
    }
}

pub fn on_d_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 3 {
        return true;
    } else {
        return false;
    }
}

pub fn on_e_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 4 {
        return true;
    } else {
        return false;
    }
}

pub fn on_f_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 5 {
        return true;
    } else {
        return false;
    }
}

pub fn on_g_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 6 {
        return true;
    } else {
        return false;
    }
}

pub fn on_h_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if piece_location.trailing_zeros() % 8 == 7 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_1(piece_location: &u64) -> bool {
    if piece_location < &0x0000000000000100 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_2(piece_location: &u64) -> bool {
    if piece_location < &0x0000000000010000 && piece_location > &0x0000000000000080 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_3(piece_location: &u64) -> bool {
    if piece_location < &0x0000000001000000 && piece_location > &0x0000000000008000 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_4(piece_location: &u64) -> bool {
    if piece_location < &0x0000000100000000 && piece_location > &0x0000000000800000 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_5(piece_location: &u64) -> bool {
    if piece_location < &0x0000010000000000 && piece_location > &0x0000000080000000 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_6(piece_location: &u64) -> bool {
    if piece_location < &0x0001000000000000 && piece_location > &0x0000008000000000 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_7(piece_location: &u64) -> bool {
    if piece_location < &0x0100000000000000 && piece_location > &0x0000800000000000 {
        return true;
    } else {
        return false;
    }
}

pub fn on_rank_8(piece_location: &u64) -> bool {
    if piece_location > &0x0080000000000000 {
        return true;
    } else {
        return false;
    }
}

pub fn bb_to_square(bitboard: &u64) -> Result<String, String> {
    let bb_null = bitboard & (bitboard - 1);
    if bb_null != 0 {
        Err("Invalid bitboard: more than one piece on the board.".to_string())
    } else {
        let bit_position = bitboard.trailing_zeros();
        let rank = (bit_position / 8) + 1;
        let file_num = (bit_position % 8) as u8; // ASCII is in u8 format.
        let file = (b'a' + file_num) as char;

        let square = format!("{}{}", file, rank);
        Ok(square)
    }
}

pub const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

pub fn square_to_bb(square: &str) -> Result<u64, String> {
    if square.chars().count() != 2 {
        return Err("Invalid square: too many characters.".to_string());
    }
    let file = square.chars().nth(0).unwrap();
    let rank = square.chars().nth(1).unwrap();
    if !(FILES.contains(&file) && RANKS.contains(&rank)) {
        return Err("Invalid square: invalid file or rank.".to_string());
    }

    let mut modifier: u64 = 0;

    modifier += (rank.to_digit(10).unwrap() as u64 - 1) * 8;
    let file_modifier = (file as u8 - b'a') as u64;
    modifier += file_modifier;

    let square: u64 = 1 << modifier;
    Ok(square)
}

pub fn print_board_binary(bitboard: &u64) {
    // prints a binary mapping of a bitboard (no piece type information) for debugging and development purposes.
    let binary_bb = format!("{:064b}", bitboard);
    for i in 0..8 {
        let line_mirrored = &binary_bb[(i * 8)..((i + 1) * 8)];
        let line: String = line_mirrored.chars().rev().collect();
        println!("{}", line);
    }
}

pub fn squares_above(square: &u64) -> u64 {
    // prints mask of all squares above specified square
    let cutoff = ((square.trailing_zeros() % 8) + 1) * 8;
    let mask: u64 = (u64::MAX).checked_shl(cutoff).unwrap_or(0);
    mask
}

pub fn squares_below(square: &u64) -> u64 {
    // prints mask of all squares below specified square
    let cutoff = ((square.leading_zeros() % 8) + 1) * 8;
    let mask: u64 = (u64::MAX).checked_shr(cutoff).unwrap_or(0);
    mask
}

pub fn squares_left(square: &u64) -> u64 {
    if square.trailing_zeros() % 8 == 0 {
        0
    } else {
        let mut mask: u64 = 0;
        let file_template: u64 = 0x8080808080808080;

        let mut file_mod: u64 = ((square.trailing_zeros() as u64) % 8) - 1;

        while file_mod < u64::MAX {
            mask = mask ^ (file_template >> file_mod);
            file_mod = file_mod.wrapping_sub(1);
        }

        mask
    }
}

pub fn squares_right(square: &u64) -> u64 {
    if square.trailing_zeros() % 8 == 8 {
        0
    } else {
        let mut mask: u64 = 0;
        let file_template: u64 = 0x0101010101010101;

        let mut file_mod: u64 = ((square.trailing_zeros() as u64) % 8) + 1;

        while file_mod < 8 {
            mask = mask ^ (file_template << file_mod);
            file_mod += 1;
        }

        mask
    }
}

// Unit Tests

const A_SQUARES: [u64; 8] = [
    0x0000000000000001,
    0x0000000000000100,
    0x0000000000010000,
    0x0000000001000000,
    0x0000000100000000,
    0x0000010000000000,
    0x0001000000000000,
    0x0100000000000000,
];
#[test]
fn test_on_a_file() {
    for i in 0..64 {
        let square: u64 = 1 << i;
        if A_SQUARES.contains(&square) {
            assert_eq!(on_a_file(&square), true);
        } else {
            assert_eq!(on_a_file(&square), false);
        }
    }
}

#[test]
fn test_on_b_file() {
    let b_squares: [u64; 8] = A_SQUARES.map(|x| x << 1);

    for i in 0..64 {
        let square: u64 = 1 << i;
        if b_squares.contains(&square) {
            assert_eq!(on_b_file(&square), true);
        } else {
            assert_eq!(on_b_file(&square), false);
        }
    }
}

#[test]
fn test_on_c_file() {
    let c_squares: [u64; 8] = A_SQUARES.map(|x| x << 2);

    for i in 0..64 {
        let square: u64 = 1 << i;
        if c_squares.contains(&square) {
            assert_eq!(on_c_file(&square), true);
        } else {
            assert_eq!(on_c_file(&square), false);
        }
    }
}

#[test]
fn test_on_d_file() {
    let d_squares: [u64; 8] = A_SQUARES.map(|x| x << 3);

    for i in 0..64 {
        let square: u64 = 1 << i;
        if d_squares.contains(&square) {
            assert_eq!(on_d_file(&square), true);
        } else {
            assert_eq!(on_d_file(&square), false);
        }
    }
}

#[test]
fn test_on_e_file() {
    let e_squares: [u64; 8] = A_SQUARES.map(|x| x << 4);

    for i in 0..64 {
        let square: u64 = 1 << i;
        if e_squares.contains(&square) {
            assert_eq!(on_e_file(&square), true);
        } else {
            assert_eq!(on_e_file(&square), false);
        }
    }
}

#[test]
fn test_on_f_file() {
    let f_squares: [u64; 8] = A_SQUARES.map(|x| x << 5);

    for i in 0..64 {
        let square: u64 = 1 << i;
        if f_squares.contains(&square) {
            assert_eq!(on_f_file(&square), true);
        } else {
            assert_eq!(on_f_file(&square), false);
        }
    }
}

#[test]
fn test_on_g_file() {
    let g_squares: [u64; 8] = A_SQUARES.map(|x| x << 6);

    for i in 0..64 {
        let square: u64 = 1 << i;
        if g_squares.contains(&square) {
            assert_eq!(on_g_file(&square), true);
        } else {
            assert_eq!(on_g_file(&square), false);
        }
    }
}

#[test]
fn test_on_h_file() {
    let h_squares: [u64; 8] = A_SQUARES.map(|x| x << 7);

    for i in 0..64 {
        let square: u64 = 1 << i;
        if h_squares.contains(&square) {
            assert_eq!(on_h_file(&square), true);
        } else {
            assert_eq!(on_h_file(&square), false);
        }
    }
}

#[test]
fn test_on_rank_1() {
    for i in 0..8 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_1(&square), true);
    }
    for i in 8..64 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_1(&square), false);
    }
}

#[test]
fn test_on_rank_2() {
    for i in (0..8).chain(16..64) {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_2(&square), false);
    }
    for i in 8..16 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_2(&square), true);
    }
}

#[test]
fn test_on_rank_3() {
    for i in (0..16).chain(24..64) {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_3(&square), false);
    }
    for i in 16..24 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_3(&square), true);
    }
}

#[test]
fn test_on_rank_4() {
    for i in (0..24).chain(32..64) {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_4(&square), false);
    }
    for i in 24..32 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_4(&square), true);
    }
}

#[test]
fn test_on_rank_5() {
    for i in (0..32).chain(40..64) {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_5(&square), false);
    }
    for i in 32..40 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_5(&square), true);
    }
}

#[test]
fn test_on_rank_6() {
    for i in (0..40).chain(48..64) {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_6(&square), false);
    }
    for i in 40..48 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_6(&square), true);
    }
}

#[test]
fn test_on_rank_7() {
    for i in (0..48).chain(56..64) {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_7(&square), false);
    }
    for i in 48..56 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_7(&square), true);
    }
}

#[test]
fn test_on_rank_8() {
    for i in 0..56 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_8(&square), false);
    }
    for i in 56..64 {
        let square: u64 = 1 << i;
        assert_eq!(on_rank_8(&square), true);
    }
}

#[test]
fn test_bb_to_square() {
    // a1
    let board1: u64 = 1;
    assert_eq!(bb_to_square(&board1).unwrap(), "a1");
    // e4
    let board2: u64 = 0x0000000010000000;
    assert_eq!(bb_to_square(&board2).unwrap(), "e4");
    // h8
    let board3: u64 = 0x8000000000000000;
    assert_eq!(bb_to_square(&board3).unwrap(), "h8");
    // c7
    let board4: u64 = 0x0004000000000000;
    assert_eq!(bb_to_square(&board4).unwrap(), "c7");
    // invalid: too many squares
    let board5: u64 = 3;
    assert_eq!(
        bb_to_square(&board5),
        Err("Invalid bitboard: more than one piece on the board.".to_string())
    );
}

#[test]
fn test_square_to_bb() {
    // a1
    let square1: &str = "a1";
    assert_eq!(square_to_bb(square1).unwrap(), 1);
    // e4
    let square2: &str = "e4";
    assert_eq!(square_to_bb(square2).unwrap(), 0x0000000010000000);
    // h8
    let square3: &str = "h8";
    assert_eq!(square_to_bb(square3).unwrap(), 0x8000000000000000);
    // c7
    let square4: &str = "c7";
    assert_eq!(square_to_bb(square4).unwrap(), 0x0004000000000000);
    // invalid: file out of range
    let square5: &str = "i1";
    assert_eq!(
        square_to_bb(square5),
        Err("Invalid square: invalid file or rank.".to_string())
    );
    // invalid: rank out of range
    let square6: &str = "a9";
    assert_eq!(
        square_to_bb(square6),
        Err("Invalid square: invalid file or rank.".to_string())
    );
    // invalid: too many chars
    let square7: &str = "a11";
    assert_eq!(
        square_to_bb(square7),
        Err("Invalid square: too many characters.".to_string())
    );
}

#[test]
fn test_squares_above() {
    // a1
    let square1: &str = "a1";
    let sq1: u64 = square_to_bb(square1).unwrap();
    assert_eq!(squares_above(&sq1), 0xFFFFFFFFFFFFFF00);
    // g7
    let square2: &str = "g7";
    let sq2: u64 = square_to_bb(square2).unwrap();
    println!("{}", sq2);
    assert_eq!(squares_above(&sq2), 0xFF00000000000000);
}

#[test]
fn test_squares_below() {
    // a1
    let square1: &str = "a1";
    let sq1: u64 = square_to_bb(square1).unwrap();
    assert_eq!(squares_below(&sq1), 0);
    // g7
    let square2: &str = "g7";
    let sq2: u64 = square_to_bb(square2).unwrap();
    assert_eq!(squares_below(&sq2), 0x0000FFFFFFFFFFFF);
}

#[test]
fn test_squares_left() {
    // a1
    let square1: &str = "a1";
    let sq1: u64 = square_to_bb(square1).unwrap();
    assert_eq!(squares_left(&sq1), 0);
    // g7
    let square2: &str = "g7";
    let sq2: u64 = square_to_bb(square2).unwrap();
    assert_eq!(squares_left(&sq2), 0xFCFCFCFCFCFCFCFC);
}

#[test]
fn test_squares_right() {
    // a1
    let square1: &str = "a1";
    let sq1: u64 = square_to_bb(square1).unwrap();
    assert_eq!(squares_right(&sq1), 0xFEFEFEFEFEFEFEFE);
    // g7
    let square2: &str = "g7";
    let sq2: u64 = square_to_bb(square2).unwrap();
    assert_eq!(squares_right(&sq2), 0x8080808080808080);
}
