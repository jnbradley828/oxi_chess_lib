use crate::utils;
use crate::zobrist_keys::ZOBRIST_CASTLING;
use crate::zobrist_keys::ZOBRIST_EP;
use crate::zobrist_keys::ZOBRIST_PIECES;
use crate::zobrist_keys::ZOBRIST_SIDE;

/// Struct representing a chess board.
/// We will let the least significant bit represent the a1 square.
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ChessBoard {
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,
    pub white_pieces: u64,
    pub black_pieces: u64,
    pub side_to_move: bool,   // let true = white to move
    pub en_passant: u64, // let u64 = en passant target (location of capture square). Let value = 0 if no en passant is possible.
    pub castling_rights: u8, // uses 4 least significant bits (from most sig to least sig: white kingside, white queenside, black kingside, black queenside)
    pub halfmove_clock: u8,  // tracks half moves since last capture or pawn move.
    pub fullmove_number: u16, // tracks full moves since start of game.
    pub zobrist_hash: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UndoInfo {
    halfmove_clock: u8,
    castling_rights: u8,
    en_passant_square: Option<u8>,
    captured_type: Option<u8>, // let (pawn, knight, bishop, rook, queen) = (0, 1, 2, 3, 4)
    zobrist_hash: u64,
}

/// Creates a new chess board with the standard starting position.
impl ChessBoard {
    pub fn initialize() -> Self {
        let mut board = ChessBoard {
            pawns: 0x00FF00000000FF00,   // pawns at ranks 2 & 7.
            knights: 0x4200000000000042, // knights at b1, g1, b8, & g8.
            bishops: 0x2400000000000024, // bishops at c1, f1, c8, & f8.
            rooks: 0x8100000000000081,   // rooks at a1, h1, a8, & h8.
            queens: 0x0800000000000008,  // queens at d1 & d8.
            kings: 0x1000000000000010,   // kings at e1 & e8.

            white_pieces: 0x000000000000FFFF,
            black_pieces: 0xFFFF000000000000,

            side_to_move: true,
            en_passant: 0,
            castling_rights: 0b1111,
            halfmove_clock: 0,
            fullmove_number: 1,
            zobrist_hash: 0,
        };
        board.zobrist_hash = board.generate_zobrist_hash();
        board
    }

    /// Creates a new empty chess board. White to move by default.
    pub fn empty() -> Self {
        let mut board = ChessBoard {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,

            white_pieces: 0,
            black_pieces: 0,

            side_to_move: true, // white to move by default.
            en_passant: 0,
            castling_rights: 0,
            halfmove_clock: 0,
            fullmove_number: 1,
            zobrist_hash: 0,
        };
        board.zobrist_hash = board.generate_zobrist_hash();
        board
    }

    /// Creates a new chess board from a FEN string.
    pub fn initialize_from_fen(fen: &str) -> Result<Self, String> {
        if !verify_fen(&fen) {
            return Err("Invalid FEN string.".to_string());
        } else {
            let fen_components: Vec<&str> = fen.split_whitespace().collect();

            // parse fen_components[0] (piece locations).
            let ranks: Vec<&str> = fen_components[0].split('/').collect();
            let mut square = 0;

            let mut pawns1: u64 = 0;
            let mut knights1: u64 = 0;
            let mut bishops1: u64 = 0;
            let mut rooks1: u64 = 0;
            let mut queens1: u64 = 0;
            let mut kings1: u64 = 0;

            let mut white_pieces1: u64 = 0;
            let mut black_pieces1: u64 = 0;

            for rank in ranks {
                for ch in rank.chars().rev() {
                    // increment by 1 for alphabetical values and set piece_type[bit] = 1,
                    // increment by numerical for numberical values
                    if ch.is_ascii_alphabetic() {
                        match ch {
                            'k' | 'K' => kings1 |= 1 << (63 - square),
                            'q' | 'Q' => queens1 |= 1 << (63 - square),
                            'r' | 'R' => rooks1 |= 1 << (63 - square),
                            'b' | 'B' => bishops1 |= 1 << (63 - square),
                            'n' | 'N' => knights1 |= 1 << (63 - square),
                            'p' | 'P' => pawns1 |= 1 << (63 - square),
                            _ => (),
                        }
                        match ch.is_uppercase() {
                            true => white_pieces1 |= 1 << (63 - square),
                            false => black_pieces1 |= 1 << (63 - square),
                        }
                        square += 1;
                    } else {
                        square += ch.to_digit(10).unwrap();
                    }
                }
            }

            // parse fen_components[2] (castling rights)
            let mut castling_rights1: u8 = 0;
            for ch in fen_components[2].chars() {
                match ch {
                    'K' => castling_rights1 |= 1 << 3,
                    'Q' => castling_rights1 |= 1 << 2,
                    'k' => castling_rights1 |= 1 << 1,
                    'q' => castling_rights1 |= 1 << 0,
                    _ => (),
                }
            }

            // parse fen_components[3] (en passant target square)
            let ep_square: u64;
            if fen_components[3].len() == 2 {
                ep_square = utils::square_to_bb(&fen_components[3]).unwrap();
            } else if fen_components[3] == '-'.to_string() {
                ep_square = 0;
            } else {
                return Err("Invalid en passant square".to_string());
            }

            let mut board = ChessBoard {
                pawns: pawns1,
                knights: knights1,
                bishops: bishops1,
                rooks: rooks1,
                queens: queens1,
                kings: kings1,

                white_pieces: white_pieces1,
                black_pieces: black_pieces1,

                // parse fen_components[1] (color to move)
                side_to_move: match fen_components[1] {
                    "w" => true,
                    _ => false,
                },
                en_passant: ep_square,
                castling_rights: castling_rights1,
                halfmove_clock: fen_components[4].parse::<u8>().unwrap(),
                fullmove_number: fen_components[5].parse::<u16>().unwrap(),
                zobrist_hash: 0,
            };
            board.zobrist_hash = board.generate_zobrist_hash();

            return Ok(board);
        }
    }

    // let (pawn, knight, bishop, rook, queen, king) = (0, 1, 2, 3, 4, 5)
    #[rustfmt::skip]
    pub fn piece_type_at(&self, sq_i: u8) -> Option<u8> {
        let sq_bb: u64 = 1 << sq_i;

        if sq_bb & self.pawns != 0 { return Some(0) };
        if sq_bb & self.knights != 0 { return Some(1) };
        if sq_bb & self.bishops != 0 { return Some(2) };
        if sq_bb & self.rooks != 0 { return Some(3) };
        if sq_bb & self.queens != 0 { return Some(4) };
        if sq_bb & self.kings != 0 { return Some(5) };
        return None;
    }

    // move: u16, most significant 6 digits = from square, next 6 = to_square, least sig 4 digits = move type flag
    // move type flags: normal = 0, capture = 1, castle = 2, en passant = 3, promotion (n,b,r,q) = (4,5,6,7) respectively, promo w/ capture (n,b,r,q) = (8,9,10,11) respectively
    #[rustfmt::skip]
    pub fn make_move(&mut self, move_int: u16) -> Result<UndoInfo, String> {
        let from_sqi = (move_int >> 10) as u8;
        let to_sqi = ((move_int >> 4) & 0b111111) as u8;
        let flag = (move_int & 0b1111) as u8;

        // collect unmake move data
        let prev_halfmove_clock = self.halfmove_clock;
        let prev_castling_rights = self.castling_rights;
        let prev_en_passant: Option<u8>;
        let capture_type: Option<u8>;

        if matches!(flag, 1 | 8 | 9 | 10 | 11) {
            // if move flag is a capture
            capture_type = self.piece_type_at(to_sqi);
        } else if flag == 3 {
            // if move flag is en passant
            capture_type = Some(0);
        } else {
            capture_type = None;
        }

        if self.en_passant == 0 {
            prev_en_passant = None;
        } else {
            prev_en_passant = Some(self.en_passant.trailing_zeros() as u8);
        }

        let undo_info = UndoInfo {
            halfmove_clock: prev_halfmove_clock,
            castling_rights: prev_castling_rights,
            en_passant_square: prev_en_passant,
            captured_type: capture_type,
            zobrist_hash: self.zobrist_hash,
        };

        // check if there is a piece of the color to move at the given square.
        let from_sq_bb: u64 = 1 << from_sqi;
        let to_sq_bb: u64 = 1 << to_sqi;
        let orig_piece_from_type = self.piece_type_at(from_sqi); // because piece from type can be changed for promotion handling.
        let mut piece_from_type = orig_piece_from_type;

        if piece_from_type.is_none() {
            return Err("No piece at given square.".to_string());
        } else {
            if self.side_to_move {
                if self.white_pieces & from_sq_bb == 0 {
                    return Err("Black piece cannot move on white's turn.".to_string());
                }
            } else {
                if self.black_pieces & from_sq_bb == 0 {
                    return Err("White piece cannot move on black's turn.".to_string());
                }
            }
        }

        // update board state: piece locations, en_passant, castling rights, halfmove clock, and zobrist hash.
        if self.en_passant != 0 {
            self.zobrist_hash ^= ZOBRIST_EP[(self.en_passant.trailing_zeros() % 8) as usize];
        }

        if piece_from_type == Some(0) {
            // pawn
            self.pawns &= !from_sq_bb;
            match flag { // promotion: add promotion type to to_sq.
                4 | 8 => {
                    self.knights |= to_sq_bb;
                    piece_from_type = Some(1); // changes piece type so that this piece is not accidentally removed during capture logic.
                },
                5 | 9 => {
                    self.bishops |= to_sq_bb;
                    piece_from_type = Some(2);
                },
                6 | 10 => {
                    self.rooks |= to_sq_bb;
                    piece_from_type = Some(3);
                },
                7 | 11 => {
                    self.queens |= to_sq_bb;
                    piece_from_type = Some(4);
                },
                _ => self.pawns |= to_sq_bb,
            }

            self.halfmove_clock = 0;
            if ((from_sqi as i16) - (to_sqi as i16)).abs() == 16 {
                if self.side_to_move {
                    self.en_passant = to_sq_bb >> 8;
                    self.zobrist_hash ^= ZOBRIST_EP[(self.en_passant.trailing_zeros() % 8) as usize]; // add new en passant file
                } else {
                    self.en_passant = to_sq_bb << 8;
                    self.zobrist_hash ^= ZOBRIST_EP[(self.en_passant.trailing_zeros() % 8) as usize]; // add new en passant file
                }
            } else {
                self.en_passant = 0;
            }
        } else if piece_from_type == Some(1) {
            // knight
            self.knights &= !from_sq_bb;
            self.knights |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
        } else if piece_from_type == Some(2) {
            self.bishops &= !from_sq_bb;
            self.bishops |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
        } else if piece_from_type == Some(3) {
            self.rooks &= !from_sq_bb;
            self.rooks |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;

            match from_sqi {
                0 => {
                    if self.castling_rights & 0b0100 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[1];
                    }
                    self.castling_rights &= !0b0100;
                },
                7 => {
                    if self.castling_rights & 0b1000 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[0];
                    }
                    self.castling_rights &= !0b1000;
                },
                56 => {
                    if self.castling_rights & 0b0001 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[3];
                    }
                    self.castling_rights &= !0b0001;
                },
                63 => {
                    if self.castling_rights & 0b0010 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[2];
                    }
                    self.castling_rights &= !0b0010;
                },
                _ => {}
            }
        } else if piece_from_type == Some(4) {
            self.queens &= !from_sq_bb;
            self.queens |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;
        } else {
            self.kings &= !from_sq_bb;
            self.kings |= to_sq_bb;
            self.en_passant = 0;
            self.halfmove_clock += 1;

            // handle castling.
            if flag == 2 {
                if self.side_to_move {
                    if to_sqi == 2 {
                        // c1
                        if self.castling_rights & 0b0100 != 0 {
                            self.rooks &= !0x0000000000000001;
                            self.zobrist_hash ^= ZOBRIST_PIECES[0][3];
                            self.rooks |= 0x00000000000000008;
                            self.zobrist_hash ^= ZOBRIST_PIECES[3][3];
                            self.white_pieces &= !0x0000000000000001;
                            self.white_pieces |= 0x00000000000000008;
                            if self.castling_rights & 0b1000 != 0 {
                                self.zobrist_hash ^= ZOBRIST_CASTLING[0];
                            }
                            self.castling_rights &= !0b1100;
                            self.zobrist_hash ^= ZOBRIST_CASTLING[1];
                        } else {
                            return Err("white cannot castle queenside.".to_string());
                        }
                    } else if to_sqi == 6 {
                        // g1
                        if self.castling_rights & 0b1000 != 0 {
                            self.rooks &= !0x0000000000000080;
                            self.zobrist_hash ^= ZOBRIST_PIECES[7][3];
                            self.rooks |= 0x00000000000000020;
                            self.zobrist_hash ^= ZOBRIST_PIECES[5][3];
                            self.white_pieces &= !0x0000000000000080;
                            self.white_pieces |= 0x00000000000000020;
                            if self.castling_rights & 0b0100 != 0 {
                                self.zobrist_hash ^= ZOBRIST_CASTLING[1];
                            }
                            self.castling_rights &= !0b1100;
                            self.zobrist_hash ^= ZOBRIST_CASTLING[0];
                        } else {
                            return Err("white cannot castle kingside.".to_string());
                        }
                    } else {
                        return Err("invalid castling move.".to_string());
                    }
                } else {
                    if to_sqi == 58 {
                        // c8
                        if self.castling_rights & 0b0001 != 0 {
                            self.rooks &= !0x0100000000000000;
                            self.zobrist_hash ^= ZOBRIST_PIECES[56][9];
                            self.rooks |= 0x0800000000000000;
                            self.zobrist_hash ^= ZOBRIST_PIECES[59][9];
                            self.black_pieces &= !0x0100000000000000;
                            self.black_pieces |= 0x0800000000000000;
                            if self.castling_rights & 0b0010 != 0 {
                                self.zobrist_hash ^= ZOBRIST_CASTLING[2];
                            }
                            self.castling_rights &= !0b0011;
                            self.zobrist_hash ^= ZOBRIST_CASTLING[3];
                        } else {
                            return Err("black cannot castle queenside.".to_string());
                        }
                    } else if to_sqi == 62 {
                        // g8
                        if self.castling_rights & 0b0010 != 0 {
                            self.rooks &= !0x8000000000000000;
                            self.zobrist_hash ^= ZOBRIST_PIECES[63][9];
                            self.rooks |= 0x2000000000000000;
                            self.zobrist_hash ^= ZOBRIST_PIECES[61][9];
                            self.black_pieces &= !0x8000000000000000;
                            self.black_pieces |= 0x2000000000000000;
                            if self.castling_rights & 0b0001 != 0 {
                                self.zobrist_hash ^= ZOBRIST_CASTLING[3];
                            }
                            self.castling_rights &= !0b0011;
                            self.zobrist_hash ^= ZOBRIST_CASTLING[2];
                        } else {
                            return Err("white cannot castle kingside.".to_string());
                        }
                    } else {
                        return Err("invalid castling move.".to_string());
                    }
                }
            } else {
                if self.side_to_move {
                    if self.castling_rights & 0b1000 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[0];
                    }
                    if self.castling_rights & 0b0100 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[1];
                    }
                    self.castling_rights &= !0b1100;


                } else {
                    if self.castling_rights & 0b0010 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[2];
                    }
                    if self.castling_rights & 0b0001 != 0 {
                        self.zobrist_hash ^= ZOBRIST_CASTLING[3];
                    }
                    self.castling_rights &= !0b0011;
                }
            }
        }

        if capture_type.is_some() {
            if flag == 3 { // en passant
                if piece_from_type != Some(0) {
                    return Err("Only pawns can capture en passant.".to_string());
                }
                self.pawns &= !(to_sq_bb >> 8);
                if self.side_to_move {
                    self.black_pieces &= !(to_sq_bb >> 8);
                    self.zobrist_hash ^= ZOBRIST_PIECES[(to_sqi - 8) as usize][6];
                } else {
                    self.white_pieces &= !(to_sq_bb << 8);
                    self.zobrist_hash ^= ZOBRIST_PIECES[(to_sqi + 8) as usize][0];
                }
            } else {
                self.halfmove_clock = 0;

                match capture_type {
                    Some(0) => if capture_type != piece_from_type {self.pawns &= !to_sq_bb},
                    Some(1) => if capture_type != piece_from_type {self.knights &= !to_sq_bb},
                    Some(2) => if capture_type != piece_from_type {self.bishops &= !to_sq_bb},
                    Some(3) => {
                        if capture_type != piece_from_type {self.rooks &= !to_sq_bb};
                        match to_sqi {
                            0 => {
                                if self.castling_rights & 0b0100 != 0 {
                                    self.zobrist_hash ^= ZOBRIST_CASTLING[1];
                                }
                                self.castling_rights &= !0b0100;
                            },
                            7 => {
                                if self.castling_rights & 0b1000 != 0 {
                                    self.zobrist_hash ^= ZOBRIST_CASTLING[0];
                                }
                                self.castling_rights &= !0b1000;
                            },
                            56 => {
                                if self.castling_rights & 0b0001 != 0 {
                                    self.zobrist_hash ^= ZOBRIST_CASTLING[3];
                                }
                                self.castling_rights &= !0b0001;
                            },
                            63 => {
                                if self.castling_rights & 0b0010 != 0 {
                                    self.zobrist_hash ^= ZOBRIST_CASTLING[2];
                                }
                                self.castling_rights &= !0b0010;
                            },
                            _ => {}
                        };
                    }
                    Some(4) => if capture_type != piece_from_type {self.queens &= !to_sq_bb},
                    _ => {}
                }
                if self.side_to_move {
                    self.black_pieces &= !to_sq_bb;
                } else {
                    self.white_pieces &= !to_sq_bb;
                }
            }
        }

        if self.side_to_move {
            self.white_pieces &= !from_sq_bb;
            self.zobrist_hash ^= ZOBRIST_PIECES[from_sqi as usize][orig_piece_from_type.unwrap() as usize];
            self.white_pieces |= to_sq_bb;
            self.zobrist_hash ^= ZOBRIST_PIECES[to_sqi as usize][piece_from_type.unwrap() as usize];
            self.side_to_move = false;
            self.zobrist_hash ^= ZOBRIST_SIDE;

            match capture_type {
                None => {},
                _ => if flag != 3 {self.zobrist_hash ^= ZOBRIST_PIECES[to_sqi as usize][capture_type.unwrap() as usize + 6]},
            }
        } else {
            self.black_pieces &= !from_sq_bb;
            self.zobrist_hash ^= ZOBRIST_PIECES[from_sqi as usize][(orig_piece_from_type.unwrap() as usize) + 6];
            self.black_pieces |= to_sq_bb;
            self.zobrist_hash ^= ZOBRIST_PIECES[to_sqi as usize][(piece_from_type.unwrap() + 6) as usize];
            self.side_to_move = true;
            self.zobrist_hash ^= ZOBRIST_SIDE;
            self.fullmove_number += 1;

            match capture_type {
                None => {},
                _ => if flag != 3 {self.zobrist_hash ^= ZOBRIST_PIECES[to_sqi as usize][capture_type.unwrap() as usize]},
            }
        }

        return Ok(undo_info);
    }

    pub fn unmake_move(&mut self, move_int: u16, undo_info: &UndoInfo) -> Result<(), String> {
        // verify valid move_int to undo
        // check that there is a piece on the to_square
        let from_sqi = (move_int >> 10) as u8;
        let to_sqi = ((move_int >> 4) & 0b111111) as u8;
        let flag = (move_int & 0b1111) as u8;

        if self.piece_type_at(to_sqi).is_none() {
            return Err("No piece at target square.".to_string());
        };

        // check that there is no piece at the from square
        let from_sq_bb: u64 = 1 << from_sqi;
        if (self.black_pieces | self.white_pieces) & from_sq_bb != 0 {
            return Err("Piece present at from square.".to_string());
        }

        // check that flag is in the right range
        if !((0..=11).contains(&(flag as i32))) {
            return Err("Invalid flag.".to_string());
        }

        // undo the move: update bitboards.

        // if promotion flag: remove to_sq piece and place pawn on from_sq
        let to_sq_bb: u64 = 1 << to_sqi;
        let to_sq_type = self.piece_type_at(to_sqi);
        if (4..=11).contains(&(flag as i32)) {
            match flag {
                4 | 8 => self.knights &= !to_sq_bb,
                5 | 9 => self.bishops &= !to_sq_bb,
                6 | 10 => self.rooks &= !to_sq_bb,
                7 | 1 => self.queens &= !to_sq_bb,
                _ => (),
            };
            self.pawns |= from_sq_bb;
        } else {
            // else: remove to_sq piece and place same piece type on from_sq
            match to_sq_type {
                Some(0) => {
                    self.pawns &= !to_sq_bb;
                    self.pawns |= from_sq_bb;
                }
                Some(1) => {
                    self.knights &= !to_sq_bb;
                    self.knights |= from_sq_bb;
                }
                Some(2) => {
                    self.bishops &= !to_sq_bb;
                    self.bishops |= from_sq_bb;
                }
                Some(3) => {
                    self.rooks &= !to_sq_bb;
                    self.rooks |= from_sq_bb;
                }
                Some(4) => {
                    self.queens &= !to_sq_bb;
                    self.queens |= from_sq_bb;
                }
                Some(5) => {
                    self.kings &= !to_sq_bb;
                    self.kings |= from_sq_bb;
                }
                _ => (),
            };
        }
        // if castle: place rook on relevant corner square and remove from castled location.
        if flag == 2 {
            if self.side_to_move {
                match to_sqi {
                    58 => {
                        self.rooks &= !0x0800000000000000;
                        self.rooks |= 0x0100000000000000;
                        self.black_pieces &= !0x0800000000000000;
                        self.black_pieces |= 0x0100000000000000;
                    }
                    62 => {
                        self.rooks &= !0x2000000000000000;
                        self.rooks |= 0x8000000000000000;
                        self.black_pieces &= !0x2000000000000000;
                        self.black_pieces |= 0x8000000000000000;
                    }
                    _ => return Err("target square not valid castling target.".to_string()),
                };
            } else {
                match to_sqi {
                    2 => {
                        self.rooks &= !0x0000000000000008;
                        self.rooks |= 1;
                        self.white_pieces &= !0x0000000000000008;
                        self.white_pieces |= 1;
                    }
                    6 => {
                        self.rooks &= !0x0000000000000020;
                        self.rooks |= 0x0000000000000080;
                        self.white_pieces &= !0x0000000000000020;
                        self.white_pieces |= 0x0000000000000080;
                    }
                    _ => return Err("target square not valid castling target.".to_string()),
                };
            }
        } else if flag == 3 {
            // if en passant flag: place pawn of opposite color on correct square
            if self.side_to_move {
                self.pawns |= to_sq_bb << 8;
                self.white_pieces |= to_sq_bb << 8;
            } else {
                self.pawns |= to_sq_bb >> 8;
                self.black_pieces |= to_sq_bb >> 8;
            }
        } else if [1, 8, 9, 10, 11].contains(&flag) {
            // else if any other capture flag: place capture type of color to move on to square
            match undo_info.captured_type {
                Some(0) => self.pawns |= to_sq_bb,
                Some(1) => self.knights |= to_sq_bb,
                Some(2) => self.bishops |= to_sq_bb,
                Some(3) => self.rooks |= to_sq_bb,
                Some(4) => self.queens |= to_sq_bb,
                Some(5) => return Err("King cannot be captured.".to_string()),
                None => {
                    return Err(
                        "If move flag is a capture, capture type cannot be None.".to_string()
                    )
                }
                _ => return Err("Invalid piece type.".to_string()),
            }
            if self.side_to_move {
                self.white_pieces |= to_sq_bb;
            } else {
                self.black_pieces |= to_sq_bb;
            }
        }
        // update color bitboards (same for all cases: remove to_sq and add from_sq for opposite color as to_move)
        if self.side_to_move {
            self.black_pieces &= !to_sq_bb;
            self.black_pieces |= from_sq_bb;
        } else {
            self.white_pieces &= !to_sq_bb;
            self.white_pieces |= from_sq_bb;
        }

        // set board state = undo_info
        (self.castling_rights, self.halfmove_clock) =
            (undo_info.castling_rights, undo_info.halfmove_clock);
        match undo_info.en_passant_square {
            None => self.en_passant = 0,
            _ => self.en_passant = 1 << undo_info.en_passant_square.unwrap(),
        }
        // if white is to move: full_move counter -= 1, set black to move
        // else: set white to move
        if self.side_to_move {
            self.fullmove_number -= 1;
            self.side_to_move = false;
        } else {
            self.side_to_move = true;
        }

        self.zobrist_hash = undo_info.zobrist_hash;
        return Ok(());
    }

    pub fn generate_zobrist_hash(&mut self) -> u64 {
        // index zobrist hashes as follows:
        // ZOBRIST_PIECES[square 0..=63][piece 0..=11]
        // ZOBRIST_CASTLING[castling right 0..=3]
        // ZOBRIST_EP[en passant file 0..=7]
        // ZOBRIST_SIDE when white to move
        let mut hash: u64 = 0;

        // xor with piece locoation keys
        for (piece_type, piece_bb) in [
            self.pawns,
            self.knights,
            self.bishops,
            self.rooks,
            self.queens,
            self.kings,
        ]
        .iter()
        .enumerate()
        {
            let mut bb = *piece_bb;
            while bb != 0 {
                let square = bb.trailing_zeros() as usize;
                let square_bb = 1 << square;
                if square_bb & self.white_pieces != 0 {
                    hash ^= ZOBRIST_PIECES[square][piece_type];
                } else {
                    hash ^= ZOBRIST_PIECES[square][piece_type + 6];
                }

                bb &= bb - 1;
            }
        }

        // xor with castling rights keys
        let mut castling_bb = self.castling_rights;
        while castling_bb != 0 {
            let cr = castling_bb.trailing_zeros() as usize;
            hash ^= ZOBRIST_CASTLING[3 - cr];
            castling_bb &= castling_bb - 1;
        }

        // xor with en passant file
        if self.en_passant != 0 {
            hash ^= ZOBRIST_EP[(self.en_passant.trailing_zeros() % 8) as usize];
        }

        if self.side_to_move {
            hash ^= ZOBRIST_SIDE;
        }

        return hash;
    }
}

