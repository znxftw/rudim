using Rudim.Board;
using Rudim.Common;
using System.Threading;

namespace Rudim.Search
{
    public static partial class Negamax
    {
        private static int PrincipalVariationSearch(BoardState boardState, int depth, int alpha, int beta,
            bool allowNullMove, CancellationToken cancellationToken)
        {
            int score = -Search(boardState, depth - 1, -alpha - 1, -alpha, allowNullMove, cancellationToken);
            if (score > alpha && score < beta)
                score = -Search(boardState, depth - 1, -beta, -alpha, allowNullMove, cancellationToken);
            return score;
        }

        private static void AlphaUpdate(int score, Move move, BoardState boardState, int depth, out int alpha, out bool foundPv, out TranspositionEntryType entryType)
        {
            entryType = TranspositionEntryType.Exact;
            if (!move.IsCapture())
                MoveOrdering.AddHistoryMove(boardState.GetPieceOn(move.Source), move, depth);
            alpha = score;
            boardState.BestMove = move;
            foundPv = true;
        }

        private static int BetaCutoff(int beta, Move move, int ply, BoardState boardState, int depth)
        {
            TranspositionTable.SubmitEntry(boardState.BoardHash, TranspositionTable.AdjustScore(beta, ply), depth, move,
                TranspositionEntryType.Beta);
            if (!move.IsCapture())
                MoveOrdering.AddKillerMove(move, ply);
            return beta;
        }

        private static void PopulateMoveScores(BoardState boardState, int ply)
        {
            Move hashMove = TranspositionTable.GetHashMove(boardState.BoardHash);
            foreach (Move move in boardState.Moves)
            {
                if (move == hashMove)
                {
                    MoveOrdering.PopulateHashMove(move);
                }
                else
                {
                    MoveOrdering.PopulateMoveScore(move, boardState, ply);
                }
            }
        }

        private static bool CanPruneNullMove(bool isPvNode, BoardState boardState, bool allowNullMove, int depth)
        {
            return allowNullMove && !isPvNode && !boardState.IsInCheck(boardState.SideToMove) && depth >= 2  &&
                   boardState.Phase > GamePhase.OnlyPawns;
        }
    }
}