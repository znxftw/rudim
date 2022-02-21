using Rudim.Board;
using System;

namespace Rudim.Common
{
    public class Move
    {
        public Square Source { get; set; }
        public Square Target { get; set; }
        public MoveType Type { get; set; }
        public int Score { get; set; }

        public static readonly Move NoMove = new(Square.NoSquare, Square.NoSquare, MoveTypes.Quiet);

        public Move(Square source, Square target, MoveType type)
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
            return Type.Value >= MoveTypes.KnightPromotion.Value && Type.Value <= MoveTypes.QueenPromotionCapture.Value;
        }

        public bool IsCastle()
        {
            return Type == MoveTypes.Castle;
        }

        public static Move ParseLongAlgebraic(string moveString)
        {
            var from = ParseFromString(moveString.Substring(0, 2));
            var to = ParseFromString(moveString.Substring(2, 2));
            var moveType = moveString.Length == 5 ? ParsePromotionType(moveString[4]) : MoveTypes.Quiet;

            return new Move(from, to, moveType);
        }

        private static MoveType ParsePromotionType(char piece)
        {
            return piece switch
            {
                'q' => MoveTypes.QueenPromotion,
                'r' => MoveTypes.RookPromotion,
                'b' => MoveTypes.BishopPromotion,
                'n' => MoveTypes.KnightPromotion,
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