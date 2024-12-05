namespace Rudim.CLI.UCI
{
    internal class IsReadyCommand(UciClient uciClient) : IUciCommand
    {
        private UciClient _uciClient = uciClient;

        public void Run(string[] parameters)
        {
            // Currently all initializations are done with UciClient - revisit later
            CliClient.WriteLine("readyok");
        }
    }
}