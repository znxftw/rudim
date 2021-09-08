using Xunit;

namespace Chess.Test
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
        public void ShouldUnsetSpecifiedBits()
        {
            var Board = new Bitboard(9223372036854775840);

            Board.UnsetBit(63);
            Assert.Equal((ulong)32, Board.Board);

            Board.UnsetBit(5);
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
        public void UnsetBitShouldBeIdempotent()
        {
            var Board = new Bitboard(9223372036854775808);

            Board.UnsetBit(63);
            Assert.Equal((ulong)0, Board.Board);

            Board.UnsetBit(63);
            Assert.Equal((ulong)0, Board.Board);
        }
    }
}