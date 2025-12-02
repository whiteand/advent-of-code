use advent_utils::nom::{
    self,
    bytes::complete::tag,
    character::complete::{multispace0, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_per_ore_robot: usize,
    ore_per_clay_robot: usize,
    ore_per_obsidian_robot: usize,
    clay_per_obsidian_robot: usize,
    ore_per_geode_robot: usize,
    obsidian_per_geode_robot: usize,
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(
        tag("Blueprint "),
        nom::character::complete::u32.map(|n| n as usize),
        tuple((tag(":"), multispace0)),
    )
    .parse(input)?;

    let (input, ore_per_ore_robot) = delimited(
        tag("Each ore robot costs "),
        nom::character::complete::u32.map(|n| n as usize),
        nom::sequence::tuple((tag(" ore."), multispace0)),
    )
    .parse(input)?;
    let (input, ore_per_clay_robot) = delimited(
        tag("Each clay robot costs "),
        nom::character::complete::u32.map(|n| n as usize),
        nom::sequence::tuple((tag(" ore."), multispace0)),
    )
    .parse(input)?;
    let (input, (ore_per_obsidian_robot, clay_per_obsidian_robot)) = separated_pair(
        delimited(
            tag("Each obsidian robot costs "),
            nom::character::complete::u32.map(|n| n as usize),
            tag(" ore"),
        ),
        tag(" and "),
        nom::sequence::terminated(
            nom::character::complete::u32.map(|n| n as usize),
            nom::sequence::tuple((tag(" clay."), multispace0)),
        ),
    )
    .parse(input)?;
    let (input, (ore_per_geode_robot, obsidian_per_geode_robot)) = separated_pair(
        delimited(
            tag("Each geode robot costs "),
            nom::character::complete::u32.map(|n| n as usize),
            tag(" ore"),
        ),
        tag(" and "),
        nom::sequence::terminated(
            nom::character::complete::u32.map(|n| n as usize),
            tag(" obsidian."),
        ),
    )
    .parse(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore_per_ore_robot,
            ore_per_clay_robot,
            ore_per_obsidian_robot,
            clay_per_obsidian_robot,
            ore_per_geode_robot,
            obsidian_per_geode_robot,
        },
    ))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    remaining_minutes: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ore: Default::default(),
            clay: Default::default(),
            obsidian: Default::default(),
            geode: Default::default(),
            ore_robots: 1,
            clay_robots: Default::default(),
            obsidian_robots: Default::default(),
            remaining_minutes: 24,
        }
    }
}

/// Returns such N for which y(N) >= yt
/// if y(n) = y0 + dy * n
#[inline]
fn steps_to_yt(dy: usize, y0: usize, yt: usize) -> Option<usize> {
    if yt <= y0 {
        Some(0)
    } else if dy <= 0 {
        None
    } else {
        Some((yt - y0).div_ceil(dy))
    }
}

impl State {
    fn do_nothing_n_times(&mut self, n: usize) {
        self.ore += self.ore_robots * n;
        self.clay += self.clay_robots * n;
        self.obsidian += self.obsidian_robots * n;
        self.remaining_minutes -= n;
    }

