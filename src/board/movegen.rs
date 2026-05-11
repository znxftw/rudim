use crate::bitboard::Bitboard;
use crate::bitboard::lookups::{
    get_bishop_attacks_from_table, get_queen_attacks_from_table, get_rook_attacks_from_table,
    king_attacks, knight_attacks, pawn_attacks,
};
use crate::board::state::BoardState;
use crate::common::castle::Castle;
use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::search::iterative_deepening;
use std::sync::atomic::AtomicBool;

impl BoardState {
    pub fn find_best_move(
        &mut self,
        depth: i32,
        cancellation_token: &AtomicBool,
        debug_mode: &mut bool,
    ) -> Move {
        iterative_deepening::search(self, depth, cancellation_token, debug_mode);
        iterative_deepening::best_move()
    }

    pub fn generate_moves(&mut self) {
        self.moves.clear();

        self.generate_pawn_moves();
        self.generate_bishop_moves();
        self.generate_knight_moves();
        self.generate_rook_moves();
        self.generate_queen_moves();
        self.generate_king_moves();
    }

    fn generate_pawn_moves(&mut self) {
        let mut bitboard = self.pieces[self.side_to_move as usize][Piece::Pawn as usize];
        while bitboard.0 > 0 {
            let source = bitboard.get_lsb() as usize;
            self.generate_pawn_pushes(source);
            self.generate_en_passants(source);
            self.generate_pawn_attacks(source);
            bitboard.clear_bit(source);
        }
    }

    fn generate_pawn_pushes(&mut self, source: usize) {
        let both_occ = self.occupancies[Side::Both as usize];

        if self.side_to_move == Side::Black {
            let one_sq = source + 8;
            if both_occ.get_bit(one_sq) != 0 {
                return;
            }
            self.add_pawn_move(source, one_sq, false, false);

            if source >= Square::A7 as usize && source <= Square::H7 as usize {
                let two_sq = one_sq + 8;
                if both_occ.get_bit(two_sq) != 0 {
                    return;
                }
                self.add_pawn_move(source, two_sq, false, true);
            }
        } else {
            let one_sq = source - 8;
            if both_occ.get_bit(one_sq) != 0 {
                return;
            }
            self.add_pawn_move(source, one_sq, false, false);

            if source >= Square::A2 as usize && source <= Square::H2 as usize {
                let two_sq = one_sq - 8;
                if both_occ.get_bit(two_sq) != 0 {
                    return;
                }
                self.add_pawn_move(source, two_sq, false, true);
            }
        }
    }

    fn generate_en_passants(&mut self, source: usize) {
        if self.en_passant_square == Square::NoSquare {
            return;
        }
        let ep_bit = 1u64 << (self.en_passant_square as usize);
        let attacks = Bitboard(pawn_attacks()[self.side_to_move as usize][source] & ep_bit);
        if attacks.0 > 0 {
            let target = attacks.get_lsb() as usize;
            self.add_pawn_move(source, target, true, false);
        }
    }

    fn generate_pawn_attacks(&mut self, source: usize) {
        let enemy_occ = self.occupancies[self.side_to_move.other() as usize];
        let mut attacks = Bitboard(pawn_attacks()[self.side_to_move as usize][source] & enemy_occ.0);

        while attacks.0 > 0 {
            let target = attacks.get_lsb() as usize;
            // TODO: revisit, is this check redundant? (enemy_occ and self_occ cannot both be 1)
            if self.occupancies[self.side_to_move as usize].get_bit(target) == 0 {
                self.add_pawn_move(source, target, false, false);
            }
            attacks.clear_bit(target);
        }
    }

    fn generate_bishop_moves(&mut self) {
        let mut bitboard = self.pieces[self.side_to_move as usize][Piece::Bishop as usize];
        while bitboard.0 > 0 {
            let source = bitboard.get_lsb() as usize;
            let attacks = get_bishop_attacks_from_table(
                Square::from(source),
                self.occupancies[Side::Both as usize],
            );
            self.add_attacks(source, attacks);
            bitboard.clear_bit(source);
        }
    }

