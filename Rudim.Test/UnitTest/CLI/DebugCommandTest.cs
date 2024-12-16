using Rudim.CLI.UCI;
using Xunit;

namespace Rudim.Test.UnitTest.CLI
{
    public class DebugCommandTest
    {
        [Fact]
        public void ShouldSetDebugModeOn()
        {
            UciClient uciClient = new UciClient();
            DebugCommand debugCommand = new DebugCommand(uciClient);
            string[] parameters = new[] { "on" };

            debugCommand.Run(parameters);

            Assert.True(uciClient.DebugMode);
        }

        [Fact]
        public void ShouldSetDebugModeOff()
        {
            UciClient uciClient = new UciClient();
            DebugCommand debugCommand = new DebugCommand(uciClient);
            string[] parameters = new[] { "off" };

            debugCommand.Run(parameters);

            Assert.False(uciClient.DebugMode);
        }

        [Fact]
        public void ShouldNotChangeDebugModeWithInvalidParameter()
        {
            UciClient uciClient = new UciClient();
            DebugCommand debugCommand = new DebugCommand(uciClient);
            bool initialDebugMode = uciClient.DebugMode;
            string[] parameters = new[] { "invalid" };

            debugCommand.Run(parameters);

            Assert.Equal(initialDebugMode, uciClient.DebugMode);
        }
    }
}