using Rudim.Board;
using Rudim.Common;
using Xunit;
using Helpers = Rudim.Test.Util.Helpers;

namespace Rudim.Test.UnitTest.Board
{
    [Collection("StateRace")]
    public class DrawTest
    {
        [Fact]
        public void ShouldDetectDrawForFiftyMoveRule()
        {
            string movesStr = "d2d4 g8f6 g1f3 g7g6 c1f4 d7d6 b1d2 f6h5 f4g5 f7f6 g5e3 e7e5 d4d5 f8e7 e3h6 c7c6 e2e4 b8d7 d5c6 b7c6 f1c4" +
                           " c8b7 d2b3 a7a5 e1g1 a5a4 b3d2 d6d5 c4d3 d7c5 a1b1 h5f4 h6f4 e5f4 d1e2 d8d7 f1e1 c5d3 e2d3 e8g8 b1d1 a8e8 " +
                           "d3d4 c6c5 d4d3 d7e6 e4d5 e6d5 d3a3 b7c6 h2h3 g8g7 g1h2 f8f7 a3c3 e8d8 c3c4 d5c4 d2c4 h7h6 d1d8 e7d8 f3d2 f7e7 " +
                           "e1d1 c6d5 c4d6 d5a2 d2e4 e7e5 e4c3 a2g8 c3a4 d8e7 d6c8 e7f8 d1d7 g8f7 a4c3 g6g5 c3b5 g7g8 d7d2 e5d5 d2d5 f7d5 " +
                           "c8d6 g8h7 g2g3 h7g6 g3g4 d5c6 h2g1 h6h5 g4h5 g6h5 g1h2 c6d7 h2g2 h5h4 b2b3 f8e7 c2c4 d7h3 g2f3 f6f5 f3e2 h4g4 d6f7 " +
                           "e7f6 f7h6 g4h5 h6f7 g5g4 f7d6 h5g6 d6b7 f6e7 b5c3 h3g2 c3d5 g2f3 e2d2 f3d5 c4d5 g4g3 f2g3 f4g3 d2e2 e7h4 e2f3 g6f6 b7c5" +
                           " f6e5 c5d3 e5d5 d3f4 d5d4 f4e2 d4e5 b3b4 g3g2 b4b5 h4d8 f3g2 e5e4 e2g3 e4f4 g3h5 f4g4 h5g3 f5f4 g3e4 f4f3 g2f1 " +
                           "g4f5 e4d6 f5f4 d6c4 f4e4 b5b6 e4d5 b6b7 d8c7 c4e3 d5c6 f1f2 c6b7 f2f3 b7c6 f3e4 c6c5 e3d5 c7a5 d5e7 a5b4 " +
                           "e7g8 b4d2 g8e7 d2b4 e7g8 b4d2 g8f6 d2e1 f6e8 e1c3 e8c7 c3f6 c7e8 f6b2 e8c7 b2f6 c7e6 c5d6 e6d4 d6c5 d4e6 c5c6 " +
                           "e6d4 c6d6 d4b5 d6c5 b5c7 f6b2 c7a6 c5c4 a6c7 c4c5 c7e6 c5d6 e6g5 b2a1 g5f7 d6c5 f7d8 c5c4 d8f7 c4c5 f7h6 a1c3 " +
                           "h6f5 c3f6 f5h6 f6c3 h6f5 c3f6 f5e3 f6g7 e3d5 c5d6 d5b4 g7f8 b4d5 f8g7 d5b4 g7f8 b4d5 f8h6 d5b6 h6g7 b6c8 d6c5 c8e7 " +
                           "g7b2 e7g8 c5d6 g8h6 d6c6 h6g8 c6d7 g8h6 d7e6 h6f5 b2a1 f5h6 e6d6 h6f5 d6c5 f5h4 a1f6 h4f3 f6c3 f3e5 c5d6 e5f3 c3f6 " +
                           "f3d4 f6e5";
            string[] moves = movesStr.Split(" ");
            BoardState boardState = BoardState.Default();
            foreach (string moveStr in moves)
            {
                boardState.GenerateMoves();
                Move move = Move.ParseLongAlgebraic(moveStr);
                move = Helpers.FindMoveFromMoveList(boardState, move);
                boardState.MakeMove(move);
            }
            Assert.False(boardState.IsDraw());
            Move fiftyMove = new Move(Square.d4, Square.c2, MoveTypes.Quiet);
            boardState.MakeMove(fiftyMove);
            Assert.True(boardState.IsDraw());
        }
        [Fact]
        public void ShouldDetectThreeFoldRepetition()
        {
            BoardState boardState = BoardState.Default();

            Move whiteKnightOut = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            Move blackKnightOut = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            Move whiteKnightBack = new Move(Square.f3, Square.g1, MoveTypes.Quiet);
            Move blackKnightBack = new Move(Square.f6, Square.g8, MoveTypes.Quiet);

            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, false);
            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, true);
        }

        [Fact]
        public void ShouldNotDetectThreeFoldRepetitionWhenMovesAreDifferent()
        {
            BoardState boardState = BoardState.Default();

            Move whiteKnightF3 = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            Move blackKnightF6 = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            Move whiteKnightE5 = new Move(Square.f3, Square.e5, MoveTypes.Quiet);
            Move blackKnightE4 = new Move(Square.f6, Square.e4, MoveTypes.Quiet);
            Move whiteKnightBackF3 = new Move(Square.e5, Square.f3, MoveTypes.Quiet);
            Move blackKnightBackF6 = new Move(Square.e4, Square.f6, MoveTypes.Quiet);

            boardState.MakeMove(whiteKnightF3);
            boardState.MakeMove(blackKnightF6);
            boardState.MakeMove(whiteKnightE5);
            boardState.MakeMove(blackKnightE4);
            boardState.MakeMove(whiteKnightBackF3);
            boardState.MakeMove(blackKnightBackF6);

            Assert.False(boardState.IsDraw());
        }

        [Fact]
        public void ShouldResetRepetitionCountAfterPawnMove()
        {
            BoardState boardState = BoardState.Default();
            Move whiteKnightOut = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            Move blackKnightOut = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            Move whiteKnightBack = new Move(Square.f3, Square.g1, MoveTypes.Quiet);
            Move blackKnightBack = new Move(Square.f6, Square.g8, MoveTypes.Quiet);
            Move whitePawnMove = new Move(Square.e2, Square.e4, MoveTypes.DoublePush);

            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whiteKnightBack, blackKnightBack, false);
            MoveAndAssert(boardState, whiteKnightOut, blackKnightOut, whitePawnMove, blackKnightBack, false);
            MoveAndAssert(boardState, whiteKnightBack, blackKnightOut, whiteKnightOut, blackKnightBack, false);
        }

        [Fact]
        public void ShouldResetRepetitionCountAfterCapture()
        {
            BoardState boardState = BoardState.Default();
            Move whiteKnightOut = new Move(Square.g1, Square.f3, MoveTypes.Quiet);
            Move blackKnightOut = new Move(Square.g8, Square.f6, MoveTypes.Quiet);
            Move whiteKnightBack = new Move(Square.f3, Square.g1, MoveTypes.Quiet);
            Move blackKnightBack = new Move(Square.f6, Square.g8, MoveTypes.Quiet);
            Move whitePawnOut = new Move(Square.e2, Square.e4, MoveTypes.DoublePush);
            Move blackPawnOut = new Move(Square.d7, Square.d5, MoveTypes.DoublePush);
            Move pawnCapture = new Move(Square.e4, Square.d5, MoveTypes.Capture);

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
            Assert.Equal(repetition, boardState.IsDraw());
        }
    }
}