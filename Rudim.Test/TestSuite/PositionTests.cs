using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading;
using Rudim.Board;
using Xunit;
using Xunit.Abstractions;

namespace Rudim.Test.TestSuite;

[Collection("StateRace")]
public class PositionTests(ITestOutputHelper testOutputHelper)
{
  [Fact]
  public void TestPositions()
  {
    // via https://www.arasanchess.org/
    var epdLines = File.ReadAllLines("TestSuite/test_positions.epd");
    int score = 0, total = 0;

    foreach (var epdLine in epdLines)
    {
      string epd = epdLine, fen = "";
      if (string.IsNullOrWhiteSpace(epd)) continue;

      var bestMoves = ParseEpd(epd, ref fen);
      if (bestMoves == null || !bestMoves.Any()) continue;

      total++;
      var board = BoardState.ParseFEN(fen.Trim());

      var bestMove = board.FindBestMove(50, new CancellationTokenSource(TimeSpan.FromSeconds(1)).Token);
      var moveStr = bestMove.ToSan(board);

      if (bestMoves.Contains(moveStr))
      {
        score++;
      }
    }

    // Calculate percentage score out of 100
    var finalScore = (score * 100.0) / total;
    testOutputHelper.WriteLine($"Score: {finalScore}/100 ({score}/{total} positions solved)");
    Assert.True(finalScore > 0);
  }

  private string[] ParseEpd(string epd, ref string fen)
  {
    for (var i = 0; i < 4; ++i)
    {
      (var fenPart, epd) = SplitAtNextSpace(epd);
      fen += fenPart;
    }

    var operations = epd.Split(';')
      .Select(s => s.Trim())
      .Where(s => !string.IsNullOrEmpty(s))
      .ToDictionary(
        s => s.Split(' ')[0].Trim(),
        s => string.Join(" ", s.Split(' ').Skip(1)).Trim()
      );

    var bestMoves = operations.GetValueOrDefault("bm")?.Split(' ');
    return bestMoves;
  }

  private (string, string) SplitAtNextSpace(string epd)
  {
    var position = epd.IndexOf(' ');
    if (position == -1) return (null, epd);
    return (epd[..(position + 1)], epd[(position + 1)..]);
  }
}
