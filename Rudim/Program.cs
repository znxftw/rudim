using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using Rudim.Board;
using Rudim.CLI;
using Rudim.Common;
using Rudim.Search;
using System.Collections.Generic;
using System.Threading;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length >= 1 && args[0] == "--benchmark")
            {
                BenchmarkRunner.Run<Benchmark>();
            }
            else
            {
                CliClient.Run();
            }
        }
    }

    [MemoryDiagnoser]
    public class Benchmark
    {
        [Benchmark]
        [ArgumentsSource(nameof(GenerateBenchmarks))]
        public void BenchmarkBestMove(BoardState boardState, int depth, CancellationToken cancellationToken)
        {
            bool debugMode = false;
            boardState.FindBestMove(depth, cancellationToken, ref debugMode);
        }

        public IEnumerable<object[]> GenerateBenchmarks()
        {
            yield return [BoardState.ParseFEN(Helpers.AdvancedMoveFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.AdvancedMoveFEN), 7, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.StartingFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.StartingFEN), 7, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.EndgameFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.EndgameFEN), 7, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.KiwiPeteFEN), 6, new CancellationToken(false)];
            yield return [BoardState.ParseFEN(Helpers.KiwiPeteFEN), 7, new CancellationToken(false)];
        }
    }
}