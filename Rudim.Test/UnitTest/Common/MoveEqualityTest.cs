using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest.Common
{
    public class MoveEqualityTest
    {
        [Fact]
        public void EqualMovesShouldBeEqual()
        {
            var move1 = new Move(Square.e2, Square.e4, MoveTypes.Quiet);
            var move2 = new Move(Square.e2, Square.e4, MoveTypes.Quiet);

            Assert.Equal(move1, move2);
            Assert.True(move1 == move2);
            Assert.False(move1 != move2);
            Assert.True(move1.Equals(move2));
        }

        [Fact]
        public void MovesWithDifferentSourcesShouldNotBeEqual()
        {
            var move1 = new Move(Square.e2, Square.e4, MoveTypes.Quiet);
            var move2 = new Move(Square.d2, Square.e4, MoveTypes.Quiet);

            Assert.NotEqual(move1, move2);
            Assert.False(move1 == move2);
            Assert.True(move1 != move2);
            Assert.False(move1.Equals(move2));
        }

        [Fact]
        public void MovesWithDifferentTargetsShouldNotBeEqual()
        {
            var move1 = new Move(Square.e2, Square.e4, MoveTypes.Quiet);
            var move2 = new Move(Square.e2, Square.e3, MoveTypes.Quiet);

            Assert.NotEqual(move1, move2);
            Assert.False(move1 == move2);
            Assert.True(move1 != move2);
            Assert.False(move1.Equals(move2));
        }

        [Fact]
        public void MovesWithDifferentTypesShouldNotBeEqual()
        {
            var move1 = new Move(Square.e2, Square.e4, MoveTypes.Quiet);
            var move2 = new Move(Square.e2, Square.e4, MoveTypes.Capture);

            Assert.NotEqual(move1, move2);
            Assert.False(move1 == move2);
            Assert.True(move1 != move2);
            Assert.False(move1.Equals(move2));
        }

        [Fact]
        public void MoveEqualityWithNull()
        {
            var move = new Move(Square.e2, Square.e4, MoveTypes.Quiet);

            Assert.False(move.Equals(null));
            Assert.False(move == null);
            Assert.False(null == move);
            Assert.True(move != null);
            Assert.True(null != move);
        }

        [Fact]
        public void NoMoveShouldEqualItself()
        {
            var move1 = Move.NoMove;
            var move2 = Move.NoMove;

            Assert.Equal(move1, move2);
            Assert.True(move1 == move2);
            Assert.False(move1 != move2);
            Assert.True(move1.Equals(move2));
        }
    }
}