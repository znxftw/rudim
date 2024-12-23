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
        [InlineData(Helpers.StartingFEN, 2938703, 7, 8)]
        [InlineData(Helpers.EndgameFEN, 525630, 36, 9)]
        [InlineData(Helpers.AdvancedMoveFEN, 8610756, 1750, 8)]
        [InlineData(Helpers.KiwiPeteFEN, 25369977, -42, 8)]
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