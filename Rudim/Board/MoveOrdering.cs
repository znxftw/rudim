using Rudim.Common;

namespace Rudim.Board
{
    public static class MoveOrdering
    {
        private static readonly int[,] MostValuableVictimLeastValuableAttacker;
        private static Move[,] _killerMoves;

        static MoveOrdering()
        {
            MostValuableVictimLeastValuableAttacker = new[,]
            { // P , N , B , R , Q , K , None
               { 15, 14, 13, 12, 11, 10, 0 }, // P
               { 25, 24, 23, 22, 21, 20, 0 }, // N
               { 35, 34, 33, 32, 31, 30, 0 }, // B
               { 45, 44, 43, 42, 41, 40, 0 }, // R
               { 55, 54, 53, 52, 51, 50, 0 }, // Q
               { 0, 0, 0, 0, 0, 0, 0 },       // K
               { 0, 0, 0, 0, 0, 0, 0 }        // None
            };
            _killerMoves = new Move[2, Constants.MaxPly];
        }
        public static void PopulateMoveScore(Move move, BoardState boardState, int ply = Constants.MaxPly - 1)
        {
            if (!move.IsCapture())
            {
                if (move == _killerMoves[0, ply])
                    move.Score = 9; // TODO : Revisit, assign better values and extract to constants
                else if (move == _killerMoves[1, ply])
                    move.Score = 8;
                else
                    move.Score = 0;
                return;
            }
            int targetPiece;
            int sourcePiece = boardState.GetPieceOn(move.Source, boardState.SideToMove);
            if (move.Type == MoveTypes.EnPassant)
                targetPiece = (int)Piece.Pawn;
            else
                targetPiece = boardState.GetPieceOn(move.Target, boardState.SideToMove.Other());
            move.Score = MostValuableVictimLeastValuableAttacker[targetPiece, sourcePiece];
        }

        public static void AddKillerMove(Move move, int ply)
        {
            if (_killerMoves[0, ply] == move)
            {
                return;
            }

            _killerMoves[1, ply] = _killerMoves[0, ply];
            _killerMoves[0, ply] = move;
        }
        public static void SortMoves(BoardState boardState)
        {
            // TODO : Partially sort within the loop only to avoid sorting elements that are not going to be queried after beta cutoff?
            boardState.Moves.Sort((a, b) => b.Score.CompareTo(a.Score));
        }
        public static void ResetKillerMoves()
        {
            _killerMoves = new Move[2, Constants.MaxPly];
        }
    }
}