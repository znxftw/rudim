namespace Rudim.Common
{
    public static class GamePhase
    {
        private static readonly int[] PieceConstants;
        public static readonly int TotalPhase;
        public static readonly double PhaseFactor;
        public static readonly int OnlyPawns;

        static GamePhase()
        {
            PieceConstants = [0, 1, 1, 2, 4, 0];
            TotalPhase = PieceConstants[(int)Piece.Pawn] * 16 + PieceConstants[(int)Piece.Knight] * 4 + PieceConstants[(int)Piece.Bishop] * 4 + PieceConstants[(int)Piece.Rook] * 4 + PieceConstants[(int)Piece.Queen] * 2;
            OnlyPawns = PieceConstants[(int)Piece.Pawn] * 16;
            PhaseFactor = 1 / (double)TotalPhase;
        }

        public static int AddPhase(int phase, Piece piece)
        {
            return phase + PieceConstants[(int)piece];
        }

        public static int RemovePhase(int phase, Piece piece)
        {
            return phase - PieceConstants[(int)piece];
        }
    }
}