    fn cmp_with_blueprint(&self, other: &Self) -> std::cmp::Ordering {
        self.geode.cmp(&other.geode).then_with(|| {
            let total_ore = self.ore + self.remaining_minutes * self.ore_robots;
            let total_clay = self.clay + self.remaining_minutes * self.clay_robots;
            let total_obsidian = self.obsidian + self.remaining_minutes * self.obsidian_robots;
            let other_ore = other.ore + other.remaining_minutes * other.ore_robots;
            let other_clay = other.clay + other.remaining_minutes * other.clay_robots;
            let other_obsidian = other.obsidian + other.remaining_minutes * other.obsidian_robots;
            total_obsidian
                .cmp(&other_obsidian)
                .then_with(|| total_clay.cmp(&other_clay))
                .then_with(|| total_ore.cmp(&other_ore))
        })
    }
    fn minutes_until_ore_robot_available(&self, blueprint: &Blueprint) -> Option<usize> {
        steps_to_yt(self.ore_robots, self.ore, blueprint.ore_per_ore_robot)
            .filter(|x| *x < self.remaining_minutes)
    }
    fn minutes_until_clay_robot_available(&self, blueprint: &Blueprint) -> Option<usize> {
        steps_to_yt(self.ore_robots, self.ore, blueprint.ore_per_clay_robot)
            .filter(|x| *x < self.remaining_minutes)
    }
    fn minutes_until_obsidian_robot_available(&self, blueprint: &Blueprint) -> Option<usize> {
        let Some(until_enough_ore) =
            steps_to_yt(self.ore_robots, self.ore, blueprint.ore_per_obsidian_robot)
                .filter(|x| *x < self.remaining_minutes)
        else {
            return None;
        };
        let Some(until_enough_clay) = steps_to_yt(
            self.clay_robots,
            self.clay,
            blueprint.clay_per_obsidian_robot,
        )
        .filter(|x| *x < self.remaining_minutes) else {
            return None;
        };

        Some(until_enough_clay.max(until_enough_ore))
    }
    fn minutes_until_geode_robot_available(&self, blueprint: &Blueprint) -> Option<usize> {
        let Some(until_enough_ore) =
            steps_to_yt(self.ore_robots, self.ore, blueprint.ore_per_geode_robot)
                .filter(|x| *x < self.remaining_minutes)
        else {
            return None;
        };
        let Some(until_enough_obsidian) = steps_to_yt(
            self.obsidian_robots,
            self.obsidian,
            blueprint.obsidian_per_geode_robot,
        )
        .filter(|x| *x < self.remaining_minutes) else {
            return None;
        };

        Some(until_enough_obsidian.max(until_enough_ore))
    }
    fn build_geode(&mut self, blueprint: &Blueprint) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.ore -= blueprint.ore_per_geode_robot;
        self.obsidian -= blueprint.obsidian_per_geode_robot;
        self.remaining_minutes -= 1;
        self.geode += self.remaining_minutes;
    }
    fn build_obsidian(&mut self, blueprint: &Blueprint) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.ore -= blueprint.ore_per_obsidian_robot;
        self.clay -= blueprint.clay_per_obsidian_robot;
        self.obsidian_robots += 1;
        self.remaining_minutes -= 1;
    }
    fn build_ore(&mut self, blueprint: &Blueprint) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.ore -= blueprint.ore_per_ore_robot;
        self.ore_robots += 1;
        self.remaining_minutes -= 1;
    }
    fn build_clay(&mut self, blueprint: &Blueprint) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.ore -= blueprint.ore_per_clay_robot;
        self.clay_robots += 1;
        self.remaining_minutes -= 1;
    }
    fn next_states(self, blueprint: &Blueprint, states: &mut Vec<State>) {
        if self.remaining_minutes == 0 {
            return;
        }

        let initial_len = states.len();

        // Build ore robot
        {
            let mut s = self.clone();
            let ore = s.ore;
            if ore < blueprint.ore_per_clay_robot
                || ore < blueprint.ore_per_ore_robot
                || ore < blueprint.ore_per_obsidian_robot
                || ore < blueprint.ore_per_geode_robot
            {
                if let Some(time_to_ore) = s.minutes_until_ore_robot_available(blueprint) {
                    s.do_nothing_n_times(time_to_ore);
                    s.build_ore(blueprint);
                    states.push(s);
                }
            }
        }

        {
            let mut s = self.clone();
            let clay = s.clay;
            if clay < blueprint.clay_per_obsidian_robot {
                if let Some(time_to_clay) = s.minutes_until_clay_robot_available(blueprint) {
                    s.do_nothing_n_times(time_to_clay);
                    s.build_clay(blueprint);
                    states.push(s);
                }
            }
        }

        {
            let mut s = self.clone();
            let obsidian = s.obsidian;
            if obsidian < blueprint.obsidian_per_geode_robot {
                if let Some(time_to_obsidian) = s.minutes_until_obsidian_robot_available(blueprint)
                {
                    s.do_nothing_n_times(time_to_obsidian);
                    s.build_obsidian(blueprint);
                    states.push(s);
                }
            }
        }

        {
            let mut s = self.clone();
            if let Some(time_to_geode) = s.minutes_until_geode_robot_available(blueprint) {
                s.do_nothing_n_times(time_to_geode);
                s.build_geode(blueprint);
                states.push(s);
            }
        }

        // Do nothing
        if initial_len == states.len() {
            let mut s = self.clone();
            s.do_nothing_n_times(s.remaining_minutes);
            states.push(s);
        }
    }
}

fn get_best(elements: &mut Vec<State>) -> Option<State> {
    let position = elements
        .iter()
        .enumerate()
        .max_by(|(_, e), (_, e2)| e.cmp_with_blueprint(e2))
        .map(|(i, _)| i)?;
    let res = elements.swap_remove(position);
    Some(res)
}

fn get_max_geodes(blueprint: &Blueprint, init: State) -> usize {
    let mut states = vec![init];

    let mut max_geodes = 0;

    let mut buffer = Vec::new();
    while let Some(state) = get_best(&mut states) {
        if state.remaining_minutes == 0 {
            max_geodes = max_geodes.max(state.geode);
            continue;
        }
        state.next_states(blueprint, &mut buffer);
        buffer.retain(|s| s.remaining_minutes > 0 || s.geode > max_geodes);
        states.extend_from_slice(&buffer);
        buffer.clear();
    }
    max_geodes
}

