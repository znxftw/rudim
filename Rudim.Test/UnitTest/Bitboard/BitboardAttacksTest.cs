using Rudim.Common;
using System.Numerics;
using Xunit;

namespace Rudim.Test.UnitTest.Bitboard
{
    public class BitboardAttacksTest
    {
        [Fact]
        public void ShouldGetAttacksForCentralPawn()
        {
            var pawnAttacksWhite = Rudim.Bitboard.GetPawnAttacks(Square.e5, Side.White);
            var pawnAttacksBlack = Rudim.Bitboard.GetPawnAttacks(Square.e5, Side.Black);

            Assert.Equal(1, pawnAttacksWhite.GetBit(Square.f6));
            Assert.Equal(1, pawnAttacksWhite.GetBit(Square.d6));
            Assert.Equal(1, pawnAttacksBlack.GetBit(Square.f4));
            Assert.Equal(1, pawnAttacksBlack.GetBit(Square.d4));
            Assert.Equal(2, BitOperations.PopCount(pawnAttacksBlack.Board));
            Assert.Equal(2, BitOperations.PopCount(pawnAttacksWhite.Board));
        }

        [Fact]
        public void ShouldGetAttacksForWhiteCornerPawn()
        {
            var pawnAttacksWhiteA1 = Rudim.Bitboard.GetPawnAttacks(Square.a1, Side.White);
            var pawnAttacksWhiteA8 = Rudim.Bitboard.GetPawnAttacks(Square.a8, Side.White);

            Assert.Equal(1, pawnAttacksWhiteA1.GetBit(Square.b2));
            Assert.Equal(1, BitOperations.PopCount(pawnAttacksWhiteA1.Board));

            Assert.Equal(0, BitOperations.PopCount(pawnAttacksWhiteA8.Board));
        }

