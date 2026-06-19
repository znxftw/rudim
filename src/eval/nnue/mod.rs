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
}
