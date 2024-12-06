using Rudim.Common;
using System;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace Rudim.CLI.UCI
{
    internal class GoCommand(UciClient uciClient) : IUciCommand
    {
        private CancellationTokenSource _currentSearch = null;
        private Move _bestMove = Move.NoMove;

        public void StopSearch()
        {
            if (_currentSearch != null)
            {
                _currentSearch.Cancel();
                if (_bestMove != Move.NoMove)
                {
                    OutputBestMove(_bestMove);
                }
            }
        }

        private void OutputBestMove(Move move)
        {
            if (move.IsPromotion())
            {
                CliClient.WriteLine("bestmove " + move.Source + move.Target + move.GetPromotionChar());
            }
            else
            {
                CliClient.WriteLine("bestmove " + move.Source + move.Target);
            }
        }

        public async void Run(string[] parameters)
        {
            // Cancel any existing search
            _currentSearch?.Cancel();
            _currentSearch = new CancellationTokenSource();
            _bestMove = Move.NoMove;

            var depth = GetParameter("depth", parameters, 5);
            var winc = GetParameter("winc", parameters, -1);
            var binc = GetParameter("binc", parameters, -1);
            var wtime = GetParameter("wtime", parameters, -1);
            var btime = GetParameter("btime", parameters, -1);
            var movetime = GetParameter("movetime", parameters, -1);
            var infinite = GetOptionlessParameter("infinite", parameters); // Not yet implemented

            var clock = uciClient.Board.SideToMove == Side.White ? wtime : btime;
            var increment = uciClient.Board.SideToMove == Side.White ? winc : binc;
            var allottedTime = movetime == -1 ? (clock == -1 ? -1 : TimeManagement.CalculateMoveTime(uciClient.Board.MoveCount, clock, increment)) : movetime;


            if (!infinite)
            {
                if (allottedTime == -1)
                {
                    _bestMove = await Task.Run(() => uciClient.Board.FindBestMove(depth, _currentSearch.Token, ref uciClient.DebugMode));
                }
                else
                {
                    var searchTask = Task.Run(() => uciClient.Board.FindBestMove(Constants.MaxSearchDepth, _currentSearch.Token, ref uciClient.DebugMode));
                    var timeoutTask = Task.Delay(allottedTime);

                    if (await Task.WhenAny(searchTask, timeoutTask) == timeoutTask)
                    {
                        await _currentSearch.CancelAsync();
                        _bestMove = await searchTask;
                    }
                    else
                    {
                        _bestMove = await searchTask;
                    }
                }

                OutputBestMove(_bestMove);
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