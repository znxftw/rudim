using Rudim.Common;
using System;

namespace Rudim.CLI.UCI
{
    public static class TimeManagement
    {
        //               Move Number  0 -  10    20   30    40    50   ...
        private static double[] Ratios = { 0.1, 0.3, 0.5, 0.5, 0.3, 0.2, 0.1, 0.075, 0.05 };
        public static int CalculateMoveTime(int moveNumber, int clock, int increment)
        {
            var moveTime = clock * Ratios[Math.Min(moveNumber / 10, 6)] / 10 + increment - Constants.BufferTime;
            return moveTime < 10 ? 10 : (int)moveTime;
        }
    }
}