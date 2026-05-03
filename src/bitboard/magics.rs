use crate::bitboard::attacks::{
    add_square_to_board_and_stop_at_occupied_square, get_bishop_attacks, get_rook_attacks,
};
use crate::bitboard::Bitboard;
use crate::common::constants::{MAX_MASK_INDEX, MAX_RETRY_COUNT};
use crate::common::random;
use crate::common::square::Square;

// Precalculated by generate_all_magic_numbers()
pub const BISHOP_MAGICS: [u64; 64] = [
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
    9297788375727620608,
];

pub const ROOK_MAGICS: [u64; 64] = [
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
    1157930883880079490,
];

pub fn get_bishop_mask(square: Square) -> Bitboard {
    let mut result_board = 0u64;
    let occupancy_board = Bitboard(0);
    let sq = square as i32;
    let bishop_rank = sq >> 3;
    let bishop_file = sq & (8 - 1);

    for (rank, file) in ((bishop_rank + 1)..7).zip((bishop_file + 1)..7) {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rank,
            file,
            occupancy_board,
        ) {
            break;
        }
    }

    for (rank, file) in (1..bishop_rank).rev().zip((bishop_file + 1)..7) {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rank,
            file,
            occupancy_board,
        ) {
            break;
        }
    }

    for (rank, file) in (1..bishop_rank).rev().zip((1..bishop_file).rev()) {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rank,
            file,
            occupancy_board,
        ) {
            break;
        }
    }

    for (rank, file) in ((bishop_rank + 1)..7).zip((1..bishop_file).rev()) {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rank,
            file,
            occupancy_board,
        ) {
            break;
        }
    }

    Bitboard(result_board)
}

pub fn get_rook_mask(square: Square) -> Bitboard {
    let mut result_board = 0u64;
    let occupancy_board = Bitboard(0);
    let sq = square as i32;
    let rook_rank = sq >> 3;
    let rook_file = sq & (8 - 1);

    for rank in (rook_rank + 1)..7 {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rank,
            rook_file,
            occupancy_board,
        ) {
            break;
        }
    }

    for rank in (1..rook_rank).rev() {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rank,
            rook_file,
            occupancy_board,
        ) {
            break;
        }
    }

    for file in (rook_file + 1)..7 {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rook_rank,
            file,
            occupancy_board,
        ) {
            break;
        }
    }

    for file in (1..rook_file).rev() {
        if add_square_to_board_and_stop_at_occupied_square(
            &mut result_board,
            rook_rank,
            file,
            occupancy_board,
        ) {
            break;
        }
    }

    Bitboard(result_board)
}

pub fn get_occupancy_mapping(index: usize, n_bits_in_mask: i32, mask: Bitboard) -> Bitboard {
    let mut occupancy_mapping = Bitboard(0);
    let mut temporary_mask = mask;

    for count in 0..n_bits_in_mask {
        let square = temporary_mask.get_lsb() as usize;
        temporary_mask.clear_bit(square);

        if (index & (1 << count)) != 0 {
            occupancy_mapping.set_bit(square);
        }
    }

    occupancy_mapping
}

fn generate_potential_magic_number() -> u64 {
    random::next_u64() & random::next_u64() & random::next_u64()
}

pub fn find_magic_number(square: Square, bits_in_mask: i32, is_bishop: bool) -> u64 {
    let max_index = 1 << bits_in_mask;
    let mut occupancy_mappings = vec![Bitboard(0); MAX_MASK_INDEX];
    let mut attacks = vec![Bitboard(0); MAX_MASK_INDEX];
    let mask = if is_bishop {
        get_bishop_mask(square)
    } else {
        get_rook_mask(square)
    };

    for index in 0..max_index {
        occupancy_mappings[index] = get_occupancy_mapping(index, bits_in_mask, mask);
        attacks[index] = if is_bishop {
            get_bishop_attacks(square, occupancy_mappings[index])
        } else {
            get_rook_attacks(square, occupancy_mappings[index])
        };
    }

    for _ in 0..MAX_RETRY_COUNT {
        let potential_magic_number = generate_potential_magic_number();

        // Early exit impossible magics
        if ((mask.0.wrapping_mul(potential_magic_number)) & 0xFF00_0000_0000_0000).count_ones() < 6
        {
            continue;
        }

        let mut magic_attacks = vec![Bitboard(0xFFFF_FFFF_FFFF_FFFF); MAX_MASK_INDEX];
        let mut failure_flag = false;

        for index in 0..max_index {
            let magic_index = ((occupancy_mappings[index]
                .0
                .wrapping_mul(potential_magic_number))
                >> (64 - bits_in_mask)) as usize;

            if magic_attacks[magic_index].0 == 0xFFFF_FFFF_FFFF_FFFF {
                magic_attacks[magic_index] = attacks[index];
            } else if magic_attacks[magic_index] != attacks[index] {
                failure_flag = true;
                break;
            }
        }

        if !failure_flag {
            return potential_magic_number;
        }
    }

    panic!("No magic number found");
}

