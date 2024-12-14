use divan::black_box;
use y16d11::solve;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    solve(black_box(y16d11::ACTUAL));
}

#[divan::bench]
fn part2() {
    solve(black_box(y16d11::ACTUAL2));
}
