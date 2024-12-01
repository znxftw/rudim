using Rudim.Board;
using System;
using System.Collections.Generic;

namespace Rudim.Common
{
    public class Move : IEquatable<Move>
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

        public override bool Equals(object obj)
        {
            return Equals(obj as Move);
        }

        public bool Equals(Move other)
        {
            return other != null &&
                   Source == other.Source &&
                   Target == other.Target &&
                   EqualityComparer<MoveType>.Default.Equals(Type, other.Type);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Source, Target, Type);
        }

        public static bool operator ==(Move left, Move right)
        {
            return EqualityComparer<Move>.Default.Equals(left, right);
        }

        public static bool operator !=(Move left, Move right)
        {
            return !(left == right);
        }

        public string ToSan(BoardState board)
        {
            if (Type == MoveTypes.Castle)
            {
                return Target == Square.g1 || Target == Square.g8 ? "O-O" : "O-O-O";
            }

            var piece = board.GetPieceOn(Source, board.SideToMove);

            if (piece == (int)Piece.Pawn)
            {
                if (IsCapture())
                {
                    return $"{Source.ToString()[0]}x{Target}{(IsPromotion() ? "=" + GetPromotionChar().ToUpper() : "")}";
                }
                return $"{Target}{(IsPromotion() ? "=" + GetPromotionChar().ToUpper() : "")}";
            }

            var pieceChar = piece switch
            {
                (int)Piece.Knight => "N",
                (int)Piece.Bishop => "B",
                (int)Piece.Rook => "R",
                (int)Piece.Queen => "Q",
                _ => ""
            };

            bool needFile = false;
            bool needRank = false;

            board.GenerateMoves();
            foreach (var move in board.Moves)
            {
                if (move.Target != Target || move == this)
                    continue;

                var otherPiece = board.GetPieceOn(move.Source, board.SideToMove);
                if (otherPiece != piece)
                    continue;

                if (move.Source.ToString()[0] == Source.ToString()[0])
                    needRank = true;
                if (move.Source.ToString()[1] == Source.ToString()[1])
                    needFile = true;
            }

            if (needFile)
                pieceChar += Source.ToString()[0];
            if (needRank)
                pieceChar += Source.ToString()[1];

            string captureSymbol = IsCapture() ? "x" : "";
            string targetSquare = Target.ToString();
            string promotion = IsPromotion() ? "=" + GetPromotionChar().ToUpper() : "";

            return $"{pieceChar}{captureSymbol}{targetSquare}{promotion}";
        }
    }
}
