use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    y16d21::part1(black_box(input), "abcdefgh");
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    y16d21::part2(black_box(input), "gbhcefad");
}
