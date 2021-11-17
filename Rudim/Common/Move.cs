namespace Rudim.Common
{
    public class Move
    {
        public Square Source { get; set; }
        public Square Target { get; set; }
        public MoveType Type { get; set; }

        public Move(Square source, Square target, MoveType type)
        {
            Source = source;
            Target = target;
            Type = type;
        }

        public bool IsCapture()
        {
            return Type is MoveType.Capture or MoveType.EnPassant or MoveType.BishopPromotionCapture or
                MoveType.KnightPromotionCapture or MoveType.QueenPromotionCapture or MoveType.RookPromotionCapture;
        }

        public bool IsPromotion()
        {
            return Type is >= MoveType.KnightPromotion and <= MoveType.QueenPromotionCapture;
        }

        public bool IsCastle()
        {
            return Type == MoveType.Castle;
        }
    }
}