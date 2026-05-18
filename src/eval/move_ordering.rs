use crate::board::state::BoardState;
use crate::common::constants::{MAX_PLY, PIECES, SQUARES};
use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::scored_moves::ScoredMove;
use std::sync::{LazyLock, Mutex};

pub struct MoveOrdering {
    pub killer_moves: [[Move; MAX_PLY]; 2],
    pub history_moves: [[i32; SQUARES]; PIECES * 2],
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
        }
    }

    pub fn populate_move_score(
        &self,
        move_obj: &mut ScoredMove,
        board_state: &BoardState,
        ply: usize,
    ) {
        if !move_obj.mv.is_capture() {
            if move_obj.mv == self.killer_moves[0][ply] {
                move_obj.score = 9000;
            } else if move_obj.mv == self.killer_moves[1][ply] {
                move_obj.score = 8000;
            } else {
                let piece = board_state.get_piece_on(move_obj.mv.source);
                if piece != -1 {
                    move_obj.score =
                        self.history_moves[piece as usize][move_obj.mv.target as usize];
                }
            }
            return;
        };
        let source_piece =
            board_state.get_piece_on_side(move_obj.mv.source, board_state.side_to_move);
        let target_piece: usize = if move_obj.mv.move_type == MoveType::EnPassant {
            Piece::Pawn as usize
        } else {
            board_state.get_piece_on_side(move_obj.mv.target, board_state.side_to_move.other())
        };

        move_obj.score = MVV_LVA[target_piece][source_piece];
    }

    pub fn add_killer_move(&mut self, move_obj: Move, ply: usize) {
        if self.killer_moves[0][ply] == move_obj {
            return;
        }

        self.killer_moves[1][ply] = self.killer_moves[0][ply];
        self.killer_moves[0][ply] = move_obj;
    }

    pub fn add_history_move(&mut self, piece: usize, move_obj: Move, depth: u8) {
        self.history_moves[piece][move_obj.target as usize] += (depth as i32) * (depth as i32);
    }

    pub fn reset(&mut self) {
        self.killer_moves = [[Move::NO_MOVE; MAX_PLY]; 2];
        self.history_moves = [[0; SQUARES]; PIECES * 2];
    }

    pub fn is_move_heuristic_empty(&self) -> bool {
        self.killer_moves
            .iter()
            .all(|row| row.iter().all(|&m| m == Move::NO_MOVE))
            && self
                .history_moves
                .iter()
                .all(|row| row.iter().all(|&s| s == 0))
    }

    pub fn populate_hash_move(move_obj: &mut ScoredMove) {
        move_obj.score = 1_000_000;
    }

    pub fn populate_pv_move(move_obj: &mut ScoredMove) {
        move_obj.score = 2_000_000;
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
}

impl Default for MoveOrdering {
    fn default() -> Self {
        Self::new()
    }
}

pub static MOVE_ORDERING: LazyLock<Mutex<MoveOrdering>> =
    LazyLock::new(|| Mutex::new(MoveOrdering::new()));

pub fn populate_move_scores(
    moves: &mut [ScoredMove],
    board_state: &BoardState,
    ply: usize,
    hash_move: Option<Move>,
    pv_move: Option<Move>,
) {
    let move_ordering = MOVE_ORDERING.lock().unwrap();
    for move_obj in moves.iter_mut() {
        if let Some(pv_mv) = pv_move
            && pv_mv != Move::NO_MOVE
            && move_obj.mv == pv_mv
        {
            MoveOrdering::populate_pv_move(move_obj);
        } else if let Some(hash_mv) = hash_move
            && hash_mv != Move::NO_MOVE
            && move_obj.mv == hash_mv
        {
            MoveOrdering::populate_hash_move(move_obj);
        } else {
            move_ordering.populate_move_score(move_obj, board_state, ply);
        }
    }
}

pub fn add_killer_move(move_obj: Move, ply: usize) {
    let mut move_ordering = MOVE_ORDERING.lock().unwrap();
    move_ordering.add_killer_move(move_obj, ply);
}

pub fn add_history_move(piece: usize, move_obj: Move, depth: u8) {
    let mut move_ordering = MOVE_ORDERING.lock().unwrap();
    move_ordering.add_history_move(piece, move_obj, depth);
}

pub fn reset_move_heuristic() {
    let mut move_ordering = MOVE_ORDERING.lock().unwrap();
    move_ordering.reset();
}

pub fn is_move_heuristic_empty() -> bool {
    let move_ordering = MOVE_ORDERING.lock().unwrap();
    move_ordering.is_move_heuristic_empty()
}

pub fn populate_hash_move(move_obj: &mut ScoredMove) {
    MoveOrdering::populate_hash_move(move_obj);
}

pub fn populate_pv_move(move_obj: &mut ScoredMove) {
    MoveOrdering::populate_pv_move(move_obj);
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
}
