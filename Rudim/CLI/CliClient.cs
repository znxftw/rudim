using System;
using System.Collections.Generic;
using System.Linq;
using Rudim.CLI.UCI;

namespace Rudim.CLI
{
    class CliClient
    {
        private static IDictionary<string, ICliCommand> Commands;

        static CliClient()
        {
            Commands = new Dictionary<string, ICliCommand>
            {
                ["info"] = new InfoCommand(),
                ["uci"] = new UciClient()
            };
        }
        public static void Run()
        {
            while (true)
            {
                var input = Console.ReadLine().Split(' ');
                var command = input[0];
                var parameters = input.Skip(1).ToArray();

                if (command == "exit")
                {
                    Environment.Exit(0);
                }

                if (Commands.ContainsKey(command))
                {
                    Commands[command].Run(parameters);
                }
                else
                {
                    WriteLine($"Unknown command {command}");
                }
            }
        }


        public static void WriteLine(string message)
        {
            Console.WriteLine(message);
            // Write to stderr instead if error message flag?
        }
    }
}