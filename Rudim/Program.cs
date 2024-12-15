using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using BenchmarkDotNet.Toolchains.DotNetCli;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp.Syntax;
using Rudim.Board;
using Rudim.CLI;
using Rudim.Common;
using Rudim.Search;
using Rudim.Test.Perft;
using System;
using System.Collections.Generic;
using System.Dynamic;
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
            else if (args.Length >= 1 && args[0] == "--perft")
            {

                var data = new List<PerftData> {
                     new(0, 1, Helpers.StartingFEN),
                     new(1, 20, Helpers.StartingFEN),
                     new(2, 400, Helpers.StartingFEN),
                     new(3, 8_902, Helpers.StartingFEN),
                     new(4, 197_281, Helpers.StartingFEN),
                     new(5, 4_865_609, Helpers.StartingFEN),
                     new(6, 119_060_324, Helpers.StartingFEN),
                     new(1, 48, Helpers.KiwiPeteFEN),
                     new(2, 2_039, Helpers.KiwiPeteFEN),
                     new(3, 97_862, Helpers.KiwiPeteFEN),
                     new(4, 4_085_603, Helpers.KiwiPeteFEN),
                     new(5, 193_690_690, Helpers.KiwiPeteFEN),
                     new(1, 14, Helpers.EndgameFEN),
                     new(2, 191, Helpers.EndgameFEN),
                     new(3, 2_812, Helpers.EndgameFEN),
                     new(4, 43_238, Helpers.EndgameFEN),
                     new(5, 674_624, Helpers.EndgameFEN),
                     new(6, 11_030_083, Helpers.EndgameFEN),
                     new(7, 178_633_661, Helpers.EndgameFEN),
                };
                foreach (var item in data)
                {
                    PerftTest.Perft(item.Depth, item.Nodes, item.Position);
                }
            }
            else
            {
                CliClient.Run();
            }
        }
    }

    internal class PerftData
    {
        public int Depth { get; set; }

        public ulong Nodes { get; set; }

        public string Position { get; set; }

        public PerftData(int depth, ulong nodes, string position)
        {
            Depth = depth;
            Nodes = nodes;
            Position = position;

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