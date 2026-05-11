use crate::board::state::BoardState;
use crate::common::random;
use crate::common::side::Side;
use crate::common::square::Square;
use std::sync::{Once, OnceLock};

static ZOBRIST_TABLE_LOCK: OnceLock<[[u64; 64]; 14]> = OnceLock::new();
static INIT_ONCE: Once = Once::new();

pub fn init() {
    INIT_ONCE.call_once(|| {
        let mut table = [[0u64; 64]; 14];
        random::reset_seed();

        // 12 piece types (6 for each color) and 64 squares, and extra - en passant, + edge cases below
        // [13][0] == white to move
        // [13][1] == black to move
        // [13][2..18] == castling rights
        for entry in table.iter_mut() {
            for square in entry.iter_mut() {
                *square = random::next_u64();
            }
        }

        ZOBRIST_TABLE_LOCK.set(table).unwrap();
    });
}

#[inline(always)]
pub fn zobrist_table() -> &'static [[u64; 64]; 14] {
    #[cfg(debug_assertions)]
    {
        if ZOBRIST_TABLE_LOCK.get().is_none() {
            crate::init();
        }
        ZOBRIST_TABLE_LOCK.get().unwrap()
    }
    #[cfg(not(debug_assertions))]
    {
        unsafe { ZOBRIST_TABLE_LOCK.get().unwrap_unchecked() }
    }
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
        init();
        let board = BoardState::parse_fen(STARTING_FEN);
        let hash = get_board_hash(&board);
        assert_eq!(hash, 17316932686648747093);
    }
}
