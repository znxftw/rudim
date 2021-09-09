using System.Numerics;
using Xunit;

namespace Rudim.Test
{
    public class BitboardAttacksTest
    {
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

        [Fact]
        public void ShouldGetPawnAttacksForWhiteCornerPawns()
        {
            var PawnAttacksWhiteA1 = Bitboard.GetPawnAttacks(Square.a1, Side.White);
            var PawnAttacksWhiteA8 = Bitboard.GetPawnAttacks(Square.a8, Side.White);
            var PawnAttacksWhiteH1 = Bitboard.GetPawnAttacks(Square.h1, Side.White);
            var PawnAttacksWhiteH8 = Bitboard.GetPawnAttacks(Square.h8, Side.White);

            Assert.Equal(1, PawnAttacksWhiteA1.GetBit(Square.b2));
            Assert.Equal(1, BitOperations.PopCount(PawnAttacksWhiteA1.Board));

            Assert.Equal(0, BitOperations.PopCount(PawnAttacksWhiteA8.Board));

            Assert.Equal(1, PawnAttacksWhiteH1.GetBit(Square.g2));
            Assert.Equal(1, BitOperations.PopCount(PawnAttacksWhiteA1.Board));

            Assert.Equal(0, BitOperations.PopCount(PawnAttacksWhiteH8.Board));
        }

        [Fact]
        public void ShouldGetPawnAttacksForBlackCornerPawns()
        {
            var PawnAttacksBlackA1 = Bitboard.GetPawnAttacks(Square.a1, Side.Black);
            var PawnAttacksBlackA8 = Bitboard.GetPawnAttacks(Square.a8, Side.Black);
            var PawnAttacksBlackH1 = Bitboard.GetPawnAttacks(Square.h1, Side.Black);
            var PawnAttacksBlackH8 = Bitboard.GetPawnAttacks(Square.h8, Side.Black);

            Assert.Equal(0, BitOperations.PopCount(PawnAttacksBlackA1.Board));

            Assert.Equal(1, PawnAttacksBlackA8.GetBit(Square.b7));
            Assert.Equal(1, BitOperations.PopCount(PawnAttacksBlackA8.Board));

            Assert.Equal(0, BitOperations.PopCount(PawnAttacksBlackH1.Board));

            Assert.Equal(1, PawnAttacksBlackH8.GetBit(Square.g7));
            Assert.Equal(1, BitOperations.PopCount(PawnAttacksBlackH8.Board));
        }

        [Fact]
        public void ShouldGetEightKnightAttacksForCentralKnight()
        {
            var KnightAttacksE5 = Bitboard.GetKnightAttacks(Square.e5);

            Assert.Equal(1, KnightAttacksE5.GetBit(Square.f3));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.g4));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.g6));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.f7));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.d7));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.c6));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.c4));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.d3));
            Assert.Equal(8, BitOperations.PopCount(KnightAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetTwoKnightAttacksForCornerKnights()
        {
            var KnightAttacksA1 = Bitboard.GetKnightAttacks(Square.a1);
            var KnightAttacksA8 = Bitboard.GetKnightAttacks(Square.a8);
            var KnightAttacksH1 = Bitboard.GetKnightAttacks(Square.h1);
            var KnightAttacksH8 = Bitboard.GetKnightAttacks(Square.h8);

            Assert.Equal(1, KnightAttacksA1.GetBit(Square.b3));
            Assert.Equal(1, KnightAttacksA1.GetBit(Square.c2));
            Assert.Equal(2, BitOperations.PopCount(KnightAttacksA1.Board));

            Assert.Equal(1, KnightAttacksA8.GetBit(Square.b6));
            Assert.Equal(1, KnightAttacksA8.GetBit(Square.c7));
            Assert.Equal(2, BitOperations.PopCount(KnightAttacksA8.Board));

            Assert.Equal(1, KnightAttacksH1.GetBit(Square.g3));
            Assert.Equal(1, KnightAttacksH1.GetBit(Square.f2));
            Assert.Equal(2, BitOperations.PopCount(KnightAttacksH1.Board));

            Assert.Equal(1, KnightAttacksH8.GetBit(Square.g6));
            Assert.Equal(1, KnightAttacksH8.GetBit(Square.f7));
            Assert.Equal(2, BitOperations.PopCount(KnightAttacksH8.Board));
        }

        [Fact]
        public void ShouldGetEightKingAttacksForCentralKing()
        {
            var KingAttacksE5 = Bitboard.GetKingAttacks(Square.e5);

            Assert.Equal(1, KingAttacksE5.GetBit(Square.e4));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.e6));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.f4));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.f5));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.f6));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.d4));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.d5));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.d6));
            Assert.Equal(8, BitOperations.PopCount(KingAttacksE5.Board));
        }
    }
}
