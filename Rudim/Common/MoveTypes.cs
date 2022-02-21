namespace Rudim.Common
{
    public record MoveType(int Value, Piece Piece = Piece.None, string PromotionChar = "", bool IsCapture = false);

    public static class MoveTypes
    {
        public static MoveType Quiet { get; } = new(0);
        public static MoveType Capture { get; } = new(1, Piece.None, null, true);
        public static MoveType EnPassant { get; } = new(2, Piece.None, null, true);
        public static MoveType DoublePush { get; } = new(3, Piece.None);
        public static MoveType KnightPromotion { get; } = new(4, Piece.Knight, "k");
        public static MoveType BishopPromotion { get; } = new(5, Piece.Bishop, "b");
        public static MoveType RookPromotion { get; } = new(6, Piece.Rook, "r");
        public static MoveType QueenPromotion { get; } = new(7, Piece.Queen, "q");
        public static MoveType KnightPromotionCapture { get; } = new(12, Piece.Knight, "k", true);
        public static MoveType BishopPromotionCapture { get; } = new(13, Piece.Bishop, "b", true);
        public static MoveType RookPromotionCapture { get; } = new(14, Piece.Rook, "r", true);
        public static MoveType QueenPromotionCapture { get; } = new(15, Piece.Queen, "q", true);
        public static MoveType Castle { get; } = new(16);
    }
}