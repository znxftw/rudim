use super::ACC_SIZE;
use super::loader::Network;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Accumulator {
    pub state: [i16; ACC_SIZE],
}

impl Accumulator {
    pub fn new() -> Self {
        Self {
            state: [0; ACC_SIZE],
        }
    }

    #[inline(always)]
    pub fn init_with_biases(&mut self, network: &Network) {
        self.state.copy_from_slice(&network.transformer_biases);
    }

    #[inline(always)]
    pub fn update_1_1(&mut self, add_idx: usize, rem_idx: usize, network: &Network) {
        let add_start = add_idx * ACC_SIZE;
        let rem_start = rem_idx * ACC_SIZE;
        let add_weights: &[i16; ACC_SIZE] = network.transformer_weights[add_start..add_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let rem_weights: &[i16; ACC_SIZE] = network.transformer_weights[rem_start..rem_start + ACC_SIZE]
            .try_into()
            .unwrap();

        for i in 0..ACC_SIZE {
            self.state[i] += add_weights[i] - rem_weights[i];
        }
    }

    #[inline(always)]
    pub fn update_1_2(
        &mut self,
        add_idx: usize,
        rem_idx1: usize,
        rem_idx2: usize,
        network: &Network,
    ) {
        let add_start = add_idx * ACC_SIZE;
        let rem1_start = rem_idx1 * ACC_SIZE;
        let rem2_start = rem_idx2 * ACC_SIZE;
        let add_weights: &[i16; ACC_SIZE] = network.transformer_weights[add_start..add_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let rem1_weights: &[i16; ACC_SIZE] = network.transformer_weights[rem1_start..rem1_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let rem2_weights: &[i16; ACC_SIZE] = network.transformer_weights[rem2_start..rem2_start + ACC_SIZE]
            .try_into()
            .unwrap();

        for i in 0..ACC_SIZE {
            self.state[i] += add_weights[i] - rem1_weights[i] - rem2_weights[i];
        }
    }

    #[inline(always)]
    pub fn update_2_1(
        &mut self,
        add_idx1: usize,
        add_idx2: usize,
        rem_idx: usize,
        network: &Network,
    ) {
        let add1_start = add_idx1 * ACC_SIZE;
        let add2_start = add_idx2 * ACC_SIZE;
        let rem_start = rem_idx * ACC_SIZE;
        let add1_weights: &[i16; ACC_SIZE] = network.transformer_weights[add1_start..add1_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let add2_weights: &[i16; ACC_SIZE] = network.transformer_weights[add2_start..add2_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let rem_weights: &[i16; ACC_SIZE] = network.transformer_weights[rem_start..rem_start + ACC_SIZE]
            .try_into()
            .unwrap();

        for i in 0..ACC_SIZE {
            self.state[i] += add1_weights[i] + add2_weights[i] - rem_weights[i];
        }
    }

    #[inline(always)]
    pub fn update_2_2(
        &mut self,
        add_idx1: usize,
        add_idx2: usize,
        rem_idx1: usize,
        rem_idx2: usize,
        network: &Network,
    ) {
        let add1_start = add_idx1 * ACC_SIZE;
        let add2_start = add_idx2 * ACC_SIZE;
        let rem1_start = rem_idx1 * ACC_SIZE;
        let rem2_start = rem_idx2 * ACC_SIZE;
        let add1_weights: &[i16; ACC_SIZE] = network.transformer_weights[add1_start..add1_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let add2_weights: &[i16; ACC_SIZE] = network.transformer_weights[add2_start..add2_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let rem1_weights: &[i16; ACC_SIZE] = network.transformer_weights[rem1_start..rem1_start + ACC_SIZE]
            .try_into()
            .unwrap();
        let rem2_weights: &[i16; ACC_SIZE] = network.transformer_weights[rem2_start..rem2_start + ACC_SIZE]
            .try_into()
            .unwrap();

        for i in 0..ACC_SIZE {
            self.state[i] += add1_weights[i] + add2_weights[i] - rem1_weights[i] - rem2_weights[i];
        }
    }

    #[inline(always)]
    pub fn add_feature(&mut self, feature_idx: usize, network: &Network) {
        let start = feature_idx * ACC_SIZE;
        let weights = &network.transformer_weights[start..start + ACC_SIZE];
        for (i, item) in weights.iter().enumerate().take(ACC_SIZE) {
            self.state[i] += *item;
        }
    }

    #[inline(always)]
    pub fn remove_feature(&mut self, feature_idx: usize, network: &Network) {
        let start = feature_idx * ACC_SIZE;
        let weights = &network.transformer_weights[start..start + ACC_SIZE];
        for (i, item) in weights.iter().enumerate().take(ACC_SIZE) {
            self.state[i] -= *item;
        }
    }
}

impl Default for Accumulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::BoardState;
    use crate::common::side::Side;

    #[test]
    fn test_accumulator_new() {
        let acc = Accumulator::new();
        assert_eq!(acc.state, [0i16; ACC_SIZE]);
    }

    #[test]
    fn test_accumulator_incremental_updates() {
        // Construct mock network in memory
        let mut network = Network::new_boxed();
        network.transformer_biases.fill(10);
        // Fill row 5 with 1s
        for i in 0..ACC_SIZE {
            network.transformer_weights[5 * ACC_SIZE + i] = 1;
        }

        let mut acc = Accumulator::new();
        acc.init_with_biases(&network);
        assert_eq!(acc.state, [10i16; ACC_SIZE]);

        acc.add_feature(5, &network);
        assert_eq!(acc.state, [11i16; ACC_SIZE]);

        acc.remove_feature(5, &network);
        assert_eq!(acc.state, [10i16; ACC_SIZE]);
    }

    #[test]
    fn test_accumulator_refresh_starting_position() {
        let mut network = Network::new_boxed();
        network.transformer_biases.fill(5);
        network.transformer_weights.fill(1);

        let mut board = BoardState::default();
        board.refresh_accumulator(Side::White, &network);

        assert_ne!(board.accumulator_white.state[0], 5);
        assert_ne!(board.accumulator_white.state[0], 0);
    }
}
