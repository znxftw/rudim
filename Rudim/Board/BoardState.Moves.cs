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
            throw new NotImplementedException();
        }

        private void GenerateQueenMoves()
        {
            throw new NotImplementedException();
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
