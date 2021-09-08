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
                    if (file == 0)
                        Console.Write(rank + "\t");
                    int square = (rank * 8) + file;
                    Console.Write(BitAt(square) + " ");
                }
                Console.Write(Environment.NewLine);
            }
            Console.WriteLine(Environment.NewLine + "\ta b c d e f g h ");
        }

        private int BitAt(int square)
        { 
            return (Board & ((ulong)1 << square)) > 0 ? 1 : 0;
        }
    }
}