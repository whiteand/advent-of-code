use advent::y23::y23d11::solve;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("benches/y23/y23d11.txt").unwrap();
    c.bench_function("y23d11: part 1", |b| {
        b.iter(|| solve::<2>(black_box(&content)))
    });
    c.bench_function("y23d11: part 2", |b| {
        b.iter(|| solve::<1000000>(black_box(&content)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
