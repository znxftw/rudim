use crate::bitboard::Bitboard;
use crate::bitboard::lookups::{
    get_bishop_attacks_from_table, get_rook_attacks_from_table, knight_attacks,
};
use crate::board::state::BoardState;
use crate::common::game_phase;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::eval::pawns::PawnStructureEvaluation;

pub struct PieceSquareTableEvaluation;

impl PieceSquareTableEvaluation {
    pub fn evaluate(board_state: &BoardState) -> i16 {
        // TODO: score is deterministic per board state, can potentially cache it
        let mut score: i16 = 0;

        score += Self::score_position(board_state);
        score += PawnStructureEvaluation::evaluate(board_state);
        score += Self::score_mobility(board_state);

        if board_state.side_to_move == Side::White {
            score
        } else {
            -score
        }
    }

    fn score_position(board_state: &BoardState) -> i16 {
        let mut positional_score = 0;
        let mid_game_phase = board_state.clipped_phase();
        let end_game_phase = game_phase::TOTAL_PHASE - mid_game_phase;

        for (piece_idx, &white_board) in board_state.pieces[Side::White as usize].iter().enumerate()
        {
            let mut white_board = white_board;
            while white_board.0 > 0 {
                let square = white_board.get_lsb() as usize;
                white_board.clear_bit(square);
                positional_score += (MID_GAME_POSITIONS[piece_idx][square] as i32 * mid_game_phase)
                    + (END_GAME_POSITIONS[piece_idx][square] as i32 * end_game_phase);
            }
        }

        for (piece_idx, &black_board) in board_state.pieces[Side::Black as usize].iter().enumerate()
        {
            let mut black_board = black_board;
            while black_board.0 > 0 {
                let square = black_board.get_lsb() as usize;
                black_board.clear_bit(square);
                let mirrored_square = Self::mirror_square(square);
                positional_score -= (MID_GAME_POSITIONS[piece_idx][mirrored_square] as i32
                    * mid_game_phase)
                    + (END_GAME_POSITIONS[piece_idx][mirrored_square] as i32 * end_game_phase);
            }
        }

        (positional_score as f64 * game_phase::PHASE_FACTOR) as i16
    }

    fn mirror_square(square: usize) -> usize {
        let row = square >> 3;
        let col = square & 7;
        ((7 - row) << 3) + col
    }

    fn score_mobility(board_state: &BoardState) -> i16 {
        let mut mobility = 0;
        let occupancy = board_state.occupancies[Side::Both as usize];
        let white_pieces = board_state.occupancies[Side::White as usize].0;
        let black_pieces = board_state.occupancies[Side::Black as usize].0;

        for piece_idx in 1..5 {
            let mut white_board = Bitboard(board_state.pieces[Side::White as usize][piece_idx].0);
            while white_board.0 > 0 {
                let square = white_board.get_lsb() as usize;
                white_board.clear_bit(square);
                let attacks = match piece_idx {
                    1 => knight_attacks()[square],
                    2 => get_bishop_attacks_from_table(Square::from(square), occupancy).0,
                    3 => get_rook_attacks_from_table(Square::from(square), occupancy).0,
                    4 => {
                        get_bishop_attacks_from_table(Square::from(square), occupancy).0
                            | get_rook_attacks_from_table(Square::from(square), occupancy).0
                    }
                    _ => 0,
                };
                mobility += (attacks & !white_pieces).count_ones() as i16;
            }

            let mut black_board = Bitboard(board_state.pieces[Side::Black as usize][piece_idx].0);
            while black_board.0 > 0 {
                let square = black_board.get_lsb() as usize;
                black_board.clear_bit(square);
                let attacks = match piece_idx {
                    1 => knight_attacks()[square],
                    2 => get_bishop_attacks_from_table(Square::from(square), occupancy).0,
                    3 => get_rook_attacks_from_table(Square::from(square), occupancy).0,
                    4 => {
                        get_bishop_attacks_from_table(Square::from(square), occupancy).0
                            | get_rook_attacks_from_table(Square::from(square), occupancy).0
                    }
                    _ => 0,
                };
                mobility -= (attacks & !black_pieces).count_ones() as i16;
            }
        }
        mobility
    }
}

