using Rudim.Common;
using System;
using System.Collections.Generic;

namespace Rudim.Board
{
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
            throw new NotImplementedException();
        }


        private void AddMoveToMovesList(int source, int target)
        {
            var moveType = Occupancies[1 - (int)SideToMove].GetBit(target) == 1 ? MoveType.Capture : MoveType.Quiet;
            var move = new Move(source: (Square)source, target: (Square)target, type: moveType);
            Moves.Add(move);
        }
    }
}
