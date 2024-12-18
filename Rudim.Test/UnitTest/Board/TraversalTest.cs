using Rudim.Board;
using Rudim.Search;
using System.Threading;
using Xunit;
using Helpers = Rudim.Common.Helpers;

namespace Rudim.Test.UnitTest.Board
{
    [Collection("StateRace")]
    public class TraversalTest
    {
        // Update these values whenever any search or eval is optimized
        // This helps keep track if certain optimizations are good enough to make up for the extra time spent
        // Compare time spent with and without the change before updating the keys
        [Theory]
        [InlineData(Helpers.StartingFEN, 1787870, 5, 8)]
        [InlineData(Helpers.EndgameFEN, 420262, 45, 9)]
        [InlineData(Helpers.AdvancedMoveFEN, 2649058, 1520, 8)]
        [InlineData(Helpers.KiwiPeteFEN, 6764976, -60, 8)]
        public void ShouldTraverseDeterministically(string position, int expectedNodes, int expectedScore, int depth)
        {
            Global.Reset();

            BoardState boardState = BoardState.ParseFEN(position);
            bool debugMode = false;
            boardState.FindBestMove(depth, new CancellationToken(false), ref debugMode);

            Assert.Equal(expectedNodes, IterativeDeepening.Nodes);
            Assert.Equal(expectedScore, IterativeDeepening.Score);
        }
    }
}