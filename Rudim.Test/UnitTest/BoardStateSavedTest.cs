using Rudim.Board;
using Rudim.Common;
using Xunit;

namespace Rudim.Test
{
    [Collection("StateRace")]
    public class BoardStateSavedTest
    {
        [Fact]
        public void ShouldSaveAndRestoreBoardState()
        {
            BoardState.ClearSavedStates();
            var originalState = BoardState.ParseFEN(Helpers.StartingFEN);
            var boardState = BoardState.ParseFEN(Helpers.StartingFEN);
            var move = new Move(Square.e2, Square.e4, MoveTypes.Quiet);

            boardState.MakeMove(move);

            Assert.NotEqual(boardState, originalState);

            boardState.UnmakeMove(move);

            Assert.Equal(boardState, originalState);
            BoardState.ClearSavedStates();
        }
    }
}
