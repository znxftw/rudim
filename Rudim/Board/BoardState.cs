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
            Moves = new List<Move>(32);
            MoveCount = 0;
            for (var square = 0; square < Constants.Squares; ++square)
                PieceMapping[square] = Piece.None;
        }

        public static BoardState Default()
        {
            return ParseFEN(Helpers.StartingFEN);
        }

        public ulong BoardHash { get; set; }
        public Bitboard[,] Pieces { get; }
        public Bitboard[] Occupancies { get; }
        public Piece[] PieceMapping { get; set; }
        public Side SideToMove { get; private set; }
        public Square EnPassantSquare { get; private set; }
        public Castle Castle { get; private set; }
        public List<Move> Moves { get; set; }
        private int LastDrawKiller { get; set; }
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
            var capturedPiece = Piece.None;
            var originalBoardHash = BoardHash;
            var originalEnPassantSquare = EnPassantSquare;
            var originalCastlingRights = Castle;
            var originalLastDrawKiller = LastDrawKiller;
            
            BoardHash ^= Zobrist.ZobristTable[GetPieceOn(move.Source), (int)move.Source];
            var movedPiece = RemovePiece(move.Source);
            if (movedPiece == Piece.Pawn)
            {
                LastDrawKiller = MoveCount;
            }


            if (move.IsCapture())
            {
                capturedPiece = HandleCapture(move);
            }

            if (move.IsPromotion())
            {
                movedPiece = move.Type.Piece;
            }

            if (move.IsCastle())
            {
                HandleCastle(move);
            }

            AddPiece(move.Target, SideToMove, movedPiece);
            BoardHash ^= Zobrist.ZobristTable[GetPieceOn(move.Target), (int)move.Target];

            UpdateCastlingRights(move);
            UpdateEnPassant(move);
            FlipSideToMove();

            History.SaveBoardHistory(capturedPiece, originalEnPassantSquare, originalCastlingRights, originalBoardHash, originalLastDrawKiller);
            Moves = new List<Move>(32);
            MoveCount++;
        }

        private void HandleCastle(Move move)
        {
            switch (move.Target)
            {
                case Square.c1:
                    MoveRookFrom(Square.a1, Square.d1, SideToMove);
                    break;
                case Square.g1:
                    MoveRookFrom(Square.h1, Square.f1, SideToMove);
                    break;
                case Square.c8:
                    MoveRookFrom(Square.a8, Square.d8, SideToMove);
                    break;
                case Square.g8:
                    MoveRookFrom(Square.h8, Square.f8, SideToMove);
                    break;
            }
        }

        private Piece HandleCapture(Move move)
        {
            Square targetSquare = move.Type == MoveTypes.EnPassant ? EnPassantSquareFor(move) : move.Target;

            BoardHash ^= Zobrist.ZobristTable[GetPieceOn(targetSquare), (int)targetSquare];
            LastDrawKiller = MoveCount;

            return RemovePiece(targetSquare);
        }

        private void FlipSideToMove()
        {
            BoardHash = Zobrist.FlipSideToMoveHashes(this, BoardHash);
            SideToMove = SideToMove.Other();
        }

        private void UpdateEnPassant(Move move)
        {
            var originalEnPassantSquare = EnPassantSquare;
            BoardHash = Zobrist.HashEnPassant(this, BoardHash);
            EnPassantSquare = move.Type == MoveTypes.DoublePush ? EnPassantSquareFor(move) : Square.NoSquare;
            BoardHash = Zobrist.HashEnPassant(this, BoardHash);
            if(originalEnPassantSquare != EnPassantSquare)
                LastDrawKiller = MoveCount;
        }

        private void UpdateCastlingRights(Move move)
        {
            var originalCastlingRights = Castle;
            BoardHash = Zobrist.HashCastlingRights(this, BoardHash);
            Castle &= (Castle)CastlingConstants[(int)move.Source];
            Castle &= (Castle)CastlingConstants[(int)move.Target];
            BoardHash = Zobrist.HashCastlingRights(this, BoardHash);
            if(Castle != originalCastlingRights)
                LastDrawKiller = MoveCount;
        }

        private void MoveRookFrom(Square source, Square target, Side sideToMove)
        {
            RemovePiece(source);
            AddPiece(target, sideToMove, Piece.Rook);
            
            int rookIndex = GetPieceOn(target);
            BoardHash ^= Zobrist.ZobristTable[rookIndex, (int)source];
            BoardHash ^= Zobrist.ZobristTable[rookIndex, (int)target];
        }


        public void UnmakeMove(Move move)
        {
            var (capturedPiece, enPassantSquare, castlingRights, boardHash, lastDrawKiller) = History.RestoreBoardHistory();

            var movedPiece = RemovePiece(move.Target);
            SideToMove = SideToMove.Other();

            if (capturedPiece != Piece.None)
            {
                if (move.Type == MoveTypes.EnPassant)
                {
                    AddPiece(EnPassantSquareFor(move), SideToMove.Other(), Piece.Pawn);
                }
                else
                {
                    AddPiece(move.Target, SideToMove.Other(), capturedPiece);
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
            LastDrawKiller = lastDrawKiller;
            BoardHash = boardHash;
            Castle = castlingRights;
            EnPassantSquare = enPassantSquare;
            MoveCount--;
        }
        private Square EnPassantSquareFor(Move move)
        {
            return move.Target + 8 * (SideToMove == Side.Black ? -1 : 1);
        }


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
            var boardHash = BoardHash;
            return CommonStateNames.TryGetValue(boardHash, out var commonName) ? commonName : boardHash.ToString();
        }

        public bool IsDraw()
        {
            if (MoveCount - LastDrawKiller > 100) return true;
            if (MoveCount - LastDrawKiller <= 7) return false;
            return History.HasHashAppearedTwice(BoardHash, LastDrawKiller);
        }
    }
}
