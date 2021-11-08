using Rudim.Common;
using System;
using System.Collections.Generic;

namespace Rudim.Board
{
    // TODO : Castling , Pawn Promotions, Pawn Pushes
    public partial class BoardState
    {
        public void GenerateMoves()
        {
            Moves = new List<Move>();

            GeneratePawnMoves();
            GenerateBishopMoves();
            GenerateKnightMoves();
            GenerateRookMoves();
            GenerateQueenMoves();
            GenerateKingMoves();
        }

        private void GenerateKingMoves()
        {
            var bitboard = Pieces[(int)SideToMove, (int)Piece.King].CreateCopy();
            while (bitboard.Board > 0)
            {
                var source = bitboard.GetLsb();
                var attacks = new Bitboard(Bitboard.KingAttacks[source]);

                while (attacks.Board > 0)
                {
                    var target = attacks.GetLsb();

                    if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                    {
                        attacks.ClearBit(target);
                        continue;
                    }

                    AddMoveToMovesList(source, target);

                    attacks.ClearBit(target);
                }

                GenerateCastleMoves();
                bitboard.ClearBit(source);
            }

        }


        private void GenerateQueenMoves()
        {
            var bitboard = Pieces[(int)SideToMove, (int)Piece.Queen].CreateCopy();
            while (bitboard.Board > 0)
            {
                var source = bitboard.GetLsb();
                var attacks = Bitboard.GetQueenAttacksFromTable((Square)source, Occupancies[(int)Side.Both]);

                while (attacks.Board > 0)
                {
                    var target = attacks.GetLsb();

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
            var bitboard = Pieces[(int)SideToMove, (int)Piece.Rook].CreateCopy();
            while (bitboard.Board > 0)
            {
                var source = bitboard.GetLsb();
                var attacks = Bitboard.GetRookAttacksFromTable((Square)source, Occupancies[(int)Side.Both]);

                while (attacks.Board > 0)
                {
                    var target = attacks.GetLsb();

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
            var bitboard = Pieces[(int)SideToMove, (int)Piece.Knight].CreateCopy();
            while (bitboard.Board > 0)
            {
                var source = bitboard.GetLsb();
                var attacks = new Bitboard(Bitboard.KnightAttacks[source]);

                while (attacks.Board > 0)
                {
                    var target = attacks.GetLsb();

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
            var bitboard = Pieces[(int)SideToMove, (int)Piece.Bishop].CreateCopy();
            while (bitboard.Board > 0)
            {
                var source = bitboard.GetLsb();
                var attacks = Bitboard.GetBishopAttacksFromTable((Square)source, Occupancies[(int)Side.Both]);

                while (attacks.Board > 0)
                {
                    var target = attacks.GetLsb();

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
            var bitboard = Pieces[(int)SideToMove, (int)Piece.King].CreateCopy();
            while (bitboard.Board > 0)
            {
                var source = bitboard.GetLsb();

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

            var attacks = new Bitboard(Bitboard.PawnAttacks[(int)SideToMove, source] & (1ul << (int)EnPassantSquare));
            if (attacks.Board > 0)
            {
                var target = attacks.GetLsb();
                AddPawnMove(source, target, true, false);
            }

        }


        private void GeneratePawnPushes(int source)
        {
            if (SideToMove == Side.Black)
            {
                var oneSquarePush = source + 8;
                AddPawnMove(source, oneSquarePush, false, false);
                if (source <= (int)Square.h7 && source >= (int)Square.a7)
                {
                    var twoSquarePush = oneSquarePush + 8;
                    AddPawnMove(source, twoSquarePush, false, true);
                }
            }
            else
            {
                var oneSquarePush = source - 8;
                AddPawnMove(source, oneSquarePush, false, false);
                if (source <= (int)Square.h2 && source >= (int)Square.a2)
                {
                    var twoSquarePush = oneSquarePush - 8;
                    AddPawnMove(source, twoSquarePush, false, true);
                }
            }
        }

        private void GeneratePawnAttacks(int source)
        {
            var attacks = new Bitboard(Bitboard.PawnAttacks[(int)SideToMove, source]);

            while (attacks.Board > 0)
            {
                var target = attacks.GetLsb();

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
            throw new NotImplementedException();
        }


        private void AddPawnMove(int source, int target, bool enpassant, bool doublePush)
        {
            // This assumes all incoming pawn moves are valid
            if ((target >= (int)Square.a1 && target <= (int)Square.h1) || (target >= (int)Square.h8 && target <= (int)Square.a8))
            {
                var capture = IsSquareCapture(target);

                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveType.KnightPromotionCapture : MoveType.KnightPromotion));
                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveType.BishopPromotionCapture : MoveType.BishopPromotion));
                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveType.RookPromotionCapture : MoveType.RookPromotion));
                Moves.Add(new Move((Square)source, (Square)target, capture ? MoveType.QueenPromotionCapture : MoveType.QueenPromotion));
            }
            else if (enpassant || doublePush)
            {
                Moves.Add(new Move((Square)source, (Square)target, enpassant ? MoveType.EnPassant : MoveType.DoublePush));
            }
            else
            {
                AddMoveToMovesList(source, target);
            }
        }


        private void AddMoveToMovesList(int source, int target)
        {
            // Makes more sense for source and target to come in as Square instead of int, refactor later
            var moveType = IsSquareCapture(target) ? MoveType.Capture : MoveType.Quiet;
            var move = new Move(source: (Square)source, target: (Square)target, type: moveType);
            Moves.Add(move);
        }
        private bool IsSquareCapture(int target)
        {
            return Occupancies[1 - (int)SideToMove].GetBit(target) == 1;
        }
    }
}