static MID_GAME_POSITIONS: [[i16; 64]; 6] = {
    let piece_values = [82, 337, 365, 477, 1025, 0];
    // Values borrowed from Rofchade
    // http://www.talkchess.com/forum3/viewtopic.php?f=2&t=68311&start=19
    let mut tables = [
        // Pawn
        [
            0, 0, 0, 0, 0, 0, 0, 0, 98, 134, 61, 95, 68, 126, 34, -11, -6, 7, 26, 31, 65, 56, 25,
            -20, -14, 13, 6, 21, 23, 12, 17, -23, -27, -2, -5, 12, 17, 6, 10, -25, -26, -4, -4,
            -10, 3, 3, 33, -12, -35, -1, -20, -23, -15, 24, 38, -22, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        // Knight
        [
            -167, -89, -34, -49, 61, -97, -15, -107, -73, -41, 72, 36, 23, 62, 7, -17, -47, 60, 37,
            65, 84, 129, 73, 44, -9, 17, 19, 53, 37, 69, 18, 22, -13, 4, 16, 13, 28, 19, 21, -8,
            -23, -9, 12, 10, 19, 17, 25, -16, -29, -53, -12, -3, -1, 18, -14, -19, -105, -21, -58,
            -33, -17, -28, -19, -23,
        ],
        // Bishop
        [
            -29, 4, -82, -37, -25, -42, 7, -8, -26, 16, -18, -13, 30, 59, 18, -47, -16, 37, 43, 40,
            35, 50, 37, -2, -4, 5, 19, 50, 37, 37, 7, -2, -6, 13, 13, 26, 34, 12, 10, 4, 0, 15, 15,
            15, 14, 27, 18, 10, 4, 15, 16, 0, 7, 21, 33, 1, -33, -3, -14, -21, -13, -12, -39, -21,
        ],
        // Rook
        [
            32, 42, 32, 51, 63, 9, 31, 43, 27, 32, 58, 62, 80, 67, 26, 44, -5, 19, 26, 36, 17, 45,
            61, 16, -24, -11, 7, 26, 24, 35, -8, -20, -36, -26, -12, -1, 9, -7, 6, -23, -45, -25,
            -16, -17, 3, 0, -5, -33, -44, -16, -20, -9, -1, 11, -6, -71, -19, -13, 1, 17, 16, 7,
            -37, -26,
        ],
        // Queen
        [
            -28, 0, 29, 12, 59, 44, 43, 45, -24, -39, -5, 1, -16, 57, 28, 54, -13, -17, 7, 8, 29,
            56, 47, 57, -27, -27, -16, -16, -1, 17, -2, 1, -9, -26, -9, -10, -2, -4, 3, -3, -14, 2,
            -11, -2, -5, 2, 14, 5, -35, -8, 11, 2, 8, 15, -3, 1, -1, -18, -9, 10, -15, -25, -31,
            -50,
        ],
        // King
        [
            -65, 23, 16, -15, -56, -34, 2, 13, 29, -1, -20, -7, -8, -4, -38, -29, -9, 24, 2, -16,
            -20, 6, 22, -22, -17, -20, -12, -27, -30, -25, -14, -36, -49, -1, -27, -39, -46, -44,
            -33, -51, -14, -14, -22, -46, -44, -30, -15, -27, 1, 7, -8, -64, -43, -16, 9, 8, -15,
            36, 12, -54, 8, -28, 24, 14,
        ],
    ];

    let mut piece = 0;
    while piece < 6 {
        let mut square = 0;
        while square < 64 {
            tables[piece][square] += piece_values[piece];
            square += 1;
        }
        piece += 1;
    }
    tables
};

static END_GAME_POSITIONS: [[i16; 64]; 6] = {
    let piece_values = [94, 281, 297, 512, 936, 0];
    let mut tables = [
        // Pawn
        [
            0, 0, 0, 0, 0, 0, 0, 0, 178, 173, 158, 134, 147, 132, 165, 187, 94, 100, 85, 67, 56,
            53, 82, 84, 32, 24, 13, 5, -2, 4, 17, 17, 13, 9, -3, -7, -7, -8, 3, -1, 4, 7, -6, 1, 0,
            -5, -1, -8, 13, 8, 8, 10, 13, 0, 2, -7, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        // Knight
        [
            -58, -38, -13, -28, -31, -27, -63, -99, -25, -8, -25, -2, -9, -25, -24, -52, -24, -20,
            10, 9, -1, -9, -19, -41, -17, 3, 22, 22, 22, 11, 8, -18, -18, -6, 16, 25, 16, 17, 4,
            -18, -23, -3, -1, 15, 10, -3, -20, -22, -42, -20, -10, -5, -2, -20, -23, -44, -29, -51,
            -23, -15, -22, -18, -50, -64,
        ],
        // Bishop
        [
            -14, -21, -11, -8, -7, -9, -17, -24, -8, -4, 7, -12, -3, -13, -4, -14, 2, -8, 0, -1,
            -2, 6, 0, 4, -3, 9, 12, 9, 14, 10, 3, 2, -6, 3, 13, 19, 7, 10, -3, -9, -12, -3, 8, 10,
            13, 3, -7, -15, -14, -18, -7, -1, 4, -9, -15, -27, -23, -9, -23, -5, -9, -16, -5, -17,
        ],
        // Rook
        [
            13, 10, 18, 15, 12, 12, 8, 5, 11, 13, 13, 11, -3, 3, 8, 3, 7, 7, 7, 5, 4, -3, -5, -3,
            4, 3, 13, 1, 2, 1, -1, 2, 3, 5, 8, 4, -5, -6, -8, -11, -4, 0, -5, -1, -7, -12, -8, -16,
            -6, -6, 0, 2, -9, -9, -11, -3, -9, 2, 3, -1, -5, -13, 4, -20,
        ],
        // Queen
        [
            -9, 22, 22, 27, 27, 19, 10, 20, -17, 20, 32, 41, 58, 25, 30, 0, -20, 6, 9, 49, 47, 35,
            19, 9, 3, 22, 24, 45, 57, 40, 57, 36, -18, 28, 19, 47, 31, 34, 39, 23, -16, -27, 15, 6,
            9, 17, 10, 5, -22, -23, -30, -16, -16, -23, -36, -32, -33, -28, -22, -43, -5, -32, -20,
            -41,
        ],
        // King
        [
            -74, -35, -18, -18, -11, 15, 4, -17, -12, 17, 14, 17, 17, 38, 23, 11, 10, 17, 23, 15,
            20, 45, 44, 13, -8, 22, 24, 27, 26, 33, 26, 3, -18, -4, 21, 24, 27, 23, 9, -11, -19,
            -3, 11, 21, 23, 16, 7, -9, -27, -11, 4, 13, 14, 4, -5, -17, -53, -34, -21, -11, -28,
            -14, -24, -43,
        ],
    ];

    let mut piece = 0;
    while piece < 6 {
        let mut square = 0;
        while square < 64 {
            tables[piece][square] += piece_values[piece];
            square += 1;
        }
        piece += 1;
    }
    tables
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::BoardState;
    use crate::common::helpers;

    #[test]
    fn should_return_consistent_score_for_starting_position_white() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board_state = BoardState::parse_fen(fen);
        let actual_score = PieceSquareTableEvaluation::evaluate(&board_state);
        assert_eq!(0, actual_score);
    }

    #[test]
    fn should_return_consistent_score_for_starting_position_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let board_state = BoardState::parse_fen(fen);
        let actual_score = PieceSquareTableEvaluation::evaluate(&board_state);
        assert_eq!(0, actual_score);
    }

    #[test]
    fn should_return_consistent_score_for_endgame() {
        let board_state = BoardState::parse_fen(helpers::ENDGAME_FEN);
        let actual_score = PieceSquareTableEvaluation::evaluate(&board_state);
        assert_eq!(-4, actual_score);
    }

    #[test]
    fn should_return_consistent_score_for_kiwipete() {
        let board_state = BoardState::parse_fen(helpers::KIWI_PETE_FEN);
        let actual_score = PieceSquareTableEvaluation::evaluate(&board_state);
        assert_eq!(61, actual_score);
    }

    #[test]
    fn should_return_consistent_score_for_advanced_move() {
        let board_state = BoardState::parse_fen(helpers::ADVANCED_MOVE_FEN);
        let actual_score = PieceSquareTableEvaluation::evaluate(&board_state);
        assert_eq!(606, actual_score);
    }
}
