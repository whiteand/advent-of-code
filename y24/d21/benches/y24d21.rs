use divan::black_box;
use y24d21::solve;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    solve::<2>(black_box(input));
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    solve::<25>(black_box(input));
}