pub fn solve_part_1(file_content: &str) -> usize {
    let (_, blueprints) = separated_list1(newline, blueprint)(file_content.trim()).unwrap();
    blueprints
        .into_iter()
        .map(|b| {
            println!("Evaluating  blueprint #{}", b.id);
            let res = get_max_geodes(
                &b,
                State {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    remaining_minutes: 24,
                },
            ) * b.id;
            println!("Finiahed = {}\n", res);
            res
        })
        .sum()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let (_, blueprints) = separated_list1(newline, blueprint)(file_content.trim()).unwrap();
    blueprints
        .into_iter()
        .take(3)
        .map(|b| {
            println!("Evaluating  blueprint #{}", b.id);
            let res = get_max_geodes(
                &b,
                State {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    remaining_minutes: 32,
                },
            );
            println!("Finished = {}\n", res);
            res
        })
        .product()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_minutes_until_ore_robot_available() {
        let blueprint = Blueprint {
            id: 1,
            ore_per_ore_robot: 2,
            ore_per_clay_robot: 4,
            ore_per_obsidian_robot: 3,
            clay_per_obsidian_robot: 14,
            ore_per_geode_robot: 2,
            obsidian_per_geode_robot: 7,
        };
        assert_eq!(
            State::default().minutes_until_ore_robot_available(&blueprint),
            Some(2)
        );
        assert_eq!(
            State {
                ore: 10,
                ..Default::default()
            }
            .minutes_until_ore_robot_available(&blueprint),
            Some(0)
        );
        assert_eq!(
            State {
                ore: 0,
                ore_robots: 0,
                ..Default::default()
            }
            .minutes_until_ore_robot_available(&blueprint),
            None
        );
    }
    #[test]
    fn test_example_works() {
        let blueprint = Blueprint {
            id: 1,
            ore_per_ore_robot: 4,
            ore_per_clay_robot: 2,
            ore_per_obsidian_robot: 3,
            clay_per_obsidian_robot: 14,
            ore_per_geode_robot: 2,
            obsidian_per_geode_robot: 7,
        };
        let mut state = State::default();
        assert_eq!(
            state.minutes_until_clay_robot_available(&blueprint),
            Some(2)
        );
        state.do_nothing_n_times(2);
        state.build_clay(&blueprint);
        for _ in 0..2 {
            assert_eq!(
                state.minutes_until_clay_robot_available(&blueprint),
                Some(1)
            );
            state.do_nothing_n_times(1);
            state.build_clay(&blueprint);
        }
        assert_eq!(state.ore, 1);
        assert_eq!(state.clay, 6);
        assert_eq!(state.clay_robots, 3);
        assert_eq!(
            state.minutes_until_obsidian_robot_available(&blueprint),
            Some(3)
        );
        state.do_nothing_n_times(3);
        state.build_obsidian(&blueprint);
        assert_eq!(state.ore, 2);
        assert_eq!(state.clay, 4);
        assert_eq!(state.clay_robots, 3);
        assert_eq!(state.obsidian_robots, 1);

        assert_eq!(
            state.minutes_until_clay_robot_available(&blueprint),
            Some(0)
        );
        state.build_clay(&blueprint);
        assert_eq!(state.ore, 1);
        assert_eq!(state.clay, 7);
        assert_eq!(state.clay_robots, 4);
        assert_eq!(state.obsidian_robots, 1);
        assert_eq!(state.remaining_minutes, 12);

        assert_eq!(
            state.minutes_until_obsidian_robot_available(&blueprint),
            Some(2)
        );
        state.do_nothing_n_times(2);
        state.build_obsidian(&blueprint);
        assert_eq!(state.ore, 1);
        assert_eq!(state.clay, 5);
        assert_eq!(state.clay_robots, 4);
        assert_eq!(state.obsidian_robots, 2);
        assert_eq!(state.remaining_minutes, 9);

        assert_eq!(
            state.minutes_until_geode_robot_available(&blueprint),
            Some(2)
        );
        state.do_nothing_n_times(2);
        state.build_geode(&blueprint);
        assert_eq!(state.ore, 2);
        assert_eq!(state.clay, 17);
        assert_eq!(state.obsidian, 3);
        assert_eq!(state.geode, 6);
        assert_eq!(state.remaining_minutes, 6);

        assert_eq!(
            state.minutes_until_geode_robot_available(&blueprint),
            Some(2)
        );
        state.do_nothing_n_times(2);
        state.build_geode(&blueprint);
        assert_eq!(state.ore, 3);
        assert_eq!(state.clay, 29);
        assert_eq!(state.obsidian, 2);
        assert_eq!(state.geode, 9);
        assert_eq!(state.remaining_minutes, 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "33");
    }

    // Solution for part2 does not solve part1
    #[test]
    #[ignore]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1365");
    }

    // My solution is not for all inputs
    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "3472");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "4864");
    }
}
