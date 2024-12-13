using Rudim.Board;
using Rudim.Common;
using System.Diagnostics;
using Xunit;
using Xunit.Abstractions;

namespace Rudim.Test.Perft
{
    [Collection("StateRace")]
    public class PerftTest
    {
        private readonly ITestOutputHelper output;

        public PerftTest(ITestOutputHelper output)
        {
            this.output = output;
        }

        [Theory]
        [InlineData(0, 1, Helpers.StartingFEN)]
        [InlineData(1, 20, Helpers.StartingFEN)]
        [InlineData(2, 400, Helpers.StartingFEN)]
        [InlineData(3, 8_902, Helpers.StartingFEN)]
        [InlineData(4, 197_281, Helpers.StartingFEN)]
        [InlineData(5, 4_865_609, Helpers.StartingFEN)]
        [InlineData(6, 119_060_324, Helpers.StartingFEN)]
        [InlineData(1, 48, Helpers.KiwiPeteFEN)]
        [InlineData(2, 2_039, Helpers.KiwiPeteFEN)]
        [InlineData(3, 97_862, Helpers.KiwiPeteFEN)]
        [InlineData(4, 4_085_603, Helpers.KiwiPeteFEN)]
        [InlineData(5, 193_690_690, Helpers.KiwiPeteFEN)]
        [InlineData(1, 14, Helpers.EndgameFEN)]
        [InlineData(2, 191, Helpers.EndgameFEN)]
        [InlineData(3, 2_812, Helpers.EndgameFEN)]
        [InlineData(4, 43_238, Helpers.EndgameFEN)]
        [InlineData(5, 674_624, Helpers.EndgameFEN)]
        [InlineData(6, 11_030_083, Helpers.EndgameFEN)]
        [InlineData(7, 178_633_661, Helpers.EndgameFEN)]
        public void Perft(int depth, ulong nodes, string position)
        {
            var timer = new Stopwatch();

            History.ClearBoardHistory();

            timer.Start();

            var boardState = BoardState.ParseFEN(position);
            PerftDriver.ResetNodeCount();
            PerftDriver.Traverse(boardState, depth);

            timer.Stop();
            Assert.Equal(nodes, PerftDriver.Nodes);

            History.ClearBoardHistory();
            output.WriteLine($"Execution Time: {timer.ElapsedMilliseconds} ms");
        }

        [Fact(Skip = "Debugging test")]
        public void PerftDebug()
        {
            var depth = 2;

            var boardState = BoardState.ParseFEN(Helpers.KiwiPeteFEN);
            ulong total = 0;
            boardState.GenerateMoves();
            foreach (var move in boardState.Moves)
            {
                PerftDriver.ResetNodeCount();
                boardState.MakeMove(move);

                if (!boardState.IsInCheck(boardState.SideToMove.Other()))
                    PerftDriver.Traverse(boardState, depth - 1);

                total += PerftDriver.Nodes;
                output.WriteLine(move.Source.ToString() + move.Target.ToString() + " " + PerftDriver.Nodes + " " + move.Type.ToString());
                boardState.UnmakeMove(move);
            }
            output.WriteLine(total.ToString());
        }
    }
}