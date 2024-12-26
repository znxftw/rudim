using Rudim.Board;
using Rudim.Common;
using System.Threading;

namespace Rudim.Search
{
    public static partial class Negamax
    {
        public static Move BestMove;
        public static int Nodes;
        private static int _searchDepth;


        public static int Search(BoardState boardState, int depth, CancellationToken cancellationToken)
        {
            _searchDepth = depth;
            Nodes = 0;
            BestMove = Move.NoMove;
            Quiescence.ResetNodes();
            int score = Search(boardState, depth, int.MinValue + 1, int.MaxValue - 1, true, cancellationToken);
            if (BestMove == Move.NoMove)
            {
                boardState.GenerateMoves();
                BestMove = boardState.Moves[0];
            }
            return score;
        }

        private static int Search(BoardState boardState, int depth, int alpha, int beta, bool allowNullMove,CancellationToken cancellationToken)
        {
            int ply = _searchDepth - depth;
            bool isPvNode = beta - alpha > 1;
            Nodes++;
            
            if (boardState.IsDraw())
                return 0;
            
            (bool hasValue, int ttScore, Move bestEvaluation) = TranspositionTable.GetEntry(boardState.BoardHash, alpha, beta, depth);
            if (hasValue)
            {
                BestMove = bestEvaluation;
                return TranspositionTable.RetrieveScore(ttScore, ply);
            }
            
            if (depth <= 0)
                return Quiescence.Search(boardState, alpha, beta, cancellationToken);
            
            if (CanPruneNullMove(isPvNode, boardState, allowNullMove, depth))
            {
                boardState.MakeNullMove();
                int score = -Search(boardState, depth - 1 - 2, -beta, -beta + 1, false, cancellationToken);
                boardState.UndoNullMove();
                if (score >= beta)
                    return score;
            }

            int originalAlpha = alpha;
            bool foundPv = false;
            TranspositionEntryType entryType = TranspositionEntryType.Alpha;

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

                int score = SearchDeeper(boardState, depth, alpha, beta, cancellationToken, foundPv, allowNullMove);

                numberOfLegalMoves++;

                boardState.UnmakeMove(move);
                if (score >= beta)
                {
                    return BetaCutoff(beta, move, ply, boardState, depth);
                }
                if (score > alpha)
                {
                    AlphaUpdate(score, move, boardState, depth, out alpha, out bestEvaluation, out foundPv, out entryType);
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
            
            TranspositionTable.SubmitEntry(boardState.BoardHash, TranspositionTable.AdjustScore(alpha, ply), depth, bestEvaluation, entryType);
            
            return alpha;
        }

        private static int SearchDeeper(BoardState boardState, int depth, int alpha, int beta,
            CancellationToken cancellationToken, bool foundPv, bool allowNullMove)
        {
            int score;
            if (foundPv)
            {
                score = PrincipalVariationSearch(boardState, depth, alpha, beta, allowNullMove, cancellationToken);
            }
            else
            {
                score = -Search(boardState, depth - 1, -beta, -alpha, allowNullMove, cancellationToken);
            }
            return score;
        }
    }
}