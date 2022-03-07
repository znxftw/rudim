using System;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace Rudim.CLI.UCI
{
    internal class GoCommand : IUciCommand
    {
        private readonly UciClient _uciClient;

        public GoCommand(UciClient uciClient)
        {
            _uciClient = uciClient;
        }

        public void Run(string[] parameters)
        {
            var depth = GetParameter("depth", parameters, 5);
            // TODO : ponder, wtime, btime, winc, binc, movestogo, searchmoves, nodes, mate, movetime
            var infinite = GetOptionlessParameter("infinite", parameters);
            var cancellationTokenSource = new CancellationTokenSource();

            // TODO : Calculate this with go parameters
            var moveTime = 1000;

            if (!infinite)
            {
                var moveTask = Task.Run(() => _uciClient.Board.FindBestMove(depth, cancellationTokenSource.Token));
                
                Thread.Sleep(moveTime);
                cancellationTokenSource.Cancel();

                var move = moveTask.Result;
                if (move.IsPromotion())
                {
                    CliClient.WriteLine("bestmove " + move.Source + move.Target + move.GetPromotionChar());
                }
                else
                {
                    CliClient.WriteLine("bestmove " + move.Source + move.Target);
                }
            }
        }

        private static bool GetOptionlessParameter(string name, string[] parameters)
        {
            return parameters.Contains(name);
        }

        private static T GetParameter<T>(string name, string[] parameters, T fallback)
        {
            for (int i = 0; i < parameters.Length; ++i)
            {
                if (parameters[i] == name)
                {
                    return (T)Convert.ChangeType(parameters[i + 1], typeof(T));
                }
            }
            return fallback;
        }
    }
}