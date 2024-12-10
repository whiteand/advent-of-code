use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use y23d09::{solve_part_1, solve_part_2};

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input.txt").unwrap();
    c.bench_function("y23d09: part 1", |b| {
        b.iter(|| solve_part_1(black_box(&content)))
    });
    c.bench_function("y23d09: part 2", |b| {
        b.iter(|| solve_part_2(black_box(&content)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
