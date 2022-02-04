using System.Collections.Generic;
using Rudim.Common;

namespace Rudim.Board
{   
    // TODO : Remove this class completely, instead do UnmakeMove(), should be faster.
    public partial class BoardState
    {
        private class SavedState
        {
            public Bitboard[,] SavedPieces { get; set; }
            public Bitboard[] SavedOccupancies { get; set; }
            public Side SavedSideToMove { get; set; }
            public Square SavedEnPassantSquare { get; set; }
            public Castle SavedCastle { get; set; }
            public IList<Move> SavedMoves { get; set; }
        }

        private static Stack<SavedState> savedStates { get; set; }

        static BoardState()
        {
            savedStates = new Stack<SavedState>();
        }

        public void SaveState()
        {
            var savedState = new SavedState();
            savedState.SavedPieces = new Bitboard[Constants.Sides, Constants.Pieces];
            savedState.SavedOccupancies = new Bitboard[Constants.SidesWithBoth];
            // TODO : Copy constructors ?
            for (var side = 0; side < Constants.Sides; ++side)
                for (var piece = 0; piece < Constants.Pieces; ++piece)
                    savedState.SavedPieces[side, piece] = Pieces[side, piece].CreateCopy();
            for (var side = 0; side < Constants.SidesWithBoth; ++side)
                savedState.SavedOccupancies[side] = Occupancies[side].CreateCopy();
            savedState.SavedSideToMove = SideToMove;
            savedState.SavedEnPassantSquare = EnPassantSquare;
            savedState.SavedCastle = Castle;
            savedState.SavedMoves = Moves is null ? null : new List<Move>(Moves);

            savedStates.Push(savedState);
        }

        public void RestoreState()
        {
            var savedState = savedStates.Pop();
            for (var side = 0; side < Constants.Sides; ++side)
                for (var piece = 0; piece < Constants.Pieces; ++piece)
                    Pieces[side, piece] = savedState.SavedPieces[side, piece].CreateCopy();
            for (var side = 0; side < Constants.SidesWithBoth; ++side)
                Occupancies[side] = savedState.SavedOccupancies[side].CreateCopy();
            SideToMove = savedState.SavedSideToMove;
            EnPassantSquare = savedState.SavedEnPassantSquare;
            Castle = savedState.SavedCastle;
            Moves = savedState.SavedMoves is null ? null : new List<Move>(savedState.SavedMoves);
        }

        public static void ClearStates()
        {
            savedStates.Clear();
        }
    }
}