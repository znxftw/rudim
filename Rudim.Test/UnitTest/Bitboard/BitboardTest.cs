using Xunit;

namespace Rudim.Test.UnitTest.Bitboard
{
    public class BitboardTest
    {
        [Fact]
        public void ShouldSetSpecifiedBits()
        {
            var board = new Rudim.Bitboard(0);

            board.SetBit(5);
            Assert.Equal(32ul, board.Board);

            board.SetBit(63);
            Assert.Equal(9223372036854775840, board.Board);
        }

        [Fact]
        public void ShouldClearSpecifiedBits()
        {
            var board = new Rudim.Bitboard(9223372036854775840);

            board.ClearBit(63);
            Assert.Equal(32ul, board.Board);

            board.ClearBit(5);
            Assert.Equal(0ul, board.Board);
        }

        [Fact]
        public void SetBitShouldBeIdempotent()
        {
            var board = new Rudim.Bitboard(0);

            board.SetBit(63);
            Assert.Equal(9223372036854775808ul, board.Board);

            board.SetBit(63);
            Assert.Equal(9223372036854775808ul, board.Board);
        }

        [Fact]
        public void ClearBitShouldBeIdempotent()
        {
            var board = new Rudim.Bitboard(9223372036854775808);

            board.ClearBit(63);
            Assert.Equal(0ul, board.Board);

            board.ClearBit(63);
            Assert.Equal(0ul, board.Board);
        }

        [Fact]
        public void ShouldGetGivenBits()
        {
            var board = new Rudim.Bitboard(9223372036854775808);

            Assert.Equal(0, board.GetBit(0));
            Assert.Equal(0, board.GetBit(5));
            Assert.Equal(1, board.GetBit(63));
        }
    }
}