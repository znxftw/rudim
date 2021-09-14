using Rudim.Board;
using System;
using Xunit;

namespace Rudim.Test
{
    public class BoardStateFENTest
    {
        [Fact]
        public void ShouldParseStartingFENCorrectly()
        {
            var fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";

            Assert.Throws<NotImplementedException>(() => BoardState.ParseFEN(fen));
        }
    }
}
