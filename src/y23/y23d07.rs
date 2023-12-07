use std::cmp::Ordering;

pub fn solve_task1(file_content: &str) -> usize {
    let hands = parse_hands(file_content, get_simple_combo).collect::<Vec<_>>();
    get_total_score(hands, &ORDER)
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    let hands = parse_hands(file_content, get_joker_combo).collect::<Vec<_>>();
    get_total_score(hands, &ORDER_2)
}
fn get_total_score(mut hands: Vec<Hand>, order: &[char]) -> usize {
    hands.sort_by(|a, b| compare(a, b, order));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum()
}

fn get_simple_combo(card: &[char]) -> Combo {
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
    counts.sort();
    counts.reverse();
    counts_to_combo(&counts)
}

fn get_joker_combo(cards: &[char]) -> Combo {
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
    counts.sort();
    counts.reverse();
    counts[0] += joker_count;
    counts_to_combo(&counts)
}

fn counts_to_combo(counts: &[usize]) -> Combo {
    if counts[0] == 5 {
        Combo::FiveOfAKind
    } else if counts[0] == 4 {
        Combo::FourOfAKind
    } else if counts[0] == 3 {
        if counts[1] == 2 {
            Combo::FullHouse
        } else {
            Combo::ThreeOfAKind
        }
    } else if counts[0] == 2 {
        if counts[1] == 2 {
            Combo::TwoPair
        } else {
            Combo::OnePair
        }
    } else {
        Combo::HighCard
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

const ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const ORDER_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    bid: usize,
    cards: Vec<char>,
    combo: Combo,
}

fn compare(a: &Hand, b: &Hand, order: &[char]) -> Ordering {
    a.combo.cmp(&b.combo).then_with(|| {
        a.cards
            .iter()
            .zip(b.cards.iter())
            .fold(Ordering::Equal, |ord, (a, b)| {
                ord.then_with(|| {
                    let a_ind = order.iter().position(|&c| c == *a).unwrap();
                    let b_ind = order.iter().position(|&c| c == *b).unwrap();
                    a_ind.cmp(&b_ind)
                })
            })
    })
}

fn parse_hands<'t, 'f: 't>(
    file_content: &'t str,
    get_combo: impl Fn(&[char]) -> Combo + 'f,
) -> impl Iterator<Item = Hand> + 't {
    file_content.lines().map(move |line| {
        let (a, b) = line.split_once(' ').unwrap();
        let cards = a.chars().collect::<Vec<_>>();
        let bid = b.parse::<usize>().unwrap();
        let combo = get_combo(&cards);
        Hand { bid, cards, combo }
    })
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
