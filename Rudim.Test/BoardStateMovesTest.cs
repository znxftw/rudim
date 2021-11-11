using Rudim.Board;
using Xunit;

namespace Rudim.Test
{
    public class BoardStateMovesTest
    {
        [Fact]
        public void ShouldGenerateMoves()
        {
            var advancedMovesPosition = BoardState.ParseFEN("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
            var randomPosition = BoardState.ParseFEN("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
            var startingPosition = BoardState.ParseFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

            advancedMovesPosition.GenerateMoves();
            randomPosition.GenerateMoves();
            startingPosition.GenerateMoves();

            // Are more rigorous asserts required here?
            Assert.Equal(42, advancedMovesPosition.Moves.Count);
            Assert.Equal(48, randomPosition.Moves.Count);
            Assert.Equal(20, startingPosition.Moves.Count);
        }
    }
}
