using Rudim.Board;
using Rudim.Common;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.CLI.UCI
{
    internal class PositionCommand(UciClient uciClient) : IUciCommand
    {
        public void Run(string[] parameters)
        {
            var position = parameters[0];
            var positionParameters = parameters.Skip(1).ToList();

            if (position == "startpos")
            {
                var moves = positionParameters.Skip(1).ToList();
                ParseStartPos(moves);
            }

            if (position == "fen")
            {
                var fen = string.Join(" ", positionParameters.Take(6));
                var moves = positionParameters.Skip(7).ToList();
                ParseFen(fen, moves);
            }
        }

        private void ParseFen(string fen, IList<string> moves)
        {
            History.ClearBoardHistory();
            uciClient.Board = BoardState.ParseFEN(fen);
            ParseMoves(moves);
        }

        private void ParseStartPos(IList<string> moves)
        {
            History.ClearBoardHistory();
            uciClient.Board = BoardState.Default();
            ParseMoves(moves);
        }

        private void ParseMoves(IList<string> moves)
        {
            foreach (var moveString in moves)
            {
                var move = Move.ParseLongAlgebraic(moveString);
                move = FindMoveFromMoveList(move);
                // Todo : Check move is valid before making move
                if (move == Move.NoMove)
                {
                    CliClient.WriteLine("Invalid Move");
                    return;
                }
                uciClient.Board.MakeMove(move);
            }
        }

        private Move FindMoveFromMoveList(Move move)
        {
            uciClient.Board.GenerateMoves();
            var moves = uciClient.Board.Moves;
            for (var i = 0; i < moves.Count; ++i)
            {
                if (moves[i].Source == move.Source && moves[i].Target == move.Target)
                {
                    // Cool little trick I saw on Cosette - requires keeping enums on certain numbers
                    // So that the we they differ by only one bit
                    // Refactor  this concept better maybe - hardcoded in the enum and here for now
                    if (move.Type == MoveTypes.Quiet || ((byte)moves[i].Type.Value & ~8) == (byte)move.Type.Value)
                    {
                        return moves[i];
                    }
                }
            }
            return Move.NoMove;
        }
    }
}