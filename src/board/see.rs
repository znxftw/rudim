use crate::bitboard::Bitboard;
use crate::bitboard::lookups::{
    get_bishop_attacks_from_table, get_rook_attacks_from_table, king_attacks, knight_attacks,
    pawn_attacks,
};
use crate::board::state::BoardState;
use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;

pub const SEE_PIECE_VALUES: [i16; 7] = [100, 300, 300, 500, 900, 20000, 0];

impl BoardState {
    pub fn see(&self, mv: Move) -> i16 {
        let source = mv.source;
        let target = mv.target;

        // 1. Initialize occupancies and active side
        let mut occupancy = self.occupancies[Side::Both as usize];
        let mut side = self.side_to_move;

        // 2. Identify the captured piece
        let captured = if mv.move_type == MoveType::EnPassant {
            Piece::Pawn
        } else {
            self.piece_mapping[target as usize]
        };

        // 3. First step gain (value of captured piece + promotion value if any)
        let mut first_gain = SEE_PIECE_VALUES[captured as usize];
        if mv.is_promotion() {
            let prom_piece = mv.move_type.promotion_piece();
            first_gain +=
                SEE_PIECE_VALUES[prom_piece as usize] - SEE_PIECE_VALUES[Piece::Pawn as usize];
        }

        // Initialize minimax list
        let mut gain = [0i16; 32];
        gain[0] = first_gain;

        // 4. Find all attackers of the target square
        let mut attackers = self.get_all_attackers(target, occupancy);

        // 5. Remove the source piece from occupancy and attackers
        occupancy.clear_bit(source as usize);
        attackers.clear_bit(source as usize);

        // If it's an en passant capture, we must also remove the actual captured pawn
        if mv.move_type == MoveType::EnPassant {
            let ep_sq = if side == Side::White {
                target as usize + 8
            } else {
                target as usize - 8
            };
            occupancy.clear_bit(ep_sq);
            attackers.clear_bit(ep_sq);
        }

        // Update diagonal/orthogonal attackers that might be revealed (X-rays)
        self.update_xrays(&mut attackers, target, occupancy);

        let mut depth = 1;
        let mut last_captured_piece = self.piece_mapping[source as usize];
        side = side.other();

        // 6. Incremental exchange loop
        while attackers.0 > 0 {
            let side_attackers = attackers.0 & self.occupancies[side as usize].0;
            if side_attackers == 0 {
                break;
            }

            // Find the least valuable attacker for the current side
            let (from_sq, piece) = self.get_least_valuable_attacker(side_attackers, side);
            if from_sq == Square::NoSquare {
                break;
            }

            // Value gained in this step is the value of the piece captured in the previous step
            let mut step_gain = SEE_PIECE_VALUES[last_captured_piece as usize];

            // Check for pawn promotions during recapture
            if piece == Piece::Pawn {
                let target_rank = target as usize / 8;
                if (side == Side::White && target_rank == 0)
                    || (side == Side::Black && target_rank == 7)
                {
                    step_gain += SEE_PIECE_VALUES[Piece::Queen as usize]
                        - SEE_PIECE_VALUES[Piece::Pawn as usize];
                }
            }

            gain[depth] = step_gain;
            depth += 1;

            // Remove the capturing piece from occupancy and attackers
            occupancy.clear_bit(from_sq as usize);
            attackers.clear_bit(from_sq as usize);

            // Update X-rays revealed by removing this piece
            self.update_xrays(&mut attackers, target, occupancy);

            last_captured_piece = piece;
            side = side.other();

            if depth >= 32 {
                break;
            }
        }

        // 7. Back-propagate scores using minimax decision tree
        for i in (1..depth).rev() {
            gain[i - 1] -= std::cmp::max(0, gain[i]);
        }

        gain[0]
    }

