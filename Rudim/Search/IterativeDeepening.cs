using Rudim.Board;
using Rudim.CLI;
using Rudim.Common;
using System;
using System.Diagnostics;
using System.Threading;

namespace Rudim.Search
{
    static class IterativeDeepening
    {
        public static Move BestMove;
        private static int Score;
        private static int Nodes;

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
                int nodesTraversed = Negamax.Nodes + Quiescent.Nodes;
                Nodes += nodesTraversed;

                timer.Stop();
                double time = Math.Max(timer.ElapsedMilliseconds, 1);
                int nps = (int)(Nodes / time * 1000);

                if (debugMode)
                {
                    CliClient.WriteLine($"info depth {i} score cp {Score} nodes {nodesTraversed} time {time} nps {nps}");
                }
            }
        }
    }
}