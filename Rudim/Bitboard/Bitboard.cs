using Rudim.Common;
using System;
using System.Numerics;

namespace Rudim
{
    public partial struct Bitboard(ulong board) : IEquatable<Bitboard>
    {
        public ulong Board { get; private set; } = board;

        public Bitboard CreateCopy()
        {
            return new(Board);
        }

        public int GetBit(Square square)
        {
            return GetBit((int)square);
        }

        public Bitboard SetBit(Square square)
        {
            SetBit((int)square);
            return this;
        }

        public Bitboard ClearBit(Square square)
        {
            ClearBit((int)square);
            return this;
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

        public override bool Equals(object obj)
        {
            return obj is Bitboard bitboard &&
                   Board == bitboard.Board;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Board);
        }

        public bool Equals(Bitboard other)
        {
            return Equals((object)other);
        }
    }
}