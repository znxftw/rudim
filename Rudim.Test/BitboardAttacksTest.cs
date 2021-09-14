using System.Numerics;
using Xunit;

namespace Rudim.Test
{
    public class BitboardAttacksTest
    {
        // TODO : Minor improvement, the first test run takes longer than the rest because of the static initializer
        //        Implement an IClassFixture and initialize before running any tests to avoid any wrongly reported test runtimes

        [Fact]
        public void ShouldGetAttacksForCentralPawn()
        {
            var PawnAttacksWhite = Bitboard.GetPawnAttacks(Square.e5, Side.White);
            var PawnAttacksBlack = Bitboard.GetPawnAttacks(Square.e5, Side.Black);

            Assert.Equal(1, PawnAttacksWhite.GetBit(Square.f6));
            Assert.Equal(1, PawnAttacksWhite.GetBit(Square.d6));
            Assert.Equal(1, PawnAttacksBlack.GetBit(Square.f4));
            Assert.Equal(1, PawnAttacksBlack.GetBit(Square.d4));
            Assert.Equal(2, BitOperations.PopCount(PawnAttacksBlack.Board));
            Assert.Equal(2, BitOperations.PopCount(PawnAttacksWhite.Board));
        }

        [Fact]
        public void ShouldGetAttacksForWhiteCornerPawn()
        {
            var PawnAttacksWhiteA1 = Bitboard.GetPawnAttacks(Square.a1, Side.White);
            var PawnAttacksWhiteA8 = Bitboard.GetPawnAttacks(Square.a8, Side.White);

            Assert.Equal(1, PawnAttacksWhiteA1.GetBit(Square.b2));
            Assert.Equal(1, BitOperations.PopCount(PawnAttacksWhiteA1.Board));

            Assert.Equal(0, BitOperations.PopCount(PawnAttacksWhiteA8.Board));
        }

