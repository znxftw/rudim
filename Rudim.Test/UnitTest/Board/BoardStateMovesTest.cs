using Rudim.Board;
using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest.Board
{
    public class BoardStateMovesTest
    {
        [Fact]
        public void ShouldGenerateMoves()
        {

            BoardState advancedMovesPosition = BoardState.ParseFEN(Helpers.AdvancedMoveFEN);
            BoardState randomPosition = BoardState.ParseFEN(Helpers.KiwiPeteFEN);
            BoardState startingPosition = BoardState.ParseFEN(Helpers.StartingFEN);

            advancedMovesPosition.GenerateMoves();
            randomPosition.GenerateMoves();
            startingPosition.GenerateMoves();

            // Are more rigorous asserts required here?
            Assert.Equal(42, advancedMovesPosition.Moves.Count);
            Assert.Equal(48, randomPosition.Moves.Count);
            Assert.Equal(20, startingPosition.Moves.Count);
        }
    }
}