namespace Rudim.CLI
{
    internal class IsReadyCommand : IUciCommand
    {
        private UciClient uciClient;

        public IsReadyCommand(UciClient uciClient)
        {
            this.uciClient = uciClient;
        }

        public void Run(string[] parameters)
        {
            // Currently all initializations are done with UciClient - revisit later
            CliClient.WriteLine("readyok");
        }
    }
}