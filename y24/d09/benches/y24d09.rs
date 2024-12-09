use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use y24d09::{arr_part2, solve, solve_part_1, solve_part_2};

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input.txt").unwrap();
    c.bench_function("y24d09: part 1", |b| {
        b.iter(|| solve_part_1(black_box(&content)))
    });
    c.bench_function("y24d09: part 2 (linked list)", |b| {
        b.iter(|| solve_part_2(black_box(&content)))
    });
    c.bench_function("y24d09: part 2 (array)", |b| {
        b.iter(|| solve(black_box(&content), arr_part2::checksum2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
