# oxi_chess_lib

A chess library written in Rust, used by [Greenseer](https://github.com/jnbradley828/Greenseer), a Rust chess engine.

## Board Representation

The board is stored as a set of `u64` bitboards — one per piece type (pawns, knights, bishops, rooks, queens, kings) plus two color masks (white/black). With LSB = a1, queries like "where are the white bishops?" become a single `&` operation. All 64 pieces of state fit in 8 integers.

## Move Encoding

Moves are packed into a `u16`: 6 bits for the from-square, 6 for the to-square, and 4 flag bits encoding the move type (normal, capture, castle, en passant, promotion, promotion-capture).

## Move Generation & Legality

Attack tables for pawns, knights, kings, and sliding pieces are precomputed at compile time using `const fn`. Legal move generation works by producing pseudo-legal moves, then testing each one by making the move and checking whether the king is left in check. Moves that leave the king in check are discarded.

## Make / Unmake

`make_move()` returns an `UndoInfo` struct capturing the previous halfmove clock, castling rights, en passant square, captured piece type, and Zobrist hash. `unmake_move()` uses this to restore the board exactly, avoiding the need to copy the full board state on every node.

## Zobrist Hashing

Each position is identified by a Zobrist hash, incrementally updated on every make/unmake. Keys cover piece locations (64 squares x 12 piece types), castling rights, en passant file, and side to move. The `ChessGame` layer maintains a `HashMap<u64, u8>` of position counts, enabling **threefold repetition** detection in O(1).

## Game Rules

The library detects all standard draw and win conditions:
- Checkmate and stalemate
- Threefold repetition (via Zobrist hash counts)
- 50-move rule (halfmove clock)
- Insufficient material (K vs K, K+B vs K, K+N vs K, same-color bishops)

## FEN Support

Boards can be initialized from FEN strings via `ChessBoard::initialize_from_fen()`, with full validation through `verify_fen()`. UCI move notation is also supported at the game level via `make_move_from_uci()`.

## Correctness

Move generation is validated with **perft** tests — node counts at fixed depths are compared against known-correct values for standard positions.
