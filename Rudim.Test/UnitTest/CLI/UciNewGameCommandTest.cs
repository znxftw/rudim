using Rudim.Board;
using Rudim.CLI.UCI;
using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest.CLI
{
    [Collection("StateRace")]
    public class UciNewGameCommandTest
    {
        [Fact]
        public void ShouldResetProgram()
        {
            UciClient uciClient = new();
            UciNewGameCommand newGameCommand = new(uciClient);
            uciClient.Board = BoardState.ParseFEN("rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            MoveOrdering.AddKillerMove(new Move(Square.e2, Square.e3, MoveTypes.Quiet), 0);
            History.SaveBoardHistory(Piece.None, Square.NoSquare, Castle.None, 0, 0);

            Assert.NotEqual(BoardState.Default(), uciClient.Board);
            Assert.False(MoveOrdering.IsMoveHeuristicEmpty());
            Assert.False(History.IsHistoryEmpty());

            newGameCommand.Run([]);

            Assert.Equal(BoardState.Default(), uciClient.Board);
            Assert.True(MoveOrdering.IsMoveHeuristicEmpty());
            Assert.True(History.IsHistoryEmpty());
        }
    }
}