    fn generate_knight_moves(&mut self) {
        let mut bitboard = self.pieces[self.side_to_move as usize][Piece::Knight as usize];
        while bitboard.0 > 0 {
            let source = bitboard.get_lsb() as usize;
            let attacks = Bitboard(knight_attacks()[source]);
            self.add_attacks(source, attacks);
            bitboard.clear_bit(source);
        }
    }

    fn generate_rook_moves(&mut self) {
        let mut bitboard = self.pieces[self.side_to_move as usize][Piece::Rook as usize];
        while bitboard.0 > 0 {
            let source = bitboard.get_lsb() as usize;
            let attacks = get_rook_attacks_from_table(
                Square::from(source),
                self.occupancies[Side::Both as usize],
            );
            self.add_attacks(source, attacks);
            bitboard.clear_bit(source);
        }
    }

    fn generate_queen_moves(&mut self) {
        let mut bitboard = self.pieces[self.side_to_move as usize][Piece::Queen as usize];
        while bitboard.0 > 0 {
            let source = bitboard.get_lsb() as usize;
            let attacks = get_queen_attacks_from_table(
                Square::from(source),
                self.occupancies[Side::Both as usize],
            );
            self.add_attacks(source, attacks);
            bitboard.clear_bit(source);
        }
    }

    fn generate_king_moves(&mut self) {
        let source =
            self.pieces[self.side_to_move as usize][Piece::King as usize].get_lsb() as usize;
        let attacks = Bitboard(king_attacks()[source]);

        self.add_attacks(source, attacks);
        self.generate_castle_moves();
    }

    // TODO: Revisit efficiency of pseudo-legal vs legal move generation
    fn generate_castle_moves(&mut self) {
        let occ = self.occupancies[Side::Both as usize];

        if self.side_to_move == Side::White {
            if self.castle.contains(Castle::WHITE_SHORT)
                && occ.get_bit(Square::F1 as usize) == 0
                && occ.get_bit(Square::G1 as usize) == 0
                && !self.is_square_attacked(Square::E1, Side::Black)
                && !self.is_square_attacked(Square::F1, Side::Black)
            {
                self.moves
                    .push(Move::new(Square::E1, Square::G1, MoveType::Castle));
            }
            if self.castle.contains(Castle::WHITE_LONG)
                && occ.get_bit(Square::D1 as usize) == 0
                && occ.get_bit(Square::C1 as usize) == 0
                && occ.get_bit(Square::B1 as usize) == 0
                && !self.is_square_attacked(Square::E1, Side::Black)
                && !self.is_square_attacked(Square::D1, Side::Black)
            {
                self.moves
                    .push(Move::new(Square::E1, Square::C1, MoveType::Castle));
            }
        } else {
            if self.castle.contains(Castle::BLACK_SHORT)
                && occ.get_bit(Square::F8 as usize) == 0
                && occ.get_bit(Square::G8 as usize) == 0
                && !self.is_square_attacked(Square::E8, Side::White)
                && !self.is_square_attacked(Square::F8, Side::White)
            {
                self.moves
                    .push(Move::new(Square::E8, Square::G8, MoveType::Castle));
            }
            if self.castle.contains(Castle::BLACK_LONG)
                && occ.get_bit(Square::D8 as usize) == 0
                && occ.get_bit(Square::C8 as usize) == 0
                && occ.get_bit(Square::B8 as usize) == 0
                && !self.is_square_attacked(Square::E8, Side::White)
                && !self.is_square_attacked(Square::D8, Side::White)
            {
                self.moves
                    .push(Move::new(Square::E8, Square::C8, MoveType::Castle));
            }
        }
    }

    fn add_attacks(&mut self, source: usize, mut attacks: Bitboard) {
        while attacks.0 > 0 {
            let target = attacks.get_lsb() as usize;
            if self.occupancies[self.side_to_move as usize].get_bit(target) == 1 {
                attacks.clear_bit(target);
                continue;
            }
            self.add_move_to_moves_list(source, target);
            attacks.clear_bit(target);
        }
    }

    fn add_move_to_moves_list(&mut self, source: usize, target: usize) {
        let move_type = if self.is_square_capture(target) {
            MoveType::Capture
        } else {
            MoveType::Quiet
        };
        self.moves.push(Move::new(
            Square::from(source),
            Square::from(target),
            move_type,
        ));
    }

