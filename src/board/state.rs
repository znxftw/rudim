use crate::bitboard::Bitboard;
use crate::bitboard::lookups::{
    get_bishop_attacks_from_table, get_rook_attacks_from_table, king_attacks, knight_attacks,
    pawn_attacks,
};
use crate::board::history::History;
use crate::common::castle::Castle;
use crate::common::constants::{PIECES, SIDES, SIDES_WITH_BOTH, SQUARES};
use crate::common::game_phase::{add_phase, remove_phase};
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::eval::pst;
use std::fmt;

#[rustfmt::skip]
pub const CASTLING_CONSTANTS: [u8; SQUARES] = [
     7, 15, 15, 15,  3, 15, 15, 11,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    13, 15, 15, 15, 12, 15, 15, 14,
];

#[derive(Debug)]
pub struct BoardState {
    pub pieces: [[Bitboard; PIECES]; SIDES],
    pub occupancies: [Bitboard; SIDES_WITH_BOTH],
    pub piece_mapping: [Piece; SQUARES],
    pub side_to_move: Side,
    pub en_passant_square: Square,
    pub castle: Castle,
    pub moves: Vec<Move>,
    pub move_count: i32,
    pub best_move: Move,
    pub phase: i32,
    pub pst_midgame_score: i32,
    pub pst_endgame_score: i32,
    pub board_hash: u64,
    pub last_draw_killer: i32,
    pub history: History,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            pieces: [[Bitboard(0); PIECES]; SIDES],
            occupancies: [Bitboard(0); SIDES_WITH_BOTH],
            piece_mapping: [Piece::None; SQUARES],
            side_to_move: Side::White,
            en_passant_square: Square::NoSquare,
            castle: Castle::NONE,
            moves: Vec::with_capacity(32),
            move_count: 0,
            best_move: Move::NO_MOVE,
            phase: 0,
            pst_midgame_score: 0,
            pst_endgame_score: 0,
            board_hash: 0,
            last_draw_killer: 0,
            history: History::new(),
        }
    }

    pub fn add_piece(&mut self, square: Square, side: Side, piece: Piece) {
        let sq = square as usize;
        self.pieces[side as usize][piece as usize].set_bit(sq);
        self.occupancies[side as usize].set_bit(sq);
        self.occupancies[Side::Both as usize].set_bit(sq);
        self.piece_mapping[sq] = piece;
        self.phase = add_phase(self.phase, piece);
        self.apply_incremental_pst_delta(side, piece, sq, 1);
    }

    pub fn remove_piece(&mut self, square: Square) -> Piece {
        let sq = square as usize;
        let piece = self.piece_mapping[sq];
        let side = if self.occupancies[Side::White as usize].get_bit(sq) == 1 {
            Side::White
        } else {
            debug_assert_eq!(
                self.occupancies[Side::Black as usize].get_bit(sq),
                1,
                "remove_piece called on empty square {}",
                square
            );
            Side::Black
        };
        self.pieces[Side::White as usize][piece as usize].clear_bit(sq);
        self.pieces[Side::Black as usize][piece as usize].clear_bit(sq);
        self.occupancies[Side::White as usize].clear_bit(sq);
        self.occupancies[Side::Black as usize].clear_bit(sq);
        self.occupancies[Side::Both as usize].clear_bit(sq);
        self.piece_mapping[sq] = Piece::None;
        self.phase = remove_phase(self.phase, piece);
        self.apply_incremental_pst_delta(side, piece, sq, -1);
        piece
    }

    pub fn get_piece_on_side(&self, square: Square, side: Side) -> usize {
        let piece = self.piece_mapping[square as usize];
        if self.occupancies[side as usize].get_bit(square as usize) == 1 {
            piece as usize
        } else {
            Piece::None as usize
        }
    }

    pub fn get_piece_on(&self, square: Square) -> i32 {
        let piece = self.piece_mapping[square as usize];
        if piece == Piece::None {
            return -1;
        }
        if self.occupancies[Side::White as usize].get_bit(square as usize) == 1 {
            piece as i32
        } else {
            // TODO: revisit this abstraction
            6 + piece as i32
        }
    }

    pub fn is_in_check(&self, side: Side) -> bool {
        let king_bb = self.pieces[side as usize][Piece::King as usize];
        let king_sq = Square::from(king_bb.get_lsb() as usize);
        self.is_square_attacked(king_sq, side.other())
    }

    pub fn is_square_attacked(&self, square: Square, attacking_side: Side) -> bool {
        let sq = square as usize;
        let occupancy = self.occupancies[Side::Both as usize];
        let defending_side = attacking_side.other();

        let queen_attacks = self.pieces[attacking_side as usize][Piece::Queen as usize].0;

        if get_bishop_attacks_from_table(square, occupancy).0
            & (self.pieces[attacking_side as usize][Piece::Bishop as usize].0 | queen_attacks)
            != 0
        {
            return true;
        }
        if get_rook_attacks_from_table(square, occupancy).0
            & (self.pieces[attacking_side as usize][Piece::Rook as usize].0 | queen_attacks)
            != 0
        {
            return true;
        }

        if pawn_attacks()[defending_side as usize][sq]
            & self.pieces[attacking_side as usize][Piece::Pawn as usize].0
            != 0
        {
            return true;
        }
        if knight_attacks()[sq] & self.pieces[attacking_side as usize][Piece::Knight as usize].0
            != 0
        {
            return true;
        }
        if king_attacks()[sq] & self.pieces[attacking_side as usize][Piece::King as usize].0 != 0 {
            return true;
        }

        false
    }

    pub fn clipped_phase(&self) -> i32 {
        crate::common::game_phase::get_clipped_phase(self.phase)
    }

    fn apply_incremental_pst_delta(&mut self, side: Side, piece: Piece, square: usize, sign: i32) {
        let (mid_game, end_game) = pst::piece_square_delta(piece, side, square);
        self.pst_midgame_score += sign * mid_game;
        self.pst_endgame_score += sign * end_game;
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::starting_position()
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces
            && self.occupancies == other.occupancies
            && self.side_to_move == other.side_to_move
            && self.en_passant_square == other.en_passant_square
            && self.castle == other.castle
            && self.moves == other.moves
            && self.phase == other.phase
            && self.pst_midgame_score == other.pst_midgame_score
            && self.pst_endgame_score == other.pst_endgame_score
    }
}

