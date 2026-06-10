use crate::board::state::BoardState;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::eval::nnue::accumulator::Accumulator;
use crate::eval::nnue::features::get_feature_index;
use crate::eval::nnue::loader::Network;

// TODO: optz, fused updates
impl BoardState {
    #[inline(always)]
    pub fn nnue_add_piece(&mut self, square: Square, side: Side, piece: Piece) {
        self.nnue_update_piece(square, side, piece, true);
    }

    #[inline(always)]
    pub fn nnue_remove_piece(&mut self, square: Square, side: Side, piece: Piece) {
        self.nnue_update_piece(square, side, piece, false);
    }

    #[inline(always)]
    fn nnue_update_piece(&mut self, square: Square, side: Side, piece: Piece, is_add: bool) {
        let network = Network::get_embedded();
        // White accumulator update
        if let Some(feature_idx_white) = get_feature_index(piece, side, square) {
            if is_add {
                self.accumulator_white
                    .add_feature(feature_idx_white, network);
            } else {
                self.accumulator_white
                    .remove_feature(feature_idx_white, network);
            }
        }

        // Black accumulator update
        let mirrored_sq = square.mirrored();
        let relative_side = side.other();
        if let Some(feature_idx_black) = get_feature_index(piece, relative_side, mirrored_sq) {
            if is_add {
                self.accumulator_black
                    .add_feature(feature_idx_black, network);
            } else {
                self.accumulator_black
                    .remove_feature(feature_idx_black, network);
            }
        }
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
            self.accumulator_white = acc;
        } else {
            self.accumulator_black = acc;
        }
    }
}
