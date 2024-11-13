use std::io::Read;
use y15d05::{solve_part_1, solve_part_2};

fn main() {
    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instant = std::time::Instant::now();

    let result = solve_part_1(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}ms", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = solve_part_2(&input);
    println!("Part 2: {}", result);
    println!("Time: {:?}ms", instant.elapsed());
}
