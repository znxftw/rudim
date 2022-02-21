using Rudim.Common;

namespace Rudim.Board
{
    static class MoveOrdering
    {
        private static readonly int[,] MVVLVA;

        static MoveOrdering()
        {
            MVVLVA = new int[,]
            { // P , N , B , R , Q , K , None
               { 15, 14, 13, 12, 11, 10, 0 }, // P
               { 25, 24, 23, 22, 21, 20, 0 }, // N
               { 35, 34, 33, 32, 31, 30, 0 }, // B
               { 45, 44, 43, 42, 41, 40, 0 }, // R
               { 55, 54, 53, 52, 51, 50, 0 }, // Q 
               { 0, 0, 0, 0, 0, 0, 0 },       // K
               { 0, 0, 0, 0, 0, 0, 0 }        // None
            };
        }
        public static void PopulateMoveScore(Move move, BoardState boardState)
        {
            if (!move.IsCapture())
                move.Score = 0;
            var targetPiece = (int)Piece.None;
            var sourcePiece = boardState.GetPieceOn(move.Source, boardState.SideToMove);
            if (move.Type == MoveTypes.EnPassant)
                targetPiece = (int)Piece.Pawn;
            else
                targetPiece = boardState.GetPieceOn(move.Target, boardState.SideToMove.Other());
            move.Score = MVVLVA[targetPiece, sourcePiece];
        }

        public static void SortMoves(BoardState boardState)
        {
            // TODO : Partially sort within the loop only to avoid sorting elements that are not going to be queried after beta cutoff?
            boardState.Moves.Sort((a, b) => b.Score.CompareTo(a.Score));
        }
    }
}
