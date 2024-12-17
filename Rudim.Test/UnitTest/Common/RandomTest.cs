using Rudim.Common;
using System.Collections.Generic;
using Xunit;

namespace Rudim.Test.UnitTest.Common
{
    // This test being run before other tests causes a different expected output.
    // Currently this is not a problem, just that this test always needs to be run, or values in 
    // Traversal Test will need to be changed.
    public class RandomTest
    {
        [Fact]
        public void ShouldGenerateUniqueULongNumbers()
        {
            HashSet<ulong> generatedNumbers = [];
            for (int i = 0; i < 500; i++)
            {
                ulong number = Random.NextULong();
                Assert.True(generatedNumbers.Add(number), $"Collision detected for ulong number: {number}");
            }
        }

        [Fact]
        public void ShouldGenerateUniqueIntNumbers()
        {
            HashSet<int> generatedNumbers = [];
            for (int i = 0; i < 500; i++)
            {
                int number = Random.NextInt();
                Assert.True(generatedNumbers.Add(number), $"Collision detected for int number: {number}");
            }
        }
    }
}