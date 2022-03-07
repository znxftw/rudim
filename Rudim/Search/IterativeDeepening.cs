using Rudim.Board;
using Rudim.CLI;
using Rudim.Common;
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
                Nodes += Negamax.Nodes + Quiescent.Nodes;

                timer.Stop();
                double time = timer.ElapsedMilliseconds;
                var nps = (int)(Nodes / time * 1000);

                CliClient.WriteLine($"info depth {i} score cp {Score} nodes {Nodes} time {time} nps {nps}"); // TODO : Refactor
            }           
        }
    }
}
