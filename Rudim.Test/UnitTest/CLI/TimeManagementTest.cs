using Rudim.CLI.UCI;
using Xunit;
using Xunit.Abstractions;

namespace Rudim.Test.UnitTest.CLI
{
    public class TimeManagementTest(ITestOutputHelper testOutputHelper)
    {
        [Theory]
        [InlineData(180000, 2000)]  
        [InlineData(300000, 0)]     
        [InlineData(600000, 5000)]  
        [InlineData(60000, 0)]  
        [InlineData(30000, 0)]  
        [InlineData(15000, 100)]  
        [InlineData(30000, 100)]  
        public void ShouldManageTimeWithoutExhausting(int startingTime, int increment)
        {
            const int maxMoves = 400; // Max possible should be ~6000 moves, but we don't need to account for that much
            const int positionParseDelay = 5;
            const int networkDelay = 0; // These should be accounted for by the orchestrators? I'll add this back later if needed
            const int engineCancelDelay = 1;

            var remainingTime = startingTime;

            for (int moveNumber = 1; moveNumber <= maxMoves; moveNumber++)
            {
                var moveTime = TimeManagement.CalculateMoveTime(moveNumber, remainingTime, increment);
                testOutputHelper.WriteLine(moveTime.ToString());
                Assert.True(moveTime >= 10, $"Move {moveNumber}: Allocated time {moveTime}ms is less than minimum 10ms");

                remainingTime -= moveTime + positionParseDelay + networkDelay + engineCancelDelay;
                remainingTime += increment;
                Assert.True(remainingTime > 0, $"Move {moveNumber}: Ran out of time. Remaining: {remainingTime}ms");
            }
        }
    }
}