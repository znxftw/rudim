# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

[Coverage Reports](https://znxftw.github.io/rudim)

## What does Rudim do?

Rudim is a chess engine written in .NET (C#) with the goal of one day being the strongest C# engine.

Rudim has been hosted on lichess, you can [play with Rudim there](https://lichess.org/@/rudim-bot).  


### What all does Rudim implement?

- Bitboards & Magics
- UCI Protocol (WIP - core functionality only)
- Piece Square Tables & Material Evaluation ( + Tapered )
- Iterative Deepening on Negamax (with Alpha Beta Pruning & PVS) & Quiescence Search
- Move Ordering with Hash Move, MVV LVA, Killer Heuristic, History Heuristic
- Transposition Tables using Zobrist Hashing
- Null Move Pruning

### What's next for Rudim?

- Finish the implementation for the UCI commands (and any changes in the implementation for Rudim that might be a result of it)
- Improve the Search, Move Generation, and Evaluation algorithms to make Rudim stronger.

## How does Rudim work?

I've written a series of blog posts on my journey through creating Rudim - you can read up on it [here](https://vishnubhagyanath.dev/tags/rudim/).

## Running Rudim

Tests - `dotnet test`

Benchmark  - `dotnet run --project Rudim -c Release --benchmark`

CLI - `dotnet run --project Rudim`

Perft - `dotnet run -c Release --perft --project Rudim` (Currently run at fixed positions and depths, in future can be made configurable)

## Contributing

Feel free to raise a PR for one of the open issues on the repo - there are not many contributing guidelines in place yet, if contributing increases, this project can have some more strict guidelines.

Some basic things to keep in mind when contributing
- Run all tests and make sure they pass
- Write test scenarios for any new logic you are adding or modify existing tests to cover logic you are modifiying
- Run a tournament with the latest commit on master (usually 5+0, 200 games is what I do as a quick check). If there is a significant loss in ELO something might be wrong or can be improved. PRs would be merged only if ELO improvement >= 10 for a new feature.
