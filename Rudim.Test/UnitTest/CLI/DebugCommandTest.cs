using Rudim.CLI.UCI;
using Xunit;

namespace Rudim.Test.UnitTest.CLI
{
    public class DebugCommandTest
    {
        [Fact]
        public void ShouldSetDebugModeOn()
        {
            var uciClient = new UciClient();
            var debugCommand = new DebugCommand(uciClient);
            var parameters = new[] { "on" };

            debugCommand.Run(parameters);

            Assert.True(uciClient.DebugMode);
        }

        [Fact]
        public void ShouldSetDebugModeOff()
        {
            var uciClient = new UciClient();
            var debugCommand = new DebugCommand(uciClient);
            var parameters = new[] { "off" };

            debugCommand.Run(parameters);

            Assert.False(uciClient.DebugMode);
        }

        [Fact]
        public void ShouldNotChangeDebugModeWithInvalidParameter()
        {
            var uciClient = new UciClient();
            var debugCommand = new DebugCommand(uciClient);
            var initialDebugMode = uciClient.DebugMode;
            var parameters = new[] { "invalid" };

            debugCommand.Run(parameters);

            Assert.Equal(initialDebugMode, uciClient.DebugMode);
        }
    }
}