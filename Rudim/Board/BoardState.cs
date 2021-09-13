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
    }
}
