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

impl State {
    fn collect(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.remaining_minutes -= 1;
    }
    fn build_geode(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.ore_per_geode_robot;
        self.obsidian -= blueprint.obsidian_per_geode_robot;
        self.geode += self.remaining_minutes;
    }
    fn build_obsidian(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.ore_per_obsidian_robot;
        self.clay -= blueprint.clay_per_obsidian_robot;
        self.obsidian_robots += 1;
    }
    fn build_ore(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.ore_per_ore_robot;
        self.ore_robots += 1;
    }
    fn build_clay(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.ore_per_clay_robot;
        self.clay_robots += 1;
    }
    fn next_states(self, blueprint: &Blueprint, states: &mut Vec<State>) {
        if self.remaining_minutes == 0 {
            return;
        }
        // Do nothing
        {
            let mut s = self.clone();
            s.collect();
            states.push(s);
        }

        // Build ore robot
        if self.ore >= blueprint.ore_per_ore_robot {
            let mut s = self.clone();
            s.build_ore(blueprint);
            s.collect();
            states.push(s);
        }

        if self.ore >= blueprint.ore_per_clay_robot {
            let mut s = self.clone();
            s.build_clay(blueprint);
            s.collect();
            states.push(s);
        }

        if self.ore >= blueprint.ore_per_obsidian_robot
            && self.clay >= blueprint.clay_per_obsidian_robot
        {
            let mut s = self.clone();
            s.build_obsidian(blueprint);
            s.collect();
            states.push(s);
        }

        if self.ore >= blueprint.ore_per_geode_robot
            && self.obsidian >= blueprint.obsidian_per_geode_robot
        {
            let mut s = self.clone();
            s.build_geode(blueprint);
            s.collect();
            states.push(s);
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_best<T: Ord>(elements: &mut Vec<T>) -> Option<T> {
    let position = elements
        .iter()
        .enumerate()
        .max_by_key(|(_, e)| *e)
        .map(|(i, _)| i)?;
    let res = elements.swap_remove(position);
    Some(res)
}

fn get_max_geodes(blueprint: &Blueprint) -> usize {
    let mut states = vec![State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        remaining_minutes: 24,
    }];

    let mut max_geodes = 0;

    while let Some(state) = get_best(&mut states) {
        if state.remaining_minutes == 0 {
            println!("{:?} {:?}", state.geode, max_geodes);
            max_geodes = max_geodes.max(state.geode);
            continue;
        }
        state.next_states(blueprint, &mut states);
    }
    max_geodes
}

pub fn solve_part_1(file_content: &str) -> usize {
    let (_, blueprints) = separated_list1(newline, blueprint)(file_content.trim()).unwrap();
    blueprints
        .into_iter()
        .map(|b| get_max_geodes(&b) * b.id)
        .sum()
}
pub fn solve_part_2(_file_content: &str) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "0");
    }

    #[test]
    #[ignore]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "0");
    }

    #[test]
    #[ignore]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
