using Rudim.Board;
using Rudim.Search;

namespace Rudim.CLI.UCI
{
    internal class UciNewGameCommand(UciClient uciClient) : IUciCommand
    {
        public void Run(string[] parameters)
        {
            uciClient.Board = BoardState.Default();
            MoveOrdering.ResetKillerMoves();
        }
    }
}