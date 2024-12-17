using Rudim.Board;
using Rudim.Common;
using Rudim.Perft;
using Rudim.Search;

namespace Rudim
{
    public static class Global
    {
        public static void Reset()
        {
            MoveOrdering.ResetMoveHeuristic();
            History.ClearBoardHistory();
            PerftDriver.ResetNodeCount();

            IterativeDeepening.Score = 0;
            IterativeDeepening.BestMove = Move.NoMove;
            IterativeDeepening.Nodes = 0;

            Negamax.Nodes = 0;
            Negamax.BestMove = Move.NoMove;

            Quiescent.ResetNodes();
            TranspositionTable.ClearTable();
        }
    }
}