using System;
using System.Collections.Generic;
using System.Linq;
using Rudim.Board;

namespace Rudim.CLI.UCI
{
    internal class UciClient : ICliCommand
    {
        private readonly Dictionary<string, IUciCommand> _commands;
        public BoardState Board;
        public UciClient()
        {
            _commands = new Dictionary<string, IUciCommand>
            {
                ["isready"] = new IsReadyCommand(this),
                ["position"] = new PositionCommand(this),
                ["go"] = new GoCommand(this)

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

                if (command == "exit")
                {
                    Environment.Exit(0);
                }

                if (command == "quit")
                {
                    // Should quit be exiting the entire application as well?
                    break;
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
            CliClient.WriteLine("id name Rudim 1.0");
            CliClient.WriteLine("id author Vishnu B");
            CliClient.WriteLine("uciok");
        }
    }
}