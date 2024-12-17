using Rudim.Board;
using System;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;

namespace Rudim.Perft
{

    [ExcludeFromCodeCoverage]
    public static class PerftTest
    {
        public static void Perft(int depth, ulong nodes, string position)
        {
            Stopwatch timer = new();
            Global.Reset();

            timer.Start();

            BoardState boardState = BoardState.ParseFEN(position);
            PerftDriver.Traverse(boardState, depth);

            timer.Stop();

            if (nodes != PerftDriver.Nodes)
            {
                Console.WriteLine($"There's a difference Expected: {nodes} Actual: {PerftDriver.Nodes}");
                Environment.Exit(2);
            }

            Console.WriteLine($"Execution Time: {timer.ElapsedMilliseconds} ms for {boardState} at depth {depth}");
            Global.Reset();
        }
    }
}