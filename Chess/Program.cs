namespace Chess
{
    class Program
    {
        static void Main(string[] args)
        {
            var Board = new Bitboard(1);
            Board.SetBit(Square.e8);
            Board.ClearBit(Square.a8);

            Board.Print();
        }
    }
}
