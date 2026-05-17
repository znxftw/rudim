use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rudim::board::state::BoardState;
use rudim::common::helpers::{ADVANCED_MOVE_FEN, ENDGAME_FEN, KIWI_PETE_FEN, STARTING_FEN};
use rudim::engine;
use std::sync::Once;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

static INIT: Once = Once::new();

fn ensure_initialized() {
    INIT.call_once(rudim::init);
}

fn benchmark_find_best_move(c: &mut Criterion) {
    ensure_initialized();

    let cases = [
        ("AdvancedMove", ADVANCED_MOVE_FEN, 9),
        ("AdvancedMove", ADVANCED_MOVE_FEN, 10),
        ("Starting", STARTING_FEN, 8),
        ("Starting", STARTING_FEN, 9),
        ("Endgame", ENDGAME_FEN, 11),
        ("Endgame", ENDGAME_FEN, 12),
        ("KiwiPete", KIWI_PETE_FEN, 6),
        ("KiwiPete", KIWI_PETE_FEN, 7),
    ];

    let mut group = c.benchmark_group("find_best_move");
    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(25));

    for (name, fen, depth) in cases {
        group.bench_with_input(
            BenchmarkId::new(name, depth),
            &(fen, depth),
            |b, &(fen, depth)| {
                b.iter(|| {
                    engine::reset();
                    let mut board_state = BoardState::parse_fen(fen);
                    let cancellation_token = AtomicBool::new(false);
                    let mut debug_mode = false;
                    let _best_move =
                        board_state.find_best_move(depth, &cancellation_token, &mut debug_mode);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_find_best_move);
criterion_main!(benches);
