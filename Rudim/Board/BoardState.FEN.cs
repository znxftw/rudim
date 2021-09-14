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
            var ranks = fen.Split('/');

            for (var rank = 0; rank < 8; rank++)
            {
                var index = rank * 8;
                for (var file = 0; file < ranks[rank].Length; file++)
                {
                    var symbol = ranks[rank][file];
                    if (char.IsLetter(symbol))
                    {
                        var piece = Piece.None;
                        switch (char.ToLower(symbol))
                        {
                            case 'p': piece = Piece.Pawn; break;
                            case 'r': piece = Piece.Rook; break;
                            case 'n': piece = Piece.Knight; break;
                            case 'b': piece = Piece.Bishop; break;
                            case 'q': piece = Piece.Queen; break;
                            case 'k': piece = Piece.King; break;
                        }
                        var side = char.IsUpper(symbol) ? Side.White : Side.Black;
                        board.AddPiece((Square)index, side, piece);
                        index++;
                    }
                    else if (char.IsDigit(symbol))
                    {
                        index += symbol - '0';
                    }
                }
            }
        }
    }
}
