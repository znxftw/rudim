﻿using Rudim.Board;
using Rudim.Common;
using System.Diagnostics.CodeAnalysis;

namespace Rudim.Perft
{
    [ExcludeFromCodeCoverage]
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
            foreach (Move move in boardState.Moves)
            {
                boardState.MakeMove(move);
                if (!boardState.IsInCheck(boardState.SideToMove.Other()))
                    Traverse(boardState, depth - 1);
                boardState.UnmakeMove(move);
            }
        }
    }
}