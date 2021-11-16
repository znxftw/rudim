using Rudim.Board;
using Rudim.Test.Common;
using Xunit;

namespace Rudim.Test
{
    public class BoardStateSavedTest
    {
        [Fact (Skip = "TODO : Write equality members for BoardState, the code is working")]
        public void ShouldSaveAndRestoreBoardState()
        {
            var originalState = BoardState.ParseFEN(Helpers.StartingFEN);
            var boardState = BoardState.ParseFEN(Helpers.StartingFEN);
            
            boardState.SaveState();
            boardState.GenerateMoves();
            
            Assert.NotEqual(boardState, originalState);
            
            boardState.RestoreState();
            
            Assert.Equal(boardState, originalState);
        }
    }
}