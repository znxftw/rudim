using Rudim.Board;
using Rudim.Search;

namespace Rudim.CLI.UCI
{
    public class UciNewGameCommand(UciClient uciClient) : IUciCommand
    {
        public void Run(string[] parameters)
        {
            uciClient.Board = BoardState.Default();
            MoveOrdering.ResetKillerMoves();
            History.ClearBoardHistory();
        }
    }
}