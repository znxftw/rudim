using System.Numerics;
using Xunit;

namespace Rudim.Test
{
    public class BitboardMagicsTest
    {
        [Fact]
        public void ShouldGetMaskForCentralBishop()
        {
            var BishopMaskE5 = Bitboard.GetBishopMask(Square.e5);

            Assert.Equal(1, BishopMaskE5.GetBit(Square.f4));
            Assert.Equal(1, BishopMaskE5.GetBit(Square.g3));

            Assert.Equal(1, BishopMaskE5.GetBit(Square.f6));
            Assert.Equal(1, BishopMaskE5.GetBit(Square.g7));

            Assert.Equal(1, BishopMaskE5.GetBit(Square.d4));
            Assert.Equal(1, BishopMaskE5.GetBit(Square.c3));
            Assert.Equal(1, BishopMaskE5.GetBit(Square.b2));

            Assert.Equal(1, BishopMaskE5.GetBit(Square.d6));
            Assert.Equal(1, BishopMaskE5.GetBit(Square.c7));

            Assert.Equal(9, BitOperations.PopCount(BishopMaskE5.Board));
        }


        [Fact]
        public void ShouldGetMaskForCornerBishop()
        {
            var BishopMaskA1 = Bitboard.GetBishopMask(Square.a1);

            Assert.Equal(1, BishopMaskA1.GetBit(Square.b2));
            Assert.Equal(1, BishopMaskA1.GetBit(Square.c3));
            Assert.Equal(1, BishopMaskA1.GetBit(Square.d4));
            Assert.Equal(1, BishopMaskA1.GetBit(Square.e5));
            Assert.Equal(1, BishopMaskA1.GetBit(Square.f6));
            Assert.Equal(1, BishopMaskA1.GetBit(Square.g7));
            Assert.Equal(6, BitOperations.PopCount(BishopMaskA1.Board));
        }

        [Fact]
        public void ShouldGetMaskForCentralRook()
        {
            var RookMaskE5 = Bitboard.GetRookMask(Square.e5);

            Assert.Equal(1, RookMaskE5.GetBit(Square.e2));
            Assert.Equal(1, RookMaskE5.GetBit(Square.e3));
            Assert.Equal(1, RookMaskE5.GetBit(Square.e4));
            Assert.Equal(1, RookMaskE5.GetBit(Square.e6));
            Assert.Equal(1, RookMaskE5.GetBit(Square.e7));

            Assert.Equal(1, RookMaskE5.GetBit(Square.b5));
            Assert.Equal(1, RookMaskE5.GetBit(Square.c5));
            Assert.Equal(1, RookMaskE5.GetBit(Square.d5));
            Assert.Equal(1, RookMaskE5.GetBit(Square.f5));
            Assert.Equal(1, RookMaskE5.GetBit(Square.g5));

            Assert.Equal(10, BitOperations.PopCount(RookMaskE5.Board));
        }

        [Fact]
        public void ShouldGetMaskForCornerRook()
        {
            var RookMaskA1 = Bitboard.GetRookMask(Square.a1);

            Assert.Equal(1, RookMaskA1.GetBit(Square.a2));
            Assert.Equal(1, RookMaskA1.GetBit(Square.a3));
            Assert.Equal(1, RookMaskA1.GetBit(Square.a4));
            Assert.Equal(1, RookMaskA1.GetBit(Square.a5));
            Assert.Equal(1, RookMaskA1.GetBit(Square.a6));
            Assert.Equal(1, RookMaskA1.GetBit(Square.a7));

            Assert.Equal(1, RookMaskA1.GetBit(Square.b1));
            Assert.Equal(1, RookMaskA1.GetBit(Square.c1));
            Assert.Equal(1, RookMaskA1.GetBit(Square.d1));
            Assert.Equal(1, RookMaskA1.GetBit(Square.e1));
            Assert.Equal(1, RookMaskA1.GetBit(Square.f1));
            Assert.Equal(1, RookMaskA1.GetBit(Square.g1));

            Assert.Equal(12, BitOperations.PopCount(RookMaskA1.Board));
        }

        [Fact]
        public void ShouldGetOccupancyMappingForBishop()
        {
            var Mask = Bitboard.GetBishopMask(Square.e5);
            var Index = 0b100100100;
            var BitsInMask = BitOperations.PopCount(Mask.Board);
            var OccupancyMapping = Bitboard.GetOccupancyMapping(Index, BitsInMask, Mask);

            Assert.Equal(1, OccupancyMapping.GetBit(Square.d6));
            Assert.Equal(1, OccupancyMapping.GetBit(Square.f4));
            Assert.Equal(1, OccupancyMapping.GetBit(Square.b2));

            Assert.Equal(3, BitOperations.PopCount(OccupancyMapping.Board));
        }

        [Fact]
        public void ShouldGetOccupancyMappingForRook()
        {
            var Mask = Bitboard.GetRookMask(Square.e5);
            var Index = 0b0100100100;
            var BitsInMask = BitOperations.PopCount(Mask.Board);
            var OccupancyMapping = Bitboard.GetOccupancyMapping(Index, BitsInMask, Mask);

            Assert.Equal(1, OccupancyMapping.GetBit(Square.e3));
            Assert.Equal(1, OccupancyMapping.GetBit(Square.f5));
            Assert.Equal(1, OccupancyMapping.GetBit(Square.b5));

            Assert.Equal(3, BitOperations.PopCount(OccupancyMapping.Board));
            Assert.NotNull(Mask);
        }

        [Fact]
        public void ShouldGenerateMagicNumbersForAllSquares()
        {
            for(int square = 0; square < 64; ++square)
            {
                Bitboard.FindMagicNumber((Square)square, Bitboard.BishopMaskBits[square], true);
                Bitboard.FindMagicNumber((Square)square, Bitboard.RookMaskBits[square], false);
            }
        }
    }
}
