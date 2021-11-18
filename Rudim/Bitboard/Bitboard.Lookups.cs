using System.Numerics;
using Rudim.Common;

namespace Rudim
{
    public partial class Bitboard
    {
        private static readonly ulong FileA = 72340172838076673;
        private static readonly ulong FileB = 144680345676153346;
        private static readonly ulong FileG = 4629771061636907072;
        private static readonly ulong FileH = 9259542123273814144;
        private static readonly ulong FileAB = FileA | FileB;
        private static readonly ulong FileGH = FileG | FileH;

        public static readonly ulong[,] PawnAttacks = new ulong[Constants.Sides, Constants.Squares];
        public static readonly ulong[] KnightAttacks = new ulong[Constants.Squares];
        public static readonly ulong[] KingAttacks = new ulong[Constants.Squares];
        private static readonly ulong[,] BishopAttacks = new ulong[Constants.Squares, Constants.MaxBishopMask];
        private static readonly ulong[,] RookAttacks = new ulong[Constants.Squares, Constants.MaxRookMask];

        private static readonly ulong[] BishopMasks = new ulong[Constants.Squares];
        private static readonly ulong[] RookMasks = new ulong[Constants.Squares];

        public static readonly int[] BishopMaskBits = new int[Constants.Squares];
        public static readonly int[] RookMaskBits = new int[Constants.Squares];

        static Bitboard()
        {
            for (int square = 0; square < Constants.Squares; ++square)
            {
                PawnAttacks[(int)Side.White, square] = GetPawnAttacks((Square)square, Side.White).Board;
                PawnAttacks[(int)Side.Black, square] = GetPawnAttacks((Square)square, Side.Black).Board;

                KnightAttacks[square] = GetKnightAttacks((Square)square).Board;

                KingAttacks[square] = GetKingAttacks((Square)square).Board;

                BishopMasks[square] = GetBishopMask((Square)square).Board;
                BishopMaskBits[square] = BitOperations.PopCount(BishopMasks[square]);

                RookMasks[square] = GetRookMask((Square)square).Board;
                RookMaskBits[square] = BitOperations.PopCount(RookMasks[square]);

                for (var index = 0; index < (1 << BishopMaskBits[square]); ++index)
                {
                    var occupancyMapping = GetOccupancyMapping(index, BishopMaskBits[square], new Bitboard(BishopMasks[square]));
                    var magicIndex = (occupancyMapping.Board * BishopMagics[square]) >> (64 - BishopMaskBits[square]);
                    BishopAttacks[square, magicIndex] = GetBishopAttacks((Square)square, occupancyMapping).Board;
                }

                for (var index = 0; index < (1 << RookMaskBits[square]); ++index)
                {
                    var occupancyMapping = GetOccupancyMapping(index, RookMaskBits[square], new Bitboard(RookMasks[square]));
                    var magicIndex = (occupancyMapping.Board * RookMagics[square]) >> (64 - RookMaskBits[square]);
                    RookAttacks[square, magicIndex] = GetRookAttacks((Square)square, occupancyMapping).Board;
                }
            }
        }

        public static Bitboard GetBishopAttacksFromTable(Square square, Bitboard occupancy)
        {
            // TODO : Test this
            var index = occupancy.Board;
            index &= BishopMasks[(int)square];
            index *= BishopMagics[(int)square];
            index >>= 64 - BishopMaskBits[(int)square];
            return new Bitboard(BishopAttacks[(int)square, index]);
        }

        public static Bitboard GetRookAttacksFromTable(Square square, Bitboard occupancy)
        {
            var index = occupancy.Board;
            index &= RookMasks[(int)square];
            index *= RookMagics[(int)square];
            index >>= 64 - RookMaskBits[(int)square];
            return new Bitboard(RookAttacks[(int)square, index]);
        }

        public static Bitboard GetQueenAttacksFromTable(Square square, Bitboard occupancy)
        {
            return new(GetRookAttacksFromTable(square, occupancy).Board | GetBishopAttacksFromTable(square, occupancy).Board);
        }

        // Precalculated - Refer Bitboard.FindMagicNumber()
        private static readonly ulong[] BishopMagics = {
                    572335195422784,
                    9225705203045892096,
                    1155322839151150592,
                    4684944281377579073,
                    9511901755049246721,
                    72218192528801800,
                    19757142156521488,
                    1266779148001381,
                    3602951187466322196,
                    2261216596869188,
                    31596674340110466,
                    11331843878028352,
                    13979177654425755648,
                    288795559522207748,
                    721148038749358145,
                    628254355639896068,
                    2747233433184108672,
                    631631016576417856,
                    571763293683725,
                    1153485640510341152,
                    72622760764965888,
                    4973662945859898496,
                    1156440496010170372,
                    1729523414332866952,
                    1130298494980176,
                    2310349082885357840,
                    2882356539283308768,
                    579847256709136448,
                    13842658983763001344,
                    16285862876542411008,
                    4820533887766656,
                    576549817342263872,
                    13837312088550279168,
                    18159671634577480,
                    40673410086601728,
                    95912632808669696,
                    144397766927056960,
                    577613059818262592,
                    2315344997304535046,
                    4612009275386004482,
                    288388774679810052,
                    1162218983791854088,
                    4616754767635423744,
                    4899916678055348228,
                    9531886212148625920,
                    18085883961999392,
                    146376093224403072,
                    4617341907319128329,
                    1154048508456608768,
                    146509926817370115,
                    1225120471797202948,
                    547885504,
                    4648005224496177152,
                    576540344087347202,
                    614213601338625024,
                    9729235348727332879,
                    1154118875380457472,
                    4521376632410114,
                    4611686297608687616,
                    2216882865184,
                    10376399751779517444,
                    4612284170613301505,
                    2594178955930118721,
                    9297788375727620608};

        private static readonly ulong[] RookMagics = {
                    11565244117967444096,
                    594492744072699904,
                    2197769949736337536,
                    1188955249696573442,
                    72075220583973632,
                    144118555532091904,
                    288255321624023816,
                    4755803406625603628,
                    108227130696925216,
                    72690981524209728,
                    1157988191899353216,
                    2378463697367468544,
                    9235194054747365888,
                    144678174821974528,
                    4644478851874944,
                    576742228362330114,
                    18050132645789696,
                    157643854024015937,
                    150083874263040,
                    166633738116530192,
                    2450526645086390272,
                    282574622818306,
                    848823010722064,
                    1152923703902765348,
                    4644339265323016,
                    9250393773131714560,
                    6917812989404913665,
                    2308095360931725320,
                    1315059889432952962,
                    146932139022091264,
                    2201179128064,
                    1153203263051464836,
                    1008947333225783810,
                    9331493613361696768,
                    576601627306233861,
                    36169603235186688,
                    3612168685415829508,
                    151997037255066112,
                    300616580864164360,
                    36284991012996,
                    54078654820483072,
                    1170971097420611584,
                    72198615062413344,
                    9227893228802441344,
                    2342434825042001925,
                    7072883491209488,
                    1729954007285497872,
                    4620974832652189698,
                    184717955613862016,
                    1452551892478469376,
                    2305878193854317696,
                    9948460375102980224,
                    2308385084459221120,
                    9241667927523076352,
                    36046394450117632,
                    433190608774955520,
                    2310365301614608385,
                    146740276384645123,
                    288300884483508738,
                    4613374937141616706,
                    4785108963951633,
                    4648277834194814978,
                    8798274129924,
                    1157930883880079490};
    }
}
