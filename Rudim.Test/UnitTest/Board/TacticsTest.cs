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
        [InlineData("r4r2/pb4kp/1p4p1/1P6/2P1pRp1/P3B3/7P/5RK1 w - - 0 29", "f4f8")]
        public void ShouldNotMissBestMoveForTactic(string fen, string moveLan)
        {
            Global.Reset();
            BoardState boardState = BoardState.ParseFEN(fen);
            
            CancellationTokenSource cancellationToken = new(5000);
            bool debugMode = false;
            Move bestMove = boardState.FindBestMove(25, cancellationToken.Token, ref debugMode);

            Move expectedMove = Move.ParseLongAlgebraic(moveLan);
            boardState.GenerateMoves();
            expectedMove = Helpers.FindMoveFromMoveList(boardState, expectedMove);
            
            Assert.Equal(expectedMove, bestMove);
        }
    }
}