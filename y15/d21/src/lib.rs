use itertools::Itertools;
use nom::Parser;

pub fn solve_part_1(file_content: &str) -> usize {
    let boss = parse_boss(file_content).map(|(_, boss)| boss).unwrap();

    let mut min_cost = usize::MAX;

    for weapon in &WEAPONS {
        let weapon_player = Player {
            hp: 100,
            damage: 0,
            armor: 0,
        }
        .upgrade(weapon);
        if weapon.cost > min_cost {
            continue;
        }
        for armor_amount in 0..=1 {
            for armor in ARMOR.iter().combinations(armor_amount) {
                let armor_cost = armor.iter().map(|x| x.cost).sum::<usize>();
                if armor_cost > min_cost {
                    continue;
                }
                let armored_player = armor
                    .into_iter()
                    .fold(weapon_player.clone(), |p, a| p.upgrade(a));
                for rings_amount in 0..=2 {
                    for rings in RINGS.iter().combinations(rings_amount) {
                        let rings_cost = rings.iter().map(|x| x.cost).sum::<usize>();
                        let total_cost = weapon.cost + armor_cost + rings_cost;

                        if total_cost >= min_cost {
                            continue;
                        }

                        let player = rings
                            .into_iter()
                            .fold(armored_player.clone(), |p, r| p.upgrade(r));

                        if player.wins(&boss) {
                            min_cost = total_cost;
                        }
                    }
                }
            }
        }
    }
    min_cost
}
pub fn solve_part_2(file_content: &str) -> usize {
    let boss = parse_boss(file_content).map(|(_, boss)| boss).unwrap();

    let mut max_cost = 0usize;

    for weapon in &WEAPONS {
        let weapon_player = Player {
            hp: 100,
            damage: 0,
            armor: 0,
        }
        .upgrade(weapon);
        if weapon_player.wins(&boss) {
            continue;
        }
        for armor_amount in 0..=1 {
            for armor in ARMOR.iter().combinations(armor_amount) {
                let armor_cost = armor.iter().map(|x| x.cost).sum::<usize>();
                let armored_player = armor
                    .into_iter()
                    .fold(weapon_player.clone(), |p, a| p.upgrade(a));
                if armored_player.wins(&boss) {
                    continue;
                }
                for rings_amount in 0..=2 {
                    for rings in RINGS.iter().combinations(rings_amount) {
                        let rings_cost = rings.iter().map(|x| x.cost).sum::<usize>();
                        let total_cost = weapon.cost + armor_cost + rings_cost;

                        if total_cost <= max_cost {
                            continue;
                        }

                        let player = rings
                            .into_iter()
                            .fold(armored_player.clone(), |p, r| p.upgrade(r));

                        if !player.wins(&boss) {
                            max_cost = total_cost;
                        }
                    }
                }
            }
        }
    }
    max_cost
}

fn parse_boss(input: &str) -> nom::IResult<&str, Player> {
    use nom::{bytes::complete::tag, character::complete};

    nom::sequence::tuple((
        nom::sequence::delimited(
            tag("Hit Points: "),
            complete::u64.map(|x| x as usize),
            nom::character::complete::newline,
        ),
        nom::sequence::delimited(
            tag("Damage: "),
            complete::u64.map(|x| x as usize),
            nom::character::complete::newline,
        ),
        nom::sequence::delimited(
            tag("Armor: "),
            complete::u64.map(|x| x as usize),
            nom::character::complete::newline,
        ),
    ))
    .map(|(hp, damage, armor)| Player { hp, damage, armor })
    .parse(input)
}

#[derive(Debug, Clone)]
struct Player {
    hp: usize,
    damage: usize,
    armor: usize,
}
impl Player {
    fn upgrade(&self, item: &Item) -> Player {
        Self {
            hp: self.hp,
            damage: self.damage + item.damage,
            armor: self.armor + item.armor,
        }
    }
    fn wins(&self, other: &Player) -> bool {
        let my_damage_per_other_strike = if other.damage >= self.armor {
            other.damage - self.armor
        } else {
            1
        };
        let other_damage_per_my_strike = if self.damage >= other.armor {
            self.damage - other.armor
        } else {
            1
        };
        let mut my_hp = self.hp;
        let mut other_hp = other.hp;
        loop {
            if other_hp > other_damage_per_my_strike {
                other_hp -= other_damage_per_my_strike;
            } else {
                other_hp = 0;
                return true;
            }
            if my_hp > my_damage_per_other_strike {
                my_hp -= my_damage_per_other_strike;
            } else {
                return false;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Item {
    armor: usize,
    damage: usize,
    name: &'static str,
    cost: usize,
}

const WEAPONS: [Item; 5] = [
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];
const ARMOR: [Item; 5] = [
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 6] = [
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "111");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "188");
    }
}
