using System;

namespace Chess
{
    class Program
    {
        static void Main(string[] args)
        {
            //var Board = Bitboard.GetPawnAttacks(Square.e5, Side.White);

            //foreach (ulong board in Bitboard.PawnAttacks)
            //{
            //    new Bitboard(board).Print();
            //}

            // Generate A File
            //var ABoard = new Bitboard(0);
            //for (int rank = 0; rank < 8; ++rank)
            //{
            //    ABoard.SetBit(rank * 8 + 6);
            //}
            //ABoard.Print();
            //Console.WriteLine(ABoard.Board);

            Bitboard.GetKnightAttacks(Square.e5).Print();
        }
    }
}
