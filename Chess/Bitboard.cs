using System;

namespace Chess
{
    public class Bitboard
    {
        public ulong Board { get; set; }

        // Precalculated Bitboards
        private static readonly ulong FileA = 72340172838076673;
        private static readonly ulong HFile = 9259542123273814144;

        // Precalculated Attacks
        public static readonly ulong[,] PawnAttacks = new ulong[Constants.Sides, Constants.Squares];

        public Bitboard(ulong board)
        {
            Board = board;
        }

        static Bitboard()
        {
            for (int side = 0; side < Constants.Sides; ++side)
                for (int square = 0; square < Constants.Squares; ++square)
                    PawnAttacks[side, square] = Bitboard.GetPawnAttacks((Square)square, (Side)side).Board;
        }

        public void Print()
        {
            for (int rank = 0; rank < 8; ++rank)
            {
                for (int file = 0; file < 8; ++file)
                {
                    if (file == 0)
                        Console.Write((8 - rank) + "\t");
                    int square = (rank * 8) + file;
                    Console.Write(GetBit(square) + " ");
                }
                Console.Write(Environment.NewLine);
            }
            Console.WriteLine(Environment.NewLine + "\ta b c d e f g h ");
        }

        public static Bitboard GetPawnAttacks(Square square, Side side)
        {
            var ResultBoard = new Bitboard(0);
            var PawnBoard = new Bitboard(0);
            PawnBoard.SetBit(square);

            if (side == Side.White)
            {
                // Only if pawn is not on A file can it attack to the left
                if ((PawnBoard.Board & FileA) == 0)
                    ResultBoard.Board |= PawnBoard.Board >> 9;
                if ((PawnBoard.Board & HFile) == 0)
                    ResultBoard.Board |= PawnBoard.Board >> 7;
            }
            else
            {
                if ((PawnBoard.Board & FileA) == 0)
                    ResultBoard.Board |= PawnBoard.Board << 7;
                if ((PawnBoard.Board & HFile) == 0)
                    ResultBoard.Board |= PawnBoard.Board << 9;
            }

            return ResultBoard;
        }

        public int GetBit(Square square)
        {
            return GetBit((int)square);
        }

        public void SetBit(Square square)
        {
            SetBit((int)square);
        }

        public void ClearBit(Square square)
        {
            ClearBit((int)square);
        }

        public int GetBit(int square)
        {
            return (Board & ((ulong)1 << square)) > 0 ? 1 : 0;
        }

        public void SetBit(int square)
        {
            Board |= (ulong)1 << square;
        }

        public void ClearBit(int square)
        {
            Board &= ~((ulong)1 << square);
        }
    }
}