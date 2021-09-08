using System.Numerics;
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
        public void UnsetBitShouldBeIdempotent()
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

        [Fact]
        public void ShouldGetPawnAttacksForCentralPawn()
        {
            var PawnAttacksWhite = Bitboard.GetPawnAttacks(Square.e5, Side.White);
            var PawnAttacksBlack = Bitboard.GetPawnAttacks(Square.e5, Side.Black);

            Assert.Equal(1, PawnAttacksWhite.GetBit(Square.f6));
            Assert.Equal(1, PawnAttacksWhite.GetBit(Square.d6));
            Assert.Equal(1, PawnAttacksBlack.GetBit(Square.f4));
            Assert.Equal(1, PawnAttacksBlack.GetBit(Square.d4));
            Assert.Equal(2, BitOperations.PopCount(PawnAttacksBlack.Board));
            Assert.Equal(2, BitOperations.PopCount(PawnAttacksWhite.Board));
        }
    }
}