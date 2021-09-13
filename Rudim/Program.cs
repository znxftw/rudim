using Rudim.Board;
using Rudim.Common;
using System;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            var board = new BoardState();
            board.Pieces[(int)Side.Black, (int)Piece.Pawn].SetBit(Square.e5);
            board.Occupancies[(int)Side.Black].SetBit(Square.e5);

            board.Pieces[(int)Side.White, (int)Piece.Knight].SetBit(Square.e1);
            board.Occupancies[(int)Side.White].SetBit(Square.e1);

            board.Print();
        }
    }
}
