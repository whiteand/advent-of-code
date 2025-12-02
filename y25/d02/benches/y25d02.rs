use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    y25d02::part1(black_box(input));
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    y25d02::part2(black_box(input));
}
