use divan::black_box;
use y16d12::{solve_part_1, solve_part_2};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    solve_part_1(black_box(input));
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    solve_part_2(black_box(input));
}
