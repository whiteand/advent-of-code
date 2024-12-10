use std::{env::args, fs::read_to_string};

use advent::y22::y22d03::solve_part1;

fn main() {
    let path_to_input = args().nth(1).unwrap();
    let file_content = read_to_string(path_to_input).unwrap();
    let s = solve_part1(&file_content);

    println!("Output: {s:?}");
}
