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

        private static void ParseEnPassant(BoardState board, string fen)
        {
            if (fen != "-")
                board.EnPassantSquare = (Square)Enum.Parse(typeof(Square), fen);
        }

        private static void ParseCastling(BoardState board, string fen)
        {
            foreach (var character in fen)
            {
                switch (character)
                {
                    case 'K': board.Castle |= Castle.WhiteShort; break;
                    case 'Q': board.Castle |= Castle.WhiteLong; break;
                    case 'k': board.Castle |= Castle.BlackShort; break;
                    case 'q': board.Castle |= Castle.BlackLong; break;
                }
            }
        }

        private static void ParseSideToMove(BoardState board, string fen)
        {
            board.SideToMove = fen == "w" ? Side.White : Side.Black;
        }

        private static void ParsePieces(BoardState board, string fen)
        {
            throw new NotImplementedException();
        }
    }
}
