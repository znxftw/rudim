using System;
using System.Collections.Generic;

namespace Rudim.Board
{
    public class BoardState
    {
        public BoardState()
        {
            Pawns = new Bitboard[Constants.Sides];
            Knights = new Bitboard[Constants.Sides];
            Bishops = new Bitboard[Constants.Sides];
            Rooks = new Bitboard[Constants.Sides];
            Kings = new Bitboard[Constants.Sides];
            Queens = new Bitboard[Constants.Sides];
        }

        public Bitboard[] Pawns { get; set; }
        public Bitboard[] Knights { get; set; }
        public Bitboard[] Bishops { get; set; }
        public Bitboard[] Rooks { get; set; }
        public Bitboard[] Kings { get; set; }
        public Bitboard[] Queens { get; set; }
        public Bitboard WhitePieces { get; set; }
        public Bitboard BlackPieces { get; set; }
        public Bitboard AllPieces { get; set; }
        public Side SideToMove { get; set; }
        public Square EnPassantSquare { get; set; }
        public Castle Castle { get; set; }


        private string AsciiPieces = "-PNBRQK";
        public void Print()
        {

            for (int rank = 0; rank < 8; ++rank)
            {
                for (int file = 0; file < 8; ++file)
                {
                    if (file == 0)
                        Console.Write((8 - rank) + "\t");
                    int square = (rank * 8) + file;

                    var piece = Piece.None;
                    foreach (var enumSide in Enum.GetValues(typeof(Side)))
                    {
                        var index = (int)enumSide;
                        if(Pawns[index].GetBit(square) == 1) { piece = Piece.Pawn; }
                        if(Knights[index].GetBit(square) == 1) { piece = Piece.Knight;}
                        if(Bishops[index].GetBit(square) == 1) { piece = Piece.Bishop;}
                        if(Rooks[index].GetBit(square) == 1) { piece = Piece.Rook; }
                        if(Queens[index].GetBit(square) == 1) { piece = Piece.Queen; }
                        if(Kings[index].GetBit(square) == 1) { piece = Piece.King; }
                    }
                    char asciiValue = BlackPieces.GetBit(square) == 0 ? Char.ToLower(AsciiPieces[(int)piece]) : AsciiPieces[(int)piece];
                    Console.Write(asciiValue + " ");
                }
                Console.Write(Environment.NewLine);
            }
            Console.WriteLine(Environment.NewLine + "\ta b c d e f g h ");
        }
    }
}
