using Rudim.Board;
using Rudim.Common;

namespace Rudim.Test.Util
{
    public static class Helpers
    {
        public static Move FindMoveFromMoveList(BoardState board, Move move)
        {
            board.GenerateMoves();
            var moves = board.Moves;
            for (var i = 0; i < moves.Count; ++i)
            {
                if (moves[i].Source == move.Source && moves[i].Target == move.Target)
                {
                    if (move.Type == MoveTypes.Quiet || ((byte)moves[i].Type.Value & ~8) == (byte)move.Type.Value)
                    {
                        return moves[i];
                    }
                }
            }
            return Move.NoMove;
        }
    }
}