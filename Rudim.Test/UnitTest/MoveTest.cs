using Rudim.Board;
using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest;

public class MoveTest
{
  public static TheoryData<Square, Square, MoveType, string, string> ValidMoveToSan => new()
  {
    { Square.e2, Square.e4, MoveTypes.Quiet, "e4", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" },
    { Square.d2, Square.d4, MoveTypes.Quiet, "d4", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" },
    { Square.g1, Square.f3, MoveTypes.Quiet, "Nf3", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" },
    { Square.c1, Square.g5, MoveTypes.Quiet, "Bg5", "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1" },
    { Square.c3, Square.e5, MoveTypes.Capture, "Bxe5", "rnbqkbnr/pppp1ppp/8/4p3/8/2B5/PPPPPPPP/RNBQK1NR w KQkq - 0 1" },
    { Square.e4, Square.d5, MoveTypes.Capture, "exd5", "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1" },
    { Square.e1, Square.g1, MoveTypes.Castle, "O-O", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1" },
    { Square.e1, Square.c1, MoveTypes.Castle, "O-O-O", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1" },
    { Square.e7, Square.e8, MoveTypes.QueenPromotion, "e8=Q", "4k3/4P3/8/8/8/8/8/4K3 w - - 0 1" },
    { Square.e7, Square.f8, MoveTypes.QueenPromotionCapture, "exf8=Q", "4k1n1/4P3/8/8/8/8/8/4K3 w - - 0 1" },
    { Square.c3, Square.d5, MoveTypes.Capture, "Ncxd5", "rnbqkbnr/pppppppp/8/3p4/8/2N1N3/PPPPPPPP/R1BQKB1R w KQkq - 0 1" },
    { Square.e3, Square.d5, MoveTypes.Quiet, "Ned5", "rnbqkbnr/pppppppp/8/8/8/2N1N3/PPPPPPPP/R1BQKB1R w KQkq - 0 1" },
    { Square.a1, Square.d1, MoveTypes.Quiet, "Rad1", "r3r1k1/ppp2ppp/8/8/8/8/PPP2PPP/R3R1K1 w - - 0 1" }
  };


  [Theory]
  [MemberData(nameof(ValidMoveToSan))]
  public void ToSAN_ShouldGenerateCorrectNotation(Square source, Square target, MoveType type, string expectedSan, string fen)
  {
    var board = BoardState.ParseFEN(fen);
    var move = new Move(source, target, type);

    var san = move.ToSan(board);

    Assert.Equal(expectedSan, san);
  }
}
