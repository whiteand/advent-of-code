mod condition;
mod item;
mod monkey;
mod operation;
mod parse;

use std::cell::RefCell;

fn gcd(a: u64, b: u64) -> u64 {
    let mut left = a.max(b);
    let mut right = a.min(b);
    while right > 0 {
        (left, right) = (right, left % right)
    }
    left
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn solve<const LOOPS: u64, const DIVIDER: u64>(file_content: &str) -> u64 {
    let monkeys = parse::parse_monkeys(file_content)
        .unwrap()
        .1
        .into_iter()
        .map(RefCell::new)
        .collect::<Vec<_>>();

    let base = monkeys
        .iter()
        .map(|x| x.borrow().condition.divisor)
        .reduce(lcm)
        .unwrap_or(1);

    let monkeys_len = monkeys.len();

    for _ in 0..LOOPS {
        for i in 0..monkeys_len {
            let transfers = {
                let mut monkey = monkeys[i].borrow_mut();
                std::mem::take(&mut monkey.items)
                    .into_iter()
                    .map(|mut item| {
                        monkey.inspect(&mut item, DIVIDER, base);
                        let next_monkey_index = monkey.choose(&item);
                        (next_monkey_index, item)
                    })
                    .collect::<Vec<_>>()
            };
            for (to, what) in transfers {
                let mut monkey = monkeys[to].borrow_mut();
                monkey.catch(what);
            }
        }
    }

    let mut inspected = monkeys
        .iter()
        .map(|x| x.borrow().inspected)
        .collect::<Vec<_>>();

    inspected.sort_by(|a, b| b.cmp(a));

    let a = inspected[0];
    let b = inspected[1];

    a * b
}

pub fn solve_task1(file_content: &str) -> u64 {
    solve::<20, 3>(file_content)
}

pub fn solve_task2(file_content: &str) -> u64 {
    solve::<10_000, 1>(file_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 57, 58
  Operation: new = old * 19
  Test: divisible by 7
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 66, 52, 59, 79, 94, 73
  Operation: new = old + 1
  Test: divisible by 19
    If true: throw to monkey 4
    If false: throw to monkey 6

Monkey 2:
  Starting items: 80
  Operation: new = old + 6
  Test: divisible by 5
    If true: throw to monkey 7
    If false: throw to monkey 5

Monkey 3:
  Starting items: 82, 81, 68, 66, 71, 83, 75, 97
  Operation: new = old + 5
  Test: divisible by 11
    If true: throw to monkey 5
    If false: throw to monkey 2

Monkey 4:
  Starting items: 55, 52, 67, 70, 69, 94, 90
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 3

Monkey 5:
  Starting items: 69, 85, 89, 91
  Operation: new = old + 7
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 7

Monkey 6:
  Starting items: 75, 53, 73, 52, 75
  Operation: new = old * 7
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 4

Monkey 7:
  Starting items: 94, 60, 79
  Operation: new = old + 2
  Test: divisible by 3
    If true: throw to monkey 1
    If false: throw to monkey 6";
    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "50830");
    }
    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "14399640002");
    }
    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(
            format!(
                "{}",
                solve_task2(
                    "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
                )
            ),
            "2713310158"
        );
    }
}
