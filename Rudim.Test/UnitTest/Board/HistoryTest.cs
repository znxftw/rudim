using Rudim.Board;
using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest.Board
{
    [Collection("StateRace")]
    public class HistoryTest
    {
        [Fact]
        public void ShouldSaveAndRestoreBoardHistory()
        {
            History.ClearBoardHistory();
            var originalState = BoardState.ParseFEN(Helpers.StartingFEN);
            var boardState = BoardState.ParseFEN(Helpers.StartingFEN);
            var move = new Move(Square.e2, Square.e4, MoveTypes.Quiet);

            boardState.MakeMove(move);

            Assert.NotEqual(boardState, originalState);

            boardState.UnmakeMove(move);

            Assert.Equal(boardState, originalState);
            History.ClearBoardHistory();
        }
    }
}