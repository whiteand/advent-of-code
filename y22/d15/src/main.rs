use std::io::Read;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

use y22d15::{solve_part_1, solve_part_2};

fn main() {
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    tracing_subscriber::registry().with(chrome_layer).init();

    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instant = std::time::Instant::now();

    let result = solve_part_1(&input, 2000000);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = solve_part_2(&input, 0..=4000000, 0..=4000000);
    println!("Part 2: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
