use std::{env::args, fs::read_to_string};

fn main() {
    let path_to_input = args().skip(1).next().unwrap();
    let mut elfes: Vec<i32> = read_to_string(path_to_input)
        .unwrap()
        .split("\n\n")
        .map(|single_str| {
            single_str
                .lines()
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    elfes.sort();

    let s: i32 = elfes[elfes.len() - 3..].iter().sum();

    println!("{s:?}");
}
