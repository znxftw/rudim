using System;
using System.Collections.Generic;

namespace Rudim.Board
{
    public class BoardState
    {
        public BoardState()
        {
            Pieces = new Bitboard[Constants.Sides, Constants.Pieces];
            for (int side = 0; side < Constants.Sides; ++side)
                for (int piece = 0; piece < Constants.Pieces; ++piece)
                    Pieces[side, piece] = new Bitboard(0);
            Occupancies = new Bitboard[Constants.SidesWithBoth];
            for (int side = 0; side < Constants.SidesWithBoth; ++side)
                Occupancies[side] = new Bitboard(0);
        }

        public Bitboard[,] Pieces { get; set; }
        public Bitboard[] Occupancies { get; set; }
        public Side SideToMove { get; set; }
        public Square EnPassantSquare { get; set; }
        public Castle Castle { get; set; }


        private readonly string AsciiPieces = "-PNBRQK";
        public void Print()
        {

            for (int rank = 0; rank < 8; ++rank)
            {
                for (int file = 0; file < 8; ++file)
                {
                    if (file == 0)
                        Console.Write((8 - rank) + "\t");
                    int square = (rank * 8) + file;

                    var boardPiece = Piece.None;
                    for (int side = 0; side < Constants.Sides; ++side)
                        for (int piece = 0; piece < Constants.Pieces; ++piece)
                            if (Pieces[side, piece].GetBit(square) == 1)
                                boardPiece = (Piece)piece;
                    char asciiValue = Occupancies[0].GetBit(square) == 0 ? char.ToLower(AsciiPieces[(int)boardPiece]) : AsciiPieces[(int)boardPiece];
                    Console.Write(asciiValue + " ");
                }
                Console.Write(Environment.NewLine);
            }
            Console.WriteLine(Environment.NewLine + "\ta b c d e f g h ");
            Console.WriteLine(Environment.NewLine + "Side to move : " + SideToMove);
            Console.WriteLine(Environment.NewLine + "En passant square : " + EnPassantSquare);
            Console.WriteLine(Environment.NewLine + "Castling rights : " + Castle);

        }
    }
}
