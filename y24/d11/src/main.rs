use std::io::Read;

use y24d11::{solve_part_1, solve_part_2};

fn main() {
    let _guard =
        tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::builder().finish());
    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instant = std::time::Instant::now();

    let result = solve_part_1(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = solve_part_2(&input);
    println!("Part 2: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
