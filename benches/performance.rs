use criterion::{black_box, criterion_group, criterion_main, Criterion};
use game_of_life::{HEIGHT, print_state, WIDTH};
use game_of_life::new::{State, Board};
use game_of_life::old::next_step;

fn play(mut board: Board, n: u64) {
    for _ in 0..n {
        board = board.next_step();
    }
}

fn play_old(mut state: State, n: u64) {
    for _ in 0..n {
        state = next_step(state);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut board = Board::new();

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            board.state[row][col] = rand::random::<bool>();
        }
    }

    let steps = 10000;

    c.bench_function(&format!("game-of-life new impl, ({}x{}), random state, {} steps", HEIGHT, WIDTH, steps), |b| b.iter(|| play(board.clone(), black_box(steps))));
    c.bench_function(&format!("game-of-life original impl, ({}x{}), random state, {} steps", HEIGHT, WIDTH, steps), |b| b.iter(|| play_old(board.clone().state, black_box(steps))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
