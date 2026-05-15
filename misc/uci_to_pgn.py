#!/usr/bin/env python3
import sys
import chess
import chess.pgn


def uci_position_to_pgn(line: str) -> str:
    parts = line.strip().split()

    # Expected format: position startpos moves e2e4 e7e5 ...
    if len(parts) < 4 or parts[0] != "position" or parts[1] != "startpos" or parts[2] != "moves":
        raise ValueError("Input must be: position startpos moves <uci1> <uci2> ...")

    moves = parts[3:]

    board = chess.Board()
    game = chess.pgn.Game()
    game.headers["Event"] = "UCI Import"
    game.headers["Site"] = "?"
    game.headers["Date"] = "????.??.??"
    game.headers["Round"] = "?"
    game.headers["White"] = "?"
    game.headers["Black"] = "?"
    game.headers["Result"] = "*"

    node = game
    for uci in moves:
        move = board.parse_uci(uci)   # validates UCI + legality on current board
        node = node.add_variation(move)
        board.push(move)

    if board.is_game_over(claim_draw=True):
        game.headers["Result"] = board.result(claim_draw=True)

    return str(game)


def main():
    if len(sys.argv) > 1:
        line = " ".join(sys.argv[1:])
    else:
        line = sys.stdin.read().strip()

    print(uci_position_to_pgn(line))


if __name__ == "__main__":
    main()
