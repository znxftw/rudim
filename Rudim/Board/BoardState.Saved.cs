using System.Collections.Generic;
using Rudim.Common;

namespace Rudim.Board
{
    // TODO : Remove this class completely, instead do UnmakeMove(), should be faster.
    public partial class BoardState
    {
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
        }

        private void SaveState(Piece capturedPiece, Square enPassant, Castle originalCastlingRights)
        {
            var savedState = new SavedState();
            savedState.SavedMoves = Moves;
            savedState.CapturedPiece = capturedPiece;
            savedState.EnPassantSquare = enPassant;
            savedState.CastlingRights = originalCastlingRights;
            SavedStates.Push(savedState);
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