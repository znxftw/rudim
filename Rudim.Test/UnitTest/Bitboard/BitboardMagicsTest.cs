using Rudim.Common;
using System.Numerics;
using Xunit;

namespace Rudim.Test.UnitTest.Bitboard
{
    public class BitboardMagicsTest
    {
        [Fact]
        public void ShouldGetMaskForCentralBishop()
        {
            var bishopMaskE5 = Rudim.Bitboard.GetBishopMask(Square.e5);

            Assert.Equal(1, bishopMaskE5.GetBit(Square.f4));
            Assert.Equal(1, bishopMaskE5.GetBit(Square.g3));

            Assert.Equal(1, bishopMaskE5.GetBit(Square.f6));
            Assert.Equal(1, bishopMaskE5.GetBit(Square.g7));

            Assert.Equal(1, bishopMaskE5.GetBit(Square.d4));
            Assert.Equal(1, bishopMaskE5.GetBit(Square.c3));
            Assert.Equal(1, bishopMaskE5.GetBit(Square.b2));

            Assert.Equal(1, bishopMaskE5.GetBit(Square.d6));
            Assert.Equal(1, bishopMaskE5.GetBit(Square.c7));

            Assert.Equal(9, BitOperations.PopCount(bishopMaskE5.Board));
        }


        [Fact]
        public void ShouldGetMaskForCornerBishop()
        {
            var bishopMaskA1 = Rudim.Bitboard.GetBishopMask(Square.a1);

            Assert.Equal(1, bishopMaskA1.GetBit(Square.b2));
            Assert.Equal(1, bishopMaskA1.GetBit(Square.c3));
            Assert.Equal(1, bishopMaskA1.GetBit(Square.d4));
            Assert.Equal(1, bishopMaskA1.GetBit(Square.e5));
            Assert.Equal(1, bishopMaskA1.GetBit(Square.f6));
            Assert.Equal(1, bishopMaskA1.GetBit(Square.g7));
            Assert.Equal(6, BitOperations.PopCount(bishopMaskA1.Board));
        }

        [Fact]
        public void ShouldGetMaskForCentralRook()
        {
            var rookMaskE5 = Rudim.Bitboard.GetRookMask(Square.e5);

            Assert.Equal(1, rookMaskE5.GetBit(Square.e2));
            Assert.Equal(1, rookMaskE5.GetBit(Square.e3));
            Assert.Equal(1, rookMaskE5.GetBit(Square.e4));
            Assert.Equal(1, rookMaskE5.GetBit(Square.e6));
            Assert.Equal(1, rookMaskE5.GetBit(Square.e7));

            Assert.Equal(1, rookMaskE5.GetBit(Square.b5));
            Assert.Equal(1, rookMaskE5.GetBit(Square.c5));
            Assert.Equal(1, rookMaskE5.GetBit(Square.d5));
            Assert.Equal(1, rookMaskE5.GetBit(Square.f5));
            Assert.Equal(1, rookMaskE5.GetBit(Square.g5));

            Assert.Equal(10, BitOperations.PopCount(rookMaskE5.Board));
        }

        [Fact]
        public void ShouldGetMaskForCornerRook()
        {
            var rookMaskA1 = Rudim.Bitboard.GetRookMask(Square.a1);

            Assert.Equal(1, rookMaskA1.GetBit(Square.a2));
            Assert.Equal(1, rookMaskA1.GetBit(Square.a3));
            Assert.Equal(1, rookMaskA1.GetBit(Square.a4));
            Assert.Equal(1, rookMaskA1.GetBit(Square.a5));
            Assert.Equal(1, rookMaskA1.GetBit(Square.a6));
            Assert.Equal(1, rookMaskA1.GetBit(Square.a7));

            Assert.Equal(1, rookMaskA1.GetBit(Square.b1));
            Assert.Equal(1, rookMaskA1.GetBit(Square.c1));
            Assert.Equal(1, rookMaskA1.GetBit(Square.d1));
            Assert.Equal(1, rookMaskA1.GetBit(Square.e1));
            Assert.Equal(1, rookMaskA1.GetBit(Square.f1));
            Assert.Equal(1, rookMaskA1.GetBit(Square.g1));

            Assert.Equal(12, BitOperations.PopCount(rookMaskA1.Board));
        }

        [Fact]
        public void ShouldGetOccupancyMappingForBishop()
        {
            var mask = Rudim.Bitboard.GetBishopMask(Square.e5);
            const int index = 0b100100100;
            var bitsInMask = BitOperations.PopCount(mask.Board);
            var occupancyMapping = Rudim.Bitboard.GetOccupancyMapping(index, bitsInMask, mask);

            Assert.Equal(1, occupancyMapping.GetBit(Square.d6));
            Assert.Equal(1, occupancyMapping.GetBit(Square.f4));
            Assert.Equal(1, occupancyMapping.GetBit(Square.b2));

            Assert.Equal(3, BitOperations.PopCount(occupancyMapping.Board));
        }

        [Fact]
        public void ShouldGetOccupancyMappingForRook()
        {
            var mask = Rudim.Bitboard.GetRookMask(Square.e5);
            const int index = 0b0100100100;
            var bitsInMask = BitOperations.PopCount(mask.Board);
            var occupancyMapping = Rudim.Bitboard.GetOccupancyMapping(index, bitsInMask, mask);

            Assert.Equal(1, occupancyMapping.GetBit(Square.e3));
            Assert.Equal(1, occupancyMapping.GetBit(Square.f5));
            Assert.Equal(1, occupancyMapping.GetBit(Square.b5));

            Assert.Equal(3, BitOperations.PopCount(occupancyMapping.Board));
        }

        [Fact(Skip = "Heavy test, one-time run function, to be re-enabled if changing FindMagicNumber / regenerating magics")]
        public void ShouldGenerateMagicNumbersForAllSquares()
        {
            for (int square = 0; square < 64; ++square)
            {
                Rudim.Bitboard.FindMagicNumber((Square)square, Rudim.Bitboard.BishopMaskBits[square], true);
                Rudim.Bitboard.FindMagicNumber((Square)square, Rudim.Bitboard.RookMaskBits[square], false);
            }
        }
    }
}