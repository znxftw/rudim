using Rudim.CLI.UCI;
using System;
using System.IO;
using Xunit;

namespace Rudim.Test.UnitTest.CLI
{
    [Collection("StateRace")]
    public class IsReadyCommandTest
    {
        [Fact]
        public void ShouldRespondWithReadyOk()
        {
            UciClient uciClient = new();
            IsReadyCommand isReadyCommand = new(uciClient);
            TextWriter originalOut = Console.Out;

            using StringWriter sw = new();
            Console.SetOut(sw);
            isReadyCommand.Run([]);
            Console.SetOut(originalOut);

            string result = sw.ToString().Trim();
            Assert.Equal("readyok", result);
        }

        [Fact]
        public void ShouldInitializeEngineWhenNotReady()
        {
            UciClient uciClient = new();
            IsReadyCommand isReadyCommand = new(uciClient);
            Global.Reset();

            Assert.False(Global.IsReady);

            TextWriter originalOut = Console.Out;
            using StringWriter sw = new();
            Console.SetOut(sw);
            isReadyCommand.Run([]);
            Console.SetOut(originalOut);

            Assert.True(Global.IsReady);
        }

        [Fact]
        public void ShouldStillRespondWithReadyOkWhenAlreadyReady()
        {
            UciClient uciClient = new();
            IsReadyCommand isReadyCommand = new(uciClient);
            Global.Reset();
            Global.SetReady();

            Assert.True(Global.IsReady);

            TextWriter originalOut = Console.Out;
            using StringWriter sw = new();
            Console.SetOut(sw);
            isReadyCommand.Run([]);
            Console.SetOut(originalOut);

            string result = sw.ToString().Trim();
            Assert.Equal("readyok", result);
            Assert.True(Global.IsReady);
        }
    }
}
