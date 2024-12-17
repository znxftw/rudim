using Rudim.Common;
using System;

namespace Rudim.CLI.UCI
{
    public static class TimeManagement
    {
        //               Move Number  0 -  10    20   30    40    50   60+
        private static readonly double[] Ratios = [0.1, 0.2, 0.375, 0.275, 0.175, 0.1];
        public static int CalculateMoveTime(int moveNumber, int clock, int increment)
        {
            double moveTime = clock * Ratios[Math.Min(moveNumber / 20, 5)] / 10 + increment - Constants.BufferTime;
            return (int)Math.Max(moveTime, 10);
        }
    }
}