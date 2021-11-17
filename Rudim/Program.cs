using Rudim.Board;
using Rudim.Common;
using System;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            var boardState = BoardState.ParseFEN("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
            boardState.Print();
            boardState.GenerateMoves();
            foreach(var move in boardState.Moves)
            {
                boardState.SaveState();
                boardState.MakeMove(move);

                Console.WriteLine(move.Source.ToString() + move.Target.ToString() + move.Type.ToString());
                boardState.Print();
                boardState.RestoreState();
                Console.Read();
            }
            Console.WriteLine(boardState.Moves.Count);
        }
    }
}
