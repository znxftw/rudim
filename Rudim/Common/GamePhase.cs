using Rudim.Board;
using System;
using System.Collections.Generic;
using System.Numerics;

namespace Rudim.Common
{
    static class GamePhase
    {
        private static readonly IDictionary<Piece, int> PieceConstants;
        public static readonly int TotalPhase;

        static GamePhase()
        {
            PieceConstants = new Dictionary<Piece, int>()
            {
                [Piece.Pawn] = 0,
                [Piece.Knight] = 1,
                [Piece.Bishop] = 1,
                [Piece.Rook] = 2,
                [Piece.Queen] = 4,
                [Piece.King] = 0
            };
            TotalPhase = PieceConstants[Piece.Pawn] * 16 + PieceConstants[Piece.Knight] * 4 + PieceConstants[Piece.Bishop] * 4 + PieceConstants[Piece.Rook] * 4 + PieceConstants[Piece.Queen] * 2;
        }
        public static int Calculate(BoardState boardState)
        {
            int phase = 0;
            for (var piece = Piece.Pawn; piece < Piece.King; ++piece)
            {
                var whiteBoard = boardState.Pieces[(int)Side.White, (int)piece].Board;
                var blackBoard = boardState.Pieces[(int)Side.Black, (int)piece].Board;

                phase += PieceConstants[piece] * BitOperations.PopCount(whiteBoard);
                phase += PieceConstants[piece] * BitOperations.PopCount(blackBoard);
            }
            return Math.Min(phase, TotalPhase);
        }
    }
}
