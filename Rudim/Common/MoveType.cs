namespace Rudim.Common
{
    public enum MoveType
    {
        Quiet,
        Capture,
        EnPassant,
        DoublePush,
        KnightPromotion = 4,
        BishopPromotion = 5,
        RookPromotion = 6,
        QueenPromotion = 7,
        KnightPromotionCapture = 12,
        BishopPromotionCapture = 13,
        RookPromotionCapture = 14,
        QueenPromotionCapture = 15,
        Castle = 16
    }
}