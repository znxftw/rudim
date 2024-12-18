using Rudim.Board;
using Rudim.Test.Util;
using Xunit;
using Helpers = Rudim.Common.Helpers;

namespace Rudim.Test.UnitTest.Board
{
    public class PieceSquareTableEvaluationTest
    {
        [Theory]
        [InlineData("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0)] // White
        [InlineData("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", 0)] // Black
        [InlineData(Helpers.EndgameFEN, 37)]
        [InlineData(Helpers.KiwiPeteFEN, 56)]
        [InlineData(Helpers.AdvancedMoveFEN, 516)]
        
        public void ShouldReturnConsistentScoreForGivenPosition(string fen, int expectedScore)
        {
            BoardState boardState = BoardState.ParseFEN(fen);

            int actualScore = PieceSquareTableEvaluation.Evaluate(boardState);

            Assert.Equal(expectedScore, actualScore);
        }
    }
}