/// Verifies if a string is a valid FEN string.
pub fn verify_fen(fen: &str) -> bool {
    // Split whitespace and create vector of fen components.
    let fen_components: Vec<&str> = fen.split_whitespace().collect();
    // fen_components indices:
    //      [0] = board position
    //      [1] = color to move
    //      [2] = castling rights
    //      [3] = en passant target square
    //      [4] = halfmove clock
    //      [5] = fullmove number
    // Steps to verify:
    // Piece locations:
    //      Split by '/' characters. If len != 8, return false.

    let ranks: Vec<&str> = fen_components[0].split('/').collect();
    if ranks.len() != 8 {
        return false;
    }

    //      For each line, take count of alphabetical characters and add value of digits. If for any line, sum != 8, return false.
    //      For efficiency, only check if characters are in "PNBRQKpnbrqk12345678".
    //      Check for 1 white and 1 black king while we're at it.
    let mut white_king_count: u8 = 0;
    let mut black_king_count: u8 = 0;

    for rank in &ranks {
        let mut square_count = 0;

        for ch in rank.chars() {
            if "kqrbnpKQRBNP".contains(ch) {
                square_count += 1;
                if ch == 'K' {
                    white_king_count += 1;
                } else if ch == 'k' {
                    black_king_count += 1;
                }
            } else if ch >= '1' && ch <= '8' {
                square_count += ch.to_digit(10).unwrap();
            } else {
                return false;
            }
        }
        if square_count != 8 {
            return false;
        }
    }

    if white_king_count != 1 {
        return false;
    }
    if black_king_count != 1 {
        return false;
    }

    // Color to move:
    //      If not in "wb" and length 1, return false.
    if !"wb".contains(fen_components[1]) || fen_components[1].len() != 1 {
        return false;
    }

    // Castling rights:
    //      If not in "KQkq-" or not len 1..4 return false.
    if !fen_components[2].chars().all(|c| "KQkq-".contains(c)) || fen_components[2].len() > 4 {
        return false;
    }
    //      Castling rights cannot be both empty and not empty.
    if fen_components[2].contains("-") && fen_components[2] != "-" {
        return false;
    }

    // En passant target square:
    //      If len > 2, return false.
    if fen_components[3].len() > 2 {
        return false;
    }
    //      If not '-':
    //          If first char not "a".."h" or second char not '6' or '3':
    //              return false.
    if fen_components[3] != "-" {
        let mut en_passant_chars = fen_components[3].chars();
        let en_passant_char1 = en_passant_chars.next().unwrap(); // safe to unwrap because the component cannot be empty (white space erased, each component has len > 1.)
        if en_passant_char1 < 'a' || en_passant_char1 > 'h' {
            return false;
        }
        // Use Some() in case there is no second char, it returns false.
        if let Some(en_passant_char2) = en_passant_chars.next() {
            if en_passant_char2 != '6' && en_passant_char2 != '3' {
                return false;
            }
        } else {
            return false;
        }
    }

    // Halfmove clock:
    //      If not a non-negative integer, return false.
    if !fen_components[4].parse::<u16>().is_ok() {
        return false;
    }
    // Fullmove number:
    //      If not a positive integer, return false.
    if !fen_components[5].parse::<u16>().is_ok() || fen_components[5] == "0" {
        return false;
    }

    // Finally: return true.
    return true;
}

