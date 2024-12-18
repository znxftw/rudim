using Rudim.Board;
using Xunit;

namespace Rudim.Test.UnitTest.Board
{
    public class SimpleEvaluationTest
    {
        [Theory]
        [InlineData("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0)]
        [InlineData("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", 0)]
        [InlineData("r4r2/pb4kp/1p4p1/1P6/2P1pRp1/P3B3/7P/5RK1 w - - 0 29", -200)]
        
        public void ShouldReturnConsistentScoreForGivenPosition(string fen, int expectedScore)
        {
            BoardState boardState = BoardState.ParseFEN(fen);

            int actualScore = SimpleEvaluation.Evaluate(boardState);

            Assert.Equal(expectedScore, actualScore);
        }
    }
}