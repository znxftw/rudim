using Rudim.Common;
using System.Numerics;

namespace Rudim
{
    public partial class Bitboard
    {
        public static Bitboard GetBishopMask(Square square)
        {
            var ResultBoard = new Bitboard(0);
            // Masking equivalent to attacaks with zero blockers and no edge square
            var OccupancyBoard = new Bitboard(0);
            var BishopRank = (int)square / 8;
            var BishopFile = (int)square % 8;

            for (int rank = BishopRank + 1, file = BishopFile + 1; rank < 7 && file < 7; ++rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, OccupancyBoard)) break;

            for (int rank = BishopRank - 1, file = BishopFile + 1; rank >= 1 && file < 7; --rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, OccupancyBoard)) break;

            for (int rank = BishopRank - 1, file = BishopFile - 1; rank >= 1 && file >= 1; --rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, OccupancyBoard)) break;

            for (int rank = BishopRank + 1, file = BishopFile - 1; rank < 7 && file >= 1; ++rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, OccupancyBoard)) break;

            return ResultBoard;
        }

        public static Bitboard GetRookMask(Square square)
        {
            var ResultBoard = new Bitboard(0);
            // Masking equivalent to attacks with zero blockers and no edge square 
            var OccupancyBoard = new Bitboard(0);
            var RookRank = (int)square / 8;
            var RookFile = (int)square % 8;

            for (int rank = RookRank + 1; rank < 7; ++rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, RookFile, OccupancyBoard)) break;

            for (int rank = RookRank - 1; rank >= 1; --rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, RookFile, OccupancyBoard)) break;

            for (int file = RookFile + 1; file < 7; ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, RookRank, file, OccupancyBoard)) break;

            for (int file = RookFile - 1; file >= 1; --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, RookRank, file, OccupancyBoard)) break;

            return ResultBoard;
        }

        public static Bitboard GetOccupancyMapping(int index, int nBitsInMask, Bitboard mask)
        {
            var OccupancyMapping = new Bitboard(0);
            for (int count = 0; count < nBitsInMask; ++count)
            {
                int square = BitOperations.TrailingZeroCount(mask.Board);
                mask.ClearBit(square);

                if ((index & (1 << count)) != 0)
                    OccupancyMapping.Board |= (ulong)1 << square;

            }
            return OccupancyMapping;
        }
        private static ulong GeneratePotentialMagicNumber()
        {
            return Random.NextULong() & Random.NextULong() & Random.NextULong();
        }
    }
}
