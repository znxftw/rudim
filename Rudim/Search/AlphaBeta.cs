using Rudim.Board;
using Rudim.Common;

namespace Rudim.Search
{
    static class AlphaBeta
    {
        public static Move BestMove;
        public static int Nodes = 0;
        private static int SearchDepth = 0;

        public static int Search(BoardState boardState, int depth, int alpha, int beta)
        {
            if (depth == 0)
                return SimpleEvaluation.Evaluate(boardState);

            Nodes++;
            var originalAlpha = alpha;
            Move bestEvaluation = null;

            boardState.GenerateMoves();
            var numberOfLegalMoves = 0;
            for (var i = 0; i < boardState.Moves.Count; ++i)
            {
                boardState.SaveState();
                boardState.MakeMove(boardState.Moves[i]);
                if (boardState.IsInCheck(boardState.SideToMove.Other()))
                {
                    boardState.RestoreState();
                    continue;
                }

                int score = -Search(boardState, depth - 1, -beta, -alpha);
                boardState.RestoreState();
                numberOfLegalMoves++;
                if (score >= beta)
                    return beta;
                if (score > alpha)
                {
                    alpha = score;
                    bestEvaluation = boardState.Moves[i];
                }
            }

            if(numberOfLegalMoves == 0)
            {
                if (boardState.IsInCheck(boardState.SideToMove))
                    return -Constants.MaxCentipawnEval + (SearchDepth - depth);
                else
                    return 0;
            }

            if (alpha != originalAlpha)
                BestMove = bestEvaluation;

            return alpha;
        }

        public static int Search(BoardState boardState, int depth)
        {
            SearchDepth = depth;
            return Search(boardState, depth, int.MinValue + 1, int.MaxValue - 1);
        }
    }
}
