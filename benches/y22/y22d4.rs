use std::fs;

use advent::y22::y22d04::{solve_task1, solve_task2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("benches/y22/y22d4.txt").unwrap();
    c.bench_function("solve 1", |b| b.iter(|| solve_task1(black_box(&content))));
    c.bench_function("solve 2", |b| b.iter(|| solve_task2(black_box(&content))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