    fn add_pawn_move(&mut self, source: usize, target: usize, enpassant: bool, double_push: bool) {
        let on_rank1 = target >= Square::A1 as usize && target <= Square::H1 as usize;
        let on_rank8 = target >= Square::A8 as usize && target <= Square::H8 as usize;

        if on_rank1 || on_rank8 {
            let capture = self.is_square_capture(target);
            let src = Square::from(source);
            let tgt = Square::from(target);
            self.moves.push(Move::new(
                src,
                tgt,
                if capture {
                    MoveType::KnightPromotionCapture
                } else {
                    MoveType::KnightPromotion
                },
            ));
            self.moves.push(Move::new(
                src,
                tgt,
                if capture {
                    MoveType::BishopPromotionCapture
                } else {
                    MoveType::BishopPromotion
                },
            ));
            self.moves.push(Move::new(
                src,
                tgt,
                if capture {
                    MoveType::RookPromotionCapture
                } else {
                    MoveType::RookPromotion
                },
            ));
            self.moves.push(Move::new(
                src,
                tgt,
                if capture {
                    MoveType::QueenPromotionCapture
                } else {
                    MoveType::QueenPromotion
                },
            ));
        } else if enpassant || double_push {
            self.moves.push(Move::new(
                Square::from(source),
                Square::from(target),
                if enpassant {
                    MoveType::EnPassant
                } else {
                    MoveType::DoublePush
                },
            ));
        } else {
            self.add_move_to_moves_list(source, target);
        }
    }

    fn is_square_capture(&self, target: usize) -> bool {
        self.occupancies[self.side_to_move.other() as usize].get_bit(target) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::helpers::{ADVANCED_MOVE_FEN, KIWI_PETE_FEN, STARTING_FEN};

    #[test]
    fn should_generate_correct_move_counts() {
        let mut starting = BoardState::parse_fen(STARTING_FEN);
        starting.generate_moves();
        assert_eq!(starting.moves.len(), 20, "Starting position move count");

        let mut kiwi = BoardState::parse_fen(KIWI_PETE_FEN);
        kiwi.generate_moves();
        assert_eq!(kiwi.moves.len(), 48, "KiwiPete move count");

        let mut advanced = BoardState::parse_fen(ADVANCED_MOVE_FEN);
        advanced.generate_moves();
        assert_eq!(advanced.moves.len(), 42, "AdvancedMove move count");
    }

    // If the move count ever goes wrong one of these tests usually helps catch edge cases missed
    #[test]
    fn starting_position_has_no_castle_moves() {
        let mut board = BoardState::parse_fen(STARTING_FEN);
        board.generate_moves();
        let castle_count = board.moves.iter().filter(|m| m.is_castle()).count();
        assert_eq!(castle_count, 0, "No castling from starting position");
    }

    #[test]
    fn kiwi_pete_has_castle_moves() {
        let mut board = BoardState::parse_fen(KIWI_PETE_FEN);
        board.generate_moves();
        let castle_count = board.moves.iter().filter(|m| m.is_castle()).count();
        assert_eq!(
            castle_count, 2,
            "KiwiPete should have exactly 2 castling options"
        );
    }

    #[test]
    fn advanced_fen_has_promotion_moves() {
        let mut board = BoardState::parse_fen(ADVANCED_MOVE_FEN);
        board.generate_moves();
        let promo_count = board.moves.iter().filter(|m| m.is_promotion()).count();
        assert_eq!(
            promo_count, 12,
            "AdvancedMove FEN should have exactly 12 promotions"
        );
    }

    #[test]
    fn advanced_fen_has_en_passant_move() {
        let mut board = BoardState::parse_fen(ADVANCED_MOVE_FEN);
        board.generate_moves();
        let ep_count = board
            .moves
            .iter()
            .filter(|m| m.move_type == MoveType::EnPassant)
            .count();
        assert_eq!(
            ep_count, 1,
            "AdvancedMove FEN should have exactly 1 en passant"
        );
    }
}
