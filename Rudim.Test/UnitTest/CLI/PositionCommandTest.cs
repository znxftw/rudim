using Rudim.CLI.UCI;
using Rudim.Board;
using Rudim.Common;
using Xunit;
using Helpers = Rudim.Test.Util.Helpers;

namespace Rudim.Test.UnitTest.CLI
{
    [Collection("StateRace")]
    public class PositionCommandTest
    {
        [Fact]
        public void ShouldSetPositionFromFEN()
        {
            UciClient uciClient = new();
            PositionCommand positionCommand = new(uciClient);
            string fen = "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

            positionCommand.Run(["fen", fen]);

            Assert.Equal(BoardState.ParseFEN(fen), uciClient.Board);
        }

        [Fact]
        public void ShouldSetPositionToStartPos()
        {
            UciClient uciClient = new();
            PositionCommand positionCommand = new(uciClient);

            positionCommand.Run(["startpos"]);

            Assert.Equal(BoardState.Default(), uciClient.Board);
        }

        [Fact]
        public void ShouldSetPositionToStartPosAndApplyMoves()
        {
            UciClient uciClient = new();
            PositionCommand positionCommand = new(uciClient);

            positionCommand.Run(["startpos", "moves", "e2e4", "e7e5"]);

            BoardState expectedState = SetupExpectedPosition();
            Assert.Equal(expectedState, uciClient.Board);
        }

        private static BoardState SetupExpectedPosition()
        {
            BoardState expectedState = BoardState.Default();
            Move whiteMove = Move.ParseLongAlgebraic("e2e4");
            expectedState.GenerateMoves();
            whiteMove = Helpers.FindMoveFromMoveList(expectedState, whiteMove);
            expectedState.MakeMove(whiteMove);
            Move blackMove = Move.ParseLongAlgebraic("e7e5");
            expectedState.GenerateMoves();
            blackMove = Helpers.FindMoveFromMoveList(expectedState, blackMove);
            expectedState.MakeMove(blackMove);
            return expectedState;
        }
    }
}