impl Eq for BoardState {}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_piece_sets_all_structures() {
        let mut board = BoardState::new();
        board.add_piece(Square::E4, Side::White, Piece::Pawn);

        let sq = Square::E4 as usize;

        assert_eq!(
            board.pieces[Side::White as usize][Piece::Pawn as usize].get_bit(sq),
            1
        );
        assert_eq!(board.occupancies[Side::White as usize].get_bit(sq), 1);
        assert_eq!(board.occupancies[Side::Both as usize].get_bit(sq), 1);
        assert_eq!(board.occupancies[Side::Black as usize].get_bit(sq), 0);
        assert_eq!(board.piece_mapping[sq], Piece::Pawn);
        assert_eq!(board.phase, 0);
    }

    #[test]
    fn remove_piece_clears_all_structures() {
        let mut board = BoardState::new();
        board.add_piece(Square::D5, Side::Black, Piece::Queen);

        let sq = Square::D5 as usize;
        let removed = board.remove_piece(Square::D5);

        assert_eq!(removed, Piece::Queen);
        assert_eq!(
            board.pieces[Side::Black as usize][Piece::Queen as usize].get_bit(sq),
            0
        );
        assert_eq!(board.occupancies[Side::Black as usize].get_bit(sq), 0);
        assert_eq!(board.occupancies[Side::Both as usize].get_bit(sq), 0);
        assert_eq!(board.piece_mapping[sq], Piece::None);
    }

    #[test]
    fn get_piece_on_side_returns_correct_piece() {
        let mut board = BoardState::new();
        board.add_piece(Square::C3, Side::White, Piece::Knight);

        assert_eq!(
            board.get_piece_on_side(Square::C3, Side::White),
            Piece::Knight as usize
        );
        assert_eq!(
            board.get_piece_on_side(Square::C3, Side::Black),
            Piece::None as usize
        );
        assert_eq!(
            board.get_piece_on_side(Square::D4, Side::White),
            Piece::None as usize
        );
    }

    #[test]
    fn get_piece_on_returns_signed_index() {
        let mut board = BoardState::new();
        board.add_piece(Square::E1, Side::White, Piece::King);
        board.add_piece(Square::E8, Side::Black, Piece::King);

        assert_eq!(board.get_piece_on(Square::E1), 5);
        assert_eq!(board.get_piece_on(Square::E8), 11);
        assert_eq!(board.get_piece_on(Square::D4), -1);
    }

    #[test]
    fn equality_works_for_blank_boards() {
        let b1 = BoardState::new();
        let b2 = BoardState::new();
        assert_eq!(b1, b2);
    }

    #[test]
    fn equality_detects_difference() {
        let mut b1 = BoardState::new();
        let b2 = BoardState::new();
        b1.add_piece(Square::E4, Side::White, Piece::Pawn);
        assert_ne!(b1, b2);
    }

    #[test]
    fn is_in_check_not_in_check_on_empty_board() {
        let mut board = BoardState::new();
        board.add_piece(Square::E1, Side::White, Piece::King);
        assert!(!board.is_in_check(Side::White));
    }

    #[test]
    fn is_in_check_detects_rook_check() {
        let mut board = BoardState::new();
        board.add_piece(Square::E1, Side::White, Piece::King);
        board.add_piece(Square::E8, Side::Black, Piece::Rook);
        assert!(board.is_in_check(Side::White));
    }

    #[test]
    fn is_in_check_detects_knight_check() {
        let mut board = BoardState::new();
        board.add_piece(Square::E4, Side::White, Piece::King);
        // D6 knight attacks E4
        board.add_piece(Square::D6, Side::Black, Piece::Knight);
        assert!(board.is_in_check(Side::White));
    }

    #[test]
    fn is_in_check_detects_bishop_check() {
        let mut board = BoardState::new();
        board.add_piece(Square::E4, Side::White, Piece::King);
        board.add_piece(Square::H7, Side::Black, Piece::Bishop);
        assert!(board.is_in_check(Side::White));
    }

    #[test]
    fn is_in_check_detects_queen_check() {
        let mut board = BoardState::new();
        board.add_piece(Square::E4, Side::White, Piece::King);
        board.add_piece(Square::E8, Side::Black, Piece::Queen);
        assert!(board.is_in_check(Side::White));
    }

    #[test]
    fn is_in_check_detects_pawn_check() {
        let mut board = BoardState::new();
        board.add_piece(Square::E4, Side::White, Piece::King);
        board.add_piece(Square::D5, Side::Black, Piece::Pawn);
        assert!(board.is_in_check(Side::White));
    }

    #[test]
    fn blocker_prevents_rook_check() {
        let mut board = BoardState::new();
        board.add_piece(Square::E1, Side::White, Piece::King);
        board.add_piece(Square::E4, Side::White, Piece::Pawn);
        board.add_piece(Square::E8, Side::Black, Piece::Rook);
        assert!(!board.is_in_check(Side::White));
    }
}
