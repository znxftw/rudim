using System;

namespace Rudim.Common
{
    [Flags]
    public enum Castle
    {
        None = 0,
        WhiteShort = 1,
        WhiteLong = 2,
        BlackShort = 4,
        BlackLong = 8
    }
}