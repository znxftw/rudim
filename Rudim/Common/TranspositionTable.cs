using System;

namespace Rudim.Common
{
    public static class TranspositionTable
    {
        // TODO : Calculate this based on a constant and in MiB, not hard numbers
        private const int Capacity = 4096 * 64;
        private static readonly TranspositionTableEntry[] Entries;
        public static bool UseTranspositionTable = true;

        static TranspositionTable()
        {
            Entries = new TranspositionTableEntry[Capacity];
        }

        public static void ClearTable()
        {
            Array.Clear(Entries);
        }

        public static (bool, int, Move) GetEntry(ulong hash, int alpha, int beta, int depth)
        {
            if (!UseTranspositionTable)
                return (false, 0, null);

            TranspositionTableEntry entry = Entries[hash & (Capacity - 1)];

            if (entry == null)
                return (false, 0, null);
            if (entry.Hash != hash)
                return (false, 0, null);
            if (entry.Depth < depth)
                return (false, 0, null);

            switch (entry.Type)
            {
                case TranspositionEntryType.Exact:
                    return (true, entry.Score, entry.BestMove);
                case TranspositionEntryType.Alpha:
                    if(entry.Score <= alpha) return (true, alpha, entry.BestMove);
                    break;
                case TranspositionEntryType.Beta:
                    if(entry.Score >= beta) return (true, beta, entry.BestMove);
                    break;
            }

            return (false, 0, null);
        }

        public static void SubmitEntry(ulong hash, int score, int depth, Move bestMove, TranspositionEntryType entryType)
        {
            if (!UseTranspositionTable)
                return;

            var index = hash & (Capacity - 1);
            if (Entries[index]?.Depth >= depth)
                return;
            Entries[index] = new TranspositionTableEntry { Hash = hash, Score = score, Depth = depth, BestMove = bestMove, Type = entryType };
        }
    }

    public class TranspositionTableEntry
    {
        public int Score { get; init; }
        public ulong Hash { get; init; }
        public int Depth { get; init; }
        public Move BestMove { get; init; }
        public TranspositionEntryType Type { get; init; }
    }

    public enum TranspositionEntryType
    {
        Exact,
        Alpha,
        Beta
    }
}
