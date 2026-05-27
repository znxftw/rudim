use crate::bitboard::Bitboard;
use crate::bitboard::lookups::{
    get_bishop_attacks_from_table, get_queen_attacks_from_table, get_rook_attacks_from_table,
    king_attacks, knight_attacks, pawn_attacks,
};
use crate::board::state::BoardState;
use crate::common::castle::Castle;
use crate::common::move_list::{MoveList, ScoredMove};
use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::search::iterative_deepening;
use std::sync::atomic::AtomicBool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveGenType {
    Captures,
    Quiets,
}

impl BoardState {
    pub fn find_best_move(
        &mut self,
        depth: u8,
        cancellation_token: &AtomicBool,
        debug_mode: &mut bool,
    ) -> Move {
        iterative_deepening::search(self, depth, cancellation_token, debug_mode);
        iterative_deepening::best_move()
    }

    pub fn generate_moves(&self, move_list: &mut MoveList) {
        move_list.clear();
        self.generate_captures_internal(move_list);
        self.generate_quiets_internal(move_list);
    }

    pub fn generate_captures(&self, move_list: &mut MoveList) {
        move_list.clear();
        self.generate_captures_internal(move_list);
    }

    pub fn generate_quiets(&self, move_list: &mut MoveList) {
        move_list.clear();
        self.generate_quiets_internal(move_list);
    }

    fn generate_captures_internal(&self, move_list: &mut MoveList) {
        self.generate_moves_selective(move_list, MoveGenType::Captures);
    }

    fn generate_quiets_internal(&self, move_list: &mut MoveList) {
        self.generate_moves_selective(move_list, MoveGenType::Quiets);
    }

    fn generate_moves_selective(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        self.generate_pawn_moves(move_list, gen_type);
        self.generate_bishop_moves(move_list, gen_type);
        self.generate_knight_moves(move_list, gen_type);
        self.generate_rook_moves(move_list, gen_type);
        self.generate_queen_moves(move_list, gen_type);
        self.generate_king_moves(move_list, gen_type);
    }

    fn generate_pawn_moves(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        let mut bitboard = self.pieces[self.side_to_move][Piece::Pawn];
        while bitboard.is_not_empty() {
            let source = bitboard.get_lsb() as usize;
            match gen_type {
                MoveGenType::Quiets => {
                    self.generate_pawn_pushes(source, move_list, gen_type);
                }
                MoveGenType::Captures => {
                    self.generate_en_passants(source, move_list, gen_type);
                    self.generate_pawn_attacks(source, move_list, gen_type);
                }
            }
            bitboard.clear_bit(source);
        }
    }

    fn generate_pawn_pushes(&self, source: usize, move_list: &mut MoveList, gen_type: MoveGenType) {
        let both_occ = self.occupancies[Side::Both];

        if self.side_to_move == Side::Black {
            let one_sq = source + 8;
            if both_occ.get_bit(one_sq) != 0 {
                return;
            }
            self.add_pawn_move(source, one_sq, false, false, move_list, gen_type);

            if source >= Square::A7 as usize && source <= Square::H7 as usize {
                let two_sq = one_sq + 8;
                if both_occ.get_bit(two_sq) != 0 {
                    return;
                }
                self.add_pawn_move(source, two_sq, false, true, move_list, gen_type);
            }
        } else {
            let one_sq = source - 8;
            if both_occ.get_bit(one_sq) != 0 {
                return;
            }
            self.add_pawn_move(source, one_sq, false, false, move_list, gen_type);

            if source >= Square::A2 as usize && source <= Square::H2 as usize {
                let two_sq = one_sq - 8;
                if both_occ.get_bit(two_sq) != 0 {
                    return;
                }
                self.add_pawn_move(source, two_sq, false, true, move_list, gen_type);
            }
        }
    }

    fn generate_en_passants(&self, source: usize, move_list: &mut MoveList, gen_type: MoveGenType) {
        if self.en_passant_square == Square::NoSquare {
            return;
        }
        let ep_bit = 1u64 << (self.en_passant_square as usize);
        let attacks = Bitboard(pawn_attacks()[self.side_to_move as usize][source] & ep_bit);
        if attacks.is_not_empty() {
            let target = attacks.get_lsb() as usize;
            self.add_pawn_move(source, target, true, false, move_list, gen_type);
        }
    }

    fn generate_pawn_attacks(
        &self,
        source: usize,
        move_list: &mut MoveList,
        gen_type: MoveGenType,
    ) {
        let enemy_occ = self.occupancies[self.side_to_move.other()];
        let mut attacks = enemy_occ & pawn_attacks()[self.side_to_move as usize][source];

        while attacks.is_not_empty() {
            let target = attacks.get_lsb() as usize;
            self.add_pawn_move(source, target, false, false, move_list, gen_type);
            attacks.clear_bit(target);
        }
    }

