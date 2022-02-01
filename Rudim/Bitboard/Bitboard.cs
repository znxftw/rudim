using System;
using System.Numerics;
using Rudim.Common;

namespace Rudim
{
    public partial class Bitboard
    {
        public ulong Board { get; private set; }


        public Bitboard(ulong board)
        {
            Board = board;
        }

        public Bitboard CreateCopy()
        {
            return new(Board);
        }

        public int GetBit(Square square)
        {
            return GetBit((int)square);
        }

        public void SetBit(Square square)
        {
            SetBit((int)square);
        }

        public void ClearBit(Square square)
        {
            ClearBit((int)square);
        }

        public int GetBit(int square)
        {
            return (Board & (1ul << square)) > 0 ? 1 : 0;
        }

        public void SetBit(int square)
        {
            Board |= 1ul << square;
        }

        public void ClearBit(int square)
        {
            Board &= ~(1ul << square);
        }

        public int GetLsb()
        {
            return BitOperations.TrailingZeroCount(Board);
        }
        public void Print()
        {
            for (var rank = 0; rank < 8; ++rank)
            {
                for (var file = 0; file < 8; ++file)
                {
                    if (file == 0)
                        Console.Write((8 - rank) + "\t");
                    var square = (rank * 8) + file;
                    Console.Write(GetBit(square) + " ");
                }
                Console.Write(Environment.NewLine);
            }
            Console.WriteLine(Environment.NewLine + "\ta b c d e f g h ");
        }

        public override bool Equals(object obj)
        {
            return obj is Bitboard bitboard &&
                   Board == bitboard.Board;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Board);
        }
    }
}