    fn get_all_attackers(&self, sq: Square, occupancy: Bitboard) -> Bitboard {
        let white_pawns = self.pieces[Side::White as usize][Piece::Pawn as usize].0;
        let black_pawns = self.pieces[Side::Black as usize][Piece::Pawn as usize].0;
        let knights = self.pieces[Side::White as usize][Piece::Knight as usize].0
            | self.pieces[Side::Black as usize][Piece::Knight as usize].0;
        let bishops = self.pieces[Side::White as usize][Piece::Bishop as usize].0
            | self.pieces[Side::Black as usize][Piece::Bishop as usize].0;
        let rooks = self.pieces[Side::White as usize][Piece::Rook as usize].0
            | self.pieces[Side::Black as usize][Piece::Rook as usize].0;
        let queens = self.pieces[Side::White as usize][Piece::Queen as usize].0
            | self.pieces[Side::Black as usize][Piece::Queen as usize].0;
        let kings = self.pieces[Side::White as usize][Piece::King as usize].0
            | self.pieces[Side::Black as usize][Piece::King as usize].0;

        let pawn_attacks = (pawn_attacks()[Side::Black as usize][sq as usize] & white_pawns)
            | (pawn_attacks()[Side::White as usize][sq as usize] & black_pawns);
        let knight_attacks = knight_attacks()[sq as usize] & knights;
        let bishop_attacks = get_bishop_attacks_from_table(sq, occupancy).0 & (bishops | queens);
        let rook_attacks = get_rook_attacks_from_table(sq, occupancy).0 & (rooks | queens);
        let king_attacks = king_attacks()[sq as usize] & kings;

        Bitboard(pawn_attacks | knight_attacks | bishop_attacks | rook_attacks | king_attacks)
    }

    fn update_xrays(&self, attackers: &mut Bitboard, target: Square, occupancy: Bitboard) {
        let bishops = self.pieces[Side::White as usize][Piece::Bishop as usize].0
            | self.pieces[Side::Black as usize][Piece::Bishop as usize].0;
        let rooks = self.pieces[Side::White as usize][Piece::Rook as usize].0
            | self.pieces[Side::Black as usize][Piece::Rook as usize].0;
        let queens = self.pieces[Side::White as usize][Piece::Queen as usize].0
            | self.pieces[Side::Black as usize][Piece::Queen as usize].0;

        // Diagonal X-rays
        let diagonal_attackers =
            get_bishop_attacks_from_table(target, occupancy).0 & (bishops | queens) & occupancy.0;
        attackers.0 |= diagonal_attackers;

        // Orthogonal X-rays
        let orthogonal_attackers =
            get_rook_attacks_from_table(target, occupancy).0 & (rooks | queens) & occupancy.0;
        attackers.0 |= orthogonal_attackers;
    }

    fn get_least_valuable_attacker(&self, side_attackers: u64, side: Side) -> (Square, Piece) {
        for piece_idx in 0..6 {
            let piece = Piece::from(piece_idx);
            let pieces_bb = self.pieces[side as usize][piece_idx].0;
            let intersection = side_attackers & pieces_bb;
            if intersection > 0 {
                let sq = Square::from(intersection.trailing_zeros() as usize);
                return (sq, piece);
            }
        }
        (Square::NoSquare, Piece::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::BoardState;

    #[test]
    fn test_see_hanging_piece() {
        // White Rook on d2 captures undefended Black Queen on d4
        // FEN: k7/8/8/8/3q4/8/3R4/K7 w - - 0 1
        let board = BoardState::parse_fen("k7/8/8/8/3q4/8/3R4/K7 w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 900); // White gains Black Queen
    }

    #[test]
    fn test_see_defended_piece() {
        // White Rook on d2 captures Black Queen on d4, which is defended by Black Pawn on e5 (Pxd4)
        // FEN: k7/8/8/4p3/3q4/8/3R4/K7 w - - 0 1
        let board = BoardState::parse_fen("k7/8/8/4p3/3q4/8/3R4/K7 w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 400); // 900 (Queen) - 500 (Rook) = 400
    }

    #[test]
    fn test_see_equal_exchange() {
        // White Knight on c3 captures Black Bishop on d5, defended by Black Pawn on e6
        // FEN: k7/8/4p3/3b4/8/2N5/8/K7 w - - 0 1
        let board = BoardState::parse_fen("k7/8/4p3/3b4/8/2N5/8/K7 w - - 0 1");
        let mv = Move::new(Square::C3, Square::D5, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 0); // 300 (Bishop) - 300 (Knight) = 0
    }

    #[test]
    fn test_see_xrays() {
        // White Rooks on d1 and d2 attack Black Queen on d4. Black has a Rook on d6.
        // FEN: k7/8/3r4/8/3q4/8/3R4/3R3K w - - 0 1
        // Capture sequence: White Rxd4 (+900), Black Rxd4 (+500), White Rxd4 (+500)
        let board = BoardState::parse_fen("k7/8/3r4/8/3q4/8/3R4/3R3K w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 900); // White Rook on d1 defends, White ends up ahead by Queen
    }

    #[test]
    fn test_see_bad_capture() {
        // White Rook on d2 captures Black Pawn on d4, defended by Black Knight on f5
        // FEN: k7/8/8/5n2/3p4/8/3R4/K7 w - - 0 1
        let board = BoardState::parse_fen("k7/8/8/5n2/3p4/8/3R4/K7 w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, -400); // 100 (Pawn) - 500 (Rook) = -400
    }
}
