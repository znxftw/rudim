using Rudim.Board;
using Rudim.Common;

namespace Rudim.Test.Perft
{
    static class PerftDriver
    {
        public static ulong nodes { get; set; }

        static PerftDriver()
        {
            nodes = 0;
        }

        public static void ResetNodeCount()
        {
            nodes = 0;
        }
        public static void Traverse(BoardState boardState, int depth)
        {
            if (depth == 0) { nodes++; return; }
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
