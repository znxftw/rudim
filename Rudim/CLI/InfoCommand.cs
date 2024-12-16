namespace Rudim.CLI
{
    public class InfoCommand : ICliCommand
    {
        public void Run(string[] parameters)
        {
            CliClient.WriteLine("Rudim v1 by znxftw");
        }
    }
}