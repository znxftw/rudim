using Rudim.Board;
using Rudim.Common;
using System.Threading;
using Xunit;
using Helpers = Rudim.Test.Util.Helpers;

namespace Rudim.Test.UnitTest.Board
{
    [Collection("StateRace")]
    public class TacticsTest
    {
        // TODO : End goal is to have NO Skip here - these should eventually all be solvable once Rudim is strong enough
        [Theory]

        // Random Puzzle Position
        [InlineData("r4r2/pb4kp/1p4p1/1P6/2P1pRp1/P3B3/7P/5RK1 w - - 0 29", "f4f8")]

        // Transposition Table Verification - without TT / wrong TT this would take too long
        [InlineData("8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - - ", "a1b1", Skip = "Requires more depth")]

        // Zugzwang Verification - NMR should not get wrong results for these nodes 
        [InlineData("8/8/1p1r1k2/p1pPN1p1/P3KnP1/1P6/8/3R4 b - - 0 1", "f4d5", Skip = "Requires more depth")]
        [InlineData("7k/5K2/5P1p/3p4/6P1/3p4/8/8 w - - 0 1", "g4g5", Skip = "Requires more depth")]
        [InlineData("8/6B1/p5p1/Pp4kp/1P5r/5P1Q/4q1PK/8 w - - 0 32", "h3h4", Skip = "Improve NMR conditions")]
        [InlineData("8/8/p1p5/1p5p/1P5p/8/PPP2K1p/4R1rk w - - 0 1", "e1f1", Skip = "Improve NMR conditions")]
        [InlineData("1q1k4/2Rr4/8/2Q3K1/8/8/8/8 w - - 0 1", "g5h6", Skip = "Improve NMR conditions")]
        public void ShouldNotMissBestMoveForTactic(string fen, string moveLan)
        {
            Global.Reset();
            BoardState boardState = BoardState.ParseFEN(fen);

            CancellationTokenSource cancellationToken = new(2000);
            bool debugMode = false;
            Move bestMove = boardState.FindBestMove(25, cancellationToken.Token, ref debugMode);

            Move expectedMove = Move.ParseLongAlgebraic(moveLan);
            boardState.GenerateMoves();
            expectedMove = Helpers.FindMoveFromMoveList(boardState, expectedMove);

            Assert.Equal(expectedMove, bestMove);
        }
    }
}