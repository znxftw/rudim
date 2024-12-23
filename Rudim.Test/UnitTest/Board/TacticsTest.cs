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
        [Theory]
        [InlineData("8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - - ", "a1b1")] // Transposition Table Verification - without TT / wrong TT this would take too long
        public void ShouldNotMissBestMoveForTactic(string fen, string moveLan)
        {
            Global.Reset();
            BoardState boardState = BoardState.ParseFEN(fen);
            
            CancellationTokenSource cancellationToken = new(10000);
            bool debugMode = false;
            Move bestMove = boardState.FindBestMove(25, cancellationToken.Token, ref debugMode);

            Move expectedMove = Move.ParseLongAlgebraic(moveLan);
            boardState.GenerateMoves();
            expectedMove = Helpers.FindMoveFromMoveList(boardState, expectedMove);
            
            Assert.Equal(expectedMove, bestMove);
        }
    }
}