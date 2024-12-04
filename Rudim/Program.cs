using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using Rudim.Board;
using Rudim.CLI;
using Rudim.Common;
using Rudim.Search;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading;
using System.Threading.Tasks;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            // TODO: better arg parse
            if (args.Length >= 1 && args[0] == "--benchmark")
            {
                BenchmarkRunner.Run<NegamaxBenchmark>();
            }
            else
            {
                CliClient.Run();
            }
        }
        // Rename when debugging
        static void DebugMain(string[] args)
        {
            var timer = new Stopwatch();
            var cancellationTokenSource = new CancellationTokenSource();
            timer.Start();
            var board = BoardState.Default();
            board = BoardState.ParseFEN("r2q1rk1/p1p1ppbp/2pp1np1/8/3PP3/2N2Q1P/PPP2PP1/R1B1K2R w KQ - 0 10");

            Task.Run(() => IterativeDeepening.Search(board, 8, cancellationTokenSource.Token));
            Thread.Sleep(4900);
            cancellationTokenSource.Cancel();
            Thread.Sleep(100);
            // This might fail with the 100ms buffer as well because of SavedState concurrency for the search.
            // In UCI we wouldn't have to actually MakeMove, so we can ignore this concurrency issue for now.
            board.MakeMove(IterativeDeepening.BestMove);
            board.Print();
            timer.Stop();

            Console.WriteLine(timer.ElapsedMilliseconds);
        }
    }

    [MemoryDiagnoser]
    public class NegamaxBenchmark
    {
        [Benchmark]
        [ArgumentsSource(nameof(GenerateBenchmarks))]
        public void BenchmarkSearch(BoardState boardState, int depth, CancellationToken cancellationToken)
        {
            Negamax.Search(boardState, depth, cancellationToken);
        }

        public IEnumerable<object[]> GenerateBenchmarks()
        {
            yield return [BoardState.ParseFEN(Helpers.AdvancedMoveFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.AdvancedMoveFEN), 7, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.AdvancedMoveFEN), 8, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.StartingFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.StartingFEN), 7, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.StartingFEN), 8, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.EndgameFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.EndgameFEN), 7, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.EndgameFEN), 8, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.KiwiPeteFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.KiwiPeteFEN), 7, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.KiwiPeteFEN), 8, new CancellationToken(false)];
        }
    }
}