using Rudim.Common;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.Board
{
    public partial class BoardState : IEquatable<BoardState>
    {
        private BoardState()
        {
            Pieces = new Bitboard[Constants.Sides, Constants.Pieces];
            Occupancies = new Bitboard[Constants.SidesWithBoth];
            SideToMove = Side.White;
            EnPassantSquare = Square.NoSquare;
            Castle = Castle.None;
            Moves = new List<Move>();

            for (var side = 0; side < Constants.Sides; ++side)
            for (var piece = 0; piece < Constants.Pieces; ++piece)
                Pieces[side, piece] = new Bitboard(0);
            for (var side = 0; side < Constants.SidesWithBoth; ++side)
                Occupancies[side] = new Bitboard(0);
        }
        
        public static BoardState Default()
        {
            return ParseFEN(Helpers.StartingFEN);
        } 

        public Bitboard[,] Pieces { get; }
        public Bitboard[] Occupancies { get; }
        public Side SideToMove { get; private set; }
        public Square EnPassantSquare { get; private set; }
        public Castle Castle { get; private set; }
        public IList<Move> Moves { get; private set; }

        private void AddPiece(Square square, Side side, Piece piece)
        {
            Pieces[(int) side, (int) piece].SetBit(square);
            Occupancies[(int) side].SetBit(square);
            Occupancies[(int) Side.Both].SetBit(square);
        }

        private Piece RemovePiece(Square square)
        {
            for (var side = 0; side < Constants.Sides; ++side)
            {
                for (var pieceType = 0; pieceType < Constants.Pieces; ++pieceType)
                {
                    if (Pieces[side, pieceType].GetBit(square) != 1) continue;
                    Pieces[side, pieceType].ClearBit(square);
                    Occupancies[side].ClearBit(square);
                    Occupancies[(int)Side.Both].ClearBit(square);
                    return (Piece)pieceType;
                }
            }
            return Piece.None;
        } 

        public bool IsInCheck(Side side)
        {
            return IsSquareAttacked((Square)Pieces[(int)side,(int)Piece.King].GetLsb(), side.Other());
        }
        public void MakeMove(Move move)
        {
            var movedPiece = RemovePiece(move.Source);

            if (move.IsCapture())
            {
                RemovePiece(move.Type == MoveType.EnPassant ? EnPassantSquareFor(move) : move.Target);
            }

            if (move.IsPromotion())
            {
                movedPiece = move.Type switch
                {
                    MoveType.KnightPromotion => Piece.Knight,
                    MoveType.KnightPromotionCapture => Piece.Knight,
                    MoveType.BishopPromotion => Piece.Bishop,
                    MoveType.BishopPromotionCapture => Piece.Bishop,
                    MoveType.RookPromotion => Piece.Rook,
                    MoveType.RookPromotionCapture => Piece.Rook,
                    MoveType.QueenPromotion => Piece.Queen,
                    MoveType.QueenPromotionCapture => Piece.Queen,
                    _ => throw new ArgumentOutOfRangeException(nameof(move.Type))
                };
            }

            if (move.IsCastle())
            {
                switch (move.Target)
                {
                    case Square.c1:
                        RemovePiece(Square.a1);
                        AddPiece(Square.d1, SideToMove, Piece.Rook);
                        break;
                    case Square.g1:
                        RemovePiece(Square.h1);
                        AddPiece(Square.f1, SideToMove, Piece.Rook);
                        break;
                    case Square.c8:
                        RemovePiece(Square.a8);
                        AddPiece(Square.d8, SideToMove, Piece.Rook);
                        break;
                    case Square.g8:
                        RemovePiece(Square.h8);
                        AddPiece(Square.f8, SideToMove, Piece.Rook);
                        break;
                    default: throw new ArgumentOutOfRangeException(nameof(move.Target));
                }
            }

            AddPiece(move.Target, SideToMove, movedPiece);

            Castle &= (Castle) CastlingRights[(int) move.Source];
            Castle &= (Castle) CastlingRights[(int) move.Target];
            EnPassantSquare = move.Type == MoveType.DoublePush ? EnPassantSquareFor(move) : Square.NoSquare;
            SideToMove = SideToMove.Other();

            Moves = new List<Move>();
        }

        private Square EnPassantSquareFor(Move move)
        {
            return move.Target + 8 * (SideToMove == Side.Black ? -1 : 1);
        }


        private const string AsciiPieces = "PNBRQK-";

        private static readonly int[] CastlingRights = {
            7, 15, 15, 15,  3, 15, 15, 11,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            13, 15, 15, 15, 12, 15, 15, 14
        };

        public void Print()
        {
            for (var rank = 0; rank < 8; ++rank)
            {
                for (var file = 0; file < 8; ++file)
                {
                    if (file == 0)
                        Console.Write((8 - rank) + "\t");
                    var square = (rank * 8) + file;

                    var boardPiece = Piece.None;
                    for (var side = 0; side < Constants.Sides; ++side)
                    for (var piece = 0; piece < Constants.Pieces; ++piece)
                        if (Pieces[side, piece].GetBit(square) == 1)
                            boardPiece = (Piece) piece;
                    char asciiValue = Occupancies[0].GetBit(square) == 0
                        ? char.ToLower(AsciiPieces[(int) boardPiece])
                        : AsciiPieces[(int) boardPiece];
                    Console.Write(asciiValue + " ");
                }

                Console.Write(Environment.NewLine);
            }

            Console.WriteLine(Environment.NewLine + "\ta b c d e f g h ");
            Console.WriteLine(Environment.NewLine + "Side to move : " + SideToMove);
            Console.WriteLine(Environment.NewLine + "En passant square : " + EnPassantSquare);
            Console.WriteLine(Environment.NewLine + "Castling rights : " + Castle);
        }


        public bool Equals(BoardState other)
        {
            if (ReferenceEquals(null, other)) return false;
            if (ReferenceEquals(this, other)) return true;

            if (Pieces.Rank != other.Pieces.Rank ||
                Enumerable.Range(0, Pieces.Rank).Any(dimension =>
                    Pieces.GetLength(dimension) != other.Pieces.GetLength(dimension)) ||
                !Pieces.Cast<Bitboard>().SequenceEqual(other.Pieces.Cast<Bitboard>()))
                return false;

            return NullRespectingSequenceEqual(Occupancies, other.Occupancies) &&
                   SideToMove == other.SideToMove && EnPassantSquare == other.EnPassantSquare &&
                   Castle == other.Castle && NullRespectingSequenceEqual(Moves, other.Moves);
        }

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(null, obj)) return false;
            if (ReferenceEquals(this, obj)) return true;
            if (obj.GetType() != this.GetType()) return false;
            return Equals((BoardState) obj);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Pieces, Occupancies, (int) SideToMove, (int) EnPassantSquare, (int) Castle, Moves);
        }

        public static bool operator ==(BoardState left, BoardState right)
        {
            return Equals(left, right);
        }

        public static bool operator !=(BoardState left, BoardState right)
        {
            return !Equals(left, right);
        }

        // This can move to an extension method
        private static bool NullRespectingSequenceEqual<T>(IEnumerable<T> first, IEnumerable<T> second)
        {
            if (first == null && second == null)
            {
                return true;
            }

            if (first == null || second == null)
            {
                return false;
            }

            return first.SequenceEqual(second);
        }
    }
}