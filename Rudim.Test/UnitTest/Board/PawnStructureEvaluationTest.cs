using Rudim.Board;
using Xunit;

namespace Rudim.Test.UnitTest.Board
{
    public class PawnStructureEvaluationTest
    {
        // Returns score from the perspective of the side to move (as PieceSquareTableEvaluation does),
        // but here we test PawnStructureEvaluation directly (score is from white's perspective).

        [Fact]
        public void ShouldScoreZeroForPositionWithNoPawns()
        {
            // Only kings, no pawns
            BoardState boardState = BoardState.ParseFEN("8/8/8/8/8/8/8/K6k w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(0, score);
        }

        [Fact]
        public void ShouldScoreZeroForSymmetricPawnStructure()
        {
            // Pawns mirrored – doubled, isolated, and passed features cancel out
            BoardState boardState = BoardState.ParseFEN("8/4p3/8/8/8/8/4P3/8 w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(0, score);
        }

        [Fact]
        public void ShouldPenaliseWhiteDoubledPawns()
        {
            // White has two pawns on the e-file; black has none
            // Doubled penalty: -10
            // Isolated penalty: both on file e with no neighbours, 2 x -20 = -40
            // Passed bonus: both passed (no black pawns), e5 row=3 +50, e4 row=4 +30 => +80
            // Total: -10 - 40 + 80 = 30
            BoardState boardState = BoardState.ParseFEN("8/8/8/4P3/4P3/8/8/K6k w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(30, score);
        }

        [Fact]
        public void ShouldPenaliseBlackDoubledPawns()
        {
            // Mirror of the doubled-pawn test: black pays the penalty, score is positive for white.
            // Black pawns on e5 and e4 are equivalent to white pawns on e5 and e4 from white's test.
            BoardState boardState = BoardState.ParseFEN("K6k/8/8/4p3/4p3/8/8/8 w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(-30, score);
        }

        [Fact]
        public void ShouldPenaliseWhiteIsolatedPawn()
        {
            // White pawn on e4, black pawn on e5 – blocks the white passed pawn.
            // White e4 is isolated (no d or f pawns) => -20
            // Black e5 is isolated => +20
            // Passed: white e4 blocked by black e5 => 0; black e5 blocked by white e4 => 0
            // Total: 0
            BoardState boardState = BoardState.ParseFEN("8/8/8/4p3/4P3/8/8/K6k w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(0, score);
        }

        [Fact]
        public void ShouldBonusWhitePassedPawn()
        {
            // White pawn on e5 (row 3), no black pawns at all => passed
            // Isolated: -20, Passed: +50 (row 3) => net +30
            BoardState boardState = BoardState.ParseFEN("8/8/8/4P3/8/8/8/K6k w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(30, score);
        }

        [Fact]
        public void ShouldBonusBlackPassedPawn()
        {
            // Black pawn on e4 (row 4), no white pawns => passed for black
            // Black: isolated => +20 (good for white)
            // Passed: bonus index = 7 - row = 7 - 4 = 3 => PassedPawnBonus[3] = 50, score -= 50 (good for black)
            // Net: -30
            BoardState boardState = BoardState.ParseFEN("K6k/8/8/8/4p3/8/8/8 w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(-30, score);
        }

        [Fact]
        public void ShouldBlockPassedPawnWhenOpponentPawnIsOnAdjacentFile()
        {
            // White pawn on e5 – black pawn on d7 blocks via the passed-pawn mask
            // White e5: not passed (black d7 is in mask), isolated => -20
            // Black d7 (row=1, file=3): adjacent files c(2) and e(4), no black pawns there => isolated => +20
            // Passed for black: white e5 is in d7's mask (row > 1, file adjacent) => NOT passed
            // Total: 0
            BoardState boardState = BoardState.ParseFEN("8/3p4/8/4P3/8/8/8/K6k w - - 0 1");
            int score = PawnStructureEvaluation.Evaluate(boardState);
            Assert.Equal(0, score);
        }
    }
}
