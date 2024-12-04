using Rudim.Board;
using Rudim.Common;
using System.Threading;

namespace Rudim.Search
{
    static class Negamax
    {
        public static Move BestMove;
        public static int Nodes = 0;
        private static int _searchDepth = 0;

        private static int Search(BoardState boardState, int depth, int alpha, int beta, CancellationToken cancellationToken)
        {
            if (depth == 0)
                return Quiescent.Search(boardState, alpha, beta, cancellationToken);

            Nodes++;
            var originalAlpha = alpha;
            Move bestEvaluation = null;

            boardState.GenerateMoves();
            var ply = _searchDepth - depth;
            // TODO : Flag in GenerateMoves to avoid extra iteration?
            foreach (var move in boardState.Moves)
            {
                MoveOrdering.PopulateMoveScore(move, boardState, ply);
            }

            MoveOrdering.SortMoves(boardState);

            var numberOfLegalMoves = 0;
            for (var i = 0; i < boardState.Moves.Count; ++i)
            {
                if (cancellationToken.IsCancellationRequested)
                    break;
                var move = boardState.Moves[i];
                boardState.MakeMove(move);
                if (boardState.IsInCheck(boardState.SideToMove.Other()))
                {
                    boardState.UnmakeMove(move);
                    continue;
                }

                int score = -Search(boardState, depth - 1, -beta, -alpha, cancellationToken);
                boardState.UnmakeMove(move);
                numberOfLegalMoves++;
                if (score >= beta)
                {
                    if (!move.IsCapture())
                        MoveOrdering.AddKillerMove(move, ply);
                    return beta;
                }
                if (score > alpha)
                {
                    alpha = score;
                    bestEvaluation = boardState.Moves[i];
                }
            }

            if (numberOfLegalMoves == 0)
            {
                if (boardState.IsInCheck(boardState.SideToMove))
                    return -Constants.MaxCentipawnEval + (_searchDepth - depth);
                else
                    return 0;
            }

            if (alpha != originalAlpha)
                BestMove = bestEvaluation;

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