#[cfg(test)]
mod tests {
    use crate::utils::encode_move;

    use super::*;

    #[test]
    fn test_initialize_board() {
        let board = ChessBoard::initialize();
        assert_eq!(board.pawns, 0x00FF00000000FF00);
        assert_eq!(board.knights, 0x4200000000000042);
        assert_eq!(board.bishops, 0x2400000000000024);
        assert_eq!(board.rooks, 0x8100000000000081);
        assert_eq!(board.queens, 0x0800000000000008);
        assert_eq!(board.kings, 0x1000000000000010);
        assert_eq!(board.white_pieces, 0x000000000000FFFF);
        assert_eq!(board.black_pieces, 0xFFFF000000000000);
        assert_eq!(board.side_to_move, true);
        assert_eq!(board.en_passant, 0);
        assert_eq!(board.castling_rights, 0b1111);
        assert_eq!(board.halfmove_clock, 0);
        assert_eq!(board.fullmove_number, 1);
    }

    #[test]
    fn test_empty_board() {
        let board = ChessBoard::empty();
        assert_eq!(board.pawns, 0);
        assert_eq!(board.knights, 0);
        assert_eq!(board.bishops, 0);
        assert_eq!(board.rooks, 0);
        assert_eq!(board.queens, 0);
        assert_eq!(board.kings, 0);
        assert_eq!(board.white_pieces, 0);
        assert_eq!(board.black_pieces, 0);
        assert_eq!(board.side_to_move, true);
        assert_eq!(board.en_passant, 0);
        assert_eq!(board.castling_rights, 0);
        assert_eq!(board.halfmove_clock, 0);
        assert_eq!(board.fullmove_number, 1);
    }

