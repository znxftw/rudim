﻿using System;

namespace Chess
{
    class Program
    {
        static void Main(string[] args)
        {
            var Board = new Bitboard(128);
            Board.SetBit(15);
            Board.Print();
        }
    }
}
