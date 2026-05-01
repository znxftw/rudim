use crate::common::piece::Piece;

pub const PIECE_CONSTANTS: [i32; 6] = [0, 1, 1, 2, 4, 0];

pub const TOTAL_PHASE: i32 = PIECE_CONSTANTS[Piece::Pawn as usize] * 16
    + PIECE_CONSTANTS[Piece::Knight as usize] * 4
    + PIECE_CONSTANTS[Piece::Bishop as usize] * 4
    + PIECE_CONSTANTS[Piece::Rook as usize] * 4
    + PIECE_CONSTANTS[Piece::Queen as usize] * 2;

pub const ONLY_PAWNS: i32 = PIECE_CONSTANTS[Piece::Pawn as usize] * 16;

pub const PHASE_FACTOR: f64 = 1.0 / (TOTAL_PHASE as f64);

pub fn add_phase(phase: i32, piece: Piece) -> i32 {
    phase + PIECE_CONSTANTS[piece as usize]
}

pub fn remove_phase(phase: i32, piece: Piece) -> i32 {
    phase - PIECE_CONSTANTS[piece as usize]
}

pub fn get_clipped_phase(phase: i32) -> i32 {
    phase.min(TOTAL_PHASE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_maximum_phase_for_starting_position() {
        let mut phase = 0;
        for _ in 0..16 {
            phase = add_phase(phase, Piece::Pawn);
        }
        for _ in 0..4 {
            phase = add_phase(phase, Piece::Knight);
            phase = add_phase(phase, Piece::Bishop);
            phase = add_phase(phase, Piece::Rook);
        }
        for _ in 0..2 {
            phase = add_phase(phase, Piece::Queen);
            phase = add_phase(phase, Piece::King);
        }

        assert_eq!(TOTAL_PHASE, phase);
    }

    #[test]
    fn should_have_minimum_phase_with_only_kings() {
        let mut phase = 0;
        phase = add_phase(phase, Piece::King);
        phase = add_phase(phase, Piece::King);

        assert_eq!(0, phase);
    }

    #[test]
    fn should_not_go_above_max_phase_for_promotions() {
        let mut phase = TOTAL_PHASE;

        phase = remove_phase(phase, Piece::Pawn);
        phase = add_phase(phase, Piece::Queen);

        let clipped_phase = get_clipped_phase(phase);

        assert_eq!(TOTAL_PHASE, clipped_phase);
    }
}
