using Rudim.Board;
using Rudim.Common;
using System.Threading;

namespace Rudim.Search
{
    public static partial class Negamax
    {
        private static int PrincipalVariationSearch(BoardState boardState, int depth, int alpha, int beta,
            CancellationToken cancellationToken)
        {
            int score = -Search(boardState, depth - 1, -alpha - 1, -alpha, cancellationToken);
            if (score > alpha && score < beta)
                score = -Search(boardState, depth - 1, -beta, -alpha, cancellationToken);
            return score;
        }

        private static void AlphaUpdate(int score, Move move, BoardState boardState, int depth, out int alpha,
            out Move bestEvaluation, out bool foundPv, out TranspositionEntryType entryType)
        {
            entryType = TranspositionEntryType.Exact;
            if (!move.IsCapture())
                MoveOrdering.AddHistoryMove(boardState.GetPieceOn(move.Source), move, depth);
            alpha = score;
            bestEvaluation = move;
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
    }
}