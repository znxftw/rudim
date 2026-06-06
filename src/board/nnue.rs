use crate::board::state::BoardState;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::eval::nnue::GLOBAL_NETWORK;
use crate::eval::nnue::accumulator::Accumulator;
use crate::eval::nnue::features::get_feature_index;
use crate::eval::nnue::loader::Network;

// TODO: optz, fused updates
#[inline(always)]
fn get_white_feature(piece: Piece, side: Side, sq: Square) -> usize {
    get_feature_index(piece, side, sq).unwrap()
}

#[inline(always)]
fn get_black_feature(piece: Piece, side: Side, sq: Square) -> usize {
    get_feature_index(piece, side.other(), sq.mirrored()).unwrap()
}

impl BoardState {
    #[inline(always)]
    pub fn nnue_make_move(
        &mut self,
        m: Move,
        moved_piece: Piece,
        final_moved_piece: Piece,
        captured_piece: Piece,
    ) {
        if let Some(network) = GLOBAL_NETWORK.get() {
            let side = self.side_to_move;

            if m.is_castle() {
                let (rook_src, rook_dst) = match m.target {
                    Square::C1 => (Square::A1, Square::D1),
                    Square::G1 => (Square::H1, Square::F1),
                    Square::C8 => (Square::A8, Square::D8),
                    Square::G8 => (Square::H8, Square::F8),
                    _ => unreachable!(),
                };

                let w_add1 = get_white_feature(Piece::King, side, m.target);
                let w_add2 = get_white_feature(Piece::Rook, side, rook_dst);
                let w_rem1 = get_white_feature(Piece::King, side, m.source);
                let w_rem2 = get_white_feature(Piece::Rook, side, rook_src);
                self.accumulator_white
                    .update_2_2(w_add1, w_add2, w_rem1, w_rem2, network);

                let b_add1 = get_black_feature(Piece::King, side, m.target);
                let b_add2 = get_black_feature(Piece::Rook, side, rook_dst);
                let b_rem1 = get_black_feature(Piece::King, side, m.source);
                let b_rem2 = get_black_feature(Piece::Rook, side, rook_src);
                self.accumulator_black
                    .update_2_2(b_add1, b_add2, b_rem1, b_rem2, network);
            } else if m.is_capture() {
                let capture_sq = if m.move_type.is_en_passant() {
                    self.en_passant_square_for(m)
                } else {
                    m.target
                };

                let w_add = get_white_feature(final_moved_piece, side, m.target);
                let w_rem1 = get_white_feature(moved_piece, side, m.source);
                let w_rem2 = get_white_feature(captured_piece, side.other(), capture_sq);
                self.accumulator_white
                    .update_1_2(w_add, w_rem1, w_rem2, network);

                let b_add = get_black_feature(final_moved_piece, side, m.target);
                let b_rem1 = get_black_feature(moved_piece, side, m.source);
                let b_rem2 = get_black_feature(captured_piece, side.other(), capture_sq);
                self.accumulator_black
                    .update_1_2(b_add, b_rem1, b_rem2, network);
            } else {
                let w_add = get_white_feature(final_moved_piece, side, m.target);
                let w_rem = get_white_feature(moved_piece, side, m.source);
                self.accumulator_white
                    .update_1_1(w_add, w_rem, network);

                let b_add = get_black_feature(final_moved_piece, side, m.target);
                let b_rem = get_black_feature(moved_piece, side, m.source);
                self.accumulator_black
                    .update_1_1(b_add, b_rem, network);
            }
        }
    }

    #[inline(always)]
    pub fn nnue_unmake_move(
        &mut self,
        m: Move,
        moved_piece: Piece,
        final_moved_piece: Piece,
        captured_piece: Piece,
    ) {
        if let Some(network) = GLOBAL_NETWORK.get() {
            let side = self.side_to_move;

            if m.is_castle() {
                let (rook_src, rook_dst) = match m.target {
                    Square::C1 => (Square::A1, Square::D1),
                    Square::G1 => (Square::H1, Square::F1),
                    Square::C8 => (Square::A8, Square::D8),
                    Square::G8 => (Square::H8, Square::F8),
                    _ => unreachable!(),
                };

                let w_add1 = get_white_feature(Piece::King, side, m.source);
                let w_add2 = get_white_feature(Piece::Rook, side, rook_src);
                let w_rem1 = get_white_feature(Piece::King, side, m.target);
                let w_rem2 = get_white_feature(Piece::Rook, side, rook_dst);
                self.accumulator_white
                    .update_2_2(w_add1, w_add2, w_rem1, w_rem2, network);

                let b_add1 = get_black_feature(Piece::King, side, m.source);
                let b_add2 = get_black_feature(Piece::Rook, side, rook_src);
                let b_rem1 = get_black_feature(Piece::King, side, m.target);
                let b_rem2 = get_black_feature(Piece::Rook, side, rook_dst);
                self.accumulator_black
                    .update_2_2(b_add1, b_add2, b_rem1, b_rem2, network);
            } else if m.is_capture() {
                let capture_sq = if m.move_type.is_en_passant() {
                    self.en_passant_square_for(m)
                } else {
                    m.target
                };

                let w_add1 = get_white_feature(moved_piece, side, m.source);
                let w_add2 = get_white_feature(captured_piece, side.other(), capture_sq);
                let w_rem = get_white_feature(final_moved_piece, side, m.target);
                self.accumulator_white
                    .update_2_1(w_add1, w_add2, w_rem, network);

                let b_add1 = get_black_feature(moved_piece, side, m.source);
                let b_add2 = get_black_feature(captured_piece, side.other(), capture_sq);
                let b_rem = get_black_feature(final_moved_piece, side, m.target);
                self.accumulator_black
                    .update_2_1(b_add1, b_add2, b_rem, network);
            } else {
                let w_add = get_white_feature(moved_piece, side, m.source);
                let w_rem = get_white_feature(final_moved_piece, side, m.target);
                self.accumulator_white
                    .update_1_1(w_add, w_rem, network);

                let b_add = get_black_feature(moved_piece, side, m.source);
                let b_rem = get_black_feature(final_moved_piece, side, m.target);
                self.accumulator_black
                    .update_1_1(b_add, b_rem, network);
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
