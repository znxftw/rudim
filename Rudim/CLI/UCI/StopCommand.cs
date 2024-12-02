using System;

namespace Rudim.CLI.UCI
{
    internal class StopCommand : IUciCommand
    {
      private readonly GoCommand _goCommand;

        public StopCommand(GoCommand goCommand)
        {
          _goCommand = goCommand;
        }

        public void Run(string[] parameters)
        {
            _goCommand.StopSearch();
        }
    }
}
