using Rudim.Common;
using System.Linq;
using System.Numerics;

namespace Rudim
{
    public partial struct Bitboard
    {
        public static ulong FindMagicNumber(Square square, int bitsInMask, bool isBishop)
        {
            var maxIndex = 1 << bitsInMask;
            var occupancyMappings = new Bitboard[Constants.MaxMaskIndex];
            var attacks = new Bitboard[Constants.MaxMaskIndex];
            var mask = isBishop ? GetBishopMask(square) : GetRookMask(square);

            for (int index = 0; index < maxIndex; ++index)
            {
                occupancyMappings[index] = GetOccupancyMapping(index, bitsInMask, mask);
                attacks[index] = isBishop ? GetBishopAttacks(square, occupancyMappings[index]) : GetRookAttacks(square, occupancyMappings[index]);
            }

            for (int count = 0; count < Constants.MaxRetryCount; ++count)
            {
                ulong potentialMagicNumber = GeneratePotentialMagicNumber();

                // Early exit impossible magics
                if (BitOperations.PopCount((mask.Board * potentialMagicNumber) & 0xFF00000000000000) < 6)
                    continue;

                var magicAttacks = Enumerable.Repeat(new Bitboard(0xFFFFFFFFFFFFFFFF), Constants.MaxMaskIndex).ToArray();
                var failureFlag = false;
                for (var index = 0; index < maxIndex; ++index)
                {
                    var magicIndex = (int)((occupancyMappings[index].Board * potentialMagicNumber) >> (64 - bitsInMask));
                    if (magicAttacks[magicIndex].Board == 0xFFFFFFFFFFFFFFFF)
                        magicAttacks[magicIndex] = attacks[index];
                    else if (!Equals(magicAttacks[magicIndex], attacks[index]))
                        failureFlag = true;
                }
                // PotentialMagicNumber is actually the magic number
                if (!failureFlag)
                    return potentialMagicNumber;
            }
            throw new ExceededMaximumRetryException("No magic number found");
        }
        public static Bitboard GetBishopMask(Square square)
        {
            var resultBoard = new Bitboard(0);
            // Masking equivalent to attacks with zero blockers and no edge square
            var occupancyBoard = new Bitboard(0);
            var bishopRank = (int)square / 8;
            var bishopFile = (int)square % 8;

            for (int rank = bishopRank + 1, file = bishopFile + 1; rank < 7 && file < 7; ++rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancyBoard))
                    break;

            for (int rank = bishopRank - 1, file = bishopFile + 1; rank >= 1 && file < 7; --rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancyBoard))
                    break;

            for (int rank = bishopRank - 1, file = bishopFile - 1; rank >= 1 && file >= 1; --rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancyBoard))
                    break;

            for (int rank = bishopRank + 1, file = bishopFile - 1; rank < 7 && file >= 1; ++rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancyBoard))
                    break;

            return resultBoard;
        }

        public static Bitboard GetRookMask(Square square)
        {
            var resultBoard = new Bitboard(0);
            // Masking equivalent to attacks with zero blockers and no edge square 
            var occupancyBoard = new Bitboard(0);
            var rookRank = (int)square / 8;
            var rookFile = (int)square % 8;

            for (int rank = rookRank + 1; rank < 7; ++rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, rookFile, occupancyBoard))
                    break;

            for (int rank = rookRank - 1; rank >= 1; --rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, rookFile, occupancyBoard))
                    break;

            for (int file = rookFile + 1; file < 7; ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rookRank, file, occupancyBoard))
                    break;

            for (int file = rookFile - 1; file >= 1; --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rookRank, file, occupancyBoard))
                    break;

            return resultBoard;
        }

        public static Bitboard GetOccupancyMapping(int index, int nBitsInMask, Bitboard mask)
        {
            var occupancyMapping = new Bitboard(0);
            var temporaryMask = new Bitboard(mask.Board);
            for (int count = 0; count < nBitsInMask; ++count)
            {
                int square = BitOperations.TrailingZeroCount(temporaryMask.Board);
                temporaryMask.ClearBit(square);

                if ((index & (1 << count)) != 0)
                    occupancyMapping.Board |= 1ul << square;
            }
            return occupancyMapping;
        }
        private static ulong GeneratePotentialMagicNumber()
        {
            return Random.NextULong() & Random.NextULong() & Random.NextULong();
        }
    }
}