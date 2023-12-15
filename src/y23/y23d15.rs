use std::{ops::Deref, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hasher {
    current: u8,
}
fn hash_string(text: &str) -> usize {
    let mut current: u8 = 0;
    for b in text.bytes() {
        current = current.wrapping_add(b);
        current = current.wrapping_shl(4).wrapping_add(current)
    }
    current as usize
}

pub fn solve_task1(file_content: &str) -> usize {
    file_content
        .split(',')
        .map(|x| x.trim_matches('\n'))
        .map(hash_string)
        .sum()
}
#[derive(Debug, Clone)]
struct Lens {
    label: Rc<str>,
    strength: usize,
}

pub fn solve_task2(file_content: &str) -> usize {
    file_content
        .split(',')
        .map(|x| x.trim_matches('\n'))
        .fold(
            vec![Vec::new(); 256],
            |mut boxes: Vec<Vec<Lens>>, instruction| {
                if instruction.ends_with('-') {
                    let label = instruction.strip_suffix('-').unwrap();
                    let ptr = hash_string(label);
                    let ind = boxes[ptr].iter().position(|x| x.label.deref().eq(label));
                    if let Some(ind) = ind {
                        boxes[ptr].remove(ind);
                    }
                    boxes
                } else {
                    let (label, count_str) = instruction.split_once('=').expect("should be a pair");
                    let count = count_str.parse::<usize>().unwrap();
                    let ptr = hash_string(label);
                    for x in boxes[ptr].iter_mut() {
                        if x.label.deref().eq(label) {
                            x.strength = count;
                            return boxes;
                        }
                    }
                    boxes[ptr].push(Lens {
                        label: Rc::from(label),
                        strength: count,
                    });
                    boxes
                }
            },
        )
        .into_iter()
        .enumerate()
        .map(|(b, lenses)| {
            if lenses.is_empty() {
                return 0;
            }
            lenses
                .into_iter()
                .enumerate()
                .map(|(slot, lens)| (b + 1) * (slot + 1) * lens.strength)
                .sum::<usize>()
        })
        .sum::<usize>()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d15/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d15.txt");

    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "1320");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "515495");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "145");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "229349");
    }
}
