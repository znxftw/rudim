# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

Rudim is a chess engine written in Rust.

You can play against Rudim on lichess: [rudim-bot](https://lichess.org/@/rudim-bot).

## Features

- Bitboards with magic bitboards for sliding move generation
- Full make/unmake move pipeline with board history and draw detection
- UCI protocol support (engine mode via stdin/stdout)
- Iterative deepening + Negamax + alpha-beta + PVS + quiescence
- Move ordering (MVV-LVA, killer moves, history heuristic)
- Transposition table with Zobrist hashing
- Null-move pruning
- Perft driver for correctness/performance validation

## Prerequisites

- Rust stable toolchain (`rustup` + `cargo`)

## Build and Run

- Build: `cargo build`
- Release build: `cargo build --release`
- Run engine (UCI loop): `cargo run`
- Run perft suite: `cargo run -- --perft`
- Generate magic numbers: `cargo run -- --generate-magics`

## Quality Checks

- Tests: `cargo test`
- Lint (Clippy): `cargo clippy --all-targets --all-features`
- Format: `cargo fmt --all`
- Format check: `cargo fmt --all -- --check`

## Benchmarks

Rudim uses Criterion benchmarks.

- Run all benches: `cargo bench`
- Main benchmark suite (`find_best_move` at depth 6-7 on standard positions) lives in `benches/search_benchmark.rs`.

## Architecture Overview

Rudim currently implements these core engine capabilities:

- Bitboard-based board representation & Magic bitboards
- Incremental make/unmake move pipeline with board history with a pseudo-legal move generator
- Draw detection (including repetition and fifty-move rule handling)
- Zobrist hashing and transposition table integration
- Iterative deepening search
- Negamax with alpha-beta pruning and principal variation search
- Quiescence search
- Move ordering heuristics (MVV-LVA, killer moves, history heuristic)
- Null-move pruning
- UCI protocol command handling (`uci`, `isready`, `position`, `go`, `stop`, `ucinewgame`, `debug`, `quit`)
- Rofchade Piece Square Tables and Simple Pawn Structure Evaluations

## Contributing

PRs are welcome.

Before opening a PR, please run:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features`
- `cargo test`

If your change affects search strength, run a tournament/regression check as well.
