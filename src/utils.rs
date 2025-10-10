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

    if (piece_location.trailing_zeros() as i8 - 1) % 8 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn on_c_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if (piece_location.trailing_zeros() as i8 - 2) % 8 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn on_d_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if (piece_location.trailing_zeros() as i8 - 3) % 8 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn on_e_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if (piece_location.trailing_zeros() as i8 - 4) % 8 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn on_f_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if (piece_location.trailing_zeros() as i8 - 5) % 8 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn on_g_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if (piece_location.trailing_zeros() as i8 - 6) % 8 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn on_h_file(piece_location: &u64) -> bool {
    // if more than one bit = 1: raise error.

    if (piece_location.trailing_zeros() as i8 - 7) % 8 == 0 {
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

