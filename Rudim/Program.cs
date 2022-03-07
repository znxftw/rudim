using Rudim.Board;
using Rudim.CLI;
using Rudim.Search;
using System;
using System.Diagnostics;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            CliClient.Run();
        }
        // Rename when debugging
        static void DebugMain(string[] args)
        {
            var timer = new Stopwatch();

            timer.Start();
            var board = BoardState.Default();
            board = BoardState.ParseFEN("r2q1rk1/p1p1ppbp/2pp1np1/8/3PP3/2N2Q1P/PPP2PP1/R1B1K2R w KQ - 0 10");
            IterativeDeepening.Search(board, 8);
            board.MakeMove(IterativeDeepening.BestMove);
            board.Print();
            timer.Stop();

            Console.WriteLine(timer.ElapsedMilliseconds);
        }
    }
}
