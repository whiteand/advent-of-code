use advent_utils::nom::{self, IResult};

type Board = Vec<Vec<char>>;

#[derive(Debug)]
struct Move {
    amount: u32,
    from: usize,
    to: usize,
}

fn split_by_empty_line(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut first = Vec::new();
    let mut second = Vec::new();
    let mut reading_first = true;
    for line in input.lines() {
        if line.is_empty() {
            reading_first = false;
            continue;
        }
        if reading_first {
            first.push(line)
        } else {
            second.push(line)
        }
    }
    (first, second)
}

fn parse_map(map_lines: Vec<&str>) -> Vec<Vec<char>> {
    let stacks_amount = (map_lines[0].chars().count() + 2) >> 2;
    let highest_stack = map_lines.len() - 1;
    let mut stacks = Vec::new();
    let chars: Vec<Vec<char>> = map_lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    for stack_index in 0..stacks_amount {
        let mut stack_list = Vec::with_capacity(highest_stack);
        let mut item = 0;
        loop {
            if item >= highest_stack {
                break;
            }
            let char = chars[highest_stack - item - 1][stack_index * 4 + 1];
            if char.is_whitespace() {
                break;
            }
            stack_list.push(char);
            item += 1;
        }

        stacks.push(stack_list);
    }
    stacks
}

fn parse_move(line: &str) -> IResult<&str, Move> {
    let (input, _) = nom::bytes::complete::tag("move ")(line)?;
    let (input, amount) = nom::character::complete::u32(input)?;
    let (input, _) = nom::bytes::complete::tag(" from ")(input)?;
    let (input, from) = nom::character::complete::u32(input)?;
    let (input, _) = nom::bytes::complete::tag(" to ")(input)?;
    let (input, to) = nom::character::complete::u32(input)?;

    Ok((
        input,
        Move {
            amount,
            from: from as usize - 1,
            to: to as usize - 1,
        },
    ))
}

fn parse_moves(moves_lines: Vec<&str>) -> impl Iterator<Item = Move> + '_ {
    moves_lines
        .into_iter()
        .map(|line| parse_move(line).unwrap().1)
}

fn parse_input(input: &str) -> (Board, impl Iterator<Item = Move> + '_) {
    let (map_lines, moves_lines) = split_by_empty_line(input);
    (parse_map(map_lines), parse_moves(moves_lines))
}

fn apply_move_one_by_one(m: Move, stacks: &mut [Vec<char>]) {
    for _ in 0..m.amount {
        let item = stacks[m.from].pop().unwrap();
        stacks[m.to].push(item);
    }
}

fn solve(file_content: &str, apply_move: impl Fn(Move, &mut [Vec<char>])) -> String {
    let (mut stacks, moves) = parse_input(file_content);
    {
        for m in moves {
            apply_move(m, &mut stacks)
        }
    }
    let result = stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>();
    result
}

pub fn solve_part_1(file_content: &str) -> String {
    solve(file_content, apply_move_one_by_one)
}

fn apply_move_with_multiple_at_once(m: Move, stacks: &mut [Vec<char>]) {
    if m.from == m.to {
        return;
    }
    let mut items = std::iter::repeat_with(|| stacks[m.from].pop().unwrap())
        .take(m.amount as usize)
        .collect::<Vec<_>>();

    items.reverse();

    stacks[m.to].extend(items);
}

pub fn solve_part_2(file_content: &str) -> impl std::fmt::Display {
    solve(file_content, apply_move_with_multiple_at_once)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "CMZ");
    }
    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "MCD");
    }
}
