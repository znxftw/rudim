using Rudim.Board;
using Xunit;

namespace Rudim.Test
{
    public class BoardStateSavedTest
    {
        [Fact (Skip = "TODO : Write equality members for BoardState, the code is working")]
        public void ShouldSaveAndRestoreBoardState()
        {
            // TODO : Extract common FENs
            var originalState = BoardState.ParseFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            var boardState = BoardState.ParseFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            
            boardState.SaveState();
            boardState.GenerateMoves();
            
            Assert.NotEqual(boardState, originalState);
            
            boardState.RestoreState();
            
            Assert.Equal(boardState, originalState);
        }
    }
}