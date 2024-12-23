using Rudim.Board;
using Rudim.CLI;
using Rudim.Common;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading;

namespace Rudim.Search
{
    public static class IterativeDeepening
    {
        public static Move BestMove;
        public static int Score;
        public static int Nodes;

        public static void Search(BoardState boardState, int depth, CancellationToken cancellationToken, ref bool debugMode)
        {
            Stopwatch timer = new Stopwatch();
            BestMove = Move.NoMove;
            Score = 0;
            Nodes = 0;
            for (int i = 1; i <= depth; ++i)
            {
                timer.Restart();
                Score = Negamax.Search(boardState, i, cancellationToken);

                if (cancellationToken.IsCancellationRequested)
                    break;

                BestMove = Negamax.BestMove;
                int nodesTraversed = Negamax.Nodes + Quiescence.Nodes;
                Nodes += nodesTraversed;

                timer.Stop();
                double time = Math.Max(timer.ElapsedMilliseconds, 1);
                int nps = (int)(Nodes / time * 1000);
                
                List<Move> pv = TranspositionTable.CollectPrincipalVariation(boardState);
                string pvString = string.Join(' ', pv.Select(move =>
                    move.Source.ToString() + move.Target.ToString() + move.GetPromotionChar()));
                MoveOrdering.RepopulatePrincipalVariationScores(pv);

                if (debugMode)
                {
                    CliClient.WriteLine($"info depth {i} score cp {Score} nodes {nodesTraversed} time {time} nps {nps} pv {pvString}");
                }
            }
        }
    }
}