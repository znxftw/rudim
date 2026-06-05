use rand::seq::IndexedRandom;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Result, Write};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;

use crate::board::state::BoardState;
use crate::common::castle::Castle as RudimCastle;
use crate::common::move_list::MoveList;
use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::piece::Piece as RudimPiece;
use crate::common::side::Side as RudimSide;
use crate::common::square::Square as RudimSquare;
use crate::search::search_state::SearchState;

use bullet_lib::game::formats::viriformat::{
    chess::board::Board as ViriBoard,
    chess::chessmove::{Move as ViriMove, MoveFlags as ViriMoveFlags},
    chess::piece::{Colour as ViriColour, Piece as ViriPiece, PieceType as ViriPieceType},
    chess::types::{CastlingRights as ViriCastlingRights, Square as ViriSquare},
    dataformat::Game as ViriGame,
};

pub struct SelfPlayPosition {
    pub board_state: BoardState,
    pub mv: Move,
    pub engine_eval: i16,
}

struct CompletedGame {
    initial_state: BoardState,
    initial_eval: i16,
    positions: Vec<SelfPlayPosition>,
    outcome: RudimSide,
    end_board_hash: u64,
}

pub fn load_opening_book(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let fens: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    if fens.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, "Opening book is empty"));
    }
    Ok(fens)
}

pub fn board_state_to_viriboard(rudim_state: &BoardState) -> ViriBoard {
    let mut viriboard = ViriBoard::new();

    *viriboard.turn_mut() = match rudim_state.side_to_move {
        RudimSide::White => ViriColour::White,
        RudimSide::Black => ViriColour::Black,
        _ => panic!("Invalid turn side"),
    };

    *viriboard.ep_sq_mut() = if rudim_state.en_passant_square == RudimSquare::NoSquare {
        None
    } else {
        // Viriboard mapping is flipped
        let viri_ep_idx = (rudim_state.en_passant_square as u8) ^ 56;
        ViriSquare::new(viri_ep_idx)
    };

    // Viriboard castling is represented differently to Rudim.
    let mut castling = ViriCastlingRights::NONE;
    if rudim_state.castle.contains(RudimCastle::WHITE_SHORT) {
        castling.wk = Some(ViriSquare::H1);
    }
    if rudim_state.castle.contains(RudimCastle::WHITE_LONG) {
        castling.wq = Some(ViriSquare::A1);
    }
    if rudim_state.castle.contains(RudimCastle::BLACK_SHORT) {
        castling.bk = Some(ViriSquare::H8);
    }
    if rudim_state.castle.contains(RudimCastle::BLACK_LONG) {
        castling.bq = Some(ViriSquare::A8);
    }
    *viriboard.castling_rights_mut() = castling;

    *viriboard.halfmove_clock_mut() = rudim_state.half_move_clock;
    viriboard.set_fullmove_clock((1 + rudim_state.move_count / 2) as u16);

    for rudim_idx in 0..64 {
        let piece = rudim_state.piece_mapping[rudim_idx];
        if piece != RudimPiece::None {
            let side = if rudim_state.occupancies[RudimSide::White].get_bit(rudim_idx) == 1 {
                ViriColour::White
            } else {
                ViriColour::Black
            };

            let viri_pt = match piece {
                RudimPiece::Pawn => ViriPieceType::Pawn,
                RudimPiece::Knight => ViriPieceType::Knight,
                RudimPiece::Bishop => ViriPieceType::Bishop,
                RudimPiece::Rook => ViriPieceType::Rook,
                RudimPiece::Queen => ViriPieceType::Queen,
                RudimPiece::King => ViriPieceType::King,
                RudimPiece::None => unreachable!(),
            };

            // Viriboard mapping is flipped
            let viri_sq_idx = (rudim_idx as u8) ^ 56;
            let viri_sq = ViriSquare::new_clamped(viri_sq_idx);

            viriboard.add_piece(viri_sq, ViriPiece::new(side, viri_pt));
        }
    }

    viriboard.regenerate_zobrist();
    viriboard.regenerate_threats();

    viriboard
}

