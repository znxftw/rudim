using Rudim.Board;
using Rudim.Common;
using Xunit;

namespace Rudim.Test.UnitTest.Common
{
    public class GamePhaseTest
    {
        [Fact]
        public void ShouldHaveMaximumPhaseForStartingPosition()
        {
            var boardState = BoardState.Default();

            int phase = GamePhase.Calculate(boardState);

            Assert.Equal(GamePhase.TotalPhase, phase);
        }

        [Fact]
        public void ShouldHaveMinimumPhaseWithOnlyKings()
        {
            const string onlyKings = "8/8/4k3/8/8/3K4/8/8 w - - 0 1";
            var boardState = BoardState.ParseFEN(onlyKings);
            
            int phase = GamePhase.Calculate(boardState);
            
            Assert.Equal(0, phase);
        }
        
        [Fact]
        public void ShouldNotGoAboveMaxPhaseForPromotions()
        {
            const string promotedQueen = "rQbq1rk1/pp1pppbp/5np1/8/8/8/P1PPPPPP/RNBQKBNR w KQq - 0 1";
            var boardState = BoardState.ParseFEN(promotedQueen);
            
            int phase = GamePhase.Calculate(boardState);
            
            Assert.Equal(GamePhase.TotalPhase, phase);
        }
    }
}