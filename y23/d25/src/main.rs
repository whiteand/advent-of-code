use std::io::Read;
use y23d25::solve_part_1;

fn main() {
    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let instant = std::time::Instant::now();

    let result = solve_part_1(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}ms", instant.elapsed());
}
