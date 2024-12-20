﻿using Rudim.Board;
using System;
using System.Collections.Generic;
using System.Numerics;

namespace Rudim.Common
{
    public static class GamePhase
    {
        private static readonly int[] PieceConstants;
        public static readonly int TotalPhase;
        public static readonly double PhaseFactor;

        static GamePhase()
        {
            PieceConstants = [0, 1, 1, 2, 4, 0];
            TotalPhase = PieceConstants[(int)Piece.Pawn] * 16 + PieceConstants[(int)Piece.Knight] * 4 + PieceConstants[(int)Piece.Bishop] * 4 + PieceConstants[(int)Piece.Rook] * 4 + PieceConstants[(int)Piece.Queen] * 2;
            PhaseFactor = 1 / (double)TotalPhase;
        }
        public static int Calculate(BoardState boardState)
        {
            int phase = 0;
            for (int piece = 0; piece < Constants.Pieces - 1; ++piece)
            {
                ulong whiteBoard = boardState.Pieces[(int)Side.White, piece].Board;
                ulong blackBoard = boardState.Pieces[(int)Side.Black, piece].Board;

                phase += PieceConstants[piece] * BitOperations.PopCount(whiteBoard);
                phase += PieceConstants[piece] * BitOperations.PopCount(blackBoard);
            }
            return Math.Min(phase, TotalPhase);
        }
    }
}