using Rudim.Board;
using Rudim.CLI;
using Rudim.Common;
using System;
using System.Diagnostics;

namespace Rudim.Search
{
    static class IterativeDeepening
    {
        public static Move BestMove;
        private static int Score;
        private static int Nodes;

        public static void Search(BoardState boardState, int depth)
        {
            var timer = new Stopwatch();
            BestMove = Move.NoMove;
            Score = 0;
            Nodes = 0;
            for(int i = 1; i <= depth; ++i)
            {
                timer.Restart();

                Score = Negamax.Search(boardState, i);
                BestMove = Negamax.BestMove;
                var NodesTraversed = Negamax.Nodes + Quiescent.Nodes;
                Nodes += NodesTraversed;

                timer.Stop();
                double time = Math.Max(timer.ElapsedMilliseconds, 1);
                var nps = (int)(Nodes / time * 1000);

                CliClient.WriteLine($"info depth {i} score cp {Score} nodes {NodesTraversed} time {time} nps {nps}"); // TODO : Refactor
            }           
        }
    }
}
