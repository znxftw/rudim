using System;

namespace Chess
{
    public class Bitboard
    {
        public ulong Board { get; set; }

        public Bitboard(ulong board)
        {
            Board = board;
        }

        public void Print()
        {
            for (int rank = 0; rank < 8; ++rank)
            {
                for (int file = 0; file < 8; ++file)
                {
                    int square = (rank * 8) + file;
                    Console.Write(BitAt(square));
                }
                Console.Write(Environment.NewLine);
            }
        }

        private int BitAt(int square)
        {
            return (Board & ((ulong)1 << square)) > 0 ? 1 : 0;
        }
    }
}