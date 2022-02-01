namespace Rudim.CLI.UCI
{
    internal class IsReadyCommand : IUciCommand
    {
        private UciClient _uciClient;

        public IsReadyCommand(UciClient uciClient)
        {
            _uciClient = uciClient;
        }

        public void Run(string[] parameters)
        {
            // Currently all initializations are done with UciClient - revisit later
            CliClient.WriteLine("readyok");
        }
    }
}