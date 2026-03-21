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
        [InlineData(10000, 10000)]  // 10s+10s
        [InlineData(5000, 20000)]   // 5s+20s (high increment)
        [InlineData(60000, 60000)]  // 60s+60s (very high increment)
        [InlineData(0, 10000)]      // 0+10s (increment-only, extreme edge case)
        public void ShouldManageTimeWithoutExhausting(int startingTime, int increment)
        {
            // Without increment, pure blitz games realistically last ~75 half-moves per player.
            // With increment, the clock can grow so we test a much longer game.
            int maxMoves = increment > 0 ? 400 : 75;
            const int positionParseDelay = 5;
            const int networkDelay = 20; // Simulate network latency; BufferTime (50ms) ensures the formula absorbs it
            const int engineCancelDelay = 1;

            int remainingTime = startingTime;

            for (int moveNumber = 1; moveNumber <= maxMoves; moveNumber++)
            {
                int moveTime = TimeManagement.CalculateMoveTime(remainingTime, increment);
                testOutputHelper.WriteLine(moveTime.ToString());
                Assert.True(moveTime >= 10, $"Move {moveNumber}: Allocated time {moveTime}ms is less than minimum 10ms");

                remainingTime -= moveTime + positionParseDelay + networkDelay + engineCancelDelay;
                remainingTime += increment;
                Assert.True(remainingTime > 0, $"Move {moveNumber}: Ran out of time. Remaining: {remainingTime}ms");
            }
        }
    }
}