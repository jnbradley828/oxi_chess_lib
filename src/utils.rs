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

// Unit Tests


const A_SQUARES: [u64; 8] = [0x0000000000000001, 0x0000000000000100, 0x0000000000010000, 0x0000000001000000, 0x0000000100000000, 0x0000010000000000, 0x0001000000000000, 0x0100000000000000]; 
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

