use std::cmp::max;

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

impl Piece {
    #[inline]
    pub const fn see_value(self) -> i16 {
        match self {
            Piece::Pawn => 100,
            Piece::Knight => 300,
            Piece::Bishop => 300,
            Piece::Rook => 500,
            Piece::Queen => 900,
            Piece::King => 20000,
            Piece::None => 0,
        }
    }
}

impl BoardState {
    // impl - https://www.chessprogramming.org/SEE_-_The_Swap_Algorithm
    pub fn see(&self, mv: Move) -> i16 {
        let (source, target) = (mv.source, mv.target);
        let mut occupancy = self.occupancies[Side::Both];
        let mut side = self.side_to_move;

        let mut gain = [0i16; 32];
        gain[0] = self.get_initial_gain(mv, self.get_initial_captured_piece(mv));
        self.clear_en_passant_square(mv, &mut occupancy, side);

        let mut attackers = self.get_all_attackers(target, occupancy);

        // First Capture
        occupancy.clear_bit(source as usize);
        attackers.clear_bit(source as usize);

        self.update_xrays(&mut attackers, target, occupancy);

        let mut depth = 1;
        let mut last_captured_piece = if mv.is_promotion() {
            mv.move_type.promotion_piece()
        } else {
            self.piece_mapping[source as usize]
        };
        side = side.other();

        while attackers.is_not_empty() {
            let side_attackers = attackers & self.occupancies[side];
            if side_attackers.is_empty() {
                break;
            }

            let (from_sq, piece) = self.get_least_valuable_attacker(side_attackers, side);
            if from_sq == Square::NoSquare {
                break;
            }

            let (step_gain, next_captured) =
                self.get_recapture_gain_and_piece(piece, target, side, last_captured_piece);

            occupancy.clear_bit(from_sq as usize);
            attackers.clear_bit(from_sq as usize);
            self.update_xrays(&mut attackers, target, occupancy);

            last_captured_piece = next_captured;
            side = side.other();
            gain[depth] = step_gain;
            depth += 1;
        }

        for i in (1..depth).rev() {
            gain[i - 1] -= max(0, gain[i]);
        }

        gain[0]
    }

    #[inline(always)]
    fn get_initial_captured_piece(&self, mv: Move) -> Piece {
        if mv.move_type == MoveType::EnPassant {
            Piece::Pawn
        } else {
            self.piece_mapping[mv.target as usize]
        }
    }

    #[inline(always)]
    fn get_initial_gain(&self, mv: Move, captured: Piece) -> i16 {
        let mut gain = captured.see_value();
        if mv.is_promotion() {
            gain += mv.move_type.promotion_piece().see_value() - Piece::Pawn.see_value();
        }
        gain
    }

    #[inline(always)]
    fn clear_en_passant_square(&self, mv: Move, occupancy: &mut Bitboard, side: Side) {
        if mv.move_type == MoveType::EnPassant {
            let ep_sq = if side == Side::White {
                mv.target as usize + 8
            } else {
                mv.target as usize - 8
            };
            occupancy.clear_bit(ep_sq);
        }
    }

    #[inline(always)]
    fn get_recapture_gain_and_piece(
        &self,
        piece: Piece,
        target: Square,
        side: Side,
        last_captured: Piece,
    ) -> (i16, Piece) {
        let mut gain = last_captured.see_value();
        let mut next_captured = piece;

        if piece == Piece::Pawn {
            let rank = target as usize / 8;
            if (side == Side::White && rank == 0) || (side == Side::Black && rank == 7) {
                gain += Piece::Queen.see_value() - Piece::Pawn.see_value();
                next_captured = Piece::Queen;
            }
        }
        (gain, next_captured)
    }

