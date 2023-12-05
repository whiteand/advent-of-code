use self::parse::{task1_inputs, task2_inputs};

mod int_map;
mod map_pipeline;
mod parse;
mod range_map;

pub fn solve_task1(file_content: &str) -> usize {
    let (_, (seeds, pipeline)) = task1_inputs(file_content).expect("failed to parse inputs");
    seeds
        .into_iter()
        .map(|s| pipeline.get(s))
        .min()
        .unwrap_or_default()
}

pub fn solve_task2(file_content: &str) -> usize {
    let (_, (seed_ranges, pipeline)) = task2_inputs(file_content).expect("failed to parse inputs");

    seed_ranges
        .iter()
        .map(|r| r.start)
        .map(|s| pipeline.get(s))
        .min()
        .unwrap_or_default()
        .min(
            (0..pipeline.levels())
                .map(|level| {
                    pipeline
                        .key_points(level)
                        .filter(|&value| {
                            let src = pipeline.get_src(value, level);
                            seed_ranges.iter().any(|r| r.contains(&src))
                        })
                        .map(|start| pipeline.get_from_level(start, level))
                        .min()
                        .unwrap_or(usize::MAX)
                })
                .min()
                .unwrap_or(usize::MAX),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d05/example.txt");
    const EXAMPLE_2: &str = include_str!("./y23d05/counter_example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d05.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "35");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "510109797");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "46");
    }
    #[test]
    fn test_task1_counter_example() {
        assert_eq!(format!("{}", solve_task1(EXAMPLE_2)), "100");
    }
    #[test]
    fn test_task2_counter_example() {
        assert_eq!(format!("{}", solve_task2(EXAMPLE_2)), "100");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "9622622");
    }
}
