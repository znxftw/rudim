# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

## What does Rudim do?

Rudim is currently a work in progress. The aim is for Rudim to be able to play above average chess.

Rudim currently has a working implementation of [Bitboards](https://en.wikipedia.org/wiki/Bitboard) and Move Generation. It uses simple straightforward bitboard logic for generating legal moves of a given position and can currently calculate roughy 1000 KN/s according to simple perft on my local.

Rudim has a basic implementation for the [UCI Protocol](https://www.shredderchess.com/chess-features/uci-universal-chess-interface.html) - i.e. if you try loading the engine into a GUI like Arena and generate moves - it would be able to play a game of chess. It currently does not evaluate the best move in the position, just picks any of the available moves arbitrarily.
What's next for Rudim?
- Write a simple evaluation algorithm to give each position a score of how advantageous it is.
- Write a search algorithm to scan through the tree, prune unnecessary nodes, and evaluate leaf positions.
- Update the UCI code to include the extra options for the `go` command. (These are mostly timers & infinite search until an async stop is called - which would need the above two steps in place first)

## How does Rudim work?

I've written a blog post on my journey through creating Rudim - you can read up on it [here](https://vishnubhagyanath.dev/blog/2022-01-28-rudim-1/).

## Running Rudim

Tests - `dotnet test`

CLI - `dotnet run -p Rudim`
