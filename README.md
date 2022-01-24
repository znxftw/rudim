# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

## What does Rudim do?

Rudim is currently a work in progress. The aim is for Rudim to be able to play above average chess.

Rudim currently has a working implementation of [Bitboards](https://en.wikipedia.org/wiki/Bitboard) and Move Generation. It uses simple straightforward bitboard logic for generating legal moves of a given position and can currently calculate roughy 1000 KN/s according to simple perft on my local.

What's next for Rudim?
- I'm currently implementing the [UCI Protocol](https://www.shredderchess.com/chess-features/uci-universal-chess-interface.html) for the work so far to be interactable with chess interfaces. It would use a random move picker in place of an actual implementation for 'find best move'.
- Once the UCI Protocol is in place and Rudim can 'play' (albeit bad) chess, the next step would be to write an actual search + evaluation algorithm to find the best of all the generated moves.
- Implement a GUI for interactive gameplay on top of UCI CLI.

## Running Rudim

Tests - `dotnet test`

CLI - `dotnet run -p Rudim`