    fn generate_bishop_moves(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        let mut bitboard = self.pieces[self.side_to_move][Piece::Bishop];
        while bitboard.is_not_empty() {
            let source = bitboard.get_lsb() as usize;
            let attacks =
                get_bishop_attacks_from_table(Square::from(source), self.occupancies[Side::Both]);
            self.add_attacks(source, attacks, move_list, gen_type);
            bitboard.clear_bit(source);
        }
    }

    fn generate_knight_moves(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        let mut bitboard = self.pieces[self.side_to_move][Piece::Knight];
        while bitboard.is_not_empty() {
            let source = bitboard.get_lsb() as usize;
            let attacks = Bitboard(knight_attacks()[source]);
            self.add_attacks(source, attacks, move_list, gen_type);
            bitboard.clear_bit(source);
        }
    }

    fn generate_rook_moves(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        let mut bitboard = self.pieces[self.side_to_move][Piece::Rook];
        while bitboard.is_not_empty() {
            let source = bitboard.get_lsb() as usize;
            let attacks =
                get_rook_attacks_from_table(Square::from(source), self.occupancies[Side::Both]);
            self.add_attacks(source, attacks, move_list, gen_type);
            bitboard.clear_bit(source);
        }
    }

    fn generate_queen_moves(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        let mut bitboard = self.pieces[self.side_to_move][Piece::Queen];
        while bitboard.is_not_empty() {
            let source = bitboard.get_lsb() as usize;
            let attacks =
                get_queen_attacks_from_table(Square::from(source), self.occupancies[Side::Both]);
            self.add_attacks(source, attacks, move_list, gen_type);
            bitboard.clear_bit(source);
        }
    }

    fn generate_king_moves(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        let source = self.pieces[self.side_to_move][Piece::King].get_lsb() as usize;
        let attacks = Bitboard(king_attacks()[source]);

        self.add_attacks(source, attacks, move_list, gen_type);
        self.generate_castle_moves(move_list, gen_type);
    }

    fn generate_castle_moves(&self, move_list: &mut MoveList, gen_type: MoveGenType) {
        if gen_type == MoveGenType::Captures {
            return;
        }
        let occ = self.occupancies[Side::Both];

        if self.side_to_move == Side::White {
            if self.castle.contains(Castle::WHITE_SHORT)
                && occ.get_bit(Square::F1 as usize) == 0
                && occ.get_bit(Square::G1 as usize) == 0
                && !self.is_square_attacked(Square::E1, Side::Black)
                && !self.is_square_attacked(Square::F1, Side::Black)
            {
                move_list.push(ScoredMove::new(Square::E1, Square::G1, MoveType::Castle));
            }
            if self.castle.contains(Castle::WHITE_LONG)
                && occ.get_bit(Square::D1 as usize) == 0
                && occ.get_bit(Square::C1 as usize) == 0
                && occ.get_bit(Square::B1 as usize) == 0
                && !self.is_square_attacked(Square::E1, Side::Black)
                && !self.is_square_attacked(Square::D1, Side::Black)
            {
                move_list.push(ScoredMove::new(Square::E1, Square::C1, MoveType::Castle));
            }
        } else {
            if self.castle.contains(Castle::BLACK_SHORT)
                && occ.get_bit(Square::F8 as usize) == 0
                && occ.get_bit(Square::G8 as usize) == 0
                && !self.is_square_attacked(Square::E8, Side::White)
                && !self.is_square_attacked(Square::F8, Side::White)
            {
                move_list.push(ScoredMove::new(Square::E8, Square::G8, MoveType::Castle));
            }
            if self.castle.contains(Castle::BLACK_LONG)
                && occ.get_bit(Square::D8 as usize) == 0
                && occ.get_bit(Square::C8 as usize) == 0
                && occ.get_bit(Square::B8 as usize) == 0
                && !self.is_square_attacked(Square::E8, Side::White)
                && !self.is_square_attacked(Square::D8, Side::White)
            {
                move_list.push(ScoredMove::new(Square::E8, Square::C8, MoveType::Castle));
            }
        }
    }

    fn add_attacks(
        &self,
        source: usize,
        mut attacks: Bitboard,
        move_list: &mut MoveList,
        gen_type: MoveGenType,
    ) {
        while attacks.is_not_empty() {
            let target = attacks.get_lsb() as usize;
            if self.occupancies[self.side_to_move].get_bit(target) == 1 {
                attacks.clear_bit(target);
                continue;
            }
            let is_capture = self.is_square_capture(target);
            if gen_type == MoveGenType::Captures && !is_capture {
                attacks.clear_bit(target);
                continue;
            }
            if gen_type == MoveGenType::Quiets && is_capture {
                attacks.clear_bit(target);
                continue;
            }
            self.add_move_to_moves_list(source, target, move_list);
            attacks.clear_bit(target);
        }
    }

    fn add_move_to_moves_list(&self, source: usize, target: usize, move_list: &mut MoveList) {
        let move_type = if self.is_square_capture(target) {
            MoveType::Capture
        } else {
            MoveType::Quiet
        };
        move_list.push(ScoredMove::new(
            Square::from(source),
            Square::from(target),
            move_type,
        ));
    }

