namespace Rudim.Common
{
    // public enum MoveType
    // {
    //     Quiet,
    //     Capture,
    //     EnPassant,
    //     DoublePush,
    //     KnightPromotion = 4,
    //     BishopPromotion = 5,
    //     RookPromotion = 6,
    //     QueenPromotion = 7,
    //     KnightPromotionCapture = 12,
    //     BishopPromotionCapture = 13,
    //     RookPromotionCapture = 14,
    //     QueenPromotionCapture = 15,
    //     Castle = 16
    // }

    public record MoveTypeRecord(int Value, Piece Piece = Piece.None, string PromotionChar = "", bool IsCapture = false);

    public static class MoveType
    {
        public static MoveTypeRecord Quiet { get; } = new(0);
        public static MoveTypeRecord Capture { get; } = new(1, Piece.None, null, true);
        public static MoveTypeRecord EnPassant { get; } = new(2, Piece.None, null, true);
        public static MoveTypeRecord DoublePush { get; } = new(3, Piece.None);
        public static MoveTypeRecord KnightPromotion { get; } = new(4, Piece.Knight, "k");
        public static MoveTypeRecord BishopPromotion { get; } = new(5, Piece.Bishop, "b");
        public static MoveTypeRecord RookPromotion { get; } = new(6, Piece.Rook, "r");
        public static MoveTypeRecord QueenPromotion { get; } = new(7, Piece.Queen, "q");
        public static MoveTypeRecord KnightPromotionCapture { get; } = new(12, Piece.Knight, "k", true);
        public static MoveTypeRecord BishopPromotionCapture { get; } = new(13, Piece.Bishop, "b", true);
        public static MoveTypeRecord RookPromotionCapture { get; } = new(14, Piece.Rook, "r", true);
        public static MoveTypeRecord QueenPromotionCapture { get; } = new(15, Piece.Queen, "q", true);
        public static MoveTypeRecord Castle { get; } = new(16);
    }
}