    fn get_all_attackers(&self, sq: Square, occupancy: Bitboard) -> Bitboard {
        let white_pawns = self.get_pieces(Side::White, Piece::Pawn);
        let black_pawns = self.get_pieces(Side::Black, Piece::Pawn);
        let knights = self.pieces[Piece::Knight];
        let bishops = self.pieces[Piece::Bishop];
        let rooks = self.pieces[Piece::Rook];
        let queens = self.pieces[Piece::Queen];
        let kings = self.pieces[Piece::King];

        let pawn_attacks = (white_pawns & pawn_attacks()[Side::Black as usize][sq as usize])
            | (black_pawns & pawn_attacks()[Side::White as usize][sq as usize]);
        let knight_attacks = knights & knight_attacks()[sq as usize];
        let bishop_attacks = get_bishop_attacks_from_table(sq, occupancy) & (bishops | queens);
        let rook_attacks = get_rook_attacks_from_table(sq, occupancy) & (rooks | queens);
        let king_attacks = kings & king_attacks()[sq as usize];

        pawn_attacks | knight_attacks | bishop_attacks | rook_attacks | king_attacks
    }

    fn update_xrays(&self, attackers: &mut Bitboard, target: Square, occupancy: Bitboard) {
        let bishops = self.pieces[Piece::Bishop];
        let rooks = self.pieces[Piece::Rook];
        let queens = self.pieces[Piece::Queen];

        let diagonal_attackers =
            get_bishop_attacks_from_table(target, occupancy) & (bishops | queens) & occupancy;
        *attackers |= diagonal_attackers;

        let orthogonal_attackers =
            get_rook_attacks_from_table(target, occupancy) & (rooks | queens) & occupancy;
        *attackers |= orthogonal_attackers;
    }

    fn get_least_valuable_attacker(&self, side_attackers: Bitboard, side: Side) -> (Square, Piece) {
        for piece in Piece::ALL {
            let pieces_bb = self.get_pieces(side, piece);
            let intersection = side_attackers & pieces_bb;
            if intersection.is_not_empty() {
                let sq = Square::from(intersection.get_lsb() as usize);
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
        let board = BoardState::parse_fen("k7/8/8/8/3q4/8/3R4/K7 w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 900); // White gains Black Queen
    }

    #[test]
    fn test_see_defended_piece() {
        let board = BoardState::parse_fen("k7/8/8/4p3/3q4/8/3R4/K7 w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 400); // 900 (Queen) - 500 (Rook) = 400
    }

    #[test]
    fn test_see_equal_exchange() {
        let board = BoardState::parse_fen("k7/8/4p3/3b4/8/2N5/8/K7 w - - 0 1");
        let mv = Move::new(Square::C3, Square::D5, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 0); // 300 (Bishop) - 300 (Knight) = 0
    }

    #[test]
    fn test_see_xrays() {
        let board = BoardState::parse_fen("k7/8/3r4/8/3q4/8/3R4/3R3K w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, 900); // White Rook on d1 defends, White ends up ahead by Queen
    }

    #[test]
    fn test_see_bad_capture() {
        let board = BoardState::parse_fen("k7/8/8/5n2/3p4/8/3R4/K7 w - - 0 1");
        let mv = Move::new(Square::D2, Square::D4, MoveType::Capture);

        let score = board.see(mv);
        assert_eq!(score, -400); // 100 (Pawn) - 500 (Rook) = -400
    }

    #[test]
    fn test_see_promotion_capture() {
        let board = BoardState::parse_fen("rr6/P7/k7/8/8/8/8/K7 w - - 0 1");

        let mv_queen = Move::new(Square::A7, Square::B8, MoveType::QueenPromotionCapture);
        assert_eq!(board.see(mv_queen), 400); // 500 (Rook) + (900 - 100) (Queen promo) - 900 (recapture) = 400

        let mv_rook = Move::new(Square::A7, Square::B8, MoveType::RookPromotionCapture);
        assert_eq!(board.see(mv_rook), 400); // 500 (Rook) + (500 - 100) (Rook promo) - 500 (recapture) = 400

        let mv_bishop = Move::new(Square::A7, Square::B8, MoveType::BishopPromotionCapture);
        assert_eq!(board.see(mv_bishop), 400); // 500 (Rook) + (300 - 100) (Bishop promo) - 300 (recapture) = 400

        let mv_knight = Move::new(Square::A7, Square::B8, MoveType::KnightPromotionCapture);
        assert_eq!(board.see(mv_knight), 400); // 500 (Rook) + (300 - 100) (Knight promo) - 300 (recapture) = 400
    }
}
