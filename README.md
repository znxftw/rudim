# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

Rudim is a chess engine written in Rust.

You can play against Rudim on lichess: [rudim-bot](https://lichess.org/@/rudim-bot). (Hosted version: v2.2.1)

Series of blog posts on how I wrote rudim : [vishnubhagyanath.dev](https://vishnubhagyanath.dev/tags/rudim/) (these reference the older C# implementation, rudim was rewritten in rust)

## Architecture Overview

Rudim currently implements these core engine capabilities:

<details>
<summary><b>Board Representation</b></summary>

- Bitboards, Magic Bitboards
- Phased Pseudo-Legal Move Generation
- Zobrist Hashing
</details>

<details>
<summary><b>Search</b></summary>

- Iterative Deepening with Aspiration Windows
- Negamax + Alpha-Beta Pruning
- Principal Variation Search
- Quiescence Search
- Two-tiered Transposition Table
- Move Ordering (SEE, MVV-LVA, Killer, History, Hash, PV)
- Null Move Pruning
- Late Move Reductions
- Futility Pruning
- Reverse Futility Pruning
</details>

<details>
<summary><b>Evaluation</b></summary>

- [NNUE](https://github.com/znxftw/rudim-networks) Architecture: (768 -> 32) x 2 -> 1
</details>

<details>
<summary><b>Other</b></summary>

- UCI Protocol support
</details>

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

## Acknowledgements

- [maksimKorzh](https://github.com/maksimKorzh) for his YouTube series on on bitboard chess engines
- [bullet](https://github.com/jw1912/bullet) - used for training the NNUE
- [Reckless](https://github.com/codedeliveryservice/Reckless), [Viridithas](https://github.com/cosmobobak/viridithas), [Hobbes](https://github.com/kelseyde/hobbes-chess-engine) - took some references for Rust optimizations they did
- ChessProgramming wiki, TalkChess, Engine Programming Discord Server, CCRL, CCI

## Contributing

PRs are welcome.

Before opening a PR, please run:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features`
- `cargo test`

If your change affects search strength, run a tournament/regression check as well.
