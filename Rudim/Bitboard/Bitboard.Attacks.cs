using Rudim.Common;

namespace Rudim
{
    public partial struct Bitboard
    {
        public static Bitboard GetPawnAttacks(Square square, Side side)
        {
            var resultBoard = new Bitboard(0);
            var pawnBoard = new Bitboard(0);
            pawnBoard.SetBit(square);

            if (side == Side.White)
            {
                resultBoard.Board |= (pawnBoard.Board >> 9) & ~FileH;
                resultBoard.Board |= (pawnBoard.Board >> 7) & ~FileA;
            }
            else
            {
                resultBoard.Board |= (pawnBoard.Board << 7) & ~FileH;
                resultBoard.Board |= (pawnBoard.Board << 9) & ~FileA;
            }

            return resultBoard;
        }

        public static Bitboard GetKnightAttacks(Square square)
        {
            var resultBoard = new Bitboard(0);
            var knightBoard = new Bitboard(0);
            knightBoard.SetBit(square);

            resultBoard.Board |= (knightBoard.Board << 17) & ~FileA;
            resultBoard.Board |= (knightBoard.Board << 10) & ~FileAb;
            resultBoard.Board |= (knightBoard.Board >> 6) & ~FileAb;
            resultBoard.Board |= (knightBoard.Board >> 15) & ~FileA;
            resultBoard.Board |= (knightBoard.Board << 15) & ~FileH;
            resultBoard.Board |= (knightBoard.Board << 6) & ~FileGh;
            resultBoard.Board |= (knightBoard.Board >> 10) & ~FileGh;
            resultBoard.Board |= (knightBoard.Board >> 17) & ~FileH;

            return resultBoard;
        }

        public static Bitboard GetKingAttacks(Square square)
        {
            var resultBoard = new Bitboard(0);
            var kingBoard = new Bitboard(0);
            kingBoard.SetBit(square);

            resultBoard.Board |= (kingBoard.Board << 1) & ~FileA;
            resultBoard.Board |= (kingBoard.Board >> 7) & ~FileA;
            resultBoard.Board |= (kingBoard.Board << 9) & ~FileA;

            resultBoard.Board |= (kingBoard.Board >> 1) & ~FileH;
            resultBoard.Board |= (kingBoard.Board << 7) & ~FileH;
            resultBoard.Board |= (kingBoard.Board >> 9) & ~FileH;

            resultBoard.Board |= (kingBoard.Board << 8);
            resultBoard.Board |= (kingBoard.Board >> 8);

            return resultBoard;
        }

        public static Bitboard GetBishopAttacks(Square square, Bitboard occupancy)
        {
            var resultBoard = new Bitboard(0);
            var bishopRank = (int)square / 8;
            var bishopFile = (int)square % 8;

            for (int rank = bishopRank + 1, file = bishopFile + 1; rank < 8 && file < 8; ++rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancy)) break;

            for (int rank = bishopRank - 1, file = bishopFile + 1; rank >= 0 && file < 8; --rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancy)) break;

            for (int rank = bishopRank - 1, file = bishopFile - 1; rank >= 0 && file >= 0; --rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancy)) break;

            for (int rank = bishopRank + 1, file = bishopFile - 1; rank < 8 && file >= 0; ++rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, file, occupancy)) break;

            return resultBoard;
        }

        public static Bitboard GetRookAttacks(Square square, Bitboard occupancy)
        {
            var resultBoard = new Bitboard(0);
            var rookRank = (int)square / 8;
            var rookFile = (int)square % 8;

            for (var rank = rookRank + 1; rank < 8; ++rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, rookFile, occupancy)) break;

            for (var rank = rookRank - 1; rank >= 0; --rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rank, rookFile, occupancy)) break;

            for (var file = rookFile + 1; file < 8; ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rookRank, file, occupancy)) break;

            for (var file = rookFile - 1; file >= 0; --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ref resultBoard, rookRank, file, occupancy)) break;

            return resultBoard;
        }

        public static Bitboard GetQueenAttacks(Square square, Bitboard occupancy)
        {
            var rookAttacks = GetRookAttacks(square, occupancy);
            var bishopAttacks = GetBishopAttacks(square, occupancy);
            return new Bitboard(rookAttacks.Board | bishopAttacks.Board);
        }

        private static bool AddSquareToBoardAndStopAtOccupiedSquare(ref Bitboard resultBoard, int rank, int file, Bitboard occupancy)
        {
            resultBoard.Board |= 1ul << (rank * 8) + file;
            return (1ul << (rank * 8) + file & occupancy.Board) > 0;
        }
    }
}