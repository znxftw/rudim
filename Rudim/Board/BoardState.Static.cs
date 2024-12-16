using Rudim.Common;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.Board
{
    public partial class BoardState
    {
        private static readonly Dictionary<ulong, string> CommonStateNames = new();

        private static readonly int[] CastlingConstants =
        {
            7, 15, 15, 15, 3, 15, 15, 11,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            13, 15, 15, 15, 12, 15, 15, 14
        };

        static BoardState()
        {

            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.StartingFEN))] = "Starting State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.EndgameFEN))] = "Endgame State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "KiwiPete State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "Random State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.AdvancedMoveFEN))] = "Advanced Move State";
        }

        public static bool operator ==(BoardState left, BoardState right)
        {
            return Equals(left, right);
        }

        public static bool operator !=(BoardState left, BoardState right)
        {
            return !Equals(left, right);
        }

        // This can move to an extension method
        private static bool NullRespectingSequenceEqual<T>(IEnumerable<T> first, IEnumerable<T> second)
        {
            if (first == null && second == null)
            {
                return true;
            }

            if (first == null || second == null)
            {
                return false;
            }

            return first.SequenceEqual(second);
        }
    }
}