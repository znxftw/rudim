use crate::board::state::BoardState;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::eval::nnue::accumulator::Accumulator;
use crate::eval::nnue::features::get_feature_index;
use crate::eval::nnue::loader::Network;

impl BoardState {
    #[inline(always)]
    pub fn nnue_add_piece(&mut self, square: Square, side: Side, piece: Piece) {
        if let Some(w_idx) = get_feature_index(piece, side, square) {
            self.pending_adds_w[self.pending_adds as usize] = w_idx;
            let mirrored_sq = square.mirrored();
            let b_idx = get_feature_index(piece, side.other(), mirrored_sq).unwrap();
            self.pending_adds_b[self.pending_adds as usize] = b_idx;
            self.pending_adds += 1;
        }
    }

    #[inline(always)]
    pub fn nnue_remove_piece(&mut self, square: Square, side: Side, piece: Piece) {
        if let Some(w_idx) = get_feature_index(piece, side, square) {
            self.pending_dels_w[self.pending_removes as usize] = w_idx;
            let mirrored_sq = square.mirrored();
            let b_idx = get_feature_index(piece, side.other(), mirrored_sq).unwrap();
            self.pending_dels_b[self.pending_removes as usize] = b_idx;
            self.pending_removes += 1;
        }
    }

    pub fn flush_pending_updates(&mut self, target_idx: usize) {
        let network = Network::get_embedded();
        match (self.pending_adds, self.pending_removes) {
            (1, 1) => {
                self.history.accumulators[target_idx].white.add_1_sub_1(
                    self.pending_adds_w[0],
                    self.pending_dels_w[0],
                    network,
                );
                self.history.accumulators[target_idx].black.add_1_sub_1(
                    self.pending_adds_b[0],
                    self.pending_dels_b[0],
                    network,
                );
            }
            (1, 2) => {
                self.history.accumulators[target_idx].white.add_1_sub_2(
                    self.pending_adds_w[0],
                    self.pending_dels_w[0],
                    self.pending_dels_w[1],
                    network,
                );
                self.history.accumulators[target_idx].black.add_1_sub_2(
                    self.pending_adds_b[0],
                    self.pending_dels_b[0],
                    self.pending_dels_b[1],
                    network,
                );
            }
            // TODO: worth it for a add_2_sub_2 for castling?
            _ => {
                for i in 0..self.pending_adds as usize {
                    self.history.accumulators[target_idx]
                        .white
                        .add_feature(self.pending_adds_w[i], network);
                    self.history.accumulators[target_idx]
                        .black
                        .add_feature(self.pending_adds_b[i], network);
                }
                for i in 0..self.pending_removes as usize {
                    self.history.accumulators[target_idx]
                        .white
                        .remove_feature(self.pending_dels_w[i], network);
                    self.history.accumulators[target_idx]
                        .black
                        .remove_feature(self.pending_dels_b[i], network);
                }
            }
        }
        self.pending_adds = 0;
        self.pending_removes = 0;
    }

    pub fn refresh_accumulator(&mut self, side: Side, network: &Network) {
        let mut acc = Accumulator::new();
        acc.init_with_biases(network);

        for &p_side in &[Side::White, Side::Black] {
            let relative_side = if p_side == side {
                Side::White
            } else {
                Side::Black
            };

            for &piece in &Piece::ALL {
                if piece == Piece::None {
                    continue;
                }
                let mut pieces_bb = self.get_pieces(p_side, piece);
                while pieces_bb.is_not_empty() {
                    let sq_raw = pieces_bb.get_lsb() as usize;
                    pieces_bb.clear_lsb();

                    let sq = Square::from(sq_raw);
                    let sq = if side == Side::White {
                        sq
                    } else {
                        sq.mirrored()
                    };

                    if let Some(feature_idx) = get_feature_index(piece, relative_side, sq) {
                        acc.add_feature(feature_idx, network);
                    }
                }
            }
        }

        if side == Side::White {
            self.history.accumulators[self.history.index].white = acc;
        } else {
            self.history.accumulators[self.history.index].black = acc;
        }
    }
}
