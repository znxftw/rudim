using Rudim.Common;
using Rudim.Search;
using System.Threading;

namespace Rudim.Board
{
    public partial class BoardState
    {
        public void GenerateMoves()
        {
            Moves.Clear();

            GeneratePawnMoves();
            GenerateBishopMoves();
            GenerateKnightMoves();
            GenerateRookMoves();
            GenerateQueenMoves();
            GenerateKingMoves();
        }

        private void GenerateKingMoves()
        {
            int source = Pieces[(int)SideToMove, (int)Piece.King].GetLsb();
            Bitboard attacks = new Bitboard(Bitboard.KingAttacks[source]);

            while (attacks.Board > 0)
            {
                int target = attacks.GetLsb();
                attacks.ClearBit(target);
                if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                {
                    continue;
                }
                AddMoveToMovesList(source, target);
            }

            GenerateCastleMoves();
        }
        public Move FindBestMove(int depth, CancellationToken cancellationToken, ref bool debugMode)
        {
            IterativeDeepening.Search(this, depth, cancellationToken, ref debugMode);
            return IterativeDeepening.BestMove;
        }

        private void GenerateQueenMoves()
        {
            Bitboard bitboard = Pieces[(int)SideToMove, (int)Piece.Queen];
            while (bitboard.Board > 0)
            {
                int source = bitboard.GetLsb();
                Bitboard attacks = Bitboard.GetQueenAttacksFromTable((Square)source, Occupancies[(int)Side.Both]);

                while (attacks.Board > 0)
                {
                    int target = attacks.GetLsb();

                    if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                    {
                        attacks.ClearBit(target);
                        continue;
                    }

                    AddMoveToMovesList(source, target);

                    attacks.ClearBit(target);
                }

                bitboard.ClearBit(source);
            }
        }

        private void GenerateRookMoves()
        {
            Bitboard bitboard = Pieces[(int)SideToMove, (int)Piece.Rook];
            while (bitboard.Board > 0)
            {
                int source = bitboard.GetLsb();
                Bitboard attacks = Bitboard.GetRookAttacksFromTable((Square)source, Occupancies[(int)Side.Both]);

                while (attacks.Board > 0)
                {
                    int target = attacks.GetLsb();

                    if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                    {
                        attacks.ClearBit(target);
                        continue;
                    }

                    AddMoveToMovesList(source, target);

                    attacks.ClearBit(target);
                }

                bitboard.ClearBit(source);
            }
        }

        private void GenerateKnightMoves()
        {
            Bitboard bitboard = Pieces[(int)SideToMove, (int)Piece.Knight];
            while (bitboard.Board > 0)
            {
                int source = bitboard.GetLsb();
                Bitboard attacks = new Bitboard(Bitboard.KnightAttacks[source]);

                while (attacks.Board > 0)
                {
                    int target = attacks.GetLsb();

                    if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                    {
                        attacks.ClearBit(target);
                        continue;
                    }

                    AddMoveToMovesList(source, target);

                    attacks.ClearBit(target);
                }

                bitboard.ClearBit(source);
            }
        }

        private void GenerateBishopMoves()
        {
            Bitboard bitboard = Pieces[(int)SideToMove, (int)Piece.Bishop];
            while (bitboard.Board > 0)
            {
                int source = bitboard.GetLsb();
                Bitboard attacks = Bitboard.GetBishopAttacksFromTable((Square)source, Occupancies[(int)Side.Both]);

                while (attacks.Board > 0)
                {
                    int target = attacks.GetLsb();

                    if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                    {
                        attacks.ClearBit(target);
                        continue;
                    }

                    AddMoveToMovesList(source, target);

                    attacks.ClearBit(target);
                }

                bitboard.ClearBit(source);
            }
        }

        private void GeneratePawnMoves()
        {
            Bitboard bitboard = Pieces[(int)SideToMove, (int)Piece.Pawn];
            while (bitboard.Board > 0)
            {
                int source = bitboard.GetLsb();
                GeneratePawnPushes(source);
                GenerateEnPassants(source);
                GeneratePawnAttacks(source);

                bitboard.ClearBit(source);
            }
        }

        private void GenerateEnPassants(int source)
        {
            if (EnPassantSquare == Square.NoSquare)
                return;

            Bitboard attacks = new Bitboard(Bitboard.PawnAttacks[(int)SideToMove, source] & (1ul << (int)EnPassantSquare));
            if (attacks.Board > 0)
            {
                int target = attacks.GetLsb();
                AddPawnMove(source, target, true, false);
            }

        }


        private void GeneratePawnPushes(int source)
        {
            if (SideToMove == Side.Black)
            {
                int oneSquarePush = source + 8;
                if (Occupancies[(int)Side.Both].GetBit(oneSquarePush) != 0) return;
                AddPawnMove(source, oneSquarePush, false, false);
                if (source is <= (int)Square.h7 and >= (int)Square.a7)
                {
                    int twoSquarePush = oneSquarePush + 8;
                    if (Occupancies[(int)Side.Both].GetBit(twoSquarePush) != 0) return;
                    AddPawnMove(source, twoSquarePush, false, true);
                }
            }
            else
            {
                int oneSquarePush = source - 8;
                if (Occupancies[(int)Side.Both].GetBit(oneSquarePush) != 0) return;
                AddPawnMove(source, oneSquarePush, false, false);
                if (source is <= (int)Square.h2 and >= (int)Square.a2)
                {
                    int twoSquarePush = oneSquarePush - 8;
                    if (Occupancies[(int)Side.Both].GetBit(twoSquarePush) != 0) return;
                    AddPawnMove(source, twoSquarePush, false, true);
                }
            }
        }

