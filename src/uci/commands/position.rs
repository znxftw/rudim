use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::uci::{UciClient, cli, reset_global, set_ready};

impl UciClient {
    pub(crate) fn run_position(&mut self, parameters: &[&str]) {
        if parameters.is_empty() {
            return;
        }

        match parameters[0] {
            "startpos" => {
                let moves = if parameters.len() > 2 && parameters[1] == "moves" {
                    &parameters[2..]
                } else {
                    &[][..]
                };
                self.parse_startpos(moves);
            }
            "fen" => {
                if parameters.len() < 7 {
                    return;
                }
                let fen = parameters[1..7].join(" ");
                let moves = if parameters.len() > 8 && parameters[7] == "moves" {
                    &parameters[8..]
                } else {
                    &[][..]
                };
                self.parse_fen(&fen, moves);
            }
            _ => {}
        }
    }

    fn parse_fen(&mut self, fen: &str, moves: &[&str]) {
        reset_global();
        *self.board.lock().unwrap() = crate::board::state::BoardState::parse_fen(fen);
        self.parse_moves(moves);
        set_ready();
    }

    fn parse_startpos(&mut self, moves: &[&str]) {
        reset_global();
        *self.board.lock().unwrap() = crate::board::state::BoardState::default();
        self.parse_moves(moves);
        set_ready();
    }

    fn parse_moves(&mut self, moves: &[&str]) {
        for &move_string in moves {
            let move_obj = match Move::parse_long_algebraic(move_string) {
                Some(m) => m,
                None => {
                    cli::write_line("Invalid Move");
                    return;
                }
            };

            let found_move = self.find_move_from_move_list(move_obj);
            if found_move == Move::NO_MOVE {
                cli::write_line("Invalid Move");
                return;
            }

            self.board.lock().unwrap().make_move(found_move);
        }
    }

    fn find_move_from_move_list(&mut self, move_obj: Move) -> Move {
        let mut board = self.board.lock().unwrap();
        board.generate_moves();

        for m in &board.moves {
            if m.mv.source == move_obj.source
                && m.mv.target == move_obj.target
                && (move_obj.move_type == MoveType::Quiet
                    || ((m.mv.move_type.value() & !8) == move_obj.move_type.value()))
            {
                return m.mv;
            }
        }

        Move::NO_MOVE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::BoardState;
    use serial_test::serial;

    fn find_move_from_move_list(board: &mut BoardState, move_obj: Move) -> Move {
        board.generate_moves();
        for m in &board.moves {
            if m.mv.source == move_obj.source
                && m.mv.target == move_obj.target
                && (move_obj.move_type == MoveType::Quiet
                    || ((m.mv.move_type.value() & !8) == move_obj.move_type.value()))
            {
                return m.mv;
            }
        }
        Move::NO_MOVE
    }

    #[test]
    #[serial]
    fn should_set_position_from_fen() {
        let mut uci_client = UciClient::new();
        let fen = "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        uci_client.run_position(&[
            "fen",
            "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
        ]);

        assert_eq!(
            BoardState::parse_fen(fen),
            *uci_client.board.lock().unwrap()
        );
    }

    #[test]
    #[serial]
    fn should_set_position_to_start_pos() {
        let mut uci_client = UciClient::new();

        uci_client.run_position(&["startpos"]);

        assert_eq!(BoardState::default(), *uci_client.board.lock().unwrap());
    }

    #[test]
    #[serial]
    fn should_set_position_to_start_pos_and_apply_moves() {
        let mut uci_client = UciClient::new();

        uci_client.run_position(&["startpos", "moves", "e2e4", "e7e5"]);

        let mut expected_state = BoardState::default();
        let white_move = find_move_from_move_list(
            &mut expected_state,
            Move::parse_long_algebraic("e2e4").unwrap(),
        );
        expected_state.make_move(white_move);
        let black_move = find_move_from_move_list(
            &mut expected_state,
            Move::parse_long_algebraic("e7e5").unwrap(),
        );
        expected_state.make_move(black_move);

        assert_eq!(expected_state, *uci_client.board.lock().unwrap());
    }
}
