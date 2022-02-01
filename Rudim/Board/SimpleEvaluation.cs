using Rudim.Common;
using System;
using System.Collections.Generic;
using System.Numerics;

namespace Rudim.Board
{
    internal class SimpleEvaluation
    {
        private static readonly IDictionary<Piece, int> PieceValues;
        private static readonly IDictionary<Piece, int[]> PositionValues;
        private static readonly int[] MidgameKingValues;
        private static readonly int[] EndgameKingValues;


        public static int Evaluate(BoardState boardState)
        {
            int score = 0;

            score += ScoreMaterial(boardState);
            score += ScorePosition(boardState);

            return score;
        }

        private static int ScorePosition(BoardState boardState)
        {
            var positionalScore = 0;
            var midgamePhase = GamePhase.Calculate(boardState);
            var endgamePhase = GamePhase.TotalPhase - midgamePhase;
            for (var piece = Piece.Pawn; piece <= Piece.King; ++piece)
            {
                var whiteBoard = new Bitboard(boardState.Pieces[(int)Side.White, (int)piece].Board);
                var blackBoard = new Bitboard(boardState.Pieces[(int)Side.Black, (int)piece].Board);

                if (piece == Piece.King)
                {
                    var whiteKing = whiteBoard.GetLsb();
                    var blackKing = MirrorSquare(blackBoard.GetLsb());

                    positionalScore += ((MidgameKingValues[whiteKing] * midgamePhase) + (EndgameKingValues[whiteKing] * endgamePhase)) / GamePhase.TotalPhase;
                    positionalScore -= ((MidgameKingValues[blackKing] * midgamePhase) + (EndgameKingValues[blackKing] * endgamePhase)) / GamePhase.TotalPhase;
                    continue;
                }

                while (whiteBoard.Board > 0)
                {
                    var square = whiteBoard.GetLsb();
                    whiteBoard.ClearBit(square);
                    positionalScore += PositionValues[piece][square];
                }

                while (blackBoard.Board > 0)
                {
                    var square = blackBoard.GetLsb();
                    blackBoard.ClearBit(square);
                    square = MirrorSquare(square);
                    positionalScore -= PositionValues[piece][square];
                }
            }
            return positionalScore;
        }

        private static int MirrorSquare(int square)
        {
            return (int)SquareExtensions.MirroredSquare[(Square)square];
        }

        private static int ScoreMaterial(BoardState boardState)
        {
            var materialScore = 0;
            for (var piece = Piece.Pawn; piece < Piece.King; ++piece)
            {
                materialScore += PieceValues[piece] * BitOperations.PopCount(boardState.Pieces[(int)Side.White, (int)piece].Board);
                materialScore -= PieceValues[piece] * BitOperations.PopCount(boardState.Pieces[(int)Side.Black, (int)piece].Board);
            }
            return materialScore;
        }

        static SimpleEvaluation()
        {
            PieceValues = new Dictionary<Piece, int>()
            {
                [Piece.Pawn] = 100,
                [Piece.Knight] = 320,
                [Piece.Bishop] = 330,
                [Piece.Rook] = 500,
                [Piece.Queen] = 900,
                [Piece.King] = 20000
            };
            PositionValues = new Dictionary<Piece, int[]>()
            {

                [Piece.Pawn] = new[]{  0,  0,  0,  0,  0,  0,  0,  0,
                                                            50, 50, 50, 50, 50, 50, 50, 50,
                                                            10, 10, 20, 30, 30, 20, 10, 10,
                                                            5,  5, 10, 25, 25, 10,  5,  5,
                                                            0,  0,  0, 20, 20,  0,  0,  0,
                                                            5, -5,-10,  0,  0,-10, -5,  5,
                                                            5, 10, 10,-20,-20, 10, 10,  5,
                                                            0,  0,  0,  0,  0,  0,  0,  0},
                [Piece.Knight] = new[]{-50,-40,-30,-30,-30,-30,-40,-50,
                                                            -40,-20,  0,  0,  0,  0,-20,-40,
                                                            -30,  0, 10, 15, 15, 10,  0,-30,
                                                            -30,  5, 15, 20, 20, 15,  5,-30,
                                                            -30,  0, 15, 20, 20, 15,  0,-30,
                                                            -30,  5, 10, 15, 15, 10,  5,-30,
                                                            -40,-20,  0,  5,  5,  0,-20,-40,
                                                            -50,-40,-30,-30,-30,-30,-40,-50},
                [Piece.Bishop] = new[]{-20,-10,-10,-10,-10,-10,-10,-20,
                                                            -10,  0,  0,  0,  0,  0,  0,-10,
                                                            -10,  0,  5, 10, 10,  5,  0,-10,
                                                            -10,  5,  5, 10, 10,  5,  5,-10,
                                                            -10,  0, 10, 10, 10, 10,  0,-10,
                                                            -10, 10, 10, 10, 10, 10, 10,-10,
                                                            -10,  5,  0,  0,  0,  0,  5,-10,
                                                            -20,-10,-10,-10,-10,-10,-10,-20},
                [Piece.Rook] = new[]{   0,  0,  0,  0,  0,  0,  0,  0,
                                                             5, 10, 10, 10, 10, 10, 10,  5,
                                                            -5,  0,  0,  0,  0,  0,  0, -5,
                                                            -5,  0,  0,  0,  0,  0,  0, -5,
                                                            -5,  0,  0,  0,  0,  0,  0, -5,
                                                            -5,  0,  0,  0,  0,  0,  0, -5,
                                                            -5,  0,  0,  0,  0,  0,  0, -5,
                                                             0,  0,  0,  5,  5,  0,  0,  0},
                [Piece.Queen] = new[]{ -20,-10,-10, -5, -5,-10,-10,-20,
                                                            -10,  0,  0,  0,  0,  0,  0,-10,
                                                            -10,  0,  5,  5,  5,  5,  0,-10,
                                                             -5,  0,  5,  5,  5,  5,  0, -5,
                                                              0,  0,  5,  5,  5,  5,  0, -5,
                                                            -10,  5,  5,  5,  5,  5,  0,-10,
                                                            -10,  0,  5,  0,  0,  0,  0,-10,
                                                            -20,-10,-10, -5, -5,-10,-10,-20}
            };
            MidgameKingValues = new[]
            {
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -20,-30,-30,-40,-40,-30,-30,-20,
                -10,-20,-20,-20,-20,-20,-20,-10,
                20, 20,  0,  0,  0,  0, 20, 20,
                20, 30, 10,  0,  0, 10, 30, 20
            };
            EndgameKingValues = new[]
            {
                -50,-40,-30,-20,-20,-30,-40,-50,
                -30,-20,-10,  0,  0,-10,-20,-30,
                -30,-10, 20, 30, 30, 20,-10,-30,
                -30,-10, 30, 40, 40, 30,-10,-30,
                -30,-10, 30, 40, 40, 30,-10,-30,
                -30,-10, 20, 30, 30, 20,-10,-30,
                -30,-30,  0,  0,  0,  0,-30,-30,
                -50,-30,-30,-30,-30,-30,-30,-50
            };
        }
    }
}