        [Fact]
        public void ShouldGetAttacksForBlackCornerPawn()
        {
            var PawnAttacksBlackA1 = Bitboard.GetPawnAttacks(Square.a1, Side.Black);
            var PawnAttacksBlackA8 = Bitboard.GetPawnAttacks(Square.a8, Side.Black);

            Assert.Equal(0, BitOperations.PopCount(PawnAttacksBlackA1.Board));

            Assert.Equal(1, PawnAttacksBlackA8.GetBit(Square.b7));
            Assert.Equal(1, BitOperations.PopCount(PawnAttacksBlackA8.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralKnight()
        {
            var KnightAttacksE5 = Bitboard.GetKnightAttacks(Square.e5);

            Assert.Equal(1, KnightAttacksE5.GetBit(Square.f3));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.g4));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.g6));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.f7));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.d7));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.c6));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.c4));
            Assert.Equal(1, KnightAttacksE5.GetBit(Square.d3));
            Assert.Equal(8, BitOperations.PopCount(KnightAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerKnight()
        {
            var KnightAttacksA1 = Bitboard.GetKnightAttacks(Square.a1);

            Assert.Equal(1, KnightAttacksA1.GetBit(Square.b3));
            Assert.Equal(1, KnightAttacksA1.GetBit(Square.c2));
            Assert.Equal(2, BitOperations.PopCount(KnightAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralKing()
        {
            var KingAttacksE5 = Bitboard.GetKingAttacks(Square.e5);

            Assert.Equal(1, KingAttacksE5.GetBit(Square.e4));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.e6));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.f4));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.f5));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.f6));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.d4));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.d5));
            Assert.Equal(1, KingAttacksE5.GetBit(Square.d6));
            Assert.Equal(8, BitOperations.PopCount(KingAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerKing()
        {
            var KingAttacksA1 = Bitboard.GetKingAttacks(Square.a1);

            Assert.Equal(1, KingAttacksA1.GetBit(Square.a2));
            Assert.Equal(1, KingAttacksA1.GetBit(Square.b1));
            Assert.Equal(1, KingAttacksA1.GetBit(Square.b2));
            Assert.Equal(3, BitOperations.PopCount(KingAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralBishopWithNoBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            var BishopAttacksE5 = Bitboard.GetBishopAttacks(Square.e5, OccupancyBoard);

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.f4));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.g3));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.h2));

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.f6));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.g7));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.h8));

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.d4));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.c3));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.b2));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.a1));

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.d6));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.c7));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.b8));

            Assert.Equal(13, BitOperations.PopCount(BishopAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralBishopWithBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            OccupancyBoard.SetBit(Square.d4); // Should prune c3,b2,a1
            OccupancyBoard.SetBit(Square.a5); // Should not cause any problems because it is not in the diagonal
            OccupancyBoard.SetBit(Square.h2); // Should not change anything as it is an edge square
            var BishopAttacksE5 = Bitboard.GetBishopAttacks(Square.e5, OccupancyBoard);

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.f4));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.g3));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.h2));

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.f6));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.g7));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.h8));

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.d4));

            Assert.Equal(1, BishopAttacksE5.GetBit(Square.d6));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.c7));
            Assert.Equal(1, BishopAttacksE5.GetBit(Square.b8));

            Assert.Equal(10, BitOperations.PopCount(BishopAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerBishopWithNoBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            var BishopAttacksA1 = Bitboard.GetBishopAttacks(Square.a1, OccupancyBoard);

            Assert.Equal(1, BishopAttacksA1.GetBit(Square.b2));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.c3));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.d4));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.e5));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.f6));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.g7));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.h8));
            Assert.Equal(7, BitOperations.PopCount(BishopAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerBishopWithBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            OccupancyBoard.SetBit(Square.e5); // Should prune f6, g7, h8
            OccupancyBoard.SetBit(Square.e4); // Should not make a difference
            var BishopAttacksA1 = Bitboard.GetBishopAttacks(Square.a1, OccupancyBoard);

            Assert.Equal(1, BishopAttacksA1.GetBit(Square.b2));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.c3));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.d4));
            Assert.Equal(1, BishopAttacksA1.GetBit(Square.e5));
            Assert.Equal(4, BitOperations.PopCount(BishopAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralRookWithNoBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            var RookAttacksE5 = Bitboard.GetRookAttacks(Square.e5, OccupancyBoard);

            Assert.Equal(1, RookAttacksE5.GetBit(Square.e1));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e2));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e3));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e4));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e6));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e7));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e8));

            Assert.Equal(1, RookAttacksE5.GetBit(Square.a5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.b5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.c5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.d5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.f5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.g5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.h5));

            Assert.Equal(14, BitOperations.PopCount(RookAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralRookWithBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            OccupancyBoard.SetBit(Square.e3); // Should prune e2, e1
            OccupancyBoard.SetBit(Square.g7); // Should not make a difference
            OccupancyBoard.SetBit(Square.e8); // Should not make a difference
            OccupancyBoard.SetBit(Square.f5); // Should prune g5, h5
            var RookAttacksE5 = Bitboard.GetRookAttacks(Square.e5, OccupancyBoard);

            Assert.Equal(1, RookAttacksE5.GetBit(Square.e3));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e4));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e6));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e7));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.e8));

            Assert.Equal(1, RookAttacksE5.GetBit(Square.a5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.b5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.c5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.d5));
            Assert.Equal(1, RookAttacksE5.GetBit(Square.f5));

            Assert.Equal(10, BitOperations.PopCount(RookAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerRookWithNoBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            var RookAttacksA1 = Bitboard.GetRookAttacks(Square.a1, OccupancyBoard);

            Assert.Equal(1, RookAttacksA1.GetBit(Square.a2));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a3));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a4));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a5));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a6));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a7));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a8));

            Assert.Equal(1, RookAttacksA1.GetBit(Square.b1));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.c1));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.d1));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.e1));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.f1));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.g1));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.h1));

            Assert.Equal(14, BitOperations.PopCount(RookAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerRookWithBlockers()
        {
            var OccupancyBoard = new Bitboard(0);
            OccupancyBoard.SetBit(Square.a5); // Should prune a6, a7, a8
            OccupancyBoard.SetBit(Square.g7); // Should not make a difference
            OccupancyBoard.SetBit(Square.e8); // Should not make a difference
            OccupancyBoard.SetBit(Square.b1); // Should prune c1, d1, e1, f1, g1, h1
            var RookAttacksA1 = Bitboard.GetRookAttacks(Square.a1, OccupancyBoard);

            Assert.Equal(1, RookAttacksA1.GetBit(Square.a2));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a3));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a4));
            Assert.Equal(1, RookAttacksA1.GetBit(Square.a5));

            Assert.Equal(1, RookAttacksA1.GetBit(Square.b1));

            Assert.Equal(5, BitOperations.PopCount(RookAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForQueen()
        {
            var OccupancyBoard = new Bitboard(0);
            var expectedBoard = Bitboard.GetBishopAttacks(Square.e5, OccupancyBoard).Board | Bitboard.GetRookAttacks(Square.e5, OccupancyBoard).Board;

            var QueenAttacksE5 = Bitboard.GetQueenAttacks(Square.e5, OccupancyBoard);

            Assert.Equal(expectedBoard, QueenAttacksE5.Board);
        }
    }
}
