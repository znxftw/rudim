using Rudim.Common;

namespace Rudim
{
    public partial class Bitboard
    {
        public static Bitboard GetPawnAttacks(Square square, Side side)
        {
            var ResultBoard = new Bitboard(0);
            var PawnBoard = new Bitboard(0);
            PawnBoard.SetBit(square);

            if (side == Side.White)
            {
                ResultBoard.Board |= (PawnBoard.Board >> 9) & ~FileH;
                ResultBoard.Board |= (PawnBoard.Board >> 7) & ~FileA;
            }
            else
            {
                ResultBoard.Board |= (PawnBoard.Board << 7) & ~FileH;
                ResultBoard.Board |= (PawnBoard.Board << 9) & ~FileA;
            }

            return ResultBoard;
        }

        public static Bitboard GetKnightAttacks(Square square)
        {
            var ResultBoard = new Bitboard(0);
            var KnightBoard = new Bitboard(0);
            KnightBoard.SetBit(square);

            ResultBoard.Board |= (KnightBoard.Board << 17) & ~FileA;
            ResultBoard.Board |= (KnightBoard.Board << 10) & ~FileAB;
            ResultBoard.Board |= (KnightBoard.Board >> 6) & ~FileAB;
            ResultBoard.Board |= (KnightBoard.Board >> 15) & ~FileA;
            ResultBoard.Board |= (KnightBoard.Board << 15) & ~FileH;
            ResultBoard.Board |= (KnightBoard.Board << 6) & ~FileGH;
            ResultBoard.Board |= (KnightBoard.Board >> 10) & ~FileGH;
            ResultBoard.Board |= (KnightBoard.Board >> 17) & ~FileH;

            return ResultBoard;
        }

        public static Bitboard GetKingAttacks(Square square)
        {
            var ResultBoard = new Bitboard(0);
            var KingBoard = new Bitboard(0);
            KingBoard.SetBit(square);

            ResultBoard.Board |= (KingBoard.Board << 1) & ~FileA;
            ResultBoard.Board |= (KingBoard.Board >> 7) & ~FileA;
            ResultBoard.Board |= (KingBoard.Board << 9) & ~FileA;

            ResultBoard.Board |= (KingBoard.Board >> 1) & ~FileH;
            ResultBoard.Board |= (KingBoard.Board << 7) & ~FileH;
            ResultBoard.Board |= (KingBoard.Board >> 9) & ~FileH;

            ResultBoard.Board |= (KingBoard.Board << 8);
            ResultBoard.Board |= (KingBoard.Board >> 8);

            return ResultBoard;
        }

        public static Bitboard GetBishopAttacks(Square square, Bitboard occupancy)
        {
            var ResultBoard = new Bitboard(0);
            var BishopRank = (int)square / 8;
            var BishopFile = (int)square % 8;

            for (int rank = BishopRank + 1, file = BishopFile + 1; rank < 8 && file < 8; ++rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, occupancy)) break;

            for (int rank = BishopRank - 1, file = BishopFile + 1; rank >= 0 && file < 8; --rank, ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, occupancy)) break;

            for (int rank = BishopRank - 1, file = BishopFile - 1; rank >= 0 && file >= 0; --rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, occupancy)) break;

            for (int rank = BishopRank + 1, file = BishopFile - 1; rank < 8 && file >= 0; ++rank, --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, file, occupancy)) break;

            return ResultBoard;
        }

        public static Bitboard GetRookAttacks(Square square, Bitboard occupancy)
        {
            var ResultBoard = new Bitboard(0);
            var RookRank = (int)square / 8;
            var RookFile = (int)square % 8;

            for (int rank = RookRank + 1; rank < 8; ++rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, RookFile, occupancy)) break;

            for (int rank = RookRank - 1; rank >= 0; --rank)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, rank, RookFile, occupancy)) break;

            for (int file = RookFile + 1; file < 8; ++file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, RookRank, file, occupancy)) break;

            for (int file = RookFile - 1; file >= 0; --file)
                if (AddSquareToBoardAndStopAtOccupiedSquare(ResultBoard, RookRank, file, occupancy)) break;

            return ResultBoard;
        }

        public static Bitboard GetQueenAttacks(Square square, Bitboard occupancy)
        {
            var rookAttacks = GetRookAttacks(square, occupancy);
            var bishopAttacks = GetBishopAttacks(square, occupancy);
            return new Bitboard(rookAttacks.Board | bishopAttacks.Board);
        }

        private static bool AddSquareToBoardAndStopAtOccupiedSquare(Bitboard resultBoard, int rank, int file, Bitboard occupancy)
        {
            resultBoard.Board |= 1ul << (rank * 8) + file;
            return (1ul << (rank * 8) + file & occupancy.Board) > 0;
        }
    }
}
