use divan::black_box;
use y16d11::{solve_part_1, solve_part_2};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    solve_part_1(black_box(y16d11::ACTUAL));
}

#[divan::bench]
fn part2() {
    solve_part_2(black_box(y16d11::ACTUAL));
}
