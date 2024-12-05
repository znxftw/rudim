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
            PieceMapping = new Piece[Constants.Squares];
            SideToMove = Side.White;
            EnPassantSquare = Square.NoSquare;
            Castle = Castle.None;
            Moves = new List<Move>(64);
            MoveCount = 0;
            for (var square = 0; square < Constants.Squares; ++square)
                PieceMapping[square] = Piece.None;
        }

        public static BoardState Default()
        {
            return ParseFEN(Helpers.StartingFEN);
        }

        public Bitboard[,] Pieces { get; }
        public Bitboard[] Occupancies { get; }
        public Piece[] PieceMapping { get; set; }
        public Side SideToMove { get; private set; }
        public Square EnPassantSquare { get; private set; }
        public Castle Castle { get; private set; }
        public List<Move> Moves { get; set; }
        public int MoveCount { get; set; }

        private void AddPiece(Square square, Side side, Piece piece)
        {
            Pieces[(int)side, (int)piece] = Pieces[(int)side, (int)piece].SetBit(square);
            Occupancies[(int)side] = Occupancies[(int)side].SetBit(square);
            Occupancies[(int)Side.Both] = Occupancies[(int)Side.Both].SetBit(square);
            PieceMapping[(int)square] = piece;
        }

        private Piece RemovePiece(Square square)
        {
            Piece pieceOnSquare = PieceMapping[(int)square];
            Pieces[(int)Side.White, (int)pieceOnSquare] = Pieces[(int)Side.White, (int)pieceOnSquare].ClearBit(square);
            Pieces[(int)Side.Black, (int)pieceOnSquare] = Pieces[(int)Side.Black, (int)pieceOnSquare].ClearBit(square);
            Occupancies[(int)Side.Black] = Occupancies[(int)Side.Black].ClearBit(square);
            Occupancies[(int)Side.White] = Occupancies[(int)Side.White].ClearBit(square);
            Occupancies[(int)Side.Both] = Occupancies[(int)Side.Both].ClearBit(square);
            PieceMapping[(int)square] = Piece.None;
            return pieceOnSquare;
        }

        public int GetPieceOn(Square square, Side side)
        {
            var piece = PieceMapping[(int)square];
            return Occupancies[(int)side].GetBit(square) == 1 ? (int)piece : (int)Piece.None;
        }

        public int GetPieceOn(Square square)
        {
            var piece = (int)PieceMapping[(int)square];
            if (piece == (int)Piece.None) return -1;
            return Occupancies[(int)Side.White].GetBit(square) == 1 ? piece : 6 + piece;
        }

        public bool IsInCheck(Side side)
        {
            return IsSquareAttacked((Square)Pieces[(int)side, (int)Piece.King].GetLsb(), side.Other());
        }
        public void MakeMove(Move move)
        {
            var movedPiece = RemovePiece(move.Source);
            var capturedPiece = Piece.None;
            var originalEnPassantSquare = EnPassantSquare;
            var originalCastlingRights = Castle;

            if (move.IsCapture())
            {
                capturedPiece = RemovePiece(move.Type == MoveTypes.EnPassant ? EnPassantSquareFor(move) : move.Target);
            }

            if (move.IsPromotion())
            {
                if (move.Type.Piece == Piece.None) throw new ArgumentOutOfRangeException(nameof(move.Type));
                movedPiece = move.Type.Piece;
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

            Castle &= (Castle)CastlingConstants[(int)move.Source];
            Castle &= (Castle)CastlingConstants[(int)move.Target];
            EnPassantSquare = move.Type == MoveTypes.DoublePush ? EnPassantSquareFor(move) : Square.NoSquare;
            SideToMove = SideToMove.Other();

            SaveState(capturedPiece, originalEnPassantSquare, originalCastlingRights);
            MoveCount++;

            Moves = new List<Move>(64);
        }


        public void UnmakeMove(Move move)
        {
            SavedState state = RestoreState();

            var movedPiece = RemovePiece(move.Target);
            SideToMove = SideToMove.Other();

            if (state.CapturedPiece != Piece.None)
            {
                if (move.Type == MoveTypes.EnPassant)
                {
                    AddPiece(EnPassantSquareFor(move), SideToMove.Other(), Piece.Pawn);
                }
                else
                {
                    AddPiece(move.Target, SideToMove.Other(), state.CapturedPiece);
                }
            }

            if (move.IsCastle())
            {
                switch (move.Target)
                {
                    case Square.c1:
                        RemovePiece(Square.d1);
                        AddPiece(Square.a1, SideToMove, Piece.Rook);
                        break;
                    case Square.g1:
                        RemovePiece(Square.f1);
                        AddPiece(Square.h1, SideToMove, Piece.Rook);
                        break;
                    case Square.c8:
                        RemovePiece(Square.d8);
                        AddPiece(Square.a8, SideToMove, Piece.Rook);
                        break;
                    case Square.g8:
                        RemovePiece(Square.f8);
                        AddPiece(Square.h8, SideToMove, Piece.Rook);
                        break;
                    default: throw new ArgumentOutOfRangeException(nameof(move.Target));
                }
            }

            AddPiece(move.Source, SideToMove, move.IsPromotion() ? Piece.Pawn : movedPiece);

            Castle = state.CastlingRights;
            EnPassantSquare = state.EnPassantSquare;
            MoveCount--;
        }
        private Square EnPassantSquareFor(Move move)
        {
            return move.Target + 8 * (SideToMove == Side.Black ? -1 : 1);
        }


        private const string AsciiPieces = "PNBRQK-";

        private static readonly int[] CastlingConstants =
        {
            7, 15, 15, 15, 3, 15, 15, 11,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15,
            13, 15, 15, 15, 12, 15, 15, 14
        };


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
            return Equals((BoardState)obj);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Pieces, Occupancies);
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

        public override string ToString()
        {
            var boardHash = Zobrist.GetBoardHash(this);
            return CommonStateNames.TryGetValue(boardHash, out var commonName) ? commonName : boardHash.ToString();
        }
    }
}