use nom::{
    bytes::complete::tag,
    character::{self},
    sequence::preceded,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq)]
struct History(u64, u32);

impl History {
    pub fn get(&self, minute: usize) -> Option<Resource> {
        let bit = 1 << minute;
        if self.1 & bit == 0 {
            return None;
        }
        let pair_bit = (self.0 >> (minute << 1)) & 3;

        Some(match pair_bit {
            0 => Resource::Ore,
            1 => Resource::Clay,
            2 => Resource::Obsidian,
            3 => Resource::Geode,
            _ => unreachable!(),
        })
    }
    pub fn set(&self, minute: usize, resource: Resource) -> Self {
        let bit = 1 << minute;
        let res: u64 = match resource {
            Resource::Ore => 0,
            Resource::Clay => 1,
            Resource::Obsidian => 2,
            Resource::Geode => 3,
        } << (minute * 2);
        Self(self.0 | res, self.1 | bit)
    }
    pub fn remove(&self, minute: usize) -> Self {
        let bit = !(1 << minute);
        let res: u64 = !(3 << (minute * 2));
        Self(self.0 & res, self.1 & bit)
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    pub fn is_enough_for(&self, other: &Resources) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
    pub fn remove(&self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.obsidian - other.obsidian,
        }
    }
    pub fn add(&self, other: &Resources) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
    pub fn add_of(&self, resource: Resource, value: usize) -> Self {
        match resource {
            Resource::Ore => Resources {
                ore: self.ore + value,
                clay: self.clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Resource::Clay => Resources {
                clay: self.clay + value,
                ore: self.ore,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Resource::Obsidian => Resources {
                obsidian: self.obsidian + value,
                clay: self.clay,
                ore: self.ore,
                geode: self.geode,
            },
            Resource::Geode => Resources {
                geode: self.geode + value,
                clay: self.clay,
                obsidian: self.obsidian,
                ore: self.ore,
            },
        }
    }
    pub fn remove_of(&self, resource: Resource, value: usize) -> Self {
        match resource {
            Resource::Ore => Resources {
                ore: self.ore - value,
                clay: self.clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Resource::Clay => Resources {
                clay: self.clay - value,
                ore: self.ore,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Resource::Obsidian => Resources {
                obsidian: self.obsidian - value,
                clay: self.clay,
                ore: self.ore,
                geode: self.geode,
            },
            Resource::Geode => Resources {
                geode: self.geode - value,
                clay: self.clay,
                obsidian: self.obsidian,
                ore: self.ore,
            },
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    cost: HashMap<Resource, Resources>,
}

fn parse_blueprint(line: &str) -> IResult<&str, Blueprint> {
    let (input, blueprint_id) = preceded(tag("Blueprint "), character::complete::u32)(line)?;
    let (input, ore_robot_ore_cost) =
        preceded(tag(": Each ore robot costs "), character::complete::u32)(input)?;
    let (input, clay_robot_ore_cost) = preceded(
        tag(" ore. Each clay robot costs "),
        character::complete::u32,
    )(input)?;
    let (input, obsidian_robot_ore_cost) = preceded(
        tag(" ore. Each obsidian robot costs "),
        character::complete::u32,
    )(input)?;
    let (input, obsidian_robot_clay_cost) =
        preceded(tag(" ore and "), character::complete::u32)(input)?;
    let (input, geode_robot_ore_cost) = preceded(
        tag(" clay. Each geode robot costs "),
        character::complete::u32,
    )(input)?;
    let (input, geode_robot_obsidian_cost) =
        preceded(tag(" ore and "), character::complete::u32)(input)?;
    let mut costs = HashMap::new();
    costs.insert(
        Resource::Ore,
        Resources {
            ore: ore_robot_ore_cost as usize,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    );
    costs.insert(
        Resource::Clay,
        Resources {
            ore: clay_robot_ore_cost as usize,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    );
    costs.insert(
        Resource::Obsidian,
        Resources {
            clay: obsidian_robot_clay_cost as usize,
            ore: obsidian_robot_ore_cost as usize,
            obsidian: 0,
            geode: 0,
        },
    );
    costs.insert(
        Resource::Geode,
        Resources {
            ore: geode_robot_ore_cost as usize,
            clay: 0,
            obsidian: geode_robot_obsidian_cost as usize,
            geode: 0,
        },
    );
    let blueprint = Blueprint {
        id: blueprint_id as usize,
        cost: costs,
    };
    Ok((input, blueprint))
}

fn get_quality_level(blueprint: &Blueprint) -> usize {
    blueprint.id * get_max_geodes(blueprint)
}

#[derive(Clone, Default)]
struct State {
    resources: Resources,
    history: History,
    robots: Resources,
    elapsed_minutes: usize,
}

impl State {
    pub fn print(&self, blueprint: &Blueprint) {
        let mut previous_states: Vec<State> =
            std::iter::successors(Some(self.clone()), |next_state| {
                get_previous_state(blueprint, next_state)
            })
            .collect::<Vec<_>>();
        previous_states.reverse();
        for s in previous_states {
            println!("Minute {}", s.elapsed_minutes + 1);
            let resource = self.history.get(s.elapsed_minutes);
            match &resource {
                Some(r) => {
                    match r {
                        Resource::Ore => println!("Build ore robot"),
                        Resource::Clay => println!("Build clay robot"),
                        Resource::Obsidian => println!("Build obsidian robot"),
                        Resource::Geode => println!("Build geode robot"),
                    }
                    let cost = blueprint.cost.get(r).unwrap();
                    println!("Spent {:?}", cost);
                }
                None => {}
            }
            println!(
                "Resources: {} ore, {} clay, {} obsidian, {} geode",
                s.resources.ore, s.resources.clay, s.resources.obsidian, s.resources.geode
            );
            println!();
        }
    }
}

const MAX_MINUTES: usize = 24;
fn get_max_geodes(blueprint: &Blueprint) -> usize {
    let mut states = Vec::new();
    states.push(State {
        resources: Resources::default(),
        history: History::default(),
        robots: Resources {
            ore: 1,
            ..Default::default()
        },
        elapsed_minutes: 0,
    });

    let mut max_geodes = 0;
    while let Some(state) = states.pop() {
        if state.elapsed_minutes >= MAX_MINUTES {
            if state.resources.geode > max_geodes {
                state.print(blueprint);
                max_geodes = state.resources.geode
            }
            continue;
        }
        if state
            .resources
            .is_enough_for(blueprint.cost.get(&Resource::Geode).unwrap())
        {
            states.push(build_robot(blueprint, Resource::Geode, &state));
            continue;
        }

        let previous_resources = get_previous_state(blueprint, &state)
            .map(|s| s.resources)
            .unwrap_or_default();

        states.push(State {
            resources: state.resources.add(&state.robots),
            robots: state.robots,
            elapsed_minutes: state.elapsed_minutes + 1,
            history: state.history,
        });
        for resource in [Resource::Ore, Resource::Clay, Resource::Obsidian] {
            let cost = blueprint.cost.get(&resource).unwrap();
            if state.elapsed_minutes > 0
                && state.history.get(state.elapsed_minutes - 1).is_none()
                && previous_resources.is_enough_for(cost)
            {
                continue;
            }
            if state.resources.is_enough_for(cost) {
                states.push(build_robot(blueprint, resource, &state));
            }
        }
    }
    max_geodes
}

fn get_previous_state(blueprint: &Blueprint, state: &State) -> Option<State> {
    if state.elapsed_minutes == 0 {
        return None;
    }
    let prev_minutes = state.elapsed_minutes - 1;
    let action = state.history.get(prev_minutes);
    Some(match action {
        Some(res) => {
            let cost = blueprint.cost.get(&res).unwrap();
            let previous_robots = state.robots.remove_of(res, 1);
            let previous_resources = state.resources.add(cost).remove(&previous_robots);
            let prev_history = state.history.remove(prev_minutes);

            State {
                resources: previous_resources,
                robots: previous_robots,
                elapsed_minutes: prev_minutes,
                history: prev_history,
            }
        }
        None => State {
            resources: state.resources.remove(&state.robots),
            history: state.history,
            robots: state.robots,
            elapsed_minutes: prev_minutes,
        },
    })
}

fn build_robot(blueprint: &Blueprint, new_robot_resource: Resource, state: &State) -> State {
    let cost = blueprint.cost.get(&new_robot_resource).unwrap();
    let new_resources = state.resources.remove(cost).add(&state.robots);
    let new_robots = state.robots.add_of(new_robot_resource, 1);
    let new_history = state.history.set(state.elapsed_minutes, new_robot_resource);
    let new_elapsed_minutes = state.elapsed_minutes + 1;
    State {
        resources: new_resources,
        robots: new_robots,
        history: new_history,
        elapsed_minutes: new_elapsed_minutes,
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    file_content
        .lines()
        .map(|line| parse_blueprint(line).unwrap().1)
        .map(|b| get_quality_level(&b))
        .sum()
}
pub fn solve_task2(_file_content: &str) -> impl std::fmt::Display {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d19/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y22/y22d19.txt");

    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "33");
    }

    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "0");
    }

    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
