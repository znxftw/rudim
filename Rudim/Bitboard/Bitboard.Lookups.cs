namespace Rudim
{
    public partial class Bitboard
    {
        private static readonly ulong FileA = 72340172838076673;
        private static readonly ulong FileB = 144680345676153346;
        private static readonly ulong FileG = 4629771061636907072;
        private static readonly ulong FileH = 9259542123273814144;
        private static readonly ulong FileAB = FileA | FileB;
        private static readonly ulong FileGH = FileG | FileH;

        public static readonly ulong[,] PawnAttacks = new ulong[Constants.Sides, Constants.Squares];

        static Bitboard()
        {
            for (int side = 0; side < Constants.Sides; ++side)
                for (int square = 0; square < Constants.Squares; ++square)
                    PawnAttacks[side, square] = Bitboard.GetPawnAttacks((Square)square, (Side)side).Board;
        }
    }
}
