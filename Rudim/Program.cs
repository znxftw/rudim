using Rudim.Board;
using Rudim.Common;
using System;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            var boardState = BoardState.ParseFEN("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
            boardState.Print();
            boardState.GenerateMoves();
            foreach(var move in boardState.Moves)
            {
                Console.WriteLine(move.Source.ToString() + move.Target.ToString() + move.Type.ToString());
            }
            Console.WriteLine(boardState.Moves.Count);
        }
    }
}
