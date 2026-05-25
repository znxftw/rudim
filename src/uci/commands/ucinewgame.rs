use crate::board::state::BoardState;
use crate::uci::{UciClient, reset_global, set_ready};

impl UciClient {
    pub(crate) fn run_ucinewgame(&mut self, _parameters: &[&str]) {
        reset_global();
        *self.board.lock().unwrap() = BoardState::default();
        set_ready();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::move_type::MoveType;
    use crate::common::moves::Move;
    use crate::common::square::Square;
    use crate::eval::move_ordering;
    use crate::uci::is_ready;
    use serial_test::serial;

    #[test]
    #[serial]
    fn should_reset_program() {
        let mut uci_client = UciClient::new();
        uci_client.board = std::sync::Arc::new(std::sync::Mutex::new(BoardState::parse_fen(
            "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        )));

        move_ordering::add_killer_move(Move::new(Square::E2, Square::E3, MoveType::Quiet), 0);

        {
            let mut board = uci_client.board.lock().unwrap();
            board.history.save(
                crate::common::piece::Piece::None,
                Square::NoSquare,
                crate::common::castle::Castle::NONE,
                0,
                0,
            );
        }

        assert_ne!(*uci_client.board.lock().unwrap(), BoardState::default());
        assert!(!move_ordering::is_move_heuristic_empty());
        assert!(!uci_client.board.lock().unwrap().history.is_empty());

        uci_client.run_ucinewgame(&[]);

        assert_eq!(*uci_client.board.lock().unwrap(), BoardState::default());
        assert!(move_ordering::is_move_heuristic_empty());
        assert!(uci_client.board.lock().unwrap().history.is_empty());
    }

    #[test]
    #[serial]
    fn should_be_ready_after_reset() {
        let mut uci_client = UciClient::new();

        uci_client.run_ucinewgame(&[]);

        assert!(is_ready());
    }

    #[test]
    #[serial]
    fn should_restore_ready_state_after_reset() {
        let mut uci_client = UciClient::new();
        set_ready();

        assert!(is_ready());

        uci_client.run_ucinewgame(&[]);

        assert!(is_ready());
    }
}
