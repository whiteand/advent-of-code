use advent_utils::{
    genetic, parse,
    rand::{self, rngs::SmallRng, Rng, SeedableRng},
};
use itertools::Itertools;
use std::{cmp::Reverse, ops::BitXor};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> String {
    let (a, b, c, program) = parse_program(file_content);
    let mut output = Vec::new();
    execute(&program, a, b, c, &mut output);
    output.into_iter().join(",")
}

fn execute(program: &[usize], mut a: usize, mut b: usize, mut c: usize, output: &mut Vec<usize>) {
    let mut ip = 0;
    while let Some(opcode) = program.get(ip) {
        match opcode {
            0 => {
                let num = a;
                let den = combo(a, b, c, program[ip + 1]);
                a = num >> den;
                ip += 2;
            }
            1 => {
                b = b.bitxor(program[ip + 1]);
                ip += 2;
            }
            2 => {
                b = combo(a, b, c, program[ip + 1]) & 0b111;
                ip += 2;
            }
            3 => {
                if a == 0 {
                    ip += 2
                } else {
                    ip = program[ip + 1];
                }
            }
            4 => {
                b = b.bitxor(c);
                ip += 2;
            }
            5 => {
                output.push(combo(a, b, c, program[ip + 1]) & 0b111);
                ip += 2;
            }
            6 => {
                let num = a;
                let den = combo(a, b, c, program[ip + 1]);
                b = num >> den;
                ip += 2;
            }
            7 => {
                let num = a;
                let den = combo(a, b, c, program[ip + 1]);
                c = num >> den;
                ip += 2;
            }
            x => {
                unreachable!("Opcode: {x}");
            }
        }
    }
}

fn parse_program(file_content: &str) -> (usize, usize, usize, Vec<usize>) {
    let mut nums = parse::nums::<usize>(file_content);
    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    let c = nums.next().unwrap();
    let program = nums.collect_vec();
    (a, b, c, program)
}

fn combo(a: usize, b: usize, c: usize, operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let (_, b, c, program) = parse_program(file_content);
    let mut tasks = vec![(0, program.len() - 1)];
    let mut output = Vec::new();
    let mut res = usize::MAX;
    while let Some((next_a, output_ptr)) = tasks.pop() {
        for i in 0..=0b111 {
            let new_a = next_a << 3 | i;
            output.truncate(0);
            execute(&program, new_a, b, c, &mut output);
            let expected_res = &program[output_ptr..];
            if output.len() != expected_res.len() {
                continue;
            }
            let actual_res = &output[(output.len() - expected_res.len())..];
            if actual_res != expected_res {
                continue;
            }
            if output_ptr == 0 {
                res = res.min(new_a);
                continue;
            }
            tasks.push((new_a, output_ptr - 1));
        }
    }
    res
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2_genetic(file_content: &str, seed: [u8; 32], max_repetitions: usize) -> usize {
    let (_, b, c, program) = parse_program(file_content);

    let mut rng = rand::rngs::SmallRng::from_seed(seed);

    let population = (0..=2048)
        .map(|_| rng.gen_range(0..usize::MAX))
        .collect_vec();
    let mut output = Vec::new();
    let mut evolution = genetic::Genetic::new(
        population,
        |p| {
            output.clear();
            execute(&program, *p, b, c, &mut output);
            if output.len() != program.len() {
                return (Reverse(0), *p);
            }
            let wrong_positions = output
                .iter()
                .rev()
                .zip(program.iter().rev())
                .take_while(|(x, y)| x == y)
                .count();

            (Reverse(wrong_positions), *p)
        },
        |father, mother, r: &mut SmallRng| {
            let bits = (usize::BITS - father.leading_zeros() + 1)
                .max(3)
                .max(usize::BITS - mother.leading_zeros() + 1)
                .min(usize::BITS);

            let mid = r.gen_range(0..bits);
            let (mother_left, mother_right) = split_num(*mother, mid);
            let (father_left, father_right) = split_num(*father, mid);
            let child1 = mother_left | father_right;
            let child2 = father_left | mother_right;

            [child1, child2].into_iter().take(2)
        },
        |child, rng| {
            let mutations = rng.gen_range(1..=10);
            for _ in 0..mutations {
                let bit = rng.gen_range(0..usize::BITS);
                *child ^= 1 << bit;
            }
        },
    );
    evolution.set_mutation_bps(9000u16);
    evolution.set_preserve_bps(5000u16);
    let mut last_best = (Reverse(0), 0);
    for g in 0.. {
        let best = evolution.select(&mut rng);
        if let Some(b) = &best {
            if b.2 != last_best {
                last_best = b.2;
            }
            if b.0 + max_repetitions < g && b.2 .0 == Reverse(program.len()) {
                return *b.1;
            }
        }
    }
    0
}
fn split_num(g: usize, right_size: u32) -> (usize, usize) {
    if right_size == 0 {
        return (g, 0);
    }
    if right_size == usize::BITS - 1 {
        return (
            (g.reverse_bits() & 0b1) << (usize::BITS - 1),
            g & (usize::MAX >> 1),
        );
    }
    if right_size >= usize::BITS {
        return (0, g);
    }
    let left = (g >> right_size) << right_size;
    let right = ((1 << (right_size + 1)) - 1) & g;
    (left, right)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::solve_part_2_genetic;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../input2.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(solve_part_1(EXAMPLE), "3,3,1,0");
    }
    #[test]
    fn test_part1_example2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(solve_part_1(EXAMPLE2), "5,5,6,4,3,5,5,0,3,3,0");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "2,1,3,0,5,2,3,7,1");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "117440");
    }

    #[rstest]
    #[case(ACTUAL, 107416732707226)]
    #[case(include_str!("../input3.txt"), 202356708354602)]
    fn test_part2_actual(#[case] input: &str, #[case] expected: usize) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(solve_part_2(input), expected);
    }
    #[rstest]
    #[case(ACTUAL, 107416732707226)]
    #[case(include_str!("../input3.txt"), 202356708354602)]
    fn test_part2_actual_genetic(#[case] input: &str, #[case] expected: usize) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(
            solve_part_2_genetic(
                input,
                [
                    178, 245, 208, 101, 141, 7, 242, 34, 144, 197, 134, 138, 51, 129, 4, 252, 145,
                    81, 204, 236, 101, 38, 146, 254, 110, 86, 18, 236, 72, 244, 117, 215
                ],
                10
            ),
            expected
        );
    }
}
