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

        for &m in &board.moves {
            if m.source == move_obj.source && m.target == move_obj.target {
                if move_obj.move_type == MoveType::Quiet
                    || ((m.move_type.value() & !8) == move_obj.move_type.value())
                {
                    return m;
                }
            }
        }

        Move::NO_MOVE
    }
}
