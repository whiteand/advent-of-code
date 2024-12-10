use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use y23d11::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input.txt").unwrap();
    c.bench_function("y23d11: part 1", |b| {
        b.iter(|| solve::<2>(black_box(&content)))
    });
    c.bench_function("y23d11: part 2", |b| {
        b.iter(|| solve::<1000000>(black_box(&content)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
