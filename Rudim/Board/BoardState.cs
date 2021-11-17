using Rudim.Common;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Rudim.Board
{
    public partial class BoardState : IEquatable<BoardState>
    {
        public BoardState()
        {
            Pieces = new Bitboard[Constants.Sides, Constants.Pieces];
            Occupancies = new Bitboard[Constants.SidesWithBoth];
            SideToMove = Side.White;
            EnPassantSquare = Square.NoSquare;
            Castle = Castle.None;
            // Revisit : Leave Moves uninitialized till GenerateMoves is called? Trying to access moves before generating shouldn't be allowed
            // Moves = new List<Move>();

            for (var side = 0; side < Constants.Sides; ++side)
            for (var piece = 0; piece < Constants.Pieces; ++piece)
                Pieces[side, piece] = new Bitboard(0);
            for (var side = 0; side < Constants.SidesWithBoth; ++side)
                Occupancies[side] = new Bitboard(0);
        }

        public Bitboard[,] Pieces { get; set; }
        public Bitboard[] Occupancies { get; set; }
        public Side SideToMove { get; set; }
        public Square EnPassantSquare { get; set; }
        public Castle Castle { get; set; }
        public IList<Move> Moves { get; set; }

        private void AddPiece(Square square, Side side, Piece piece)
        {
            Pieces[(int) side, (int) piece].SetBit(square);
            Occupancies[(int) side].SetBit(square);
            Occupancies[(int) Side.Both].SetBit(square);
        }

        public void MakeMove(Move move)
        {
            // WIP function - have to handle a few more cases before it is usable
            // TODO - Castle rook movement, enpassant reset, enpassant set for double push, Update castling rights
            var movedPiece = RemovePieceFromSquare(move.Source);

            if (move.IsCapture())
            {
                if (move.Type == MoveType.EnPassant)
                    RemovePieceFromSquare(move.Target + 8 * (SideToMove == Side.Black ? -1 : 1));
                else
                    RemovePieceFromSquare(move.Target);
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
            
            AddPiece(move.Target, SideToMove, movedPiece);

            SideToMove = SideToMove.Other();
        }

        private Piece RemovePieceFromSquare(Square square)
        {
            for (var pieceType = 0; pieceType < Constants.Pieces; ++pieceType)
            {
                if (Pieces[(int) SideToMove, pieceType].GetBit(square) == 1)
                {
                    Pieces[(int) SideToMove, pieceType].ClearBit(square);
                    Occupancies[(int) SideToMove].ClearBit(square);
                    return (Piece) pieceType;
                }
            }

            return Piece.None;
        }

        private const string AsciiPieces = "PNBRQK-";

        public void Print()
        {
            for (int rank = 0; rank < 8; ++rank)
            {
                for (int file = 0; file < 8; ++file)
                {
                    if (file == 0)
                        Console.Write((8 - rank) + "\t");
                    int square = (rank * 8) + file;

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