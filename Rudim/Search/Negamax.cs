﻿using Rudim.Board;
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
            Quiescence.ResetNodes();
            int score = Search(boardState, depth, int.MinValue + 1, int.MaxValue - 1, cancellationToken, true);
            if (BestMove == Move.NoMove)
            {
                boardState.GenerateMoves();
                BestMove = boardState.Moves[0];
            }
            return score;
        }

        private static int Search(BoardState boardState, int depth, int alpha, int beta, CancellationToken cancellationToken, bool followPv = false)
        {
            int ply = _searchDepth - depth;
            Nodes++;
            
            (bool hasValue, int ttScore, Move bestEvaluation) = TranspositionTable.GetEntry(boardState.BoardHash, alpha, beta, depth);
            if (hasValue)
            {
                BestMove = bestEvaluation;
                return TranspositionTable.RetrieveScore(ttScore, ply);
            }
            
            if (boardState.IsDraw())
                return 0;

            if (depth == 0)
                return Quiescence.Search(boardState, alpha, beta, cancellationToken);

            int originalAlpha = alpha;
            bool firstMove = true;
            TranspositionEntryType entryType = TranspositionEntryType.Alpha;

            boardState.GenerateMoves();
            PopulateMoveScores(boardState, ply, followPv);
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

                int score = SearchDeeper(boardState, depth, alpha, beta, cancellationToken, firstMove, followPv);

                numberOfLegalMoves++;

                boardState.UnmakeMove(move);
                firstMove = false;
                
                if (score >= beta)
                {
                    TranspositionTable.SubmitEntry(boardState.BoardHash, TranspositionTable.AdjustScore(beta, ply), depth, move, TranspositionEntryType.Beta);
                    return BetaCutoff(beta, move, ply);
                }
                if (score > alpha)
                {
                    entryType = TranspositionEntryType.Exact;
                    AlphaUpdate(out alpha, score, move, out bestEvaluation, boardState, depth);
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
            CancellationToken cancellationToken, bool firstMove, bool followPv)
        {
            int score;
            if (!firstMove)
            {
                score = -Search(boardState, depth - 1, -alpha - 1, -alpha, cancellationToken);
                if (score > alpha && score < beta)
                    score = -Search(boardState, depth - 1, -beta, -alpha, cancellationToken);
            }
            else
            {
                score = -Search(boardState, depth - 1, -beta, -alpha, cancellationToken, followPv);
            }
            return score;
        }

        private static void AlphaUpdate(out int alpha, int score, Move move, out Move bestEvaluation,
            BoardState boardState, int depth)
        {
            if(!move.IsCapture())
                MoveOrdering.AddHistoryMove(boardState.GetPieceOn(move.Source), move, depth);
            alpha = score;
            bestEvaluation = move;
        }

        private static int BetaCutoff(int beta, Move move, int ply)
        {
            if (!move.IsCapture())
                MoveOrdering.AddKillerMove(move, ply);
            return beta;
        }

        private static void PopulateMoveScores(BoardState boardState, int ply, bool followPv)
        {
            foreach (Move move in boardState.Moves)
            {
                MoveOrdering.PopulateMoveScore(move, boardState, followPv, ply);
            }
        }
    }
}