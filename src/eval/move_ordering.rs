use crate::board::state::BoardState;
use crate::common::constants::{MAX_PLY, PIECES, SIDES, SQUARES};
use crate::common::move_list::ScoredMove;
use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;

pub struct MoveOrdering {
    pub killer_moves: [[Move; MAX_PLY]; 2],
    pub history_moves: [[i32; SQUARES]; PIECES * 2],
    pub counter_moves: [[[Move; SQUARES]; PIECES]; SIDES],
}

#[rustfmt::skip]
const MVV_LVA: [[i32; 7]; 7] = [
    // P , N , B , R , Q , K , None
    [ 15_000, 14_000, 13_000, 12_000, 11_000, 10_000, 0 ], // P
    [ 25_000, 24_000, 23_000, 22_000, 21_000, 20_000, 0 ], // N
    [ 35_000, 34_000, 33_000, 32_000, 31_000, 30_000, 0 ], // B
    [ 45_000, 44_000, 43_000, 42_000, 41_000, 40_000, 0 ], // R
    [ 55_000, 54_000, 53_000, 52_000, 51_000, 50_000, 0 ], // Q
    [ 65_000, 64_000, 63_000, 62_000, 61_000, 60_000, 0 ], // K
    [ 0, 0, 0, 0, 0, 0, 0 ] // None
];

impl MoveOrdering {
    pub fn new() -> Self {
        Self {
            killer_moves: [[Move::NO_MOVE; MAX_PLY]; 2],
            history_moves: [[0; SQUARES]; PIECES * 2],
            counter_moves: [[[Move::NO_MOVE; SQUARES]; PIECES]; SIDES],
        }
    }

    pub fn add_killer_move(&mut self, move_obj: Move, ply: usize) {
        if self.killer_moves[0][ply] == move_obj {
            return;
        }

        self.killer_moves[1][ply] = self.killer_moves[0][ply];
        self.killer_moves[0][ply] = move_obj;
    }

    pub fn update_history(&mut self, piece: usize, move_obj: Move, bonus: i32) {
        const MAX_HISTORY: i32 = 16384;
        let target = move_obj.target as usize;
        let clamped_bonus = bonus.clamp(-MAX_HISTORY, MAX_HISTORY);
        let current_score = self.history_moves[piece][target];
        self.history_moves[piece][target] +=
            clamped_bonus - current_score * clamped_bonus.abs() / MAX_HISTORY;
    }

    pub fn reset(&mut self) {
        self.killer_moves = [[Move::NO_MOVE; MAX_PLY]; 2];
        self.history_moves = [[0; SQUARES]; PIECES * 2];
        self.counter_moves = [[[Move::NO_MOVE; SQUARES]; PIECES]; SIDES];
    }

    pub fn decay_history(&mut self) {
        for row in self.history_moves.iter_mut() {
            for score in row.iter_mut() {
                *score /= 2;
            }
        }
    }

    pub fn is_move_heuristic_empty(&self) -> bool {
        self.killer_moves
            .iter()
            .all(|row| row.iter().all(|&m| m == Move::NO_MOVE))
            && self
                .history_moves
                .iter()
                .all(|row| row.iter().all(|&s| s == 0))
            && self.counter_moves.iter().all(|side_row| {
                side_row
                    .iter()
                    .all(|piece_row| piece_row.iter().all(|&m| m == Move::NO_MOVE))
            })
    }

    pub fn sort_next_best_move(moves: &mut [ScoredMove], starting_index: usize) {
        if let Some((best_offset, _)) = moves[starting_index..]
            .iter()
            .enumerate()
            .max_by_key(|(_, m)| m.score)
        {
            let best_index = starting_index + best_offset;
            if best_index != starting_index {
                moves.swap(best_index, starting_index);
            }
        }
    }

    pub fn populate_quiet_scores(
        &self,
        moves: &mut [ScoredMove],
        board_state: &BoardState,
        ply: usize,
        previous_move: Option<Move>,
    ) {
        let counter_move = if let Some(prev_mv) = previous_move {
            let prev_side = board_state.side_to_move.other();
            let prev_piece = board_state.piece_mapping[prev_mv.target as usize];
            if prev_piece != Piece::None {
                self.counter_moves[prev_side as usize][prev_piece as usize][prev_mv.target as usize]
            } else {
                Move::NO_MOVE
            }
        } else {
            Move::NO_MOVE
        };

        for move_obj in moves.iter_mut() {
            let prom_piece = move_obj.mv.move_type.promotion_piece();
            if prom_piece == Piece::Queen {
                move_obj.score = 25000;
            } else if move_obj.mv == self.killer_moves[0][ply] {
                move_obj.score = 22000;
            } else if move_obj.mv == self.killer_moves[1][ply] {
                move_obj.score = 21000;
            } else if counter_move != Move::NO_MOVE && move_obj.mv == counter_move {
                move_obj.score = 20000;
            } else if prom_piece != Piece::None {
                move_obj.score = -20000;
            } else {
                let piece = board_state.get_piece_on(move_obj.mv.source);
                if piece != -1 {
                    let history_score =
                        self.history_moves[piece as usize][move_obj.mv.target as usize];
                    move_obj.score = history_score;
                }
            }
        }
    }

