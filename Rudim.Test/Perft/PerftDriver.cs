using Rudim.Board;
using Rudim.Common;

namespace Rudim.Test.Perft
{
    static class PerftDriver
    {
        // Not thread safe
        public static ulong Nodes { get; set; }

        static PerftDriver()
        {
            Nodes = 0;
        }

        public static void ResetNodeCount()
        {
            Nodes = 0;
        }
        public static void Traverse(BoardState boardState, int depth)
        {
            if (depth == 0) { Nodes++; return; }
            boardState.GenerateMoves();
            foreach (var move in boardState.Moves)
            {
                boardState.SaveState();
                boardState.MakeMove(move);
                if (!boardState.IsInCheck(boardState.SideToMove.Other()))
                    Traverse(boardState, depth - 1);
                boardState.RestoreState();
            }
        }
    }
}
