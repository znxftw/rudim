using System.Collections.Generic;
using Rudim.Common;

namespace Rudim.Board
{
    public partial class BoardState
    {
        private static Bitboard[,] SavedPieces { get; set; }
        private static Bitboard[] SavedOccupancies { get; set; }
        private static Side SavedSideToMove { get; set; }
        private static Square SavedEnPassantSquare { get; set; }
        private static Castle SavedCastle { get; set; }
        private static IList<Move> SavedMoves { get; set; }

        public void SaveState()
        {
            SavedPieces = new Bitboard[Constants.Sides, Constants.Pieces];
            SavedOccupancies = new Bitboard[Constants.SidesWithBoth];
            // TODO : Copy constructors ?
            for (var side = 0; side < Constants.Sides; ++side)
                for (var piece = 0; piece < Constants.Pieces; ++piece)
                    SavedPieces[side, piece] = Pieces[side,piece].CreateCopy();
            for (var side = 0; side < Constants.SidesWithBoth; ++side)
                SavedOccupancies[side] = Occupancies[side].CreateCopy();
            SavedSideToMove = SideToMove;
            SavedEnPassantSquare = EnPassantSquare;
            SavedCastle = Castle;
            SavedMoves = Moves is null ? null : new List<Move>(Moves);
        }

        public void RestoreState()
        {
            for (var side = 0; side < Constants.Sides; ++side)
                for (var piece = 0; piece < Constants.Pieces; ++piece)
                    Pieces[side, piece] = SavedPieces[side,piece].CreateCopy();
            for (var side = 0; side < Constants.SidesWithBoth; ++side)
                Occupancies[side] = SavedOccupancies[side].CreateCopy();
            SideToMove = SavedSideToMove;
            EnPassantSquare = SavedEnPassantSquare;
            Castle = SavedCastle;
            Moves = SavedMoves is null ? null : new List<Move>(SavedMoves);
        }
    }
}