    #[test]
    fn test_verify_fen() {
        let starting_fen = verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"); // Starting position.
        assert_eq!(starting_fen, true);

        let empty_fen = verify_fen("8/8/8/8/8/8/8/8 w - - 0 1"); // No kings.
        assert_eq!(empty_fen, false);

        let random_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R1BQR1K1 w - - 1 18"); // Valid FEN of a middlegame.
        assert_eq!(random_fen, true);

        let too_many_ranks_false_fen = verify_fen(
            "1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R1BQR1K1/1rbq1rk1 w - - 1 18", // 9 ranks != 8.
        );
        assert_eq!(too_many_ranks_false_fen, false);

        let too_few_ranks_false_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1 w - - 1 18"); // 7 ranks != 7.
        assert_eq!(too_few_ranks_false_fen, false);

        let too_many_squares_false_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R2BQR1K1 w - - 1 18"); // 1st rank has too many squares.
        assert_eq!(too_many_squares_false_fen, false);

        let too_few_squares_false_fen =
            verify_fen("1rbq1rk1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/RBQR1K1 w - - 1 18"); // 1st rank has too few squares.
        assert_eq!(too_few_squares_false_fen, false);

        let wrong_characters_false_fen =
            verify_fen("1rbq1ak1/5pbp/2pNn1p1/p1Pn4/Pp1P4/1B3N1P/1P3PP1/R1BQR1K1 w - - 1 18"); // 8th rank contains the char 'a'.
        assert_eq!(wrong_characters_false_fen, false);

        let invalid_color_to_move_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR v KQkq - 0 1"); // Starting position with invalid color to move - 'v'.
        assert_eq!(invalid_color_to_move_false_fen, false);

        let color_to_move_len_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR ww KQkq - 0 1"); // Starting position with invalid color to move - 'ww'.
        assert_eq!(color_to_move_len_false_fen, false);

        let castle_rights_len_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkqK - 0 1"); // Castling rights too long.
        assert_eq!(castle_rights_len_false_fen, false);

        let castle_invalid_char_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQka - 0 1"); // Invalid character in castling rights.
        assert_eq!(castle_invalid_char_false_fen, false);

        let castling_contradiction_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w -K - 0 1");
        assert_eq!(castling_contradiction_false_fen, false);

        let invalid_len_en_passant_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a33 0 1");
        assert_eq!(invalid_len_en_passant_false_fen, false);

        let invalid_char_en_passant_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq i3 0 1");
        assert_eq!(invalid_char_en_passant_false_fen, false);

        let invalid_num_en_passant_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a2 0 1");
        assert_eq!(invalid_num_en_passant_false_fen, false);

        let invalid_halfmove_negative_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1");
        assert_eq!(invalid_halfmove_negative_false_fen, false);

        let invalid_halfmove_char_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - a 1");
        assert_eq!(invalid_halfmove_char_false_fen, false);

        let invalid_fullmove_negative_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 -1");
        assert_eq!(invalid_fullmove_negative_false_fen, false);

        let invalid_fullmove_zero_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0");
        assert_eq!(invalid_fullmove_zero_false_fen, false);

        let invalid_fullmove_char_false_fen =
            verify_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 a");
        assert_eq!(invalid_fullmove_char_false_fen, false);

        let random_fen2 =
            verify_fen("rnbqk2r/pp2nppp/2pbp3/3p4/3P4/1P1BPN2/PBP2PPP/RN1QK2R b KQkq - 2 6");
        assert_eq!(random_fen2, true);
    }

