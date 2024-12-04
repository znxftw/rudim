namespace Rudim.Common
{
    /*
     * This class is to generate fast random numbers using
     * https://en.wikipedia.org/wiki/Xorshift
     */
    internal static class Random
    {
        // Arbitrary starting seed
        private static ulong _ulongState = 1804289383;
        private static int _intState = 1804289383;

        public static ulong NextULong()
        {
            var RandomNumber = _ulongState;
            RandomNumber ^= RandomNumber << 13;
            RandomNumber ^= RandomNumber >> 7;
            RandomNumber ^= RandomNumber << 17;
            return _ulongState = RandomNumber;
        }

        public static int NextInt()
        {
            var RandomNumber = _intState;
            RandomNumber ^= RandomNumber << 13;
            RandomNumber ^= RandomNumber >> 17;
            RandomNumber ^= RandomNumber << 5;
            return _intState = RandomNumber;
        }
    }
}