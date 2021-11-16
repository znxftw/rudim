using Rudim.Board;
using Rudim.Common;
using Rudim.Test.Common;
using Xunit;

namespace Rudim.Test
{
    public class BoardStateFENTest
    {
        [Fact]
        public void ShouldParseStartingFENCorrectly()
        {
            var fen = Helpers.StartingFEN;

            var result = BoardState.ParseFEN(fen);

            Assert.Equal(71776119061217280ul, result.Pieces[(int)Side.White,(int)Piece.Pawn].Board);
            Assert.Equal(4755801206503243776ul, result.Pieces[(int)Side.White,(int)Piece.Knight].Board);
            Assert.Equal(2594073385365405696ul, result.Pieces[(int)Side.White,(int)Piece.Bishop].Board);
            Assert.Equal(9295429630892703744ul, result.Pieces[(int)Side.White,(int)Piece.Rook].Board);
            Assert.Equal(576460752303423488ul, result.Pieces[(int)Side.White,(int)Piece.Queen].Board);
            Assert.Equal(1152921504606846976ul, result.Pieces[(int)Side.White,(int)Piece.King].Board);

            Assert.Equal(65280ul, result.Pieces[(int)Side.Black, (int)Piece.Pawn].Board);
            Assert.Equal(66ul, result.Pieces[(int)Side.Black, (int)Piece.Knight].Board);
            Assert.Equal(36ul, result.Pieces[(int)Side.Black, (int)Piece.Bishop].Board);
            Assert.Equal(129ul, result.Pieces[(int)Side.Black, (int)Piece.Rook].Board);
            Assert.Equal(8ul, result.Pieces[(int)Side.Black, (int)Piece.Queen].Board);
            Assert.Equal(16ul, result.Pieces[(int)Side.Black, (int)Piece.King].Board);

            Assert.Equal(65535ul, result.Occupancies[(int)Side.Black].Board);
            Assert.Equal(18446462598732840960ul, result.Occupancies[(int)Side.White].Board);
            Assert.Equal(18446462598732906495ul, result.Occupancies[(int)Side.Both].Board);

            Assert.Equal(Side.White, result.SideToMove);
            Assert.Equal((Castle) 15, result.Castle);
            Assert.Equal(Square.NoSquare, result.EnPassantSquare);
        }
    }
}
