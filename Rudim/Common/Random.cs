namespace Rudim.Common
{
    /* 
     * This class is to generate fast random numbers using 
     * https://en.wikipedia.org/wiki/Xorshift
     */
    class Random
    {
        // Arbitrary starting seed
        private static ulong State = 1804289383;

        public static ulong NextULong()
        {
            ulong RandomNumber = State;
            RandomNumber ^= RandomNumber << 13;
            RandomNumber ^= RandomNumber >> 7;
            RandomNumber ^= RandomNumber << 17;
            return State = RandomNumber;
        }
    }
}
