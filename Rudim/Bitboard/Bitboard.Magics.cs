using Rudim.Common;

namespace Rudim
{
    public partial class Bitboard
    {
        public static Bitboard GetBishopMask(Square square)
        {
            var ResultBoard = new Bitboard(0);
            // Masking equivalent to zero blockers and no edge square
            var blockers = new Bitboard(0);
            var BishopRank = (int)square / 8;
            var BishopFile = (int)square % 8;

            for (int rank = BishopRank + 1, file = BishopFile + 1; rank < 7 && file < 7; ++rank, ++file)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, rank, file, blockers)) break;

            for (int rank = BishopRank - 1, file = BishopFile + 1; rank >= 1 && file < 7; --rank, ++file)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, rank, file, blockers)) break;

            for (int rank = BishopRank - 1, file = BishopFile - 1; rank >= 1 && file >= 1; --rank, --file)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, rank, file, blockers)) break;

            for (int rank = BishopRank + 1, file = BishopFile - 1; rank < 7 && file >= 1; ++rank, --file)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, rank, file, blockers)) break;

            return ResultBoard;
        }

        public static Bitboard GetRookMask(Square square)
        {
            var ResultBoard = new Bitboard(0);
            // Masking equivalent to zero blockers and no edge square
            var blockers = new Bitboard(0);
            var RookRank = (int)square / 8;
            var RookFile = (int)square % 8;

            for (int rank = RookRank + 1; rank < 7; ++rank)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, rank, RookFile, blockers)) break;

            for (int rank = RookRank - 1; rank >= 1; --rank)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, rank, RookFile, blockers)) break;

            for (int file = RookFile + 1; file < 7; ++file)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, RookRank, file, blockers)) break;

            for (int file = RookFile - 1; file >= 1; --file)
                if (AddSquareToBoardAndStopAtBlockers(ResultBoard, RookRank, file, blockers)) break;

            return ResultBoard;
        }

        private static ulong GeneratePotentialMagicNumber()
        {
            return Random.NextULong() & Random.NextULong() & Random.NextULong();
        }
    }
}
