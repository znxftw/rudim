using Rudim.Board;
using Rudim.Common;

namespace Rudim.Search
{
    static class AlphaBeta
    {
        public static Move BestMove;
        public static int Nodes = 0;
        public static int Search(BoardState boardState, int depth, int alpha, int beta)
        {
            if (depth <= 0)
                return SimpleEvaluation.Evaluate(boardState);

            Nodes++;
            var originalAlpha = alpha;
            Move bestEvaluation = null;

            boardState.GenerateMoves();
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

                if (score >= beta)
                    return beta;
                if (score > alpha)
                {
                    alpha = score;
                    bestEvaluation = boardState.Moves[i];
                }
            }

            if (alpha != originalAlpha)
                BestMove = bestEvaluation;

            return alpha;
        }

        public static int Search(BoardState boardState, int depth)
        {
            return Search(boardState, depth, int.MinValue + 1, int.MaxValue - 1);
        }
    }
}
