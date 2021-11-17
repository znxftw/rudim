using Rudim.Board;
using Rudim.Common;
using System;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            var boardState = BoardState.ParseFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

            var move = new Move(Square.a2, Square.a4, MoveType.DoublePush);
            boardState.MakeMove(move);
            boardState.Print();

            boardState.GenerateMoves();
            foreach (var m in boardState.Moves)
                Console.WriteLine(m.Source.ToString() + m.Target.ToString());

        }
    }
}
