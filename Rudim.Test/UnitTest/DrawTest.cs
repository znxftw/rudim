using Rudim.Board;
using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest
{
    [Collection("StateRace")]
    public class DrawTest
    {
        [Fact]
        public void ShouldDetectThreeFoldRepetition()
        {
            var boardState = BoardState.Default();

            var whiteKnightOut = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            var blackKnightOut = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            var whiteKnightBack = new Move(Square.f3, Square.g1, MoveTypes.Quiet);
            var blackKnightBack = new Move(Square.f6, Square.g8, MoveTypes.Quiet);

            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, false);
            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, true);
         }

        [Fact]
        public void ShouldNotDetectThreeFoldRepetitionWhenMovesAreDifferent()
        {
            var boardState = BoardState.Default();

            var whiteKnightF3 = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            var blackKnightF6 = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            var whiteKnightE5 = new Move(Square.f3, Square.e5, MoveTypes.Quiet);
            var blackKnightE4 = new Move(Square.f6, Square.e4, MoveTypes.Quiet);
            var whiteKnightBackF3 = new Move(Square.e5, Square.f3, MoveTypes.Quiet);
            var blackKnightBackF6 = new Move(Square.e4, Square.f6, MoveTypes.Quiet);

            boardState.MakeMove(whiteKnightF3);
            boardState.MakeMove(blackKnightF6);
            boardState.MakeMove(whiteKnightE5);
            boardState.MakeMove(blackKnightE4);
            boardState.MakeMove(whiteKnightBackF3);
            boardState.MakeMove(blackKnightBackF6);

            Assert.False(boardState.IsRepetition());
        }

        [Fact]
        public void ShouldResetRepetitionCountAfterPawnMove()
        {
            var boardState = BoardState.Default();
            var whiteKnightOut = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            var blackKnightOut = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            var whiteKnightBack = new Move(Square.f3, Square.g1, MoveTypes.Quiet);
            var blackKnightBack = new Move(Square.f6, Square.g8, MoveTypes.Quiet);
            var whitePawnMove = new Move(Square.e2, Square.e4, MoveTypes.DoublePush);

            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, false);
            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whitePawnMove, blackKnightBack, false);
            MoveAndAssert(boardState, whiteKnightBack, blackKnightOut, whiteKnightOut, blackKnightBack, false);
        }

        [Fact]
        public void ShouldResetRepetitionCountAfterCapture()
        {
            var boardState = BoardState.Default();
            var whiteKnightOut = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            var blackKnightOut = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            var whiteKnightBack = new Move(Square.f3, Square.g1, MoveTypes.Quiet);
            var blackKnightBack = new Move(Square.f6, Square.g8, MoveTypes.Quiet);
            var whitePawnOut = new Move(Square.e2, Square.e4, MoveTypes.DoublePush);
            var blackPawnOut = new Move(Square.d7, Square.d5, MoveTypes.DoublePush);
            var pawnCapture = new Move(Square.e4, Square.d5, MoveTypes.Capture);

            boardState.MakeMove(whitePawnOut);
            boardState.MakeMove(blackPawnOut);
            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, false);
            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, false);
            boardState.MakeMove(pawnCapture);
            MoveAndAssert(boardState, blackKnightOut, whiteKnightOut, blackKnightBack, whiteKnightBack, false);
            MoveAndAssert(boardState, blackKnightOut, whiteKnightOut, blackKnightBack, whiteKnightBack, true);
        }

        private static void MoveAndAssert(BoardState boardState, Move first, Move second, Move third, Move fourth,
            bool repetition)
        {
            boardState.MakeMove(first);
            boardState.MakeMove(second);
            boardState.MakeMove(third);
            boardState.MakeMove(fourth);
            Assert.Equal(repetition, boardState.IsRepetition());
        }
    }
}