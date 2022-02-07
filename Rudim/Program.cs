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
            AlphaBeta.Search(board, 8);
            Console.WriteLine(AlphaBeta.Nodes);
            Console.WriteLine(Quiescent.Nodes);
            board.MakeMove(AlphaBeta.BestMove);
            board.Print();
            timer.Stop();

            Console.WriteLine(timer.ElapsedMilliseconds);
        }
    }
}