/// Regenerates all magic numbers by brute-force search and prints them to stdout.
/// Intended to be called from `main` via a CLI flag (e.g. `--generate-magics`).
pub fn generate_all_magic_numbers() {
    println!("pub const BISHOP_MAGICS: [u64; 64] = [");
    for square in 0..64 {
        let sq = Square::from(square);
        let bits = get_bishop_mask(sq).0.count_ones() as i32;
        let magic = find_magic_number(sq, bits, true);
        println!("    {magic},");
    }
    println!("];");

    println!("pub const ROOK_MAGICS: [u64; 64] = [");
    for square in 0..64 {
        let sq = Square::from(square);
        let bits = get_rook_mask(sq).0.count_ones() as i32;
        let magic = find_magic_number(sq, bits, false);
        println!("    {magic},");
    }
    println!("]");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_mask_for_central_bishop() {
        let bishop_mask_e5 = get_bishop_mask(Square::E5);

        assert_eq!(1, bishop_mask_e5.get_bit(Square::F4 as usize));
        assert_eq!(1, bishop_mask_e5.get_bit(Square::G3 as usize));

        assert_eq!(1, bishop_mask_e5.get_bit(Square::F6 as usize));
        assert_eq!(1, bishop_mask_e5.get_bit(Square::G7 as usize));

        assert_eq!(1, bishop_mask_e5.get_bit(Square::D4 as usize));
        assert_eq!(1, bishop_mask_e5.get_bit(Square::C3 as usize));
        assert_eq!(1, bishop_mask_e5.get_bit(Square::B2 as usize));

        assert_eq!(1, bishop_mask_e5.get_bit(Square::D6 as usize));
        assert_eq!(1, bishop_mask_e5.get_bit(Square::C7 as usize));

        assert_eq!(9, bishop_mask_e5.0.count_ones());
    }

    #[test]
    fn should_get_mask_for_corner_bishop() {
        let bishop_mask_a1 = get_bishop_mask(Square::A1);

        assert_eq!(1, bishop_mask_a1.get_bit(Square::B2 as usize));
        assert_eq!(1, bishop_mask_a1.get_bit(Square::C3 as usize));
        assert_eq!(1, bishop_mask_a1.get_bit(Square::D4 as usize));
        assert_eq!(1, bishop_mask_a1.get_bit(Square::E5 as usize));
        assert_eq!(1, bishop_mask_a1.get_bit(Square::F6 as usize));
        assert_eq!(1, bishop_mask_a1.get_bit(Square::G7 as usize));
        assert_eq!(6, bishop_mask_a1.0.count_ones());
    }

    #[test]
    fn should_get_mask_for_central_rook() {
        let rook_mask_e5 = get_rook_mask(Square::E5);

        assert_eq!(1, rook_mask_e5.get_bit(Square::E2 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::E3 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::E4 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::E6 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::E7 as usize));

        assert_eq!(1, rook_mask_e5.get_bit(Square::B5 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::C5 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::D5 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::F5 as usize));
        assert_eq!(1, rook_mask_e5.get_bit(Square::G5 as usize));

        assert_eq!(10, rook_mask_e5.0.count_ones());
    }

    #[test]
    fn should_get_mask_for_corner_rook() {
        let rook_mask_a1 = get_rook_mask(Square::A1);

        assert_eq!(1, rook_mask_a1.get_bit(Square::A2 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::A3 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::A4 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::A5 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::A6 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::A7 as usize));

        assert_eq!(1, rook_mask_a1.get_bit(Square::B1 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::C1 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::D1 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::E1 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::F1 as usize));
        assert_eq!(1, rook_mask_a1.get_bit(Square::G1 as usize));

        assert_eq!(12, rook_mask_a1.0.count_ones());
    }

    #[test]
    fn should_get_occupancy_mapping_for_bishop() {
        let mask = get_bishop_mask(Square::E5);
        let index = 0b100100100;
        let bits_in_mask = mask.0.count_ones() as i32;
        let occupancy_mapping = get_occupancy_mapping(index, bits_in_mask, mask);

        assert_eq!(1, occupancy_mapping.get_bit(Square::D6 as usize));
        assert_eq!(1, occupancy_mapping.get_bit(Square::F4 as usize));
        assert_eq!(1, occupancy_mapping.get_bit(Square::B2 as usize));

        assert_eq!(3, occupancy_mapping.0.count_ones());
    }

    #[test]
    fn should_get_occupancy_mapping_for_rook() {
        let mask = get_rook_mask(Square::E5);
        let index = 0b0100100100;
        let bits_in_mask = mask.0.count_ones() as i32;
        let occupancy_mapping = get_occupancy_mapping(index, bits_in_mask, mask);

        assert_eq!(1, occupancy_mapping.get_bit(Square::E3 as usize));
        assert_eq!(1, occupancy_mapping.get_bit(Square::F5 as usize));
        assert_eq!(1, occupancy_mapping.get_bit(Square::B5 as usize));

        assert_eq!(3, occupancy_mapping.0.count_ones());
    }
}
