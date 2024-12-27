using Rudim.Board;
using Rudim.Common;
using System.Collections.Generic;
using Xunit;

namespace Rudim.Test.UnitTest.Board
{
    public class MoveOrderingTest
    {
        [Fact]
        public void ShouldSortMovesByScore()
        {
            List<Move> moves = new()
            {
                new Move(Square.e2, Square.e4, MoveTypes.Quiet) { Score = 100 },
                new Move(Square.d2, Square.d4, MoveTypes.Quiet) { Score = 300 },
                new Move(Square.g1, Square.f3, MoveTypes.Quiet) { Score = 200 }
            };

            MoveOrdering.SortNextBestMove(moves, 0);

            Assert.Equal(300, moves[0].Score);
        }

        [Fact]
        public void ShouldNotChangeOrderIfAlreadySorted()
        {
            List<Move> moves = new()
            {
                new Move(Square.d2, Square.d4, MoveTypes.Quiet) { Score = 300 },
                new Move(Square.g1, Square.f3, MoveTypes.Quiet) { Score = 200 },
                new Move(Square.e2, Square.e4, MoveTypes.Quiet) { Score = 100 }
            };

            MoveOrdering.SortNextBestMove(moves, 1);

            Assert.Equal(300, moves[0].Score);
            Assert.Equal(200, moves[1].Score);
            Assert.Equal(100, moves[2].Score);
        }
    }
}