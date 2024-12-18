use std::io::Read;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

use y15d25::solve_part_1;

fn main() {
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    tracing_subscriber::registry().with(chrome_layer).init();

    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let instant = std::time::Instant::now();

    let result = solve_part_1(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
