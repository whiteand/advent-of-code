use std::io::Read;

use advent_utils::rand::{
    self,
    rngs::{SmallRng, ThreadRng},
    Rng,
};
use itertools::Itertools;
use y24d17::{solve_part_1, solve_part_2, solve_part_2_genetic};

fn main() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::FmtSubscriber::builder()
            .without_time()
            .finish(),
    );

    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instant = std::time::Instant::now();

    let result = solve_part_1(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let proper_result = solve_part_2(&input);
    println!("Part 2: {}", proper_result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();

    let genetic_result = solve_part_2_genetic(
        &input,
        [
            178, 245, 208, 101, 141, 7, 242, 34, 144, 197, 134, 138, 51, 129, 4, 252, 145, 81, 204,
            236, 101, 38, 146, 254, 110, 86, 18, 236, 72, 244, 117, 215,
        ],
        10,
    );

    println!("Part 2 (genetic): {}", genetic_result);
    println!("Time: {:?}", instant.elapsed());

    if proper_result != genetic_result {
        println!("Invalid genetic answer")
    }
}
