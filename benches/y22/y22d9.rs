use advent::y22::y22d09::{solve_task1, solve_task2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{fs, time::Duration};

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("benches/y22/y22d9.txt").unwrap();
    let mut group = c.benchmark_group("day 9");

    group.measurement_time(Duration::from_secs(6));

    group.bench_function("solve 1", |b| b.iter(|| solve_task1(black_box(&content))));
    group.bench_function("solve 2", |b| b.iter(|| solve_task2(black_box(&content))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
