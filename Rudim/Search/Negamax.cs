using Rudim.Board;
using Rudim.Common;
using System.Threading;

namespace Rudim.Search
{
    public static class Negamax
    {
        public static Move BestMove;
        public static int Nodes = 0;
        private static int _searchDepth = 0;

        private static int Search(BoardState boardState, int depth, int alpha, int beta, CancellationToken cancellationToken)
        {
            var ply = _searchDepth - depth;
            var originalAlpha = alpha;
            var entryType = TranspositionEntryType.Alpha;
            
            var (hasValue, ttScore, bestEvaluation) = TranspositionTable.GetEntry(boardState.BoardHash, alpha, beta, depth);
            if (hasValue)
            {
                BestMove = bestEvaluation;
                return ttScore;
            }
            
            if (boardState.IsRepetition())
                return 0;

            if (depth == 0)
            {
                int eval = Quiescent.Search(boardState, alpha, beta, cancellationToken);
                TranspositionTable.SubmitEntry(boardState.BoardHash, eval, depth, BestMove, TranspositionEntryType.Exact);
                return eval;
            }

            Nodes++;

            boardState.GenerateMoves();
            foreach (var move in boardState.Moves)
            {
                MoveOrdering.PopulateMoveScore(move, boardState, ply);
            }

            MoveOrdering.SortMoves(boardState);

            var numberOfLegalMoves = 0;
            foreach (var move in boardState.Moves)
            {
                if (cancellationToken.IsCancellationRequested)
                    break;
                boardState.MakeMove(move);
                if (boardState.IsInCheck(boardState.SideToMove.Other()))
                {
                    boardState.UnmakeMove(move);
                    continue;
                }
                var score = -Search(boardState, depth - 1, -beta, -alpha, cancellationToken);
                boardState.UnmakeMove(move);
                numberOfLegalMoves++;
                if (score >= beta)
                {
                    if (!move.IsCapture())
                        MoveOrdering.AddKillerMove(move, ply);
                    TranspositionTable.SubmitEntry(boardState.BoardHash, beta, depth, BestMove, TranspositionEntryType.Beta);
                    return beta;
                }
                if (score > alpha)
                {
                    alpha = score;
                    bestEvaluation = move;
                    entryType = TranspositionEntryType.Exact;
                }
            }

            if (numberOfLegalMoves == 0)
            {
                if (boardState.IsInCheck(boardState.SideToMove))
                    return -Constants.MaxCentipawnEval + (_searchDepth - depth);
                return 0;
            }

            if (alpha != originalAlpha)
                BestMove = bestEvaluation;
            
            TranspositionTable.SubmitEntry(boardState.BoardHash, alpha, depth, BestMove, entryType);
            
            return alpha;
        }

        public static int Search(BoardState boardState, int depth, CancellationToken cancellationToken)
        {
            _searchDepth = depth;
            Nodes = 0;
            BestMove = Move.NoMove;
            Quiescent.ResetNodes();
            return Search(boardState, depth, int.MinValue + 1, int.MaxValue - 1, cancellationToken);
        }
    }
}