    pub fn add_counter_move(
        &mut self,
        prev_side: Side,
        prev_piece: Piece,
        prev_square: Square,
        counter_move: Move,
    ) {
        self.counter_moves[prev_side as usize][prev_piece as usize][prev_square as usize] =
            counter_move;
    }
}

impl Default for MoveOrdering {
    fn default() -> Self {
        Self::new()
    }
}

pub fn populate_capture_scores(moves: &mut [ScoredMove], board_state: &BoardState) {
    for move_obj in moves.iter_mut() {
        let source_piece =
            board_state.get_piece_on_side(move_obj.mv.source, board_state.side_to_move);
        let target_piece: usize = if move_obj.mv.move_type == MoveType::EnPassant {
            Piece::Pawn as usize
        } else {
            board_state.get_piece_on_side(move_obj.mv.target, board_state.side_to_move.other())
        };

        let mut score = MVV_LVA[target_piece][source_piece];
        let prom_piece = move_obj.mv.move_type.promotion_piece();
        if prom_piece == Piece::Queen {
            score += 50000;
        } else if prom_piece != Piece::None {
            score -= 20000;
        }
        move_obj.score = score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::square::Square;

    #[test]
    fn should_sort_moves_by_score() {
        let mut moves = vec![
            ScoredMove {
                mv: Move {
                    source: Square::E2,
                    target: Square::E4,
                    move_type: MoveType::Quiet,
                },
                score: 100,
            },
            ScoredMove {
                mv: Move {
                    source: Square::D2,
                    target: Square::D4,
                    move_type: MoveType::Quiet,
                },
                score: 300,
            },
            ScoredMove {
                mv: Move {
                    source: Square::G1,
                    target: Square::F3,
                    move_type: MoveType::Quiet,
                },
                score: 200,
            },
        ];

        MoveOrdering::sort_next_best_move(&mut moves, 0);

        assert_eq!(moves[0].score, 300);
    }

    #[test]
    fn should_not_change_order_if_already_sorted() {
        let mut moves = vec![
            ScoredMove {
                mv: Move {
                    source: Square::D2,
                    target: Square::D4,
                    move_type: MoveType::Quiet,
                },
                score: 300,
            },
            ScoredMove {
                mv: Move {
                    source: Square::G1,
                    target: Square::F3,
                    move_type: MoveType::Quiet,
                },
                score: 200,
            },
            ScoredMove {
                mv: Move {
                    source: Square::E2,
                    target: Square::E4,
                    move_type: MoveType::Quiet,
                },
                score: 100,
            },
        ];

        MoveOrdering::sort_next_best_move(&mut moves, 1);

        assert_eq!(moves[0].score, 300);
        assert_eq!(moves[1].score, 200);
        assert_eq!(moves[2].score, 100);
    }

    #[test]
    fn should_prioritize_queen_promotions_and_penalize_under_promotions() {
        let board = BoardState::parse_fen("1r5k/P1Q5/8/8/8/8/8/K7 w - - 0 1");
        let mut move_ordering = MoveOrdering::new();

        let mut quiet_moves = vec![
            ScoredMove {
                mv: Move {
                    source: Square::A7,
                    target: Square::A8,
                    move_type: MoveType::QueenPromotion,
                },
                score: 0,
            },
            ScoredMove {
                mv: Move {
                    source: Square::A7,
                    target: Square::A8,
                    move_type: MoveType::RookPromotion,
                },
                score: 0,
            },
            ScoredMove {
                mv: Move {
                    source: Square::C7,
                    target: Square::C8,
                    move_type: MoveType::Quiet,
                },
                score: 0,
            },
        ];

        move_ordering.populate_quiet_scores(&mut quiet_moves, &board, 0, None);

        assert_eq!(quiet_moves[0].score, 25000);
        assert_eq!(quiet_moves[1].score, -20000);
        assert_eq!(quiet_moves[2].score, 0);

        let under_prom = Move {
            source: Square::A7,
            target: Square::A8,
            move_type: MoveType::RookPromotion,
        };
        move_ordering.add_killer_move(under_prom, 0);

        let mut killer_quiet_moves = vec![ScoredMove {
            mv: under_prom,
            score: 0,
        }];
        move_ordering.populate_quiet_scores(&mut killer_quiet_moves, &board, 0, None);
        assert_eq!(killer_quiet_moves[0].score, 22000);

        let mut capture_moves = vec![
            ScoredMove {
                mv: Move {
                    source: Square::A7,
                    target: Square::B8,
                    move_type: MoveType::QueenPromotionCapture,
                },
                score: 0,
            },
            ScoredMove {
                mv: Move {
                    source: Square::A7,
                    target: Square::B8,
                    move_type: MoveType::RookPromotionCapture,
                },
                score: 0,
            },
            ScoredMove {
                mv: Move {
                    source: Square::C7,
                    target: Square::B8,
                    move_type: MoveType::Capture,
                },
                score: 0,
            },
        ];

        populate_capture_scores(&mut capture_moves, &board);

        assert_eq!(capture_moves[0].score, 95000);
        assert_eq!(capture_moves[1].score, 25000);
        assert_eq!(capture_moves[2].score, 41000);
    }
}
