use crate::board::state::BoardState;
use crate::common::castle::Castle;
use crate::common::helpers::STARTING_FEN;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::common::zobrist;

impl BoardState {
    pub fn parse_fen(fen: &str) -> Self {
        let mut board = BoardState::new();
        let sections: Vec<&str> = fen.split(' ').collect();

        parse_pieces(&mut board, sections[0]);
        parse_side_to_move(&mut board, sections[1]);
        parse_castling(&mut board, sections[2]);
        parse_en_passant(&mut board, sections[3]);
        if sections.len() > 4 {
            parse_ply(&mut board, sections[4]);
        }
        if sections.len() > 5 {
            parse_move_count(&mut board, sections[5]);
        }
        board.board_hash = zobrist::get_board_hash(&board);

        board
    }

    pub fn starting_position() -> Self {
        Self::parse_fen(STARTING_FEN)
    }
}

fn parse_pieces(board: &mut BoardState, fen: &str) {
    for (rank, rank_str) in fen.split('/').enumerate() {
        let mut index = rank * 8;
        for symbol in rank_str.chars() {
            if symbol.is_ascii_alphabetic() {
                board.add_piece(
                    Square::from(index),
                    symbol_to_side(symbol),
                    symbol_to_piece(symbol),
                );
                index += 1;
            } else if let Some(skip) = symbol.to_digit(10) {
                index += skip as usize;
            }
        }
    }
}

fn parse_side_to_move(board: &mut BoardState, fen: &str) {
    board.side_to_move = if fen == "w" { Side::White } else { Side::Black };
}

fn parse_castling(board: &mut BoardState, fen: &str) {
    for ch in fen.chars() {
        match ch {
            'K' => board.castle |= Castle::WHITE_SHORT,
            'Q' => board.castle |= Castle::WHITE_LONG,
            'k' => board.castle |= Castle::BLACK_SHORT,
            'q' => board.castle |= Castle::BLACK_LONG,
            _ => {}
        }
    }
}

fn parse_en_passant(board: &mut BoardState, fen: &str) {
    if fen == "-" {
        return;
    }
    let bytes = fen.as_bytes();
    let file = (bytes[0] - b'a') as usize;
    let rank = (bytes[1] - b'1') as usize;
    let sq_index = (7 - rank) * 8 + file;
    board.en_passant_square = Square::from(sq_index);
}

fn parse_ply(board: &mut BoardState, halfmove: &str) {
    if let Ok(clock) = halfmove.parse::<u8>() {
        board.half_move_clock = clock;
    }
}

fn parse_move_count(board: &mut BoardState, fullmove: &str) {
    if let Ok(fm) = fullmove.parse::<i32>() {
        let plies = (fm.max(1) - 1) * 2;
        board.move_count = plies
            + if board.side_to_move == Side::White {
                0
            } else {
                1
            };
    }
}

pub fn symbol_to_piece(symbol: char) -> Piece {
    match symbol.to_ascii_lowercase() {
        'p' => Piece::Pawn,
        'r' => Piece::Rook,
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'q' => Piece::Queen,
        'k' => Piece::King,
        _ => Piece::None,
    }
}

pub fn symbol_to_side(symbol: char) -> Side {
    if symbol.is_ascii_uppercase() {
        Side::White
    } else {
        Side::Black
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::helpers::STARTING_FEN;

    #[test]
    fn should_parse_starting_fen_piece_bitboards() {
        let board = BoardState::parse_fen(STARTING_FEN);

        assert_eq!(board.pieces[Side::White][Piece::Pawn].0, 71776119061217280);
        assert_eq!(
            board.pieces[Side::White][Piece::Knight].0,
            4755801206503243776
        );
        assert_eq!(
            board.pieces[Side::White][Piece::Bishop].0,
            2594073385365405696
        );
        assert_eq!(
            board.pieces[Side::White][Piece::Rook].0,
            9295429630892703744
        );
        assert_eq!(
            board.pieces[Side::White][Piece::Queen].0,
            576460752303423488
        );
        assert_eq!(
            board.pieces[Side::White][Piece::King].0,
            1152921504606846976
        );

        assert_eq!(board.pieces[Side::Black][Piece::Pawn].0, 65280);
        assert_eq!(board.pieces[Side::Black][Piece::Knight].0, 66);
        assert_eq!(board.pieces[Side::Black][Piece::Bishop].0, 36);
        assert_eq!(board.pieces[Side::Black][Piece::Rook].0, 129);
        assert_eq!(board.pieces[Side::Black][Piece::Queen].0, 8);
        assert_eq!(board.pieces[Side::Black][Piece::King].0, 16);
    }
}
