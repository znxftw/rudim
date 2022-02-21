using Rudim.Board;
using System;

namespace Rudim.Common
{
    public class Move
    {
        public Square Source { get; set; }
        public Square Target { get; set; }
        public MoveTypeRecord Type { get; set; }
        public int Score { get; set; }

        public static readonly Move NoMove = new(Square.NoSquare, Square.NoSquare, MoveType.Quiet);

        public Move(Square source, Square target, MoveTypeRecord type)
        {
            Source = source;
            Target = target;
            Type = type;
        }

        public bool IsCapture()
        {
            return Type.IsCapture;
        }

        public string GetPromotionChar()
        {
            return Type.PromotionChar;
        }

        public bool IsPromotion()
        {
            return Type.Value >= MoveType.KnightPromotion.Value && Type.Value <= MoveType.QueenPromotionCapture.Value;
        }

        public bool IsCastle()
        {
            return Type == MoveType.Castle;
        }

        public static Move ParseLongAlgebraic(string moveString)
        {
            var from = ParseFromString(moveString.Substring(0, 2));
            var to = ParseFromString(moveString.Substring(2, 2));
            var moveType = moveString.Length == 5 ? ParsePromotionType(moveString[4]) : MoveType.Quiet;

            return new Move(from, to, moveType);
        }

        private static MoveTypeRecord ParsePromotionType(char piece)
        {
            return piece switch
            {
                'q' => MoveType.QueenPromotion,
                'r' => MoveType.RookPromotion,
                'b' => MoveType.BishopPromotion,
                'n' => MoveType.KnightPromotion,
                _ => throw new InvalidOperationException(),
            };
        }

        private static Square ParseFromString(string squareString)
        {
            var square = Square.NoSquare;
            _ = Enum.TryParse(squareString, out square);
            return square;
        }
    }
}