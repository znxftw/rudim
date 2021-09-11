using Rudim.Common;
using System;

namespace Rudim
{
    class Program
    {
        static void Main(string[] args)
        {
            var BishopMagicNumbers = new ulong[64];
            var RookMagicNumbers = new ulong[64];
            for (int square = 0; square < 64; ++square)
            {
                BishopMagicNumbers[square] = Bitboard.FindMagicNumber((Square)square, Bitboard.BishopMaskBits[square], true);
                RookMagicNumbers[square] = Bitboard.FindMagicNumber((Square)square, Bitboard.RookMaskBits[square], false);
            }
            Console.WriteLine("Bishop Magics : \n");
            foreach(var magic in BishopMagicNumbers)
            {
                Console.WriteLine(magic + ",");
            }

            Console.WriteLine("Rook Magics : \n");
            foreach (var magic in RookMagicNumbers)
            {
                Console.WriteLine(magic + ",");
            }
        }
    }
}
