using Rudim.Board;
using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest.Board
{
    public class MoveOrderingTest
    {
        [Fact]
        public void ShouldOrderMovesByScore()
        {
            var boardState = BoardState.ParseFEN(Helpers.KiwiPeteFEN);

            boardState.GenerateMoves();
            foreach (var move in boardState.Moves)
            {
                MoveOrdering.PopulateMoveScore(move, boardState);
            }
            MoveOrdering.SortMoves(boardState);

            // TODO: Improve assertions, verify proper order as per MVV LVA
            Assert.True(boardState.Moves[0].IsCapture());
        }
    }
}