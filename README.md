# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

Rudim is a UCI compatible chess engine written in Rust which uses Efficiently Updatable Neural Networks (NNUE).

You can play against Rudim on lichess: [rudim-bot](https://lichess.org/@/rudim-bot). (Hosted version: v3.0.3)

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

- [NNUE](https://github.com/znxftw/rudim-networks) Architecture: (768 -> 256) x 2 -> 1
- Trained purely on Self-Play games from scratch without any external data or games of HCE version of Rudim
</details>

<details>
<summary><b>Other</b></summary>

- UCI Protocol support
</details>

## Estimated Strength

Ratings and rankings from [CCRL 40/15](https://computerchess.org.uk/4040/), [CCRL Blitz](https://computerchess.org.uk/404/), and the [Computer Chess Index (CCI)](https://github.com/computer-chess-index/cci/blob/main/engines/Rudim.md):

<details>
<summary><b>Ratings & Rankings</b></summary>

| Version | Release Date | CCRL 40/15 | CCRL Blitz | CCI STC | CCI VLTC |
| ------- | ------------ | ---------- | ---------- | ------- | -------- |
| v3.0.4  | 2026-06-20   | -          | -          | 2618 (#103) | 2858 (#116) |
| v3.0.3  | 2026-06-18   | -          | -          | 2527        | 2862        |
| v3.0.2  | 2026-06-13   | -          | -          | 2448        | 2776        |
| v3.0.1  | 2026-06-09   | -          | -          | 2277        | 2593        |
| v3.0.0  | 2026-06-06   | 2614 (#372)| -          | 2226        | 2585        |
| v2.2.2  | 2026-05-29   | -          | -          | -           | -           |
| v2.2.1  | 2026-05-28   | -          | -          | -           | -           |
| v2.2.0  | 2026-05-26   | -          | -          | -           | -           |
| v2.1.3  | 2026-05-23   | -          | -          | -           | -           |
| v2.1.2  | 2026-05-20   | -          | -          | 1804        | 2149        |
| v2.1.1  | 2026-05-16   | -          | -          | 1719        | 2072        |
| v2.1.0  | 2026-05-14   | -          | -          | 1732        | 1944        |
| v2.0.0  | 2026-05-03   | -          | -          | 1650        | 1949        |
| v1.5    | 2026-04-30   | -          | 1848 (#651)| 1590        | 1953        |
| v1.4.1  | 2024-12-18   | -          | -          | -           | -           |
| v1.4    | 2024-12-18   | -          | -          | -           | -           |
| v1.3    | 2024-12-05   | -          | 1434       | -           | -           |
| v1.2    | 2022-02-25   | -          | -          | -           | -           |
| v1.1    | 2022-02-08   | -          | -          | -           | -           |
| v1.0    | 2022-02-06   | -          | -          | -           | -           |

*Ranks are shown in brackets when available in the official rating lists.*
</details>

## Usage

- Build Binary : `cargo build --release`
- Run engine : `cargo run --release`
- Run benchmark: `cargo bench`
- Misc : `cargo run --release -- --generate-magics`, `cargo run --release --features cuda -- --train <binpack_path>`
- Use unoptimized versions (non `--release`) only if debugging

## Quality Checks

- `cargo test`
- `cargo clippy --all-targets`
- `cargo fmt --all`

## Acknowledgements

- [maksimKorzh](https://github.com/maksimKorzh) for his YouTube series on on bitboard chess engines
- [bullet](https://github.com/jw1912/bullet) - used for training the NNUE
- [Reckless](https://github.com/codedeliveryservice/Reckless), [Viridithas](https://github.com/cosmobobak/viridithas), [Hobbes](https://github.com/kelseyde/hobbes-chess-engine) - took some references for Rust optimizations they did
- ChessProgramming wiki, TalkChess, Engine Programming Discord Server, CCRL, CCI

## Contributing

PRs are welcome.

Before opening a PR, please run all the quality checks, perft and benchmark.

If your change affects search strength, run a 1000 match 10+0.1 tournament as well.
