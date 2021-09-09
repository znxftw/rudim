using System;

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
    }
}
