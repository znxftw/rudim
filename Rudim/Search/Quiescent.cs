using Rudim.Board;
using Rudim.Common;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Rudim.Search
{
    static class Quiescent
    {
        public static int Nodes = 0;
        public static int Search(BoardState boardState, int alpha, int beta)
        {
            Nodes++;

            var eval = SimpleEvaluation.Evaluate(boardState);

            if (eval >= beta)
                return beta;
            if (eval > alpha)
                alpha = eval;

            boardState.GenerateMoves();
            // TODO : Flag in GenerateMoves to avoid extra iteration?
            foreach (var move in boardState.Moves)
            {
                MoveOrdering.PopulateMoveScore(move, boardState);
            }
            MoveOrdering.SortMoves(boardState);
            

            for (var i = 0; i < boardState.Moves.Count; ++i)
            {
                if (!boardState.Moves[i].IsCapture())
                    break; // If sorted, once a quiet move is reached we won't need to visit the remaining nodes

                boardState.SaveState();
                boardState.MakeMove(boardState.Moves[i]);
                if (boardState.IsInCheck(boardState.SideToMove.Other()))
                {
                    boardState.RestoreState();
                    continue;
                }
                var score = -Search(boardState, -beta, -alpha);
                boardState.RestoreState();

                if (score >= beta)
                    return beta;
                if (score > alpha)
                    alpha = score;
            }
            return alpha;
        }
    }
}
