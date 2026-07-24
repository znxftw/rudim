pub mod accumulator;
pub mod features;
pub mod loader;

use crate::board::state::BoardState;
use crate::common::side::Side;

use self::loader::Network;

pub const ACC_SIZE: usize = 256;
pub const INPUT_SIZE: usize = 768;

pub const SCALE: i32 = 400;

pub fn evaluate(board: &BoardState) -> i16 {
    let network = Network::get_embedded();
    evaluate_internal(board, network)
}

pub fn evaluate_internal(board: &BoardState, network: &Network) -> i16 {
    #[cfg(target_arch = "x86_64")]
    {
        #[cfg(target_feature = "avx2")]
        {
            unsafe { evaluate_internal_avx2(board, network) }
        }
        #[cfg(not(target_feature = "avx2"))]
        {
            if std::is_x86_feature_detected!("avx2") {
                unsafe { evaluate_internal_avx2(board, network) }
            } else {
                evaluate_internal_scalar(board, network)
            }
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        evaluate_internal_scalar(board, network)
    }
}

pub fn evaluate_internal_scalar(board: &BoardState, network: &Network) -> i16 {
    let side_to_move = board.side_to_move;
    let (acc_active, acc_passive) = if side_to_move == Side::White {
        (
            &board.history.accumulators[board.history.index].white,
            &board.history.accumulators[board.history.index].black,
        )
    } else {
        (
            &board.history.accumulators[board.history.index].black,
            &board.history.accumulators[board.history.index].white,
        )
    };

    let mut output: i32 = 0;

    for (&input, &weight) in acc_active
        .state
        .iter()
        .zip(&network.output_weights[0..ACC_SIZE])
    {
        let val = i32::from(input).clamp(0, 255);
        let screlu = val * val;
        output += screlu * i32::from(weight);
    }

    for (&input, &weight) in acc_passive
        .state
        .iter()
        .zip(&network.output_weights[ACC_SIZE..2 * ACC_SIZE])
    {
        let val = i32::from(input).clamp(0, 255);
        let screlu = val * val;
        output += screlu * i32::from(weight);
    }

    // QA=255, QB=64, SCALE=400
    output /= 255;
    output += i32::from(network.output_bias);
    output *= SCALE;
    output /= 255 * 64;

    output.clamp(-29000, 29000) as i16
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn evaluate_internal_avx2(board: &BoardState, network: &Network) -> i16 {
    use std::arch::x86_64::*;

    let side_to_move = board.side_to_move;
    let (acc_active, acc_passive) = if side_to_move == Side::White {
        (
            &board.history.accumulators[board.history.index].white,
            &board.history.accumulators[board.history.index].black,
        )
    } else {
        (
            &board.history.accumulators[board.history.index].black,
            &board.history.accumulators[board.history.index].white,
        )
    };

    let mut acc_sum = _mm256_setzero_si256();
    unsafe {
        let zero = _mm256_setzero_si256();
        let max_val = _mm256_set1_epi16(255);

        // Process active accumulator
        let active_ptr = acc_active.state.as_ptr();
        let weights_active_ptr = network.output_weights[0..ACC_SIZE].as_ptr();

        for i in (0..ACC_SIZE).step_by(16) {
            let v_in = _mm256_loadu_si256(active_ptr.add(i) as *const __m256i);
            let v_w = _mm256_loadu_si256(weights_active_ptr.add(i) as *const __m256i);

            let v_clamped = _mm256_min_epi16(_mm256_max_epi16(v_in, zero), max_val);

            let v_low = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(v_clamped));
            let v_high = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(v_clamped, 1));

            let s_low = _mm256_mullo_epi32(v_low, v_low);
            let s_high = _mm256_mullo_epi32(v_high, v_high);

            let w_low = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(v_w));
            let w_high = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(v_w, 1));

            let p_low = _mm256_mullo_epi32(s_low, w_low);
            let p_high = _mm256_mullo_epi32(s_high, w_high);

            acc_sum = _mm256_add_epi32(acc_sum, p_low);
            acc_sum = _mm256_add_epi32(acc_sum, p_high);
        }

        // Process passive accumulator
        let passive_ptr = acc_passive.state.as_ptr();
        let weights_passive_ptr = network.output_weights[ACC_SIZE..2 * ACC_SIZE].as_ptr();

        for i in (0..ACC_SIZE).step_by(16) {
            let v_in = _mm256_loadu_si256(passive_ptr.add(i) as *const __m256i);
            let v_w = _mm256_loadu_si256(weights_passive_ptr.add(i) as *const __m256i);

            let v_clamped = _mm256_min_epi16(_mm256_max_epi16(v_in, zero), max_val);

            let v_low = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(v_clamped));
            let v_high = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(v_clamped, 1));

            let s_low = _mm256_mullo_epi32(v_low, v_low);
            let s_high = _mm256_mullo_epi32(v_high, v_high);

            let w_low = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(v_w));
            let w_high = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(v_w, 1));

            let p_low = _mm256_mullo_epi32(s_low, w_low);
            let p_high = _mm256_mullo_epi32(s_high, w_high);

            acc_sum = _mm256_add_epi32(acc_sum, p_low);
            acc_sum = _mm256_add_epi32(acc_sum, p_high);
        }
    }

    // Horizontal reduction of acc_sum (8 x i32)
    let sum1 = _mm256_hadd_epi32(acc_sum, acc_sum);
    let sum2 = _mm256_hadd_epi32(sum1, sum1);
    let val0 = _mm256_extract_epi32(sum2, 0);
    let val4 = _mm256_extract_epi32(sum2, 4);
    let mut output = val0 + val4;

    // QA=255, QB=64, SCALE=400
    output /= 255;
    output += i32::from(network.output_bias);
    output *= SCALE;
    output /= 255 * 64;

    output.clamp(-29000, 29000) as i16
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::BoardState;

    #[test]
    fn test_nnue_forward_pass_mathematical_correctness() {
        let mut network = Network::new_boxed();
        network.output_bias = 10;
        for i in 0..ACC_SIZE {
            network.output_weights[i] = 2;
        }
        for i in ACC_SIZE..2 * ACC_SIZE {
            network.output_weights[i] = 3;
        }

        let mut board = BoardState::new();

        let idx = board.history.index;
        board.history.accumulators[idx].white.state.fill(10);
        board.history.accumulators[idx].black.state.fill(20);

        board.side_to_move = Side::White;
        let score = evaluate_internal(&board, &network);

        // Active state value: 10.clamp(0, 255) = 10. screlu = 10 * 10 = 100.
        // Passive state value: 20.clamp(0, 255) = 20. screlu = 20 * 20 = 400.
        // sum = 256 * (100 * 2) + 256 * (400 * 3) = 51200 + 307200 = 358400
        // Dequantize:
        // output = 358400 / 255 = 1405
        // output += 10 (bias) = 1415
        // output *= 400 (SCALE) = 566000
        // output /= 16320 (QA * QB) = 34
        assert_eq!(score, 34);
    }

    #[test]
    fn test_nnue_avx2_matches_scalar() {
        #[cfg(target_arch = "x86_64")]
        {
            if std::is_x86_feature_detected!("avx2") {
                let mut network = Network::new_boxed();
                network.randomize();

                let mut board = BoardState::new();
                let idx = board.history.index;

                // Fill with positive, negative, and out-of-range values
                for i in 0..ACC_SIZE {
                    board.history.accumulators[idx].white.state[i] =
                        ((i as i16) * 17 - 100).clamp(-500, 500);
                    board.history.accumulators[idx].black.state[i] =
                        ((i as i16) * -13 + 50).clamp(-500, 500);
                }

                board.side_to_move = Side::White;
                let scalar_score = evaluate_internal_scalar(&board, &network);
                let avx2_score = unsafe { evaluate_internal_avx2(&board, &network) };
                assert_eq!(scalar_score, avx2_score, "White side AVX2 score mismatch");

                board.side_to_move = Side::Black;
                let scalar_score_b = evaluate_internal_scalar(&board, &network);
                let avx2_score_b = unsafe { evaluate_internal_avx2(&board, &network) };
                assert_eq!(scalar_score_b, avx2_score_b, "Black side AVX2 score mismatch");
            }
        }
    }
}

