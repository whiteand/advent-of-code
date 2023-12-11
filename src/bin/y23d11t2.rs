use advent::y23::y23d11::solve;
use std::{env::args, fs::read_to_string};

fn main() {
    let path_to_input = args().nth(1).unwrap();
    let file_content = read_to_string(path_to_input).unwrap();
    let answer = solve::<1000000>(&file_content);

    println!("Answer: {answer}")
}
