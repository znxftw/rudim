using Rudim.Board;
using Rudim.Common;
using Rudim.Perft;
using Rudim.Search;

namespace Rudim
{
    public static class Global
    {
        private static bool _isReady = false;

        public static bool IsReady => _isReady;

        public static void Reset()
        {
            _isReady = false;

            MoveOrdering.ResetMoveHeuristic();
            History.ClearBoardHistory();
            PerftDriver.ResetNodeCount();

            IterativeDeepening.Score = 0;
            IterativeDeepening.BestMove = Move.NoMove;
            IterativeDeepening.Nodes = 0;

            Negamax.Nodes = 0;

            Quiescence.ResetNodes();
            TranspositionTable.ClearTable();
        }

        public static void SetReady()
        {
            _isReady = true;
        }
    }
}