use crate::board::state::BoardState;
use crate::common::constants;
use crate::common::piece::Piece;
use crate::common::side::Side;

const DOUBLED_PAWN_PENALTY: i16 = 10;
const ISOLATED_PAWN_PENALTY: i16 = 20;
const PASSED_PAWN_BONUS: i16 = 50;

pub struct PawnStructureEvaluation;

impl PawnStructureEvaluation {
    pub fn evaluate(board_state: &BoardState) -> i16 {
        let white_pawns = board_state.pieces[Side::White as usize][Piece::Pawn as usize].0;
        let black_pawns = board_state.pieces[Side::Black as usize][Piece::Pawn as usize].0;

        let mut score: i16 = 0;
        score += Self::score_doubled_pawns(white_pawns, black_pawns);
        score += Self::score_pawn_features(white_pawns, black_pawns);
        score
    }

    fn score_doubled_pawns(white_pawns: u64, black_pawns: u64) -> i16 {
        let mut score = 0;
        for &mask in &FILE_MASKS {
            let white_count = (white_pawns & mask).count_ones() as i16;
            let black_count = (black_pawns & mask).count_ones() as i16;
            if white_count > 1 {
                score -= (white_count - 1) * DOUBLED_PAWN_PENALTY;
            }
            if black_count > 1 {
                score += (black_count - 1) * DOUBLED_PAWN_PENALTY;
            }
        }
        score
    }

    fn score_pawn_features(white_pawns: u64, black_pawns: u64) -> i16 {
        let mut score = 0;
        let mut wp = white_pawns;
        while wp != 0 {
            let sq = wp.trailing_zeros() as usize;
            wp &= wp - 1;
            if (white_pawns & ADJACENT_FILE_MASKS[sq & 7]) == 0 {
                score -= ISOLATED_PAWN_PENALTY;
            }
            if (black_pawns & PASSED_PAWN_MASKS[Side::White as usize][sq]) == 0 {
                score += PASSED_PAWN_BONUS;
            }
        }

        let mut bp = black_pawns;
        while bp != 0 {
            let sq = bp.trailing_zeros() as usize;
            bp &= bp - 1;
            if (black_pawns & ADJACENT_FILE_MASKS[sq & 7]) == 0 {
                score += ISOLATED_PAWN_PENALTY;
            }
            if (white_pawns & PASSED_PAWN_MASKS[Side::Black as usize][sq]) == 0 {
                score -= PASSED_PAWN_BONUS;
            }
        }
        score
    }
}

static FILE_MASKS: [u64; 8] = {
    let mut masks = [0u64; 8];
    let mut file = 0;
    while file < 8 {
        let mut mask = 0;
        let mut row = 0;
        while row < 8 {
            mask |= 1u64 << (row * 8 + file);
            row += 1;
        }
        masks[file] = mask;
        file += 1;
    }
    masks
};

static ADJACENT_FILE_MASKS: [u64; 8] = {
    let mut masks = [0u64; 8];
    let mut file = 0;
    while file < 8 {
        let mut mask = 0;
        if file > 0 {
            mask |= FILE_MASKS[file - 1];
        }
        if file < 7 {
            mask |= FILE_MASKS[file + 1];
        }
        masks[file] = mask;
        file += 1;
    }
    masks
};

static PASSED_PAWN_MASKS: [[u64; 64]; 2] = {
    let mut masks = [[0u64; 64]; 2];
    let mut sq = 0;
    while sq < constants::SQUARES {
        let file = sq & 7;
        let row = sq >> 3;

        let mut white_mask = 0;
        let mut r = 0;
        while r < row {
            white_mask |= 1u64 << (r * 8 + file);
            if file > 0 {
                white_mask |= 1u64 << (r * 8 + file - 1);
            }
            if file < 7 {
                white_mask |= 1u64 << (r * 8 + file + 1);
            }
            r += 1;
        }
        masks[Side::White as usize][sq] = white_mask;

        let mut black_mask = 0;
        let mut r = row + 1;
        while r < 8 {
            black_mask |= 1u64 << (r * 8 + file);
            if file > 0 {
                black_mask |= 1u64 << (r * 8 + file - 1);
            }
            if file < 7 {
                black_mask |= 1u64 << (r * 8 + file + 1);
            }
            r += 1;
        }
        masks[Side::Black as usize][sq] = black_mask;
        sq += 1;
    }
    masks
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_score_zero_for_position_with_no_pawns() {
        let board_state = BoardState::parse_fen("8/8/8/8/8/8/8/K6k w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(0, score);
    }

    #[test]
    fn should_score_zero_for_symmetric_pawn_structure() {
        let board_state = BoardState::parse_fen("8/4p3/8/8/8/8/4P3/8 w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(0, score);
    }

    #[test]
    fn should_penalise_white_doubled_pawns() {
        let board_state = BoardState::parse_fen("8/8/8/4P3/4P3/8/8/K6k w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(50, score);
    }

    #[test]
    fn should_penalise_black_doubled_pawns() {
        let board_state = BoardState::parse_fen("K6k/8/8/4p3/4p3/8/8/8 w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(-50, score);
    }

    #[test]
    fn should_penalise_white_isolated_pawn() {
        let board_state = BoardState::parse_fen("8/8/8/4p3/4P3/8/8/K6k w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(0, score);
    }

    #[test]
    fn should_bonus_white_passed_pawn() {
        let board_state = BoardState::parse_fen("8/8/8/4P3/8/8/8/K6k w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(30, score);
    }

    #[test]
    fn should_bonus_black_passed_pawn() {
        let board_state = BoardState::parse_fen("K6k/8/8/8/4p3/8/8/8 w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(-30, score);
    }

    #[test]
    fn should_block_passed_pawn_when_opponent_pawn_is_on_adjacent_file() {
        let board_state = BoardState::parse_fen("8/3p4/8/4P3/8/8/8/K6k w - - 0 1");
        let score = PawnStructureEvaluation::evaluate(&board_state);
        assert_eq!(0, score);
    }
}
