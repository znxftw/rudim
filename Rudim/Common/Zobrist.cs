using Rudim.Board;

namespace Rudim.Common
{
    public static class Zobrist
    {
        public static readonly ulong[,] ZobristTable;

        static Zobrist()
        {
            // 12 piece types (6 for each color) and 64 squares, and extra - en passant, + edge cases below
            ZobristTable = new ulong[14, 64];
            for (int piece = 0; piece < 13; piece++)
            {
                for (int square = 0; square < 64; square++)
                {
                    ZobristTable[piece, square] = Random.NextULong();
                }
            }

            // Both not needed?
            ZobristTable[13, 0] = Random.NextULong(); // White to move
            ZobristTable[13, 1] = Random.NextULong(); // Black to move

            ZobristTable[13, 2] = Random.NextULong();
            ZobristTable[13, 3] = Random.NextULong();
            ZobristTable[13, 4] = Random.NextULong();
            ZobristTable[13, 5] = Random.NextULong();
        }

        public static ulong GetBoardHash(BoardState boardState)
        {
            ulong currentHash = 0;

            // Go through piece + loop Lsb() instead of squares?
            for (int square = 0; square < 64; square++)
            {
                int piece = boardState.GetPieceOn((Square)square);
                if (piece != -1)
                {
                    currentHash ^= ZobristTable[piece, square];
                }
            }

            currentHash = HashSideToMove(boardState, currentHash);

            currentHash = HashCastlingRights(boardState, currentHash);

            currentHash = HashEnPassant(boardState, currentHash);

            return currentHash;
        }

        public static ulong HashCastlingRights(BoardState boardState, ulong currentHash)
        {
            currentHash ^= ZobristTable[13, (int)boardState.Castle];
            return currentHash;
        }

        private static ulong HashSideToMove(BoardState boardState, ulong currentHash)
        {
            currentHash ^= boardState.SideToMove == Side.White ? ZobristTable[13, 0] : ZobristTable[13, 1];
            return currentHash;
        }

        public static ulong FlipSideToMoveHashes(BoardState boardState, ulong currentHash)
        {
            currentHash ^= ZobristTable[13, 0];
            currentHash ^= ZobristTable[13, 1];
            return currentHash;
        }

        public static ulong HashEnPassant(BoardState boardState, ulong currentHash)
        {
            if (boardState.EnPassantSquare != Square.NoSquare)
            {
                currentHash ^= ZobristTable[12, (int)boardState.EnPassantSquare];
            }

            return currentHash;
        }
    }
}