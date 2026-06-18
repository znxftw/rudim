use crate::board::state::BoardState;
use crate::common::move_list::{MoveList, ScoredMove};
use crate::common::moves::Move;
use crate::eval::move_ordering::{self, MoveOrdering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchPhase {
    PvMove,
    TtMove,
    GenerateCaptures,
    GoodCaptures,
    GenerateQuiets,
    Quiets,
    BadCaptures,
    Done,
}

pub struct MovePicker {
    phase: SearchPhase,
    pv_move: Option<Move>,
    tt_best: Option<Move>,
    previous_move: Option<Move>,
    good_captures_count: usize,
    current_index: usize,
    ply: usize,
    is_qsearch: bool,
}

impl MovePicker {
    pub fn new(
        pv_move: Option<Move>,
        tt_best: Option<Move>,
        previous_move: Option<Move>,
        ply: usize,
    ) -> Self {
        Self {
            phase: SearchPhase::PvMove,
            pv_move,
            tt_best,
            previous_move,
            good_captures_count: 0,
            current_index: 0,
            ply,
            is_qsearch: false,
        }
    }

    pub fn new_qsearch(ply: usize) -> Self {
        Self {
            phase: SearchPhase::PvMove,
            pv_move: None,
            tt_best: None,
            previous_move: None,
            good_captures_count: 0,
            current_index: 0,
            ply,
            is_qsearch: true,
        }
    }

    pub fn next(
        &mut self,
        board_state: &mut BoardState,
        move_ordering: &MoveOrdering,
        captures: &mut MoveList,
        quiets: &mut MoveList,
    ) -> Option<Move> {
        loop {
            match self.phase {
                SearchPhase::PvMove => {
                    self.phase = SearchPhase::TtMove;
                    if let Some(mv) = self.pv_move
                        && mv != Move::NO_MOVE
                    {
                        return Some(mv);
                    }
                }
                SearchPhase::TtMove => {
                    self.phase = SearchPhase::GenerateCaptures;
                    if let Some(mv) = self.tt_best
                        && mv != Move::NO_MOVE
                        && Some(mv) != self.pv_move
                    {
                        return Some(mv);
                    }
                }
                SearchPhase::GenerateCaptures => {
                    captures.clear();
                    self.current_index = 0;

                    board_state.generate_captures(captures);
                    move_ordering::populate_capture_scores(captures, board_state);

                    // Partition in-place: good captures (SEE >= 0) to the left, bad captures (SEE < 0) to the right
                    let mut left = 0;
                    let mut right = captures.len() as i32 - 1;
                    while left <= right {
                        if board_state.see(captures[left as usize].mv) >= 0 {
                            left += 1;
                        } else {
                            captures.swap(left as usize, right as usize);
                            right -= 1;
                        }
                    }
                    self.good_captures_count = left as usize;
                    self.phase = SearchPhase::GoodCaptures;
                }
                SearchPhase::GoodCaptures => {
                    if let Some(mv) = get_next_valid_move(
                        &mut captures[..self.good_captures_count],
                        &mut self.current_index,
                        self.pv_move,
                        self.tt_best,
                    ) {
                        return Some(mv);
                    } else if self.is_qsearch {
                        self.phase = SearchPhase::Done;
                    } else {
                        self.phase = SearchPhase::GenerateQuiets;
                    }
                }
                SearchPhase::GenerateQuiets => {
                    quiets.clear();
                    self.current_index = 0;
                    board_state.generate_quiets(quiets);
                    move_ordering.populate_quiet_scores(
                        quiets,
                        board_state,
                        self.ply,
                        self.previous_move,
                    );
                    self.phase = SearchPhase::Quiets;
                }
                SearchPhase::Quiets => {
                    if let Some(mv) = get_next_valid_move(
                        quiets,
                        &mut self.current_index,
                        self.pv_move,
                        self.tt_best,
                    ) {
                        return Some(mv);
                    } else {
                        self.phase = SearchPhase::BadCaptures;
                        self.current_index = self.good_captures_count;
                    }
                }
                SearchPhase::BadCaptures => {
                    if let Some(mv) = get_next_valid_move(
                        captures,
                        &mut self.current_index,
                        self.pv_move,
                        self.tt_best,
                    ) {
                        return Some(mv);
                    } else {
                        self.phase = SearchPhase::Done;
                    }
                }
                SearchPhase::Done => return None,
            }
        }
    }
}

fn get_next_valid_move(
    moves: &mut [ScoredMove],
    current_index: &mut usize,
    pv_move: Option<Move>,
    tt_best: Option<Move>,
) -> Option<Move> {
    let limit = moves.len();
    while *current_index < limit {
        MoveOrdering::sort_next_best_move(moves, *current_index);
        let mv = moves[*current_index].mv;
        *current_index += 1;

        if Some(mv) != pv_move && Some(mv) != tt_best {
            return Some(mv);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::move_type::MoveType;
    use crate::common::square::Square;

    #[test]
    fn test_move_picker_qsearch_only_good_captures() {
        let mut board = BoardState::parse_fen("k7/8/8/5n2/1p1p4/2B5/3R4/K7 w - - 0 1");

        let mut picker = MovePicker::new_qsearch(0);
        let mut captures = MoveList::new();
        let mut quiets = MoveList::new();
        let mut good_captures = Vec::new();
        let move_ordering = MoveOrdering::new();
        while let Some(mv) = picker.next(&mut board, &move_ordering, &mut captures, &mut quiets) {
            good_captures.push(mv);
        }
        assert!(!good_captures.is_empty());

        let has_good_capture = good_captures
            .iter()
            .any(|m| m.source == Square::C3 && m.target == Square::B4);
        let has_bad_capture = good_captures
            .iter()
            .any(|m| m.source == Square::D2 && m.target == Square::D4);
        let has_quiets = good_captures.iter().any(|m| m.move_type == MoveType::Quiet);

        assert!(has_good_capture);
        assert!(!has_bad_capture);
        assert!(!has_quiets);
    }

    #[test]
    fn test_move_picker_normal_search_all_phases() {
        let mut board = BoardState::parse_fen("k7/8/8/5n2/1p1p4/2B5/3R4/K7 w - - 0 1");

        let mut picker = MovePicker::new(None, None, None, 0);
        let mut captures = MoveList::new();
        let mut quiets = MoveList::new();
        let mut returned_moves = Vec::new();
        let move_ordering = MoveOrdering::new();
        while let Some(mv) = picker.next(&mut board, &move_ordering, &mut captures, &mut quiets) {
            returned_moves.push(mv);
        }

        let good_capture_idx = returned_moves
            .iter()
            .position(|m| m.source == Square::C3 && m.target == Square::B4)
            .unwrap();
        let bad_capture_idx = returned_moves
            .iter()
            .position(|m| m.source == Square::D2 && m.target == Square::D4)
            .unwrap();

        assert!(good_capture_idx < bad_capture_idx);

        let quiet_indices: Vec<usize> = returned_moves
            .iter()
            .enumerate()
            .filter(|(_, m)| m.move_type == MoveType::Quiet)
            .map(|(i, _)| i)
            .collect();

        assert!(!quiet_indices.is_empty());
        for &qi in &quiet_indices {
            assert!(
                good_capture_idx < qi,
                "Good capture should be searched before quiet moves"
            );
            assert!(
                qi < bad_capture_idx,
                "Bad capture should be searched after quiet moves"
            );
        }
    }
}
