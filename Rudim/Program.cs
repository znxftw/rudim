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
            board.Pawns[0].SetBit(Square.e5);
            board.BlackPieces.SetBit(Square.e5);

            board.Knights[0].SetBit(Square.e6);
            board.WhitePieces.SetBit(Square.e6);

            board.Kings[0].SetBit(Square.e1);
            board.BlackPieces.SetBit(Square.e1);

            board.Print();
        }
    }
}
