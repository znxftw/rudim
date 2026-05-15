use crate::board::state::BoardState;
use crate::common::side::Side;
use crate::common::square::Square;

const ZOBRIST_SEED: u64 = 1_804_289_383;

static ZOBRIST_TABLE: [[u64; 64]; 14] = generate_zobrist_table();

const fn generate_zobrist_table() -> [[u64; 64]; 14] {
    let mut table = [[0u64; 64]; 14];
    let mut state = ZOBRIST_SEED;

    // 12 piece types (6 for each color) and 64 squares, and extra - en passant, + edge cases below
    // [13][0] == white to move
    // [13][1] == black to move
    // [13][2..18] == castling rights
    let mut entry = 0;
    while entry < 14 {
        let mut square = 0;
        while square < 64 {
            let next = next_zobrist_u64(state);
            state = next;
            table[entry][square] = next;
            square += 1;
        }
        entry += 1;
    }

    table
}

const fn next_zobrist_u64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

pub fn init() {}

#[inline(always)]
pub fn zobrist_table() -> &'static [[u64; 64]; 14] {
    &ZOBRIST_TABLE
}

pub fn get_board_hash(board_state: &BoardState) -> u64 {
    let mut current_hash = 0;

    for square in 0..64 {
        let piece = board_state.get_piece_on(Square::from(square));
        if piece != -1 {
            current_hash ^= zobrist_table()[piece as usize][square];
        }
    }

    current_hash = hash_side_to_move(board_state, current_hash);
    current_hash = hash_castling_rights(board_state, current_hash);
    current_hash = hash_en_passant(board_state, current_hash);

    current_hash
}

pub fn hash_castling_rights(board_state: &BoardState, current_hash: u64) -> u64 {
    // Offset by 2 to avoid collision with side-to-move keys (which use [13][0] and [13][1])
    current_hash ^ zobrist_table()[13][2 + board_state.castle.bits() as usize]
}

fn hash_side_to_move(board_state: &BoardState, current_hash: u64) -> u64 {
    current_hash
        ^ if board_state.side_to_move == Side::White {
            zobrist_table()[13][0]
        } else {
            zobrist_table()[13][1]
        }
}

pub fn flip_side_to_move_hashes(_board_state: &BoardState, current_hash: u64) -> u64 {
    current_hash ^ zobrist_table()[13][0] ^ zobrist_table()[13][1]
}

pub fn hash_en_passant(board_state: &BoardState, current_hash: u64) -> u64 {
    if board_state.en_passant_square != Square::NoSquare {
        current_hash ^ zobrist_table()[12][board_state.en_passant_square as usize]
    } else {
        current_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::helpers::STARTING_FEN;

    #[test]
    fn test_starting_hash_is_deterministic() {
        let board = BoardState::parse_fen(STARTING_FEN);
        let hash = get_board_hash(&board);
        assert_eq!(hash, 17316932686648747093);
    }

    #[test]
    fn next_zobrist_u64_returns_same_value_for_same_seed() {
        assert_eq!(next_zobrist_u64(42), next_zobrist_u64(42));
    }

    #[test]
    fn next_zobrist_u64_matches_known_sequence() {
        let mut state = ZOBRIST_SEED;
        let expected = [
            1_934_242_336_581_872_173,
            13_464_673_579_788_154_553,
            11_709_177_447_654_686_868,
            2_257_752_617_580_451_733,
            7_898_199_364_665_293_946,
        ];

        for value in expected {
            state = next_zobrist_u64(state);
            assert_eq!(state, value);
        }
    }

    #[test]
    fn generated_zobrist_table_starts_with_prng_sequence() {
        let table = zobrist_table();
        let first = next_zobrist_u64(ZOBRIST_SEED);
        let second = next_zobrist_u64(first);
        let third = next_zobrist_u64(second);

        assert_eq!(table[0][0], first);
        assert_eq!(table[0][1], second);
        assert_eq!(table[0][2], third);
    }
}
