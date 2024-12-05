using System;

namespace Rudim.CLI.UCI
{
    internal class StopCommand(GoCommand goCommand) : IUciCommand
    {
        public void Run(string[] parameters)
        {
            goCommand.StopSearch();
        }
    }
}