namespace Rudim.Common
{
    /* 
     * This class is to generate fast random numbers using 
     * https://en.wikipedia.org/wiki/Xorshift
     */
    internal static class Random
    {
        // Arbitrary starting seed
        private static ulong _state = 1804289383;

        public static ulong NextULong()
        {
            ulong RandomNumber = _state;
            RandomNumber ^= RandomNumber << 13;
            RandomNumber ^= RandomNumber >> 7;
            RandomNumber ^= RandomNumber << 17;
            return _state = RandomNumber;
        }
    }
}
