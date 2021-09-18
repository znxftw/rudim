using Rudim.Board;
using System;
using Xunit;

namespace Rudim.Test
{
    public class BoardStateMovesTest
    {
        [Fact]
        public void ShouldGenerateMoves()
        {
            var boardState = new BoardState();
            Assert.Throws<NotImplementedException>(() => boardState.GenerateMoves());
        }
    }
}
