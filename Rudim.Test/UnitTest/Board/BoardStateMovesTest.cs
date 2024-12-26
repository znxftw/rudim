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

        [Fact]
        public void ShouldMakeAndUndoNullMoveCorrectly()
        {
            BoardState boardState = BoardState.Default();
            ulong originalBoardHash = boardState.BoardHash;
            Side originalSideToMove = boardState.SideToMove;
            Square originalEnPassantSquare = boardState.EnPassantSquare;
            Castle originalCastlingRights = boardState.Castle;
            int originalMoveCount = boardState.MoveCount;
            
            
            boardState.MakeNullMove();
            Assert.NotEqual(originalBoardHash, boardState.BoardHash);
            Assert.NotEqual(originalSideToMove, boardState.SideToMove);
            Assert.Equal(Square.NoSquare, boardState.EnPassantSquare);

            // Make one legal move for each side
            Move whiteMove = new(Square.e2, Square.e4, MoveTypes.DoublePush);
            boardState.MakeMove(whiteMove);
            Move blackMove = new(Square.e7, Square.e5, MoveTypes.DoublePush);
            boardState.MakeMove(blackMove);

            boardState.UnmakeMove(blackMove);
            boardState.UnmakeMove(whiteMove);
            

            boardState.UndoNullMove();

            Assert.Equal(originalBoardHash, boardState.BoardHash);
            Assert.Equal(originalSideToMove, boardState.SideToMove);
            Assert.Equal(originalEnPassantSquare, boardState.EnPassantSquare);
            Assert.Equal(originalCastlingRights, boardState.Castle);
            Assert.Equal(originalMoveCount, boardState.MoveCount);
        }
    }
}