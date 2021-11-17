using Rudim.Board;
using Rudim.Test.Common;
using Xunit;

namespace Rudim.Test
{
    [Collection("StateRace")]
    public class BoardStateSavedTest
    {
        [Fact]
        public void ShouldSaveAndRestoreBoardState()
        {
            BoardState.ClearStates();
            var originalState = BoardState.ParseFEN(Helpers.StartingFEN);
            var boardState = BoardState.ParseFEN(Helpers.StartingFEN);
            
            boardState.SaveState();
            boardState.GenerateMoves();
            
            Assert.NotEqual(boardState, originalState);
            
            boardState.RestoreState();
            
            Assert.Equal(boardState, originalState);
            BoardState.ClearStates();
        }
    }
}