use advent_utils::glam::IVec2;
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Copy, Hash, Debug)]
enum RobotTask {
    MoveTo(IVec2),
    Press(usize),
}

fn min_steps_for_code(code: &str, robots: usize) -> usize {
    let x = std::iter::once(b'A')
        .chain(code.as_bytes().iter().copied())
        .tuple_windows()
        .map(|(a, b)| {
            if a == b {
                return vec![vec![RobotTask::Press(1)]];
            }
            todo!("{a} {b}");
        })
        .multi_cartesian_product()
        .map(|x| x.into_iter().flatten().collect_vec())
        .collect_vec();

    todo!("{x:?}");

    0
}
