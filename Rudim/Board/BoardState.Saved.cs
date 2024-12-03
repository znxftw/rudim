using System.Collections.Generic;
using Rudim.Common;

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

            // TODO: To be moved out into a separate class and fixed before working on this again
            ZobristTable = new ulong[15, 64]; // 12 piece types (6 for each color) and 64 squares, and extra 3 rows
            for (int piece = 0; piece < 14; piece++)
            {
              for (int square = 0; square < 64; square++)
              {
                ZobristTable[piece, square] = Random.NextULong() << 32 | Random.NextULong();
              }
            }

            ZobristTable[14, 0] = Random.NextULong() << 32 | Random.NextULong(); // White to move
            ZobristTable[14, 1] = Random.NextULong() << 32 | Random.NextULong(); // Black to move

            // Add Zobrist values for castling rights
            ZobristTable[14, 2] = Random.NextULong() << 32 | Random.NextULong();
            ZobristTable[14, 3] = Random.NextULong() << 32 | Random.NextULong();
            ZobristTable[14, 4] = Random.NextULong() << 32 | Random.NextULong();
            ZobristTable[14, 5] = Random.NextULong() << 32 | Random.NextULong();

            CommonStateNames[ParseFEN(Helpers.StartingFEN).GetBoardHash()] = "Starting State";
            CommonStateNames[ParseFEN(Helpers.EndgameFEN).GetBoardHash()] = "Endgame State";
            CommonStateNames[ParseFEN(Helpers.KiwiPeteFEN).GetBoardHash()] = "KiwiPete State";
            CommonStateNames[ParseFEN(Helpers.KiwiPeteFEN).GetBoardHash()] = "Random State";
            CommonStateNames[ParseFEN(Helpers.AdvancedMoveFEN).GetBoardHash()] = "Advanced Move State";
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
