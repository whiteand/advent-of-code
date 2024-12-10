use std::io::Read;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

use y22d17::{solve_part_1, solve_part_2};

fn main() {
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    tracing_subscriber::registry().with(chrome_layer).init();

    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instant = std::time::Instant::now();

    let result = solve_part_1::<7>(&input, 2022);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = solve_part_2::<1_000_000_000_000>(&input);
    println!("Part 2: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
