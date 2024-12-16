using Rudim.CLI;
using System;
using System.IO;
using Xunit;

namespace Rudim.Test.UnitTest.CLI
{
    public class InfoCommandTest
    {
        [Fact]
        public void ShouldOutputCorrectInfo()
        {
            InfoCommand infoCommand = new();
            string expectedOutput = "Rudim v1 by znxftw";

            using StringWriter sw = new();
            Console.SetOut(sw);
            infoCommand.Run([]);

            string result = sw.ToString().Trim();
            Assert.Equal(expectedOutput, result);
        }
    }
}