use divan::black_box;
use y24d20::solve;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    solve::<100, 2>(black_box(input));
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    solve::<100, 20>(black_box(input));
}
