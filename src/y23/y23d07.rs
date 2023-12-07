use std::cmp::Ordering;

pub fn solve_task1(file_content: &str) -> usize {
    Part1::solve(file_content)
}
pub fn solve_task2(file_content: &str) -> usize {
    Part2::solve(file_content)
}

trait AdventPoker {
    const ORDER: [char; 13];

    fn get_combo(cards: &[char]) -> Combo;

    fn solve(file_content: &str) -> usize {
        let mut hands = file_content
            .lines()
            .map(move |line| {
                let (a, b) = line.split_once(' ').unwrap();
                let cards = a.chars().collect::<Vec<_>>();
                let bid = b.parse::<usize>().unwrap();
                let combo = Self::get_combo(&cards);
                Hand { bid, cards, combo }
            })
            .collect::<Vec<_>>();

        hands.sort_by(|a, b| {
            a.combo.cmp(&b.combo).then_with(|| {
                a.cards
                    .iter()
                    .zip(b.cards.iter())
                    .fold(Ordering::Equal, |ord, (a, b)| {
                        ord.then_with(|| {
                            let a_ind = Self::ORDER.iter().position(|&c| c == *a).unwrap();
                            let b_ind = Self::ORDER.iter().position(|&c| c == *b).unwrap();
                            a_ind.cmp(&b_ind)
                        })
                    })
            })
        });

        hands
            .into_iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum()
    }
}

struct Part1;

impl AdventPoker for Part1 {
    const ORDER: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    fn get_combo(card: &[char]) -> Combo {
        let mut ordered = card.iter().collect::<Vec<_>>();
        ordered.sort();
        let mut counts: Vec<usize> = Vec::new();
        let mut i = 0;
        while i < ordered.len() {
            let j = (i + 1..ordered.len())
                .find(|&j| j < ordered.len() && ordered[j] != ordered[i])
                .unwrap_or(ordered.len());
            counts.push(j - i);
            i = j;
        }
        counts.sort_by(|a, b| b.cmp(a));
        counts_to_combo(&counts)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    bid: usize,
    cards: Vec<char>,
    combo: Combo,
}

struct Part2;
impl AdventPoker for Part2 {
    fn get_combo(cards: &[char]) -> Combo {
        let mut ordered = cards.iter().collect::<Vec<_>>();
        ordered.sort();
        let mut counts: Vec<usize> = Vec::new();
        let mut joker_count = 0;
        let mut i = 0;
        while i < ordered.len() {
            let j = (i + 1..ordered.len())
                .find(|&j| j < ordered.len() && ordered[j] != ordered[i])
                .unwrap_or(ordered.len());
            let cnt = j - i;
            if *ordered[i] == 'J' {
                joker_count += cnt;
            } else {
                counts.push(cnt);
            }
            i = j;
        }
        if counts.is_empty() {
            counts.push(0);
        }
        counts.sort_by(|a, b| b.cmp(a));
        counts[0] += joker_count;
        counts_to_combo(&counts)
    }

    const ORDER: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
}

/// Expects counts of the combination in descending order
fn counts_to_combo(counts: &[usize]) -> Combo {
    match counts[0] {
        5 => Combo::FiveOfAKind,
        4 => Combo::FourOfAKind,
        3 => match counts[1] {
            2 => Combo::FullHouse,
            _ => Combo::ThreeOfAKind,
        },
        2 => match counts[1] {
            2 => Combo::TwoPair,
            _ => Combo::OnePair,
        },
        _ => Combo::HighCard,
    }
}

const COMBOS_ORDER: [Combo; 7] = [
    Combo::FiveOfAKind,
    Combo::FourOfAKind,
    Combo::FullHouse,
    Combo::ThreeOfAKind,
    Combo::TwoPair,
    Combo::OnePair,
    Combo::HighCard,
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Combo {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for Combo {
    fn cmp(&self, other: &Self) -> Ordering {
        let ind1 = (0..COMBOS_ORDER.len())
            .find(|&i| COMBOS_ORDER[i] == *self)
            .unwrap();
        let ind2 = (0..COMBOS_ORDER.len())
            .find(|&i| COMBOS_ORDER[i] == *other)
            .unwrap();
        ind2.cmp(&ind1)
    }
}

impl PartialOrd for Combo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d07/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d07.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "6440");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "255048101");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "5905");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "253718286");
    }
}
