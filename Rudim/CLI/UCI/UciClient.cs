using Rudim.Board;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.CLI.UCI
{
    public class UciClient : ICliCommand
    {
        private readonly Dictionary<string, IUciCommand> _commands;
        public BoardState Board;
        private bool _debugMode = true;
        public ref bool DebugMode => ref _debugMode;

        public UciClient()
        {
            GoCommand goCommand = new GoCommand(this);
            _commands = new Dictionary<string, IUciCommand>
            {
                ["isready"] = new IsReadyCommand(this),
                ["position"] = new PositionCommand(this),
                ["go"] = goCommand,
                ["stop"] = new StopCommand(goCommand),
                ["ucinewgame"] = new UciNewGameCommand(this),
                ["debug"] = new DebugCommand(this)
            };
            Board = BoardState.Default();
        }

        public void Run(string[] parameters)
        {
            WriteId();
            while (true)
            {
                string[] input = Console.ReadLine().Split(' ');
                string command = input[0];
                string[] commandParameters = input.Skip(1).ToArray();

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
            CliClient.WriteLine("id name Rudim 1.3");
            CliClient.WriteLine("id author Vishnu B");
            CliClient.WriteLine("uciok");
        }
    }
}