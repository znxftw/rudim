using Rudim.Common;
using System.Numerics;

namespace Rudim.Board
{
    public static class PawnStructureEvaluation
    {
        private const int DoubledPawnPenalty = 10;
        private const int IsolatedPawnPenalty = 20;

        // Passed pawn bonus indexed by row (0 = rank 8, 7 = rank 1).
        // Row 0 and row 7 are 0 because white pawns promote at rank 8 (row 0) before reaching it,
        // and rank 1 (row 7) is below white's starting rank so unreachable as a pawn.
        // Smaller row index = further advanced for white = larger bonus.
        private static readonly int[] PassedPawnBonus = [0, 100, 70, 50, 30, 20, 10, 0];

        private static readonly ulong[] FileMasks = new ulong[8];
        private static readonly ulong[] AdjacentFileMasks = new ulong[8];

        // PassedPawnMasks[side, square] = squares in front of the pawn on same/adjacent files
        private static readonly ulong[,] PassedPawnMasks = new ulong[2, Constants.Squares];

        static PawnStructureEvaluation()
        {
            for (int file = 0; file < 8; file++)
            {
                ulong mask = 0;
                for (int row = 0; row < 8; row++)
                    mask |= 1ul << (row * 8 + file);
                FileMasks[file] = mask;
            }

            for (int file = 0; file < 8; file++)
            {
                AdjacentFileMasks[file] = 0;
                if (file > 0) AdjacentFileMasks[file] |= FileMasks[file - 1];
                if (file < 7) AdjacentFileMasks[file] |= FileMasks[file + 1];
            }

            for (int sq = 0; sq < Constants.Squares; sq++)
            {
                int file = sq & 7;
                int row = sq >> 3;

                ulong whiteMask = 0;
                for (int r = 0; r < row; r++)
                {
                    whiteMask |= 1ul << (r * 8 + file);
                    if (file > 0) whiteMask |= 1ul << (r * 8 + file - 1);
                    if (file < 7) whiteMask |= 1ul << (r * 8 + file + 1);
                }
                PassedPawnMasks[(int)Side.White, sq] = whiteMask;

                ulong blackMask = 0;
                for (int r = row + 1; r < 8; r++)
                {
                    blackMask |= 1ul << (r * 8 + file);
                    if (file > 0) blackMask |= 1ul << (r * 8 + file - 1);
                    if (file < 7) blackMask |= 1ul << (r * 8 + file + 1);
                }
                PassedPawnMasks[(int)Side.Black, sq] = blackMask;
            }
        }

        // Returns score from white's perspective (positive = good for white)
        public static int Evaluate(BoardState boardState)
        {
            ulong whitePawns = boardState.Pieces[(int)Side.White, (int)Piece.Pawn].Board;
            ulong blackPawns = boardState.Pieces[(int)Side.Black, (int)Piece.Pawn].Board;

            int score = 0;
            score += ScoreDoubledPawns(whitePawns, blackPawns);
            score += ScorePawnFeatures(whitePawns, blackPawns);
            return score;
        }

        private static int ScoreDoubledPawns(ulong whitePawns, ulong blackPawns)
        {
            int score = 0;
            for (int file = 0; file < 8; file++)
            {
                int whiteCount = BitOperations.PopCount(whitePawns & FileMasks[file]);
                int blackCount = BitOperations.PopCount(blackPawns & FileMasks[file]);
                if (whiteCount > 1) score -= (whiteCount - 1) * DoubledPawnPenalty;
                if (blackCount > 1) score += (blackCount - 1) * DoubledPawnPenalty;
            }
            return score;
        }

        private static int ScorePawnFeatures(ulong whitePawns, ulong blackPawns)
        {
            int score = 0;
            ulong wp = whitePawns;
            while (wp != 0)
            {
                int sq = BitOperations.TrailingZeroCount(wp);
                wp &= wp - 1;
                if ((whitePawns & AdjacentFileMasks[sq & 7]) == 0)
                    score -= IsolatedPawnPenalty;
                if ((blackPawns & PassedPawnMasks[(int)Side.White, sq]) == 0)
                    score += PassedPawnBonus[sq >> 3];
            }

            ulong bp = blackPawns;
            while (bp != 0)
            {
                int sq = BitOperations.TrailingZeroCount(bp);
                bp &= bp - 1;
                if ((blackPawns & AdjacentFileMasks[sq & 7]) == 0)
                    score += IsolatedPawnPenalty;
                if ((whitePawns & PassedPawnMasks[(int)Side.Black, sq]) == 0)
                    score -= PassedPawnBonus[7 - (sq >> 3)];
            }
            return score;
        }
    }
}
