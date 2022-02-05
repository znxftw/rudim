# Rudim
[![Pipeline](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml/badge.svg)](https://github.com/znxftw/rudim/actions/workflows/pipeline.yml)

## What does Rudim do?

Rudim is currently a work in progress but is in an MVP phase - you can [play with Rudim on Lichess](https://lichess.org/@/rudim-bot).  

If you see Rudim as offline - the server might be down. If Rudim is online but not accepting your challenge, Rudim might either be playing someone else (currently can play only one person at a time) or the server might be restarting - try again later.

### What all does Rudim implement?

- Bitboards & Magic Bitboards
- UCI Protocol
- Simplified Evaluation (Piece Square Tables)
- Tapered Evaluation
- Negamax with Alpha Beta Pruning
- Quiescent Search
- Move Ordering - MVV LVA

### What's next for Rudim?

- Finish the implementation for the UCI commands (and any changes in the implementation for Rudim that might be a result of it)
- Improve the Search, Move Generation, and Evaluation algorithms to make Rudim stronger.

## How does Rudim work?

I've written a series of blog posts on my journey through creating Rudim - you can read up on it [here](https://vishnubhagyanath.dev/tags/rudim/).

## Running Rudim

Tests - `dotnet test`

CLI - `dotnet run -p Rudim`
