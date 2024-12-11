use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use y22d17::{solve_part_1, solve_part_2};

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input.txt").unwrap();
    c.bench_function("y22d17: part 1", |b| {
        b.iter(|| solve_part_1::<7>(black_box(&content), black_box(2022)))
    });
    c.bench_function("y22d17: part 2", |b| {
        b.iter(|| solve_part_2::<1_000_000_000_000>(black_box(&content)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
