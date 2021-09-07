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
                    ulong bit = Board & ((ulong)1 << square);
                    Console.Write(bit > 0 ? 1 : 0);
                }
                Console.Write(Environment.NewLine);
            }
        }
    }
}