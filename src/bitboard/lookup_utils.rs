use crate::common::constants::SQUARES;

pub(crate) const fn mask_bits_for_masks(masks: [u64; SQUARES]) -> [u32; SQUARES] {
    let mut bits = [0u32; SQUARES];
    let mut sq = 0;

    while sq < SQUARES {
        bits[sq] = masks[sq].count_ones();
        sq += 1;
    }

    bits
}

pub(crate) const fn occupancy_mapping(index: usize, bits: u32, mask: u64) -> u64 {
    let mut occupancy = 0u64;
    let mut temporary_mask = mask;
    let mut count = 0;

    while count < bits {
        let square = temporary_mask.trailing_zeros() as usize;
        temporary_mask &= !(1u64 << square);

        if (index & (1usize << count)) != 0 {
            occupancy |= 1u64 << square;
        }

        count += 1;
    }

    occupancy
}

pub(crate) const fn magic_index(occupancy: u64, magic: u64, bits: u32) -> usize {
    occupancy.wrapping_mul(magic).wrapping_shr(64 - bits) as usize
}

pub(crate) const fn ray_mask_without_edges(
    start_rank: i32,
    start_file: i32,
    rank_delta: i32,
    file_delta: i32,
) -> u64 {
    let mut result = 0u64;
    let mut rank = start_rank + rank_delta;
    let mut file = start_file + file_delta;

    while is_on_board(rank, file) && is_on_board(rank + rank_delta, file + file_delta) {
        result = set_square(result, rank, file);
        rank += rank_delta;
        file += file_delta;
    }

    result
}

pub(crate) const fn ray_attacks(
    start_rank: i32,
    start_file: i32,
    rank_delta: i32,
    file_delta: i32,
    occupancy: u64,
) -> u64 {
    let mut result = 0u64;
    let mut rank = start_rank + rank_delta;
    let mut file = start_file + file_delta;

    while is_on_board(rank, file) {
        result = set_square(result, rank, file);
        if square_is_occupied(rank, file, occupancy) {
            break;
        }
        rank += rank_delta;
        file += file_delta;
    }

    result
}

const fn is_on_board(rank: i32, file: i32) -> bool {
    rank >= 0 && rank < 8 && file >= 0 && file < 8
}

const fn set_square(board: u64, rank: i32, file: i32) -> u64 {
    board | (1u64 << ((rank * 8 + file) as usize))
}

const fn square_is_occupied(rank: i32, file: i32, occupancy: u64) -> bool {
    (occupancy & (1u64 << ((rank * 8 + file) as usize))) != 0
}
