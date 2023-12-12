use std::fs;
use advent::y23::y23d12::{solve_task1, solve_task2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("benches/y23/y23d12.txt").unwrap();
    c.bench_function("y23d12: part 1", |b| b.iter(|| solve_task1(black_box(&content))));
    c.bench_function("y23d12: part 2", |b| b.iter(|| solve_task2(black_box(&content))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
