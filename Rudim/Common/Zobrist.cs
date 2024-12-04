using Rudim.Board;

namespace Rudim.Common
{
    public static class Zobrist
    {
        private static readonly ulong[,] ZobristTable;

        static Zobrist()
        {
            // 12 piece types (6 for each color) and 64 squares, and extra - en passant, + edge cases below
            ZobristTable = new ulong[14, 64];
            for (int piece = 0; piece < 13; piece++)
            {
                for (int square = 0; square < 64; square++)
                {
                    ZobristTable[piece, square] = Random.NextULong() << 32 | Random.NextULong();
                }
            }

            // Both not needed?
            ZobristTable[13, 0] = Random.NextULong() << 32 | Random.NextULong(); // White to move
            ZobristTable[13, 1] = Random.NextULong() << 32 | Random.NextULong(); // Black to move

            ZobristTable[13, 2] = Random.NextULong() << 32 | Random.NextULong();
            ZobristTable[13, 3] = Random.NextULong() << 32 | Random.NextULong();
            ZobristTable[13, 4] = Random.NextULong() << 32 | Random.NextULong();
            ZobristTable[13, 5] = Random.NextULong() << 32 | Random.NextULong();
        }

        public static ulong GetBoardHash(BoardState boardState)
        {
            ulong currentHash = 0;

            // Go through piece + loop Lsb() instead of squares?
            for (var square = 0; square < 64; square++)
            {
                var piece = boardState.GetPieceOn((Square)square);
                if (piece != -1)
                {
                    currentHash ^= ZobristTable[piece, square];
                }
            }

            currentHash ^= boardState.SideToMove == Side.White ? ZobristTable[13, 0] : ZobristTable[13, 1];

            currentHash ^= boardState.Castle.HasFlag(Castle.WhiteLong) ? ZobristTable[13, 2] : 0;
            currentHash ^= boardState.Castle.HasFlag(Castle.BlackShort) ? ZobristTable[13, 3] : 0;
            currentHash ^= boardState.Castle.HasFlag(Castle.BlackLong) ? ZobristTable[13, 4] : 0;
            currentHash ^= boardState.Castle.HasFlag(Castle.WhiteShort) ? ZobristTable[13, 5] : 0;

            if (boardState.EnPassantSquare != Square.NoSquare)
            {
                currentHash ^= ZobristTable[12, (int)boardState.EnPassantSquare];
            }

            return currentHash;
        }
    }
}