        private void GeneratePawnAttacks(int source)
        {
            Bitboard attacks = new Bitboard(Bitboard.PawnAttacks[(int)SideToMove, source] & Occupancies[(int)SideToMove.Other()].Board);

            while (attacks.Board > 0)
            {
                int target = attacks.GetLsb();

                if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                {
                    attacks.ClearBit(target);
                    continue;
                }

                AddPawnMove(source, target, false, false);

                attacks.ClearBit(target);
            }
        }
        private void GenerateCastleMoves()
        {
            // Squares should be empty and shouldn't castle through check, can avoid checking if landing position is check to do legal move check in make move function

            Bitboard occ = Occupancies[(int)Side.Both];
            if (SideToMove == Side.White)
            {
                if (Castle.HasFlag(Castle.WhiteShort))
                {
                    if (occ.GetBit(Square.f1) == 0 && occ.GetBit(Square.g1) == 0 && !IsSquareAttacked(Square.e1, Side.Black) && !IsSquareAttacked(Square.f1, Side.Black))
                        Moves.Add(new Move(Square.e1, Square.g1, MoveTypes.Castle));
                }
                if (Castle.HasFlag(Castle.WhiteLong))
                {
                    if (occ.GetBit(Square.d1) == 0 && occ.GetBit(Square.c1) == 0 && occ.GetBit(Square.b1) == 0 && !IsSquareAttacked(Square.e1, Side.Black) && !IsSquareAttacked(Square.d1, Side.Black))
                        Moves.Add(new Move(Square.e1, Square.c1, MoveTypes.Castle));
                }
            }
            else
            {
                if (Castle.HasFlag(Castle.BlackShort))
                {
                    if (occ.GetBit(Square.f8) == 0 && occ.GetBit(Square.g8) == 0 && !IsSquareAttacked(Square.e8, Side.White) && !IsSquareAttacked(Square.f8, Side.White))
                        Moves.Add(new Move(Square.e8, Square.g8, MoveTypes.Castle));
                }
                if (Castle.HasFlag(Castle.BlackLong))
                {
                    if (occ.GetBit(Square.d8) == 0 && occ.GetBit(Square.c8) == 0 && occ.GetBit(Square.b8) == 0 && !IsSquareAttacked(Square.e8, Side.White) && !IsSquareAttacked(Square.d8, Side.White))
                        Moves.Add(new Move(Square.e8, Square.c8, MoveTypes.Castle));
                }
            }
        }

        private bool IsSquareAttacked(Square square, Side attackingSide)
        {
            if ((Bitboard.GetBishopAttacksFromTable(square, Occupancies[(int)Side.Both]).Board & Pieces[(int)attackingSide, (int)Piece.Bishop].Board) != 0)
                return true;
            if ((Bitboard.GetRookAttacksFromTable(square, Occupancies[(int)Side.Both]).Board & Pieces[(int)attackingSide, (int)Piece.Rook].Board) != 0)
                return true;
            if ((Bitboard.GetQueenAttacksFromTable(square, Occupancies[(int)Side.Both]).Board & Pieces[(int)attackingSide, (int)Piece.Queen].Board) != 0)
                return true;
            if ((Bitboard.KnightAttacks[(int)square] & Pieces[(int)attackingSide, (int)Piece.Knight].Board) != 0)
                return true;
            if ((Bitboard.PawnAttacks[(int)attackingSide.Other(), (int)square] & Pieces[(int)attackingSide, (int)Piece.Pawn].Board) != 0)
                return true;
            if ((Bitboard.KingAttacks[(int)square] & Pieces[(int)attackingSide, (int)Piece.King].Board) != 0)
                return true;
            return false;
        }

        private void AddPawnMove(int source, int target, bool enpassant, bool doublePush)
        {
            // This assumes all incoming pawn moves are valid
            if (target is >= (int)Square.a1 and <= (int)Square.h1 || target is <= (int)Square.h8 and >= (int)Square.a8)
            {
                bool capture = IsSquareCapture(target);

                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveTypes.KnightPromotionCapture : MoveTypes.KnightPromotion));
                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveTypes.BishopPromotionCapture : MoveTypes.BishopPromotion));
                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveTypes.RookPromotionCapture : MoveTypes.RookPromotion));
                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveTypes.QueenPromotionCapture : MoveTypes.QueenPromotion));
            }
            else if (enpassant || doublePush)
            {
                Moves.Add(new Move((Square)source, (Square)target, enpassant ? MoveTypes.EnPassant : MoveTypes.DoublePush));
            }
            else
            {
                AddMoveToMovesList(source, target);
            }
        }


        private void AddMoveToMovesList(int source, int target)
        {
            MoveType moveType = IsSquareCapture(target) ? MoveTypes.Capture : MoveTypes.Quiet;
            Move move = new Move((Square)source, (Square)target, moveType);
            Moves.Add(move);
        }
        private bool IsSquareCapture(int target)
        {
            return Occupancies[(int)SideToMove.Other()].GetBit(target) == 1;
        }

        public void MakeNullMove()
        {
            History.SaveBoardHistory(Piece.None, EnPassantSquare, Castle, BoardHash, LastDrawKiller);
            UpdateEnPassant(Move.NoMove);
            FlipSideToMove();
        }

        public void UndoNullMove()
        {
            History.BoardHistory history = History.RestoreBoardHistory();
            FlipSideToMove();
            LastDrawKiller = history.LastDrawKiller;
            BoardHash = history.BoardHash;
            Castle = history.CastlingRights;
            EnPassantSquare = history.EnPassantSquare;
        }
    }
}