        [Fact]
        public void ShouldGetAttacksForBlackCornerPawn()
        {
            var pawnAttacksBlackA1 = Rudim.Bitboard.GetPawnAttacks(Square.a1, Side.Black);
            var pawnAttacksBlackA8 = Rudim.Bitboard.GetPawnAttacks(Square.a8, Side.Black);

            Assert.Equal(0, BitOperations.PopCount(pawnAttacksBlackA1.Board));

            Assert.Equal(1, pawnAttacksBlackA8.GetBit(Square.b7));
            Assert.Equal(1, BitOperations.PopCount(pawnAttacksBlackA8.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralKnight()
        {
            var knightAttacksE5 = Rudim.Bitboard.GetKnightAttacks(Square.e5);

            Assert.Equal(1, knightAttacksE5.GetBit(Square.f3));
            Assert.Equal(1, knightAttacksE5.GetBit(Square.g4));
            Assert.Equal(1, knightAttacksE5.GetBit(Square.g6));
            Assert.Equal(1, knightAttacksE5.GetBit(Square.f7));
            Assert.Equal(1, knightAttacksE5.GetBit(Square.d7));
            Assert.Equal(1, knightAttacksE5.GetBit(Square.c6));
            Assert.Equal(1, knightAttacksE5.GetBit(Square.c4));
            Assert.Equal(1, knightAttacksE5.GetBit(Square.d3));
            Assert.Equal(8, BitOperations.PopCount(knightAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerKnight()
        {
            var knightAttacksA1 = Rudim.Bitboard.GetKnightAttacks(Square.a1);

            Assert.Equal(1, knightAttacksA1.GetBit(Square.b3));
            Assert.Equal(1, knightAttacksA1.GetBit(Square.c2));
            Assert.Equal(2, BitOperations.PopCount(knightAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralKing()
        {
            var kingAttacksE5 = Rudim.Bitboard.GetKingAttacks(Square.e5);

            Assert.Equal(1, kingAttacksE5.GetBit(Square.e4));
            Assert.Equal(1, kingAttacksE5.GetBit(Square.e6));
            Assert.Equal(1, kingAttacksE5.GetBit(Square.f4));
            Assert.Equal(1, kingAttacksE5.GetBit(Square.f5));
            Assert.Equal(1, kingAttacksE5.GetBit(Square.f6));
            Assert.Equal(1, kingAttacksE5.GetBit(Square.d4));
            Assert.Equal(1, kingAttacksE5.GetBit(Square.d5));
            Assert.Equal(1, kingAttacksE5.GetBit(Square.d6));
            Assert.Equal(8, BitOperations.PopCount(kingAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerKing()
        {
            var kingAttacksA1 = Rudim.Bitboard.GetKingAttacks(Square.a1);

            Assert.Equal(1, kingAttacksA1.GetBit(Square.a2));
            Assert.Equal(1, kingAttacksA1.GetBit(Square.b1));
            Assert.Equal(1, kingAttacksA1.GetBit(Square.b2));
            Assert.Equal(3, BitOperations.PopCount(kingAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralBishopWithNoBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            var bishopAttacksE5 = Rudim.Bitboard.GetBishopAttacks(Square.e5, occupancyBoard);

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.f4));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.g3));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.h2));

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.f6));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.g7));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.h8));

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.d4));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.c3));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.b2));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.a1));

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.d6));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.c7));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.b8));

            Assert.Equal(13, BitOperations.PopCount(bishopAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralBishopWithBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            occupancyBoard.SetBit(Square.d4); // Should prune c3,b2,a1
            occupancyBoard.SetBit(Square.a5); // Should not cause any problems because it is not in the diagonal
            occupancyBoard.SetBit(Square.h2); // Should not change anything as it is an edge square
            var bishopAttacksE5 = Rudim.Bitboard.GetBishopAttacks(Square.e5, occupancyBoard);

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.f4));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.g3));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.h2));

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.f6));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.g7));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.h8));

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.d4));

            Assert.Equal(1, bishopAttacksE5.GetBit(Square.d6));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.c7));
            Assert.Equal(1, bishopAttacksE5.GetBit(Square.b8));

            Assert.Equal(10, BitOperations.PopCount(bishopAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerBishopWithNoBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            var bishopAttacksA1 = Rudim.Bitboard.GetBishopAttacks(Square.a1, occupancyBoard);

            Assert.Equal(1, bishopAttacksA1.GetBit(Square.b2));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.c3));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.d4));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.e5));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.f6));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.g7));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.h8));
            Assert.Equal(7, BitOperations.PopCount(bishopAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerBishopWithBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            occupancyBoard.SetBit(Square.e5); // Should prune f6, g7, h8
            occupancyBoard.SetBit(Square.e4); // Should not make a difference
            var bishopAttacksA1 = Rudim.Bitboard.GetBishopAttacks(Square.a1, occupancyBoard);

            Assert.Equal(1, bishopAttacksA1.GetBit(Square.b2));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.c3));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.d4));
            Assert.Equal(1, bishopAttacksA1.GetBit(Square.e5));
            Assert.Equal(4, BitOperations.PopCount(bishopAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralRookWithNoBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            var rookAttacksE5 = Rudim.Bitboard.GetRookAttacks(Square.e5, occupancyBoard);

            Assert.Equal(1, rookAttacksE5.GetBit(Square.e1));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e2));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e3));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e4));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e6));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e7));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e8));

            Assert.Equal(1, rookAttacksE5.GetBit(Square.a5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.b5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.c5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.d5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.f5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.g5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.h5));

            Assert.Equal(14, BitOperations.PopCount(rookAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCentralRookWithBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            occupancyBoard.SetBit(Square.e3); // Should prune e2, e1
            occupancyBoard.SetBit(Square.g7); // Should not make a difference
            occupancyBoard.SetBit(Square.e8); // Should not make a difference
            occupancyBoard.SetBit(Square.f5); // Should prune g5, h5
            var rookAttacksE5 = Rudim.Bitboard.GetRookAttacks(Square.e5, occupancyBoard);

            Assert.Equal(1, rookAttacksE5.GetBit(Square.e3));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e4));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e6));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e7));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.e8));

            Assert.Equal(1, rookAttacksE5.GetBit(Square.a5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.b5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.c5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.d5));
            Assert.Equal(1, rookAttacksE5.GetBit(Square.f5));

            Assert.Equal(10, BitOperations.PopCount(rookAttacksE5.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerRookWithNoBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            var rookAttacksA1 = Rudim.Bitboard.GetRookAttacks(Square.a1, occupancyBoard);

            Assert.Equal(1, rookAttacksA1.GetBit(Square.a2));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a3));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a4));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a5));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a6));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a7));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a8));

            Assert.Equal(1, rookAttacksA1.GetBit(Square.b1));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.c1));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.d1));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.e1));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.f1));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.g1));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.h1));

            Assert.Equal(14, BitOperations.PopCount(rookAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForCornerRookWithBlockers()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            occupancyBoard.SetBit(Square.a5); // Should prune a6, a7, a8
            occupancyBoard.SetBit(Square.g7); // Should not make a difference
            occupancyBoard.SetBit(Square.e8); // Should not make a difference
            occupancyBoard.SetBit(Square.b1); // Should prune c1, d1, e1, f1, g1, h1
            var rookAttacksA1 = Rudim.Bitboard.GetRookAttacks(Square.a1, occupancyBoard);

            Assert.Equal(1, rookAttacksA1.GetBit(Square.a2));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a3));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a4));
            Assert.Equal(1, rookAttacksA1.GetBit(Square.a5));

            Assert.Equal(1, rookAttacksA1.GetBit(Square.b1));

            Assert.Equal(5, BitOperations.PopCount(rookAttacksA1.Board));
        }

        [Fact]
        public void ShouldGetAttacksForQueen()
        {
            var occupancyBoard = new Rudim.Bitboard(0);
            var expectedBoard = Rudim.Bitboard.GetBishopAttacks(Square.e5, occupancyBoard).Board | Rudim.Bitboard.GetRookAttacks(Square.e5, occupancyBoard).Board;

            var queenAttacksE5 = Rudim.Bitboard.GetQueenAttacks(Square.e5, occupancyBoard);

            Assert.Equal(expectedBoard, queenAttacksE5.Board);
        }
    }
}