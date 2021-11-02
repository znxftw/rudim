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
                var position = bitboard.GetLsb();
                var attacks = new Bitboard(Bitboard.KingAttacks[position]);

                while (attacks.Board > 0)
                {
                    var target = attacks.GetLsb();

                    if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                    {
                        attacks.ClearBit(target);
                        continue;
                    }

                    var moveType = Occupancies[1 - (int)SideToMove].GetBit(target) == 1 ? MoveType.Capture : MoveType.Quiet;
                    var move = new Move(source: (Square)position, target: (Square)target, type: moveType);
                    Moves.Add(move);

                    attacks.ClearBit(target);
                }

                bitboard.ClearBit(position);
            }
        }

        private void GenerateQueenMoves()
        {
            var bitboard = Pieces[(int)SideToMove,(int)Piece.Queen].CreateCopy();
            while(bitboard.Board > 0)
            {
                var position = bitboard.GetLsb();
                var attacks = Bitboard.GetQueenAttacksFromTable((Square)position, Occupancies[(int)Side.Both]);
                
                while(attacks.Board > 0)
                {
                    var target = attacks.GetLsb();

                    if (Occupancies[(int)SideToMove].GetBit(target) == 1)
                    {
                        attacks.ClearBit(target);
                        continue;
                    }

                    var moveType = Occupancies[1 - (int)SideToMove].GetBit(target) == 1 ? MoveType.Capture : MoveType.Quiet;
                    var move = new Move(source: (Square)position, target: (Square)target, type: moveType);
                    Moves.Add(move);

                    attacks.ClearBit(target);
                }

                bitboard.ClearBit(position);
            }
        }

        private void GenerateRookMoves()
        {
            throw new NotImplementedException();
        }

        private void GenerateKnightMoves()
        {
            throw new NotImplementedException();
        }

        private void GenerateBishopMoves()
        {
            throw new NotImplementedException();
        }

        private void GeneratePawnMoves()
        {
            throw new NotImplementedException();
        }
    }
}
