use crate::bitboard::Bitboard;
use crate::bitboard::lookups::{
    get_bishop_attacks_from_table, get_rook_attacks_from_table, king_attacks, knight_attacks,
    pawn_attacks,
};
use crate::board::history::History;
use crate::common::castle::Castle;
use crate::common::constants::{PIECES, SIDES, SQUARES};
use crate::common::game_phase::{add_phase, get_clipped_phase, remove_phase};
use crate::common::piece::{Piece, PieceMap};
use crate::common::side::{Side, SideMap};
use crate::common::square::Square;
use crate::eval::nnue::accumulator::Accumulator;
use crate::eval::pst::get_pst_values;
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

#[derive(Debug, Clone)]
pub struct BoardState {
    pub pieces: PieceMap<Bitboard>,
    pub occupancies: SideMap<Bitboard>,
    pub piece_mapping: [Piece; SQUARES],
    pub side_to_move: Side,
    pub en_passant_square: Square,
    pub castle: Castle,
    pub move_count: i32,
    pub phase: i32,
    pub board_hash: u64,
    pub half_move_clock: u8,
    pub history: History,
    pub pst_mid: i32,
    pub pst_end: i32,
    pub accumulator_white: Accumulator,
    pub accumulator_black: Accumulator,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            pieces: PieceMap([Bitboard(0); PIECES]),
            occupancies: SideMap([Bitboard(0); SIDES]),
            piece_mapping: [Piece::None; SQUARES],
            side_to_move: Side::White,
            en_passant_square: Square::NoSquare,
            castle: Castle::NONE,
            move_count: 0,
            phase: 0,
            board_hash: 0,
            half_move_clock: 0,
            history: History::new(),
            pst_mid: 0,
            pst_end: 0,
            accumulator_white: Accumulator::new(),
            accumulator_black: Accumulator::new(),
        }
    }

    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.occupancies[Side::White] | self.occupancies[Side::Black]
    }

    #[inline(always)]
    pub fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.occupancies[side]
    }

    pub fn add_piece(&mut self, square: Square, side: Side, piece: Piece) {
        let sq = square as usize;
        self.pieces[piece].set_bit(sq);
        self.occupancies[side].set_bit(sq);
        self.piece_mapping[sq] = piece;
        self.phase = add_phase(self.phase, piece);

        let (mid_val, end_val) = get_pst_values(piece, square, side);
        self.pst_mid += mid_val;
        self.pst_end += end_val;

        // TODO: make_move + history instead?
        self.nnue_add_piece(square, side, piece);
    }

    pub fn remove_piece(&mut self, square: Square) -> Piece {
        let sq = square as usize;
        let piece = self.piece_mapping[sq];

        let side = if self.occupancies[Side::White].get_bit(sq) == 1 {
            Side::White
        } else {
            Side::Black
        };

        self.pieces[piece].clear_bit(sq);
        self.occupancies[Side::White].clear_bit(sq);
        self.occupancies[Side::Black].clear_bit(sq);
        self.piece_mapping[sq] = Piece::None;
        self.phase = remove_phase(self.phase, piece);

        let (mid_val, end_val) = get_pst_values(piece, square, side);
        self.pst_mid -= mid_val;
        self.pst_end -= end_val;

        // TODO: make_move + history instead?
        self.nnue_remove_piece(square, side, piece);

        piece
    }

    pub fn get_piece_on_side(&self, square: Square, side: Side) -> usize {
        let piece = self.piece_mapping[square as usize];
        if self.occupancies[side].get_bit(square as usize) == 1 {
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
        if self.occupancies[Side::White].get_bit(square as usize) == 1 {
            piece as i32
        } else {
            6 + piece as i32
        }
    }

    pub fn is_in_check(&self, side: Side) -> bool {
        let king_bb = self.get_pieces(side, Piece::King);
        let king_sq = Square::from(king_bb.get_lsb() as usize);
        self.is_square_attacked(king_sq, side.other())
    }

    pub fn is_square_attacked(&self, square: Square, attacking_side: Side) -> bool {
        let sq = square as usize;
        let occupancy = self.occupancy();
        let defending_side = attacking_side.other();

        if (self.get_pieces(attacking_side, Piece::Pawn)
            & pawn_attacks()[defending_side as usize][sq])
            .is_not_empty()
        {
            return true;
        }
        if (self.get_pieces(attacking_side, Piece::Knight) & knight_attacks()[sq]).is_not_empty() {
            return true;
        }
        if (self.get_pieces(attacking_side, Piece::King) & king_attacks()[sq]).is_not_empty() {
            return true;
        }

        if (get_bishop_attacks_from_table(square, occupancy)
            & (self.get_pieces(attacking_side, Piece::Bishop)
                | self.get_pieces(attacking_side, Piece::Queen)))
        .is_not_empty()
        {
            return true;
        }
        if (get_rook_attacks_from_table(square, occupancy)
            & (self.get_pieces(attacking_side, Piece::Rook)
                | self.get_pieces(attacking_side, Piece::Queen)))
        .is_not_empty()
        {
            return true;
        }

        false
    }

    pub fn clipped_phase(&self) -> i32 {
        get_clipped_phase(self.phase)
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

        assert_eq!(board.get_pieces(Side::White, Piece::Pawn).get_bit(sq), 1);
        assert_eq!(board.occupancies[Side::White].get_bit(sq), 1);
        assert_eq!(board.occupancy().get_bit(sq), 1);
        assert_eq!(board.occupancies[Side::Black].get_bit(sq), 0);
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
        assert_eq!(board.get_pieces(Side::Black, Piece::Queen).get_bit(sq), 0);
        assert_eq!(board.occupancies[Side::Black].get_bit(sq), 0);
        assert_eq!(board.occupancy().get_bit(sq), 0);
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

    #[test]
    fn test_accumulator_make_unmake_consistency() {
        use crate::common::move_list::MoveList;
        use crate::eval::nnue::GLOBAL_NETWORK;
        use crate::eval::nnue::loader::Network;

        let network = *GLOBAL_NETWORK.get_or_init(|| {
            let mut net = Network::new_boxed();
            net.transformer_biases.fill(3);
            net.transformer_weights.fill(2);
            Box::leak(net)
        });

        let mut board = BoardState::starting_position();

        let expected_white = board.accumulator_white;
        let expected_black = board.accumulator_black;
        board.refresh_accumulator(Side::White, network);
        board.refresh_accumulator(Side::Black, network);
        assert_eq!(board.accumulator_white, expected_white);
        assert_eq!(board.accumulator_black, expected_black);

        let mut move_history = Vec::new();
        for _ in 0..10 {
            let mut moves = MoveList::new();
            board.generate_moves(&mut moves);
            if moves.count == 0 {
                break;
            }

            // Find the first legal move
            let mut legal_move = None;
            for m_entry in moves.iter() {
                let m = m_entry.mv;
                board.make_move(m);
                let is_legal = !board.is_in_check(board.side_to_move.other());
                board.unmake_move(m);
                if is_legal {
                    legal_move = Some(m);
                    break;
                }
            }

            let Some(m) = legal_move else {
                break;
            };

            board.make_move(m);
            move_history.push(m);

            let current_white = board.accumulator_white;
            let current_black = board.accumulator_black;

            board.refresh_accumulator(Side::White, network);
            board.refresh_accumulator(Side::Black, network);

            assert_eq!(
                board.accumulator_white, current_white,
                "Failed white accumulator check after move {:?}",
                m
            );
            assert_eq!(
                board.accumulator_black, current_black,
                "Failed black accumulator check after move {:?}",
                m
            );
        }

        while let Some(m) = move_history.pop() {
            board.unmake_move(m);

            let current_white = board.accumulator_white;
            let current_black = board.accumulator_black;

            board.refresh_accumulator(Side::White, network);
            board.refresh_accumulator(Side::Black, network);

            assert_eq!(
                board.accumulator_white, current_white,
                "Failed white accumulator check after unmake {:?}",
                m
            );
            assert_eq!(
                board.accumulator_black, current_black,
                "Failed black accumulator check after unmake {:?}",
                m
            );
        }

        assert_eq!(board.accumulator_white, expected_white);
        assert_eq!(board.accumulator_black, expected_black);
    }
}
