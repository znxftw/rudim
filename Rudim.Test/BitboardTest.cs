using Xunit;

namespace Rudim.Test
{
    public class BitboardTest
    {
        [Fact]
        public void ShouldSetSpecifiedBits()
        {
            var Board = new Bitboard(0);

            Board.SetBit(5);
            Assert.Equal((ulong)32, Board.Board);

            Board.SetBit(63);
            Assert.Equal(9223372036854775840, Board.Board);
        }

        [Fact]
        public void ShouldClearSpecifiedBits()
        {
            var Board = new Bitboard(9223372036854775840);

            Board.ClearBit(63);
            Assert.Equal((ulong)32, Board.Board);

            Board.ClearBit(5);
            Assert.Equal((ulong)0, Board.Board);
        }

        [Fact]
        public void SetBitShouldBeIdempotent()
        {
            var Board = new Bitboard(0);

            Board.SetBit(63);
            Assert.Equal(9223372036854775808, Board.Board);

            Board.SetBit(63);
            Assert.Equal(9223372036854775808, Board.Board);
        }

        [Fact]
        public void ClearBitShouldBeIdempotent()
        {
            var Board = new Bitboard(9223372036854775808);

            Board.ClearBit(63);
            Assert.Equal((ulong)0, Board.Board);

            Board.ClearBit(63);
            Assert.Equal((ulong)0, Board.Board);
        }

        [Fact]
        public void ShouldGetGivenBits()
        {
            var Board = new Bitboard(9223372036854775808);

            Assert.Equal(0, Board.GetBit(0));
            Assert.Equal(0, Board.GetBit(5));
            Assert.Equal(1, Board.GetBit(63));
        }
    }
}