pub fn map_rudim_move(m: &Move) -> ViriMove {
    let from_viri = ViriSquare::new_clamped((m.source as u8) ^ 56);
    let to_viri = if m.move_type == MoveType::Castle {
        // Viriboard maps castle differently
        let rook_sq = match m.target {
            RudimSquare::G1 => RudimSquare::H1,
            RudimSquare::C1 => RudimSquare::A1,
            RudimSquare::G8 => RudimSquare::H8,
            RudimSquare::C8 => RudimSquare::A8,
            _ => m.target,
        };
        ViriSquare::new_clamped((rook_sq as u8) ^ 56)
    } else {
        ViriSquare::new_clamped((m.target as u8) ^ 56)
    };
    match m.move_type {
        MoveType::EnPassant => {
            ViriMove::new_with_flags(from_viri, to_viri, ViriMoveFlags::EnPassant)
        }
        MoveType::Castle => ViriMove::new_with_flags(from_viri, to_viri, ViriMoveFlags::Castle),
        MoveType::QueenPromotion | MoveType::QueenPromotionCapture => {
            ViriMove::new_with_promo(from_viri, to_viri, ViriPieceType::Queen)
        }
        MoveType::RookPromotion | MoveType::RookPromotionCapture => {
            ViriMove::new_with_promo(from_viri, to_viri, ViriPieceType::Rook)
        }
        MoveType::BishopPromotion | MoveType::BishopPromotionCapture => {
            ViriMove::new_with_promo(from_viri, to_viri, ViriPieceType::Bishop)
        }
        MoveType::KnightPromotion | MoveType::KnightPromotionCapture => {
            ViriMove::new_with_promo(from_viri, to_viri, ViriPieceType::Knight)
        }
        _ => ViriMove::new(from_viri, to_viri),
    }
}

pub fn write_game_to_binpack<W: Write>(
    initial_state: &BoardState,
    initial_eval: i16,
    positions: &[SelfPlayPosition],
    outcome: RudimSide,
    writer: &mut W,
) -> Result<()> {
    let start_board = board_state_to_viriboard(initial_state);

    let initial_white_pov_eval = if initial_state.side_to_move == RudimSide::White {
        initial_eval
    } else {
        -initial_eval
    };

    // 2 = Win, 1 = Draw, 0 = Loss
    let wdl_outcome = match outcome {
        RudimSide::White => 2,
        RudimSide::Black => 0,
        _ => 1,
    };

    // Direct creation of PackedBoard using to_marlinformat (which handles private fields)
    let initial_position = start_board.to_marlinformat(initial_white_pov_eval, wdl_outcome, 0);

    let mut game = ViriGame {
        initial_position,
        moves: Vec::with_capacity(positions.len()),
    };

    for pos in positions {
        let viri_move = map_rudim_move(&pos.mv);
        let white_pov_eval = if pos.board_state.side_to_move == RudimSide::White {
            pos.engine_eval
        } else {
            -pos.engine_eval
        };
        game.add_move(viri_move, white_pov_eval);
    }

    game.serialise_into(writer)
}

