use crate::common::constants;

pub fn calculate_move_time(clock: i32, increment: i32) -> i32 {
    let move_time = clock / 20 + increment / 2;
    std::cmp::max(
        10,
        std::cmp::min(move_time, clock - constants::BUFFER_TIME as i32),
    )
}
