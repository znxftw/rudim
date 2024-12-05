using Rudim.Common;
using System.Collections.Generic;

namespace Rudim.Board
{
    public partial class BoardState
    {
        private static readonly Dictionary<ulong, string> CommonStateNames = new();
        private static SavedState[] _savedStates = new SavedState[2048];
        private static int _currentState = 0;


        private class SavedState
        {
            public Piece CapturedPiece { get; set; }
            public Square EnPassantSquare { get; set; }
            public Castle CastlingRights { get; internal set; }
        }


        static BoardState()
        {

            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.StartingFEN))] = "Starting State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.EndgameFEN))] = "Endgame State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "KiwiPete State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "Random State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.AdvancedMoveFEN))] = "Advanced Move State";
        }

        private void SaveState(Piece capturedPiece, Square enPassant, Castle originalCastlingRights)
        {
            _savedStates[_currentState++] = new SavedState
            {
                CapturedPiece = capturedPiece,
                EnPassantSquare = enPassant,
                CastlingRights = originalCastlingRights
            };
        }

        private SavedState RestoreState()
        {
            return _savedStates[--_currentState];
        }

        public static void ClearSavedStates()
        {
            _savedStates = new SavedState[2048];
        }
    }
}