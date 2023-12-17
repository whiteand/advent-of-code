use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use y23d17::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input.txt").unwrap();
    c.bench_function("y23d17: part 1", |b| {
        b.iter(|| solve(black_box(&content), black_box(1), black_box(3)))
    });
    c.bench_function("y23d17: part 2", |b| {
        b.iter(|| solve(black_box(&content), black_box(4), black_box(10)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
