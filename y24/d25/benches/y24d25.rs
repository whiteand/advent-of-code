use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    y24d25::part1(black_box(input));
}
