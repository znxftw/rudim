using CommandLine;
using Rudim.Board;
using Rudim.Common;
using System;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;

namespace Rudim.Test.Perft
{

    public static class PerftTest
    {
        public static void Perft(int depth, ulong nodes, string position)
        {
            Stopwatch timer = new Stopwatch();

            History.ClearBoardHistory();

            timer.Start();

            BoardState boardState = BoardState.ParseFEN(position);
            PerftDriver.ResetNodeCount();
            PerftDriver.Traverse(boardState, depth);

            timer.Stop();

            if (nodes != PerftDriver.Nodes)
            {
                Console.WriteLine($"There's a difference Expected: {nodes} Actual: {PerftDriver.Nodes}");
                Environment.Exit(2);
            }

            History.ClearBoardHistory();
            Console.WriteLine($"Execution Time: {timer.ElapsedMilliseconds} ms for {boardState} at depth {depth}");
        }

        public static void PerftDebug()
        {
            int depth = 2;

            BoardState boardState = BoardState.ParseFEN(Helpers.KiwiPeteFEN);
            ulong total = 0;
            boardState.GenerateMoves();
            foreach (Move move in boardState.Moves)
            {
                PerftDriver.ResetNodeCount();
                boardState.MakeMove(move);

                if (!boardState.IsInCheck(boardState.SideToMove.Other()))
                    PerftDriver.Traverse(boardState, depth - 1);

                total += PerftDriver.Nodes;
                Console.WriteLine(move.Source.ToString() + move.Target.ToString() + " " + PerftDriver.Nodes + " " + move.Type.ToString());
                boardState.UnmakeMove(move);
            }
            Console.WriteLine(total.ToString());
        }
    }
}