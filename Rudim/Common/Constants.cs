namespace Rudim.Common
{
    public static class Constants
    {
        public const int Sides = 2;
        public const int SidesWithBoth = Sides + 1;
        public const int Squares = 64;
        public const int Pieces = 6;
        public const int MaxRetryCount = 100_000_000;
        public const int MaxBishopMask = 1 << 9;
        public const int MaxRookMask = 1 << 12;
        public const int MaxMaskIndex = MaxRookMask;
        public const int MaxCentipawnEval = 49000;
        public const int MaxPly = 64;
        public const int BufferTime = 50;
        public const int MaxSearchDepth = 64;
    }
}