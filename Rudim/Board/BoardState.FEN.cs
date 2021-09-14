using System;

namespace Rudim.Board
{
    public partial class BoardState
    {
        public static BoardState ParseFEN(string FEN)
        {
            var board = new BoardState();
            var sections = FEN.Split(' ');
            ParsePieces(board, sections[0]);
            ParseSideToMove(board, sections[1]);
            ParseCastling(board, sections[2]);
            ParseEnPassant(board, sections[3]);
            // ParsePly(board, sections[4]);
            return board;
        }

        private static void ParseEnPassant(BoardState board, string v)
        {
            throw new NotImplementedException();
        }

        private static void ParseCastling(BoardState board, string v)
        {
            throw new NotImplementedException();
        }

        private static void ParseSideToMove(BoardState board, string v)
        {
            throw new NotImplementedException();
        }

        private static void ParsePieces(BoardState board, string v)
        {
            throw new NotImplementedException();
        }
    }
}
