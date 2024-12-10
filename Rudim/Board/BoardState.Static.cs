using Rudim.Common;
using System.Collections.Generic;

namespace Rudim.Board
{
    public partial class BoardState
    {
        private static readonly Dictionary<ulong, string> CommonStateNames = new();

        static BoardState()
        {

            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.StartingFEN))] = "Starting State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.EndgameFEN))] = "Endgame State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "KiwiPete State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "Random State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.AdvancedMoveFEN))] = "Advanced Move State";
        }
    }
}