use crate::board::state::BoardState;
use crate::common::moves::Move;
use crate::common::scored_moves::MoveList;
use crate::eval::move_ordering;

// TODO: revisit / refactor. promotions? refactor scoring MVV_LVA etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchPhase {
    PvMove,
    TtMove,
    GenerateCaptures,
    Captures,
    GenerateQuiets,
    Quiets,
    Done,
}

pub struct MovePicker {
    phase: SearchPhase,
    pv_move: Option<Move>,
    tt_best: Option<Move>,
    moves: MoveList,
    current_index: usize,
    ply: usize,
}

impl MovePicker {
    pub fn new(pv_move: Option<Move>, tt_best: Option<Move>, ply: usize) -> Self {
        Self {
            phase: SearchPhase::PvMove,
            pv_move,
            tt_best,
            moves: MoveList::new(),
            current_index: 0,
            ply,
        }
    }

    pub fn next(&mut self, board_state: &mut BoardState) -> Option<Move> {
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
                    self.moves.clear();
                    self.current_index = 0;
                    board_state.generate_captures(&mut self.moves);
                    move_ordering::populate_move_scores(
                        &mut self.moves,
                        board_state,
                        self.ply,
                        self.tt_best,
                        self.pv_move,
                    );
                    self.phase = SearchPhase::Captures;
                }
                SearchPhase::Captures => {
                    if self.current_index < self.moves.len() {
                        move_ordering::MoveOrdering::sort_next_best_move(
                            &mut self.moves,
                            self.current_index,
                        );
                        let mv = self.moves[self.current_index].mv;
                        self.current_index += 1;

                        if Some(mv) == self.pv_move || Some(mv) == self.tt_best {
                            continue; // Already tried
                        }
                        return Some(mv);
                    } else {
                        self.phase = SearchPhase::GenerateQuiets;
                    }
                }
                SearchPhase::GenerateQuiets => {
                    self.moves.clear();
                    self.current_index = 0;
                    board_state.generate_quiets(&mut self.moves);
                    move_ordering::populate_move_scores(
                        &mut self.moves,
                        board_state,
                        self.ply,
                        self.tt_best,
                        self.pv_move,
                    );
                    self.phase = SearchPhase::Quiets;
                }
                SearchPhase::Quiets => {
                    if self.current_index < self.moves.len() {
                        move_ordering::MoveOrdering::sort_next_best_move(
                            &mut self.moves,
                            self.current_index,
                        );
                        let mv = self.moves[self.current_index].mv;
                        self.current_index += 1;

                        if Some(mv) == self.pv_move || Some(mv) == self.tt_best {
                            continue; // Already tried
                        }
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
