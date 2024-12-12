use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, 3)
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, 4)
}

fn solve(file_content: &str, groups: usize) -> usize {
    let packages = file_content
        .trim()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let sum = packages.iter().copied().sum::<usize>();
    if sum % groups != 0 {
        panic!("These packages cannot be grouped into 3 groups");
    }
    let target_group_weight = sum / groups;

    let mut indexes = (0..packages.len()).collect_vec();
    let mut min_quantum_entanglement = usize::MAX;

    let max_group_size = packages.len() - groups + 1;

    for first_group_size in 0..=max_group_size {
        for first_group in (0..packages.len()).combinations(first_group_size) {
            let first_group_sum = first_group.iter().map(|i| packages[*i]).sum::<usize>();
            if first_group_sum != target_group_weight {
                continue;
            }
            indexes.retain(|x| !first_group.contains(x));
            if can_be_split(&packages, &mut indexes, target_group_weight, groups - 1) {
                let quantum_entanglement =
                    first_group.iter().map(|i| packages[*i]).product::<usize>();

                if min_quantum_entanglement > quantum_entanglement {
                    min_quantum_entanglement = quantum_entanglement;
                }
            }
            indexes.extend_from_slice(first_group.as_slice());
        }
        if min_quantum_entanglement != usize::MAX {
            break;
        }
    }
    min_quantum_entanglement
}
fn can_be_split(
    packages: &[usize],
    indexes: &mut Vec<usize>,
    target_group_weight: usize,
    groups: usize,
) -> bool {
    if groups == 1 {
        return indexes.iter().copied().map(|i| packages[i]).sum::<usize>() == target_group_weight;
    }

    for size in 1..indexes.len() {
        for first_group in indexes
            .iter()
            .copied()
            .collect_vec()
            .into_iter()
            .combinations(size)
        {
            let sum = first_group
                .iter()
                .copied()
                .map(|i| packages[i])
                .sum::<usize>();
            if sum != target_group_weight {
                continue;
            }
            indexes.retain(|i| !first_group.contains(i));

            let possible = can_be_split(packages, indexes, target_group_weight, groups - 1);

            indexes.extend(first_group);

            if possible {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::solve;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve(EXAMPLE, 3)), "99");
    }

    #[test]
    #[ignore] // 13s
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "10723906903");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "44");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "74850409");
    }
}