    #[test]
    fn test_intialize_from_fen() {
        let starting_board = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(starting_board.pawns, 0x00FF00000000FF00);
        assert_eq!(starting_board.knights, 0x4200000000000042);
        assert_eq!(starting_board.bishops, 0x2400000000000024);
        assert_eq!(starting_board.rooks, 0x8100000000000081);
        assert_eq!(starting_board.queens, 0x0800000000000008);
        assert_eq!(starting_board.kings, 0x1000000000000010);
        assert_eq!(starting_board.white_pieces, 0x000000000000FFFF);
        assert_eq!(starting_board.black_pieces, 0xFFFF000000000000);
        assert_eq!(starting_board.side_to_move, true);
        assert_eq!(starting_board.en_passant, 0);
        assert_eq!(starting_board.castling_rights, 0b1111);
        assert_eq!(starting_board.halfmove_clock, 0);
        assert_eq!(starting_board.fullmove_number, 1);

        let board1 = ChessBoard::initialize_from_fen(
            "r2qkbnr/1p2pppp/p1n5/3p4/3P1B2/3Q1N2/PPP2PPP/RN2K1R1 b Qkq - 2 8",
        )
        .unwrap();
        assert_eq!(
            board1.pawns,
            0b0000000011110010000000010000100000001000000000001110011100000000
        );
        assert_eq!(
            board1.knights,
            0b0100000000000000000001000000000000000000001000000000000000000010
        );
        assert_eq!(
            board1.bishops,
            0b0010000000000000000000000000000000100000000000000000000000000000
        );
        assert_eq!(
            board1.rooks,
            0b1000000100000000000000000000000000000000000000000000000001000001
        );
        assert_eq!(
            board1.queens,
            0b0000100000000000000000000000000000000000000010000000000000000000
        );
        assert_eq!(
            board1.kings,
            0b0001000000000000000000000000000000000000000000000000000000010000
        );
        assert_eq!(
            board1.white_pieces,
            0b0000000000000000000000000000000000101000001010001110011101010011
        );
        assert_eq!(
            board1.black_pieces,
            0b1111100111110010000001010000100000000000000000000000000000000000
        );
        assert_eq!(board1.side_to_move, false);
        assert_eq!(board1.en_passant, 0);
        assert_eq!(board1.castling_rights, 0b0111);
        assert_eq!(board1.halfmove_clock, 2);
        assert_eq!(board1.fullmove_number, 8);

        let board2 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/6pP/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1",
        )
        .unwrap();
        assert_eq!(
            board2.pawns,
            0b0000000011111111000000000000000011000000000000000111111100000000
        );
        assert_eq!(board2.knights, 0x4200000000000042);
        assert_eq!(board2.bishops, 0x2400000000000024);
        assert_eq!(board2.rooks, 0x8100000000000081);
        assert_eq!(board2.queens, 0x0800000000000008);
        assert_eq!(board2.kings, 0x1000000000000010);
        assert_eq!(
            board2.white_pieces,
            0b0000000000000000000000000000000010000000000000000111111111111111
        );
        assert_eq!(
            board2.black_pieces,
            0b1111111111111111000000000000000001000000000000000000000000000000
        );
        assert_eq!(board2.side_to_move, false);
        assert_eq!(
            board2.en_passant,
            0b0000000000000000000000000000000000000000100000000000000000000000
        );
        assert_eq!(board2.castling_rights, 0b1111);
        assert_eq!(board2.halfmove_clock, 0);
        assert_eq!(board2.fullmove_number, 1);
    }

    #[test]
    fn test_piece_type_at() {
        let board1 = ChessBoard::initialize();
        let mut type_array: [Option<u8>; 64] = [None; 64];
        for i in 0..64 {
            let piece_type = board1.piece_type_at(i);
            type_array[i as usize] = piece_type;
        }
        let correct_type_array: [Option<u8>; 64] = [
            Some(3),
            Some(1),
            Some(2),
            Some(4),
            Some(5),
            Some(2),
            Some(1),
            Some(3),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(3),
            Some(1),
            Some(2),
            Some(4),
            Some(5),
            Some(2),
            Some(1),
            Some(3),
        ];

        assert_eq!(type_array, correct_type_array);
    }

    #[test]
    pub fn test_make_move() {
        let mut board1 = ChessBoard::initialize();
        let board1_zob = board1.zobrist_hash;

        let move1_int = 0b0011000111000000; // e2-e4
        let move1_undo = board1.make_move(move1_int);

        let correct_resulting_board1 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        )
        .unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board1_zob,
        });

        assert_eq!(board1, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let board1_zob = board1.zobrist_hash;
        let move2_int = 0b1111101011010000; // g8-f6
        let move2_undo = board1.make_move(move2_int);

        let correct_resulting_board2 = ChessBoard::initialize_from_fen(
            "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2",
        )
        .unwrap();
        let correct_undo2: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: Some(20),
            captured_type: None,
            zobrist_hash: board1_zob,
        });

        assert_eq!(board1, correct_resulting_board2);
        assert_eq!(move2_undo, correct_undo2);

        let move3_int = 0b0111001001000000; // e4-e5
        board1.make_move(move3_int);
        let move4_int = 0b1101111011110000; // h7-h6
        board1.make_move(move4_int);

        let board1_zob = board1.zobrist_hash;
        let move5_int = 0b1001001011010001; // e5 x f6;
        let move5_undo = board1.make_move(move5_int);
        let correct_resulting_board5 = ChessBoard::initialize_from_fen(
            "rnbqkb1r/ppppppp1/5P1p/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3",
        )
        .unwrap();

        let correct_undo5: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: None,
            captured_type: Some(1),
            zobrist_hash: board1_zob,
        });

        assert_eq!(board1, correct_resulting_board5);
        assert_eq!(move5_undo, correct_undo5);

        let mut board2 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1",
        )
        .unwrap();
        let board2_zob = board2.zobrist_hash;

        let move1_int = 0b1001001010110011; // e5 x d5 en passant
        let move1_undo = board2.make_move(move1_int);

        let correct_resulting_board1 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
        )
        .unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: Some(43),
            captured_type: Some(0),
            zobrist_hash: board2_zob,
        });

        assert_eq!(board2, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board3 = ChessBoard::initialize_from_fen(
            "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1",
        )
        .unwrap();
        let board3_zob = board3.zobrist_hash;

        let move1_int = 0b0001000001100010; // e1 - g1 (white kingside castle)
        let move1_undo = board3.make_move(move1_int);

        let correct_resulting_board1 = ChessBoard::initialize_from_fen(
            "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 1",
        )
        .unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board3_zob,
        });

        assert_eq!(board3, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let board3_zob = board3.zobrist_hash;
        let move2_int = 0b1111001111100010; // e8 - g8 (black kingside castle)
        let move2_undo = board3.make_move(move2_int);

        let correct_resulting_board2 = ChessBoard::initialize_from_fen(
            "rnbq1rk1/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 w - - 2 2",
        )
        .unwrap();
        let correct_undo2: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 1,
            castling_rights: 0b11,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board3_zob,
        });

        assert_eq!(board3, correct_resulting_board2);
        assert_eq!(move2_undo, correct_undo2);

        let mut board4 = ChessBoard::initialize_from_fen(
            "r3kbnr/pppqpppp/2n5/3p1b2/3P1B2/2N1P3/PPPQ1PPP/R3KBNR b KQkq - 0 1",
        )
        .unwrap();
        let board4_zob = board4.zobrist_hash;

        let move1_int = 0b1111001110100010; // e8 - c8 (black queenside castle)
        let move1_undo = board4.make_move(move1_int);

        let correct_resulting_board1 = ChessBoard::initialize_from_fen(
            "2kr1bnr/pppqpppp/2n5/3p1b2/3P1B2/2N1P3/PPPQ1PPP/R3KBNR w KQ - 1 2",
        )
        .unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board4_zob,
        });

        assert_eq!(board4, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let board4_zob = board4.zobrist_hash;
        let move2_int = 0b0001000000100010; // e1 - c1 (white queenside castle)
        let move2_undo = board4.make_move(move2_int);

        let correct_resulting_board2 = ChessBoard::initialize_from_fen(
            "2kr1bnr/pppqpppp/2n5/3p1b2/3P1B2/2N1P3/PPPQ1PPP/2KR1BNR b - - 2 2",
        )
        .unwrap();
        let correct_undo2: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 1,
            castling_rights: 0b1100,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board4_zob,
        });

        assert_eq!(board4, correct_resulting_board2);
        assert_eq!(move2_undo, correct_undo2);

        let mut board5 =
            ChessBoard::initialize_from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let board5_zob = board5.zobrist_hash;

        let move1_int = 0b0000000000010000; // a1 - b1
        let move1_undo = board5.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 1 1").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board5_zob,
        });

        assert_eq!(board5, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let board5_zob = board5.zobrist_hash;
        let move2_int = 0b1110001110010000; // a8 - b8
        let move2_undo = board5.make_move(move2_int);

        let correct_resulting_board2 =
            ChessBoard::initialize_from_fen("1r2k2r/8/8/8/8/8/8/1R2K2R w Kk - 2 2").unwrap();
        let correct_undo2: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 1,
            castling_rights: 0b1011,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board5_zob,
        });

        assert_eq!(board5, correct_resulting_board2);
        assert_eq!(move2_undo, correct_undo2);

        let board5_zob = board5.zobrist_hash;
        let move3_int = 0b0001110001100000; // h1 - g1
        let move3_undo = board5.make_move(move3_int);

        let correct_resulting_board3 =
            ChessBoard::initialize_from_fen("1r2k2r/8/8/8/8/8/8/1R2K1R1 b k - 3 2").unwrap();
        let correct_undo3: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 2,
            castling_rights: 0b1010,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board5_zob,
        });

        assert_eq!(board5, correct_resulting_board3);
        assert_eq!(move3_undo, correct_undo3);

        let board5_zob = board5.zobrist_hash;
        let move4_int = 0b1111111111100000; // h8 - g8
        let move4_undo = board5.make_move(move4_int);

        let correct_resulting_board4 =
            ChessBoard::initialize_from_fen("1r2k1r1/8/8/8/8/8/8/1R2K1R1 w - - 4 3").unwrap();
        let correct_undo4: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 3,
            castling_rights: 0b0010,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board5_zob,
        });

        assert_eq!(board5, correct_resulting_board4);
        assert_eq!(move4_undo, correct_undo4);

        let mut board6 =
            ChessBoard::initialize_from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let board6_zob = board6.zobrist_hash;

        let move1_int = 0b0000001110000001; // a1 x a8
        let move1_undo = board6.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("R3k2r/8/8/8/8/8/8/4K2R b Kk - 0 1").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0b1111,
            en_passant_square: None,
            captured_type: Some(3),
            zobrist_hash: board6_zob,
        });

        assert_eq!(board6, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board7 = ChessBoard::initialize_from_fen("8/P7/8/8/8/8/8/K1k5 w - - 0 1").unwrap();
        let board7_zob = board7.zobrist_hash;

        let move1_int = 0b1100001110000111; // a7 - a8 promote to q
        let move1_undo = board7.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("Q7/8/8/8/8/8/8/K1k5 b - - 0 1").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board7_zob,
        });

        assert_eq!(board7, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board7 = ChessBoard::initialize_from_fen("8/P7/8/8/8/8/8/K1k5 w - - 0 1").unwrap();
        let board7_zob = board7.zobrist_hash;

        let move1_int = 0b1100001110000110; // a7 - a8 promote to r
        let move1_undo = board7.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("R7/8/8/8/8/8/8/K1k5 b - - 0 1").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board7_zob,
        });

        assert_eq!(board7, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board7 = ChessBoard::initialize_from_fen("8/P7/8/8/8/8/8/K1k5 w - - 0 1").unwrap();
        let board7_zob = board7.zobrist_hash;

        let move1_int = 0b1100001110000101; // a7 - a8 promote to b
        let move1_undo = board7.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("B7/8/8/8/8/8/8/K1k5 b - - 0 1").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board7_zob,
        });

        assert_eq!(board7, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board7 = ChessBoard::initialize_from_fen("8/P7/8/8/8/8/8/K1k5 w - - 0 1").unwrap();
        let board7_zob = board7.zobrist_hash;

        let move1_int = 0b1100001110000100; // a7 - a8 promote to n
        let move1_undo = board7.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("N7/8/8/8/8/8/8/K1k5 b - - 0 1").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board7_zob,
        });

        assert_eq!(board7, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board8 = ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/p7/8 b - - 0 1").unwrap();
        let board8_zob = board8.zobrist_hash;

        let move1_int = 0b0010000000000111; // a2 - a1 promote to q
        let move1_undo = board8.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/8/q7 w - - 0 2").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board8_zob,
        });

        assert_eq!(board8, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board8 = ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/p7/8 b - - 0 1").unwrap();
        let board8_zob = board8.zobrist_hash;

        let move1_int = 0b0010000000000110; // a2 - a1 promote to r
        let move1_undo = board8.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/8/r7 w - - 0 2").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board8_zob,
        });

        assert_eq!(board8, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board8 = ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/p7/8 b - - 0 1").unwrap();
        let board8_zob = board8.zobrist_hash;

        let move1_int = 0b0010000000000101; // a2 - a1 promote to b
        let move1_undo = board8.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/8/b7 w - - 0 2").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board8_zob,
        });

        assert_eq!(board8, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board8 = ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/p7/8 b - - 0 1").unwrap();
        let board8_zob = board8.zobrist_hash;

        let move1_int = 0b0010000000000100; // a2 - a1 promote to n
        let move1_undo = board8.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/8/n7 w - - 0 2").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: None,
            zobrist_hash: board8_zob,
        });

        assert_eq!(board8, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board9 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/p7/1R6 b - - 0 1").unwrap();
        let board9_zob = board9.zobrist_hash;

        let move1_int = 0b0010000000011011; // a2 x b1 promote to q
        let move1_undo = board9.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/8/1q6 w - - 0 2").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: Some(3),
            zobrist_hash: board9_zob,
        });

        assert_eq!(board9, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);

        let mut board9 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/p7/1R6 b - - 0 1").unwrap();
        let board9_zob = board9.zobrist_hash;

        let move1_int = 0b0010000000011010; // a2 x b1 promote to r
        let move1_undo = board9.make_move(move1_int);

        let correct_resulting_board1 =
            ChessBoard::initialize_from_fen("k1K5/8/8/8/8/8/8/1r6 w - - 0 2").unwrap();
        let correct_undo1: Result<UndoInfo, String> = Ok(UndoInfo {
            halfmove_clock: 0,
            castling_rights: 0,
            en_passant_square: None,
            captured_type: Some(3),
            zobrist_hash: board9_zob,
        });

        assert_eq!(board9, correct_resulting_board1);
        assert_eq!(move1_undo, correct_undo1);
    }

    #[test]
    pub fn test_unmake_move() {
        let mut board1 = ChessBoard::initialize();
        let board1_copy = board1.clone();

        let move1_int = 0b0011000111000000; // e2-e4
        let move1_undo_info = board1.make_move(move1_int).unwrap();
        let move1_undo_result = board1.unmake_move(move1_int, &move1_undo_info);

        assert_eq!(move1_undo_result, Ok(()));
        assert_eq!(board1, board1_copy);

        let mut board2 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        )
        .unwrap();
        let board2_copy = board2.clone();

        let move2_int = 0b1100001000000000; // a7 - a5
        let move2_undo_info = board2.make_move(move2_int).unwrap();
        let move2_undo_result = board2.unmake_move(move2_int, &move2_undo_info);

        assert_eq!(move2_undo_result, Ok(()));
        assert_eq!(board2, board2_copy);

        let mut board3 = ChessBoard::initialize_from_fen(
            "rnbqkb1r/pppppppp/8/5n2/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
        )
        .unwrap();
        let board3_copy = board3.clone();

        let move3_int = 0b0111001001010001; // e4 x f5
        let move3_undo_info = board3.make_move(move3_int).unwrap();
        let move3_undo_result = board3.unmake_move(move3_int, &move3_undo_info);

        assert_eq!(move3_undo_result, Ok(()));
        assert_eq!(board3, board3_copy);

        let mut board4 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/ppp1pppp/8/8/3pP3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        )
        .unwrap();
        let board4_copy = board4.clone();

        let move4_int = 0b0110110101000011; // d4 x e3 en passant
        let move4_undo_info = board4.make_move(move4_int).unwrap();
        let move4_undo_result = board4.unmake_move(move4_int, &move4_undo_info);

        assert_eq!(move4_undo_result, Ok(()));
        assert_eq!(board4, board4_copy);

        let mut board5 = ChessBoard::initialize_from_fen("k7/7P/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let board5_copy = board5.clone();

        let move5_int = 0b1101111111110100; // h7 - h8 promote to knight
        let move5_undo_info = board5.make_move(move5_int).unwrap();
        let move5_undo_result = board5.unmake_move(move5_int, &move5_undo_info);

        assert_eq!(move5_undo_result, Ok(()));
        assert_eq!(board5, board5_copy);

        let mut board5 = ChessBoard::initialize_from_fen("k7/7P/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let board5_copy = board5.clone();

        let move5_int = 0b1101111111110101; // h7 - h8 promote to bishop
        let move5_undo_info = board5.make_move(move5_int).unwrap();
        let move5_undo_result = board5.unmake_move(move5_int, &move5_undo_info);

        assert_eq!(move5_undo_result, Ok(()));
        assert_eq!(board5, board5_copy);

        let mut board5 = ChessBoard::initialize_from_fen("k7/7P/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let board5_copy = board5.clone();

        let move5_int = 0b1101111111110110; // h7 - h8 promote to rook
        let move5_undo_info = board5.make_move(move5_int).unwrap();
        let move5_undo_result = board5.unmake_move(move5_int, &move5_undo_info);

        assert_eq!(move5_undo_result, Ok(()));
        assert_eq!(board5, board5_copy);

        let mut board5 = ChessBoard::initialize_from_fen("k7/7P/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let board5_copy = board5.clone();

        let move5_int = 0b1101111111110111; // h7 - h8 promote to queen
        let move5_undo_info = board5.make_move(move5_int).unwrap();
        let move5_undo_result = board5.unmake_move(move5_int, &move5_undo_info);

        assert_eq!(move5_undo_result, Ok(()));
        assert_eq!(board5, board5_copy);

        let mut board6 = ChessBoard::initialize_from_fen("k5q1/7P/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let board6_copy = board6.clone();

        let move6_int = 0b1101111111101011; // h7 x g8 promote to queen
        let move6_undo_info = board6.make_move(move6_int).unwrap();
        let move6_undo_result = board6.unmake_move(move6_int, &move6_undo_info);

        assert_eq!(move6_undo_result, Ok(()));
        assert_eq!(board6, board6_copy);

        let mut board7 =
            ChessBoard::initialize_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        let board7_copy = board7.clone();

        let move7_int = 0b0001000001100010; // e1 - g1 white kingside castles
        let move7_undo_info = board7.make_move(move7_int).unwrap();
        let move7_undo_result = board7.unmake_move(move7_int, &move7_undo_info);

        assert_eq!(move7_undo_result, Ok(()));
        assert_eq!(board7, board7_copy);

        let mut board7 =
            ChessBoard::initialize_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        let board7_copy = board7.clone();

        let move7_int = 0b0001000000100010; // e1 - c1 white queenside castles
        let move7_undo_info = board7.make_move(move7_int).unwrap();
        let move7_undo_result = board7.unmake_move(move7_int, &move7_undo_info);

        assert_eq!(move7_undo_result, Ok(()));
        assert_eq!(board7, board7_copy);

        let mut board8 =
            ChessBoard::initialize_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1")
                .unwrap();
        let board8_copy = board8.clone();

        let move8_int = 0b1111001111100010; // e8 - g8 black kingside castles
        let move8_undo_info = board8.make_move(move8_int).unwrap();
        let move8_undo_result = board8.unmake_move(move8_int, &move8_undo_info);

        assert_eq!(move8_undo_result, Ok(()));
        assert_eq!(board8, board8_copy);

        let mut board8 =
            ChessBoard::initialize_from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1")
                .unwrap();
        let board8_copy = board8.clone();

        let move8_int = 0b1111001110100010; // e8 - c8 black queenside castles
        let move8_undo_info = board8.make_move(move8_int).unwrap();
        let move8_undo_result = board8.unmake_move(move8_int, &move8_undo_info);

        assert_eq!(move8_undo_result, Ok(()));
        assert_eq!(board8, board8_copy);
    }

    fn test_generate_zobrist_hash() {
        let mut board1 = ChessBoard::initialize();
        board1.make_move(encode_move(8, 16, 0));
        let mut board2 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1",
        )
        .unwrap();
        assert_eq!(
            board1.generate_zobrist_hash(),
            board2.generate_zobrist_hash()
        );

        let mut board3 = ChessBoard::initialize();
        let mut board4 = ChessBoard::initialize_from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        )
        .unwrap();
        assert_ne!(
            board3.generate_zobrist_hash(),
            board4.generate_zobrist_hash()
        );
    }
}