pub fn run(output_path: &str, num_games: usize, book_path: &str, depth: u8, num_threads: usize) {
    println!("Loading opening book from '{}'...", book_path);
    let book_fens = match load_opening_book(book_path) {
        Ok(fens) => fens,
        Err(e) => {
            println!("Error loading opening book: {}", e);
            return;
        }
    };
    println!("Loaded {} starting positions from book.", book_fens.len());

    let file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path)
    {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening output file: {}", e);
            return;
        }
    };
    let mut writer = BufWriter::new(file);

    let (tx, rx) = mpsc::sync_channel(256);

    let games_per_thread = num_games / num_threads;
    let remainder = num_games % num_threads;

    let book_fens_ref = &book_fens;

    std::thread::scope(|s| {
        for t in 0..num_threads {
            let tx = tx.clone();
            let thread_games = games_per_thread + if t < remainder { 1 } else { 0 };

            s.spawn(move || {
                let mut rng = rand::rng();
                let cancellation_token = AtomicBool::new(false);
                let mut debug_mode = false;
                let mut search_state = SearchState::new();

                for _ in 0..thread_games {
                    let starting_fen = book_fens_ref.choose(&mut rng).unwrap();
                    let mut board_state = BoardState::parse_fen(starting_fen);
                    let initial_state = board_state.clone();

                    search_state.reset_search();
                    let _ = board_state.find_best_move(
                        depth,
                        &cancellation_token,
                        &mut debug_mode,
                        &mut search_state,
                    );
                    let initial_eval = search_state.score;

                    let mut positions = Vec::new();
                    let outcome;

                    loop {
                        if board_state.is_draw() {
                            outcome = RudimSide::Both;
                            break;
                        }

                        let mut move_list = MoveList::new();
                        board_state.generate_moves(&mut move_list);

                        let mut has_legal_move = false;
                        for m_entry in move_list.iter() {
                            let m = m_entry.mv;
                            board_state.make_move(m);
                            let is_legal =
                                !board_state.is_in_check(board_state.side_to_move.other());
                            board_state.unmake_move(m);
                            if is_legal {
                                has_legal_move = true;
                                break;
                            }
                        }

                        if !has_legal_move {
                            if board_state.is_in_check(board_state.side_to_move) {
                                outcome = board_state.side_to_move.other();
                            } else {
                                outcome = RudimSide::Both;
                            }
                            break;
                        }

                        search_state.reset_search();
                        let best_move = board_state.find_best_move(
                            depth,
                            &cancellation_token,
                            &mut debug_mode,
                            &mut search_state,
                        );
                        let score = search_state.score;

                        positions.push(SelfPlayPosition {
                            board_state: board_state.clone(),
                            mv: best_move,
                            engine_eval: score,
                        });

                        board_state.make_move(best_move);
                    }

                    let game_data = CompletedGame {
                        initial_state,
                        initial_eval,
                        positions,
                        outcome,
                        end_board_hash: board_state.board_hash,
                    };
                    let _ = tx.send(game_data);
                }
            });
        }

        // Drop the main thread's sender so that when all worker threads exit,
        // no senders remain and the receiver loop terminates.
        drop(tx);

        s.spawn(move || {
            let mut games_written = 0;
            let mut end_positions = HashSet::new();
            let mut total_positions = 0;
            let mut white_wins = 0;
            let mut black_wins = 0;
            let mut draws = 0;

            while let Ok(game) = rx.recv() {
                end_positions.insert(game.end_board_hash);
                if let Err(e) = write_game_to_binpack(
                    &game.initial_state,
                    game.initial_eval,
                    &game.positions,
                    game.outcome,
                    &mut writer,
                ) {
                    println!("Error writing game to file: {}", e);
                    break;
                }
                games_written += 1;
                total_positions += game.positions.len();

                match game.outcome {
                    RudimSide::White => white_wins += 1,
                    RudimSide::Black => black_wins += 1,
                    _ => draws += 1,
                }

                if games_written % 1000 == 0 {
                    let _ = writer.flush();
                }

                if games_written % 10000 == 0 {
                    let avg_len = total_positions as f64 / games_written as f64;
                    println!("----------------------------------------");
                    println!("Games completed     : {}", games_written);
                    println!("Games left          : {}", num_games - games_written);
                    println!("Total positions     : {}", total_positions);
                    println!("Total White Wins    : {}", white_wins);
                    println!("Total Black Wins    : {}", black_wins);
                    println!("Total Draws         : {}", draws);
                    println!("Average Game Length : {:.2}", avg_len);
                    println!("----------------------------------------");
                }
            }

            let _ = writer.flush();
            let avg_len = if games_written > 0 {
                total_positions as f64 / games_written as f64
            } else {
                0.0
            };
            println!("----------------------------------------");
            println!("Final Data Generation Summary:");
            println!("Games completed     : {}", games_written);
            println!("Games left          : {}", num_games - games_written);
            println!("Total positions     : {}", total_positions);
            println!("Total White Wins    : {}", white_wins);
            println!("Total Black Wins    : {}", black_wins);
            println!("Total Draws         : {}", draws);
            println!("Average Game Length : {:.2}", avg_len);
            println!("----------------------------------------");
            println!(
                "Data generation finished. Successfully wrote {} games to '{}' ({} unique end positions).",
                games_written,
                output_path,
                end_positions.len()
            );
        });
    });
}
