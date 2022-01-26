using System;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.CLI
{
    internal class UciClient : ICliCommand
    {
        private Dictionary<string, IUciCommand> Commands;

        public UciClient()
        {
            Commands = new Dictionary<string, IUciCommand>();
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
                    break;
                }

                if (Commands.ContainsKey(command))
                {
                    Commands[command].Run(commandParameters);
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