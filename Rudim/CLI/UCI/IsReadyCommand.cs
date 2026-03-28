namespace Rudim.CLI.UCI
{
    public class IsReadyCommand(UciClient uciClient) : IUciCommand
    {
        private UciClient _uciClient = uciClient;

        public void Run(string[] parameters)
        {
            if (!Global.IsReady)
            {
                Global.Reset();
                Global.SetReady();
            }
            CliClient.WriteLine("readyok");
        }
    }
}