    fn add_pawn_move(
        &self,
        source: usize,
        target: usize,
        enpassant: bool,
        double_push: bool,
        move_list: &mut MoveList,
        gen_type: MoveGenType,
    ) {
        let on_rank1 = target >= Square::A1 as usize && target <= Square::H1 as usize;
        let on_rank8 = target >= Square::A8 as usize && target <= Square::H8 as usize;

        if on_rank1 || on_rank8 {
            let capture = self.is_square_capture(target);
            if gen_type == MoveGenType::Captures && !capture {
                return;
            }
            if gen_type == MoveGenType::Quiets && capture {
                return;
            }
            let src = Square::from(source);
            let tgt = Square::from(target);
            move_list.push(ScoredMove::new(
                src,
                tgt,
                if capture {
                    MoveType::KnightPromotionCapture
                } else {
                    MoveType::KnightPromotion
                },
            ));
            move_list.push(ScoredMove::new(
                src,
                tgt,
                if capture {
                    MoveType::BishopPromotionCapture
                } else {
                    MoveType::BishopPromotion
                },
            ));
            move_list.push(ScoredMove::new(
                src,
                tgt,
                if capture {
                    MoveType::RookPromotionCapture
                } else {
                    MoveType::RookPromotion
                },
            ));
            move_list.push(ScoredMove::new(
                src,
                tgt,
                if capture {
                    MoveType::QueenPromotionCapture
                } else {
                    MoveType::QueenPromotion
                },
            ));
        } else if enpassant || double_push {
            if enpassant {
                if gen_type == MoveGenType::Quiets {
                    return;
                }
            } else {
                if gen_type == MoveGenType::Captures {
                    return;
                }
            }
            move_list.push(ScoredMove::new(
                Square::from(source),
                Square::from(target),
                if enpassant {
                    MoveType::EnPassant
                } else {
                    MoveType::DoublePush
                },
            ));
        } else {
            let is_capture = self.is_square_capture(target);
            if gen_type == MoveGenType::Captures && !is_capture {
                return;
            }
            if gen_type == MoveGenType::Quiets && is_capture {
                return;
            }
            self.add_move_to_moves_list(source, target, move_list);
        }
    }

    fn is_square_capture(&self, target: usize) -> bool {
        self.occupancies[self.side_to_move.other()].get_bit(target) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::helpers::{ADVANCED_MOVE_FEN, KIWI_PETE_FEN, STARTING_FEN};

    #[test]
    fn should_generate_correct_move_counts() {
        let starting = BoardState::parse_fen(STARTING_FEN);
        let mut move_list = MoveList::new();
        starting.generate_moves(&mut move_list);
        assert_eq!(move_list.len(), 20, "Starting position move count");

        let kiwi = BoardState::parse_fen(KIWI_PETE_FEN);
        let mut move_list2 = MoveList::new();
        kiwi.generate_moves(&mut move_list2);
        assert_eq!(move_list2.len(), 48, "KiwiPete move count");

        let advanced = BoardState::parse_fen(ADVANCED_MOVE_FEN);
        let mut move_list3 = MoveList::new();
        advanced.generate_moves(&mut move_list3);
        assert_eq!(move_list3.len(), 42, "AdvancedMove move count");
    }

    // If the move count ever goes wrong one of these tests usually helps catch edge cases missed
    #[test]
    fn starting_position_has_no_castle_moves() {
        let board = BoardState::parse_fen(STARTING_FEN);
        let mut move_list = MoveList::new();
        board.generate_moves(&mut move_list);
        let castle_count = move_list.iter().filter(|m| m.mv.is_castle()).count();
        assert_eq!(castle_count, 0, "No castling from starting position");
    }

    #[test]
    fn kiwi_pete_has_castle_moves() {
        let board = BoardState::parse_fen(KIWI_PETE_FEN);
        let mut move_list = MoveList::new();
        board.generate_moves(&mut move_list);
        let castle_count = move_list.iter().filter(|m| m.mv.is_castle()).count();
        assert_eq!(
            castle_count, 2,
            "KiwiPete should have exactly 2 castling options"
        );
    }

    #[test]
    fn advanced_fen_has_promotion_moves() {
        let board = BoardState::parse_fen(ADVANCED_MOVE_FEN);
        let mut move_list = MoveList::new();
        board.generate_moves(&mut move_list);
        let promo_count = move_list.iter().filter(|m| m.mv.is_promotion()).count();
        assert_eq!(
            promo_count, 12,
            "AdvancedMove FEN should have exactly 12 promotions"
        );
    }

    #[test]
    fn advanced_fen_has_en_passant_move() {
        let board = BoardState::parse_fen(ADVANCED_MOVE_FEN);
        let mut move_list = MoveList::new();
        board.generate_moves(&mut move_list);
        let ep_count = move_list
            .iter()
            .filter(|m| m.mv.move_type == MoveType::EnPassant)
            .count();
        assert_eq!(
            ep_count, 1,
            "AdvancedMove FEN should have exactly 1 en passant"
        );
    }
}
