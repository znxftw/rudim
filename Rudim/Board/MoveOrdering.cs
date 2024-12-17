using Rudim.Common;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.Board
{
    public static class MoveOrdering
    {
        private static readonly int[,] MostValuableVictimLeastValuableAttacker;
        private static Move[,] _killerMoves;
        private static int[,] _historyMoves;

        static MoveOrdering()
        {
            MostValuableVictimLeastValuableAttacker = new[,]
            {
                // P , N , B , R , Q , K , None
                { 15_000, 14_000, 13_000, 12_000, 11_000, 10_000, 0 }, // P
                { 25_000, 24_000, 23_000, 22_000, 21_000, 20_000, 0 }, // N
                { 35_000, 34_000, 33_000, 32_000, 31_000, 30_000, 0 }, // B
                { 45_000, 44_000, 43_000, 42_000, 41_000, 40_000, 0 }, // R
                { 55_000, 54_000, 53_000, 52_000, 51_000, 50_000, 0 }, // Q
                { 0, 0, 0, 0, 0, 0, 0 }, // K
                { 0, 0, 0, 0, 0, 0, 0 } // None
            };
            ResetMoveHeuristic();
        }

        public static void PopulateMoveScore(Move move, BoardState boardState, int ply = Constants.MaxPly - 1)
        {
            if (!move.IsCapture())
            {
                if (move == _killerMoves[0, ply])
                    move.Score = 9000; // TODO : Revisit, assign better values and extract to constants
                else if (move == _killerMoves[1, ply])
                    move.Score = 8000;
                else
                    move.Score = _historyMoves[boardState.GetPieceOn(move.Source), (int)move.Target];
                return;
            }
            int targetPiece;
            int sourcePiece = boardState.GetPieceOn(move.Source, boardState.SideToMove);
            if (move.Type == MoveTypes.EnPassant)
                targetPiece = (int)Piece.Pawn;
            else
                targetPiece = boardState.GetPieceOn(move.Target, boardState.SideToMove.Other());
            move.Score = MostValuableVictimLeastValuableAttacker[targetPiece, sourcePiece];
        }

        public static void AddKillerMove(Move move, int ply)
        {
            if (_killerMoves[0, ply] == move)
            {
                return;
            }

            _killerMoves[1, ply] = _killerMoves[0, ply];
            _killerMoves[0, ply] = move;
        }

        public static void AddHistoryMove(int piece, Move move, int depth)
        {
            _historyMoves[piece, (int)move.Target] += depth * depth;
        }

        public static void SortMoves(BoardState boardState)
        {
            // TODO : Partially sort within the loop only to avoid sorting elements that are not going to be queried after beta cutoff?
            boardState.Moves.Sort((a, b) => b.Score.CompareTo(a.Score));
        }

        public static void ResetMoveHeuristic()
        {
            _killerMoves = new Move[Constants.Sides, Constants.MaxPly];
            _historyMoves = new int[Constants.Pieces * 2, Constants.Squares];
        }

        public static bool IsMoveHeuristicEmpty()
        {
            return _killerMoves.Cast<Move>().All(move => move == null) && _historyMoves.Cast<int>().All(move => move == 0);
        }
    }
}