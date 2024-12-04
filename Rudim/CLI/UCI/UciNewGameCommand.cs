using Rudim.Board;
using Rudim.Search;

namespace Rudim.CLI.UCI
{
    internal class UciNewGameCommand : IUciCommand
    {
        private readonly UciClient _uciClient;

        public UciNewGameCommand(UciClient uciClient)
        {
            _uciClient = uciClient;
        }

        public void Run(string[] parameters)
        {
            _uciClient.Board = BoardState.Default();
            MoveOrdering.ResetKillerMoves();
        }
    }
}