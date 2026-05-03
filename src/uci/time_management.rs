use crate::common::constants;

pub fn calculate_move_time(clock: i32, increment: i32) -> i32 {
    let move_time = clock / 20 + increment / 2;
    std::cmp::max(
        10,
        std::cmp::min(move_time, clock - constants::BUFFER_TIME as i32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_manage_time_without_exhausting(starting_time: i32, increment: i32) {
        let max_moves = if increment > 0 { 400 } else { 75 };
        let position_parse_delay = 5;
        let network_delay = 20;
        let engine_cancel_delay = 1;

        let mut remaining_time = starting_time;

        for _move_number in 1..=max_moves {
            let move_time = calculate_move_time(remaining_time, increment);
            assert!(
                move_time >= 10,
                "Allocated time {move_time}ms is less than minimum 10ms"
            );

            remaining_time -=
                move_time + position_parse_delay + network_delay + engine_cancel_delay;
            remaining_time += increment;
            assert!(
                remaining_time > 0,
                "Ran out of time. Remaining: {remaining_time}ms"
            );
        }
    }

    macro_rules! time_case {
        ($name:ident, $starting_time:expr, $increment:expr) => {
            #[test]
            fn $name() {
                assert_manage_time_without_exhausting($starting_time, $increment);
            }
        };
    }

    time_case!(case_180000_2000, 180000, 2000);
    time_case!(case_300000_0, 300000, 0);
    time_case!(case_600000_5000, 600000, 5000);
    time_case!(case_60000_0, 60000, 0);
    time_case!(case_30000_0, 30000, 0);
    time_case!(case_15000_100, 15000, 100);
    time_case!(case_30000_100, 30000, 100);
    time_case!(case_10000_10000, 10000, 10000);
    time_case!(case_5000_20000, 5000, 20000);
    time_case!(case_60000_60000, 60000, 60000);
    time_case!(case_0_10000, 0, 10000);
}
