using Rudim.Board;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.CLI.UCI
{
    internal class UciClient : ICliCommand
    {
        private readonly Dictionary<string, IUciCommand> _commands;
        public BoardState Board;
        public UciClient()
        {
            var goCommand = new GoCommand(this);
            _commands = new Dictionary<string, IUciCommand>
            {
                ["isready"] = new IsReadyCommand(this),
                ["position"] = new PositionCommand(this),
                ["go"] = goCommand,
                ["stop"] = new StopCommand(goCommand),
                ["ucinewgame"] = new UciNewGameCommand(this)
            };
            Board = BoardState.Default();
        }

        public void Run(string[] parameters)
        {
            WriteId();
            while (true)
            {
                var input = Console.ReadLine().Split(' ');
                var command = input[0];
                var commandParameters = input.Skip(1).ToArray();

                if (command == "quit")
                {
                    Environment.Exit(0);
                }

                if (_commands.ContainsKey(command))
                {
                    _commands[command].Run(commandParameters);
                }
                else
                {
                    CliClient.WriteLine($"Unknown command {command}");
                }
            }
        }

        private static void WriteId()
        {
            CliClient.WriteLine("id name Rudim 1.2");
            CliClient.WriteLine("id author Vishnu B");
            CliClient.WriteLine("uciok");
        }
    }
}