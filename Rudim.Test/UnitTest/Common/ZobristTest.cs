using Rudim.Board;
using Rudim.Common;
using Xunit;
using Helpers = Rudim.Test.Util.Helpers;

namespace Rudim.Test.UnitTest.Common
{
    [Collection("StateRace")]
    public class ZobristHashingTest
    {
        [Theory]
        // Quiet, Captures & Promotions
        [InlineData("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "e2e4")]
        [InlineData("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2", "e4d5")]
        [InlineData("rnbqkbnr/ppp2ppp/8/3Pp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 1", "d5e6")]
        [InlineData("rnbqkbnr/ppppp1P1/8/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 1", "g7h8q")]
        // En Passant
        [InlineData("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1", "d7d5")]
        [InlineData("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2", "e5d6")]
        [InlineData("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2", "e5e6")]
        // Castling Rights
        [InlineData("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1", "e1g1")]
        [InlineData("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1", "e1c1")]
        [InlineData("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1", "e8g8")]
        [InlineData("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1", "e8c8")]
        public void ShouldRestoreCorrectHashAfterUnmakingMove(string fen, string moveStr)
        {
            var boardState = BoardState.ParseFEN(fen);
            boardState.GenerateMoves();
            var move = Helpers.FindMoveFromMoveList(boardState, Move.ParseLongAlgebraic(moveStr));
            var originalHash = boardState.BoardHash;

            boardState.MakeMove(move);
            Assert.Equal(Zobrist.GetBoardHash(boardState), boardState.BoardHash);

            boardState.UnmakeMove(move);
            Assert.Equal(originalHash, boardState.BoardHash);
            Assert.Equal(Zobrist.GetBoardHash(boardState), boardState.BoardHash);
        }

        
    }
}