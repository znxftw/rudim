using Rudim.Common;
using System.Collections.Generic;

namespace Rudim.Board
{
    public partial class BoardState
    {
        private static readonly Dictionary<ulong, string> CommonStateNames = new();

        private class SavedState
        {
            public Piece CapturedPiece { get; set; }
            public List<Move> SavedMoves { get; set; }
            public Square EnPassantSquare { get; set; }
            public Castle CastlingRights { get; internal set; }
        }

        private static Stack<SavedState> SavedStates { get; set; }

        static BoardState()
        {
            SavedStates = new Stack<SavedState>();

            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.StartingFEN))] = "Starting State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.EndgameFEN))] = "Endgame State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "KiwiPete State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.KiwiPeteFEN))] = "Random State";
            CommonStateNames[Zobrist.GetBoardHash(ParseFEN(Helpers.AdvancedMoveFEN))] = "Advanced Move State";
        }

        private void SaveState(Piece capturedPiece, Square enPassant, Castle originalCastlingRights)
        {
            SavedStates.Push(new SavedState
            {
                SavedMoves = Moves,
                CapturedPiece = capturedPiece,
                EnPassantSquare = enPassant,
                CastlingRights = originalCastlingRights
            });
        }

        private SavedState RestoreState()
        {
            var savedState = SavedStates.Pop();
            Moves = savedState.SavedMoves;
            return savedState;
        }

        public static void ClearSavedStates()
        {
            SavedStates.Clear();
        }
    }
}