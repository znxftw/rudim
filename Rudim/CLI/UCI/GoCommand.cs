using Rudim.Common;
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
            var winc = GetParameter("winc", parameters, -1);
            var binc = GetParameter("binc", parameters, -1);
            var wtime = GetParameter("wtime", parameters, -1);
            var btime = GetParameter("btime", parameters, -1);
            var movetime = GetParameter("movetime", parameters, -1);
            // TODO : ponder, movestogo, searchmoves, nodes, mate, infinite
            var infinite = GetOptionlessParameter("infinite", parameters);

            var cancellationTokenSource = new CancellationTokenSource();

            var clock = _uciClient.Board.SideToMove == Side.White ? wtime : btime;
            var increment = _uciClient.Board.SideToMove == Side.White ? winc : binc;

            var allottedTime = movetime == -1 ? (clock == -1 ? -1 : TimeManagement.CalculateMoveTime(_uciClient.Board.MoveCount, clock, increment)) : movetime;

            if (!infinite)
            {
                var move = Move.NoMove;
                if(allottedTime == -1)
                {
                    move = _uciClient.Board.FindBestMove(depth, cancellationTokenSource.Token);
                }
                else
                {
                    var moveTask = Task.Run(() => _uciClient.Board.FindBestMove(Constants.MaxSearchDepth, cancellationTokenSource.Token));

                    Thread.Sleep(allottedTime);
                    cancellationTokenSource.Cancel();

                    move = moveTask.Result;
                }
                

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