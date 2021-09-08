using System;

namespace Chess
{
    class Program
    {
        static void Main(string[] args)
        {
            var Board = Bitboard.GetPawnAttacks(Square.e5, Side.White);

            Board.Print();

            // Generate A File
            //var ABoard = new Bitboard(0);
            //for(int rank = 0; rank < 8; ++rank)
            //{
            //    ABoard.SetBit(rank * 8);
            //}
            //ABoard.Print();
            //Console.WriteLine(ABoard.Board);

            //Generate H File
            //var HBoard = new Bitboard(0);
            //for (int rank = 0; rank < 8; ++rank)
            //{
            //    HBoard.SetBit(rank * 8 + 7);
            //}
            //HBoard.Print();
            //Console.WriteLine(HBoard.Board);



        }
    }
}
