using Rudim.Board;
using Rudim.Common;
using Rudim.Test.Common;
using System;
using System.Diagnostics;
using Xunit;
using Xunit.Abstractions;

namespace Rudim.Test.Perft
{
    [Collection("StateRace")]
    public class Test
    {
        private readonly ITestOutputHelper output;

        public Test(ITestOutputHelper output)
        {
            this.output = output;
        }

        [Theory]
        [InlineData(0, 1)]
        [InlineData(1, 20)]
        [InlineData(2, 400)]
        [InlineData(3, 8_902)]
        [InlineData(4, 197_281)]
        [InlineData(5, 4_865_609)]
        public void PerftStartingPosition(int depth, ulong nodes)
        {
            var timer = new Stopwatch();
            
            BoardState.ClearStates();
            
            timer.Start();
            
            var boardState = BoardState.ParseFEN(Helpers.StartingFEN);
            PerftDriver.ResetNodeCount();
            PerftDriver.Traverse(boardState, depth);

            timer.Stop();
            Assert.Equal(nodes, PerftDriver.nodes);

            BoardState.ClearStates();
            output.WriteLine($"Execution Time: {timer.ElapsedMilliseconds} ms");
        }

        [Fact (Skip = "Debugging test")]
        public void PerftDebug()
        {
            var depth = 2;

            var boardState = BoardState.ParseFEN("rnbqkbnr/p1pppppp/8/1p6/P7/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 2");
            ulong total = 0;
            boardState.GenerateMoves();
            foreach(var move in boardState.Moves)
            {
                PerftDriver.ResetNodeCount();
                boardState.SaveState();
                boardState.MakeMove(move);

                if(!boardState.IsInCheck(boardState.SideToMove.Other()))
                   PerftDriver.Traverse(boardState, depth - 1);

                total += PerftDriver.nodes;
                output.WriteLine(move.Source.ToString() + move.Target.ToString() + " " + PerftDriver.nodes + " " + move.Type.ToString());
                boardState.RestoreState();
            }
            output.WriteLine(total.ToString());
        }
    }
}
