using Rudim.Board;
using Rudim.Common;
using System;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            var fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";

            var result = BoardState.ParseFEN(fen);

            foreach(var board in result.Pieces)
            {
                board.Print();
                Console.WriteLine(board.Board);
            }

            result.Print();
        }
    }
}
