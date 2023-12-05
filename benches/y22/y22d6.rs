use advent::y22::y22d06::{solve_task1, solve_task2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("benches/y22/y22d6.txt").unwrap();
    c.bench_function("solve 1", |b| b.iter(|| solve_task1(black_box(&content))));
    c.bench_function("solve 2", |b| b.iter(|| solve_task2(black_box(&content))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
