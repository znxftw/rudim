using Rudim.Common;
using System;

namespace Rudim.CLI.UCI
{
    public static class TimeManagement
    {
        public static int CalculateMoveTime(int clock, int increment)
        {
            // Spread remaining clock over ~20 expected moves and add a bonus from increment
            // Cap at clock minus buffer to ensure we never timeout due to clock or network delays
            int moveTime = clock / 20 + increment / 2;
            return Math.Max(10, Math.Min(moveTime, clock - Constants.BufferTime));
        }
    }
}