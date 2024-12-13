namespace Rudim.CLI.UCI;

public class DebugCommand(UciClient uciClient) : IUciCommand
{
    public void Run(string[] parameters)
    {
        if (parameters?.Length > 0)
        {
            string mode = parameters[0].ToLower();
            uciClient.DebugMode = mode switch
            {
                "on" => true,
                "off" => false,
                _ => uciClient.DebugMode
            };
        }
    }
}