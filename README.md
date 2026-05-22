# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

Rudim is a chess engine written in Rust.

You can play against Rudim on lichess: [rudim-bot](https://lichess.org/@/rudim-bot).

Series of blog posts on how I wrote rudim : [vishnubhagyanath.dev](https://vishnubhagyanath.dev/tags/rudim/) (these reference the older C# implementation, rudim was rewritten in rust)

## Architecture Overview

Rudim currently implements these core engine capabilities:

### Board Representation

- Bitboards, Magic Bitboards
- Make/Unmake Move (pseudo-legal movegen)
- Zobrist Hashing

### Search

- Iterative Deepening with Aspiration Windows
- Negamax + Alpha-Beta Pruning
- Principal Variation Search
- Quiescence Search
- Transposition Table
- Move Ordering (MVV-LVA, Killer, History, Hash, PV)
- Null Move Pruning
- Late Move Reductions
- Reverse Futility Pruning

### Evaluation

- Piece-Square Tables
- Pawn Structure (Doubled, Isolated, Passed)
- Draw Detection

### Other

- UCI Protocol support

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

## Benchmarks

Rudim uses Criterion benchmarks to validate how some sample position searches are performing

- Run all benches: `cargo bench`
- Main benchmark suite (`find_best_move` at depth 6-7 on standard positions) lives in `benches/search_benchmark.rs`.

## Contributing

PRs are welcome.

Before opening a PR, please run:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features`
- `cargo test`

If your change affects search strength, run a tournament/regression check as well.
