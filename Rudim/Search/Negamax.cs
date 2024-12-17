using Rudim.Board;
using Rudim.Common;
using System.Threading;

namespace Rudim.Search
{
    static class Negamax
    {
        public static Move BestMove;
        public static int Nodes;
        private static int _searchDepth;
        
        
        public static int Search(BoardState boardState, int depth, CancellationToken cancellationToken)
        {
            _searchDepth = depth;
            Nodes = 0;
            BestMove = Move.NoMove;
            Quiescent.ResetNodes();
            int score = Search(boardState, depth, int.MinValue + 1, int.MaxValue - 1, cancellationToken);
            if (BestMove == Move.NoMove)
            {
                boardState.GenerateMoves();
                BestMove = boardState.Moves[0];
            }
            return score;
        }

        private static int Search(BoardState boardState, int depth, int alpha, int beta, CancellationToken cancellationToken)
        {
            
            if (boardState.IsDraw())
                return 0;

            if (depth == 0)
                return Quiescent.Search(boardState, alpha, beta, cancellationToken);

            int originalAlpha = alpha;
            int ply = _searchDepth - depth;
            bool foundPv = false;
            Move bestEvaluation = Move.NoMove;
            Nodes++;
            
            boardState.GenerateMoves();
            PopulateMoveScores(boardState, ply);
            MoveOrdering.SortMoves(boardState);

            int numberOfLegalMoves = 0;
            foreach (Move move in boardState.Moves)
            {
                if (cancellationToken.IsCancellationRequested)
                    break;
                boardState.MakeMove(move);
                if (boardState.IsInCheck(boardState.SideToMove.Other()))
                {
                    boardState.UnmakeMove(move);
                    continue;
                }

                int score = SearchDeeper(boardState, depth, alpha, beta, cancellationToken, foundPv);
                
                numberOfLegalMoves++;

                boardState.UnmakeMove(move);
                if (score >= beta)
                {
                    return BetaCutoff(beta, move, ply);
                }
                if (score > alpha)
                {
                    AlphaUpdate(out alpha, score, move, out bestEvaluation, out foundPv);
                }
            }

            if (numberOfLegalMoves == 0)
            {
                if (boardState.IsInCheck(boardState.SideToMove))
                    return -Constants.MaxCentipawnEval + ply;
                return 0;
            }

            if (alpha != originalAlpha)
                BestMove = bestEvaluation;

            return alpha;
        }

        private static int SearchDeeper(BoardState boardState, int depth, int alpha, int beta,
            CancellationToken cancellationToken, bool foundPv)
        {
            int score;
            if (foundPv)
            {
                score = -Search(boardState, depth - 1, -alpha - 1, -alpha, cancellationToken);
                if (score > alpha && score < beta) 
                    score = -Search(boardState,depth - 1, -beta, -alpha, cancellationToken);
            }
            else
            {
                score = -Search(boardState, depth - 1, -beta, -alpha, cancellationToken);
            }
            return score;
        }

        private static void AlphaUpdate(out int alpha, int score, Move move, out Move bestEvaluation, out bool foundPv)
        {
            alpha = score;
            bestEvaluation = move;
            foundPv = true;
        }

        private static int BetaCutoff(int beta, Move move, int ply)
        {
            if (!move.IsCapture())
                MoveOrdering.AddKillerMove(move, ply);
            return beta;
        }

        private static void PopulateMoveScores(BoardState boardState, int ply)
        {
            foreach (Move move in boardState.Moves)
            {
                MoveOrdering.PopulateMoveScore(move, boardState, ply);
            }
        }
    }
}