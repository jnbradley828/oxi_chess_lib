import random


def main():
    keys = [hex(random.getrandbits(64)) for _ in range(781)]

    print("// ZOBRIST_PIECES[square][piece] = [P, N, B, R, Q, K, p, n, b, r, q, k]")
    print("pub const ZOBRIST_PIECES: [[u64; 12]; 64] = [")
    for square in range(64):
        print("    [")
        for piece in range(12):
            print(f"        {keys[square * 12 + piece]},")
        print("    ],")
    print("];")

    print(
        "\n// ZOBRIST_CASTLING[0-3] = [white kingside, white queenside, black kingside, black queenside]"
    )
    print("pub const ZOBRIST_CASTLING: [u64; 4] = [")
    for key in keys[768:772]:
        print(f"    {key},")
    print("];")

    print("\n// ZOBRIST_EP[0-7] = file a-h")
    print("pub const ZOBRIST_EP: [u64; 8] = [")
    for key in keys[772:780]:
        print(f"    {key},")
    print("];")

    print("\n// ZOBRIST_SIDE = side to move (XOR in when black to move)")
    print(f"pub const ZOBRIST_SIDE: u64 = {keys[780]};")


if __name__ == "__main__":
    main()
