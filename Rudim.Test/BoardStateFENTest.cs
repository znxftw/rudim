using Rudim.Board;
using System;
using Xunit;

namespace Rudim.Test
{
    public class BoardStateFENTest
    {
        [Fact]
        public void ShouldParseStartingFENCorrectly()
        {
            var fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";

            var result = BoardState.ParseFEN(fen);

            Assert.Equal((ulong)71776119061217280, result.Pieces[(int)Side.White,(int)Piece.Pawn].Board);
            Assert.Equal((ulong)4755801206503243776, result.Pieces[(int)Side.White,(int)Piece.Knight].Board);
            Assert.Equal((ulong)2594073385365405696, result.Pieces[(int)Side.White,(int)Piece.Bishop].Board);
            Assert.Equal((ulong)9295429630892703744, result.Pieces[(int)Side.White,(int)Piece.Rook].Board);
            Assert.Equal((ulong)576460752303423488, result.Pieces[(int)Side.White,(int)Piece.Queen].Board);
            Assert.Equal((ulong)1152921504606846976, result.Pieces[(int)Side.White,(int)Piece.King].Board);

            Assert.Equal((ulong)65280, result.Pieces[(int)Side.Black, (int)Piece.Pawn].Board);
            Assert.Equal((ulong)66, result.Pieces[(int)Side.Black, (int)Piece.Knight].Board);
            Assert.Equal((ulong)36, result.Pieces[(int)Side.Black, (int)Piece.Bishop].Board);
            Assert.Equal((ulong)129, result.Pieces[(int)Side.Black, (int)Piece.Rook].Board);
            Assert.Equal((ulong)8, result.Pieces[(int)Side.Black, (int)Piece.Queen].Board);
            Assert.Equal((ulong)16, result.Pieces[(int)Side.Black, (int)Piece.King].Board);

            Assert.Equal(Side.White, result.SideToMove);
            Assert.Equal((Castle) 15, result.Castle);
            Assert.Equal(Square.NoSquare, result.EnPassantSquare);
        }
    }
}
