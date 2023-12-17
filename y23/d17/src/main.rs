use std::io::Read;
use y23d17::solve;

fn main() {
    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instant = std::time::Instant::now();

    let result = solve(&input, 1, 3);
    println!("Task 1: {}", result);
    println!("Time: {:?}ms", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = solve(&input, 4, 10);
    println!("Task 2: {}", result);
    println!("Time: {:?}ms", instant.elapsed());
}
