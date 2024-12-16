using Rudim.Board;
using Rudim.Common;
using System.Threading;

namespace Rudim.Search
{
    static class Quiescent
    {
        public static int Nodes { get; private set; } = 0;
        public static int Search(BoardState boardState, int alpha, int beta, CancellationToken cancellationToken)
        {
            Nodes++;

            if (boardState.IsDraw())
                return 0;

            int eval = SimpleEvaluation.Evaluate(boardState);

            if (eval >= beta)
                return beta;
            if (eval > alpha)
                alpha = eval;

            boardState.GenerateMoves();
            foreach (Move move in boardState.Moves)
            {
                MoveOrdering.PopulateMoveScore(move, boardState);
            }
            MoveOrdering.SortMoves(boardState);


            for (int i = 0; i < boardState.Moves.Count; ++i)
            {
                if (cancellationToken.IsCancellationRequested)
                    break;
                Move move = boardState.Moves[i];
                if (!move.IsCapture())
                    break; // If sorted, once a quiet move is reached we won't need to visit the remaining nodes

                boardState.MakeMove(move);
                if (boardState.IsInCheck(boardState.SideToMove.Other()))
                {
                    boardState.UnmakeMove(move);
                    continue;
                }
                int score = -Search(boardState, -beta, -alpha, cancellationToken);
                boardState.UnmakeMove(move);

                if (score >= beta)
                    return beta;
                if (score > alpha)
                    alpha = score;
            }
            return alpha;
        }

        public static void ResetNodes()
        {
            Nodes = 0;
        }
    }
}