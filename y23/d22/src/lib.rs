use std::{
    collections::{BTreeMap, VecDeque},
    ops::Range,
};

use itertools::Itertools;

#[derive(Clone, Copy)]
struct P3 {
    x: usize,
    y: usize,
    z: usize,
}

impl P3 {
    fn from_iter<T: Iterator<Item = usize>>(it: &mut T) -> Result<Self, String> {
        let x = it.next().ok_or_else(|| "expected x".to_owned())?;
        let y = it.next().ok_or_else(|| "expected y".to_owned())?;
        let z = it.next().ok_or_else(|| "expected z".to_owned())?;
        Ok(Self { x, y, z })
    }
}

impl std::fmt::Debug for P3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

struct Brick {
    points: [P3; 2],
}

impl Brick {
    fn get_min_z(&self) -> usize {
        self.points.iter().map(|p| p.z).min().expect("no min")
    }
    fn iter(&self) -> impl Iterator<Item = P3> {
        let p0 = self.points[0];
        let p1 = self.points[1];
        let min_x = p0.x.min(p1.x);
        let max_x = p0.x.max(p1.x);
        let min_y = p0.y.min(p1.y);
        let max_y = p0.y.max(p1.y);
        let min_z = p0.z.min(p1.z);
        let max_z = p0.z.max(p1.z);

        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;

        x_range.flat_map(move |x| {
            let z_range = min_z..=max_z;
            y_range
                .clone()
                .flat_map(move |y| z_range.clone().map(move |z| P3 { x, y, z }))
        })
    }
}

impl std::fmt::Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}~{:?}", self.points[0], self.points[1])
    }
}

struct Relations {
    supported_by: Vec<Vec<usize>>,
    supports: Vec<Vec<usize>>,
}
impl Relations {
    fn ids(&self) -> Range<usize> {
        0..self.supports.len()
    }
    fn get_supporters_for(&self, id: usize) -> &[usize] {
        &self.supported_by[id]
    }
    fn get_supported_by(&self, id: usize) -> &[usize] {
        &self.supports[id]
    }
    fn get_fallen_if_removed(&self, id: usize) -> impl Iterator<Item = usize> + '_ {
        self.get_supported_by(id)
            .iter()
            .filter(move |supported_id| {
                self.get_supporters_for(**supported_id)
                    .iter()
                    .filter(move |supporter_id| **supporter_id != id)
                    .count()
                    == 0
            })
            .copied()
    }

    fn get_fallen_count_if_removed_recursive(&self, id: usize) -> usize {
        let mut to_fall = VecDeque::from([id]);
        let mut fallen = self.ids().map(|_| false).collect_vec();
        let mut count = 0;
        while let Some(id) = to_fall.pop_front() {
            if fallen[id] {
                continue;
            }
            fallen[id] = true;
            count += 1;
            for supported_id in self.get_supported_by(id).iter().filter(|id| !fallen[**id]) {
                assert!(!self.get_supporters_for(*supported_id).is_empty());

                let has_not_fallen_suporters = self
                    .get_supporters_for(*supported_id)
                    .iter()
                    .any(|supporter_id| !fallen[*supporter_id]);

                if has_not_fallen_suporters {
                    continue;
                }
                to_fall.push_back(*supported_id);
            }
        }
        count - 1
    }
}

fn get_relations(bricks: impl Iterator<Item = Brick>) -> Relations {
    let mut bricks = bricks.collect_vec();
    bricks.sort_by_key(|b| b.get_min_z());

    let mut height_map: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();
    let mut supported_by: Vec<Vec<usize>> = bricks.iter().map(|_| Vec::new()).collect_vec();
    let mut supports: Vec<Vec<usize>> = bricks.iter().map(|_| Vec::new()).collect_vec();

    for (id, brick) in bricks.iter().enumerate() {
        let min_z = brick.get_min_z();
        if min_z == 1 {
            for p in brick.iter() {
                let relative_height = p.z - min_z + 1;
                height_map
                    .entry((p.x, p.y))
                    .and_modify(|boxes_in_pos| {
                        boxes_in_pos.retain(|(i, _)| *i != id);
                    })
                    .or_default()
                    .push((id, relative_height));
            }
            continue;
        }

        let supported_by_ids = supported_by.get_mut(id).unwrap();

        let mut base_height = 0;

        for p in brick.iter() {
            for (id, height) in height_map.get(&(p.x, p.y)).into_iter().flatten().rev() {
                if *height > p.z {
                    unreachable!(
                        "there is no way something is placed here already above the current brick"
                    );
                }
                if *height == p.z {
                    unreachable!("collision");
                }
                if *height < base_height {
                    continue;
                }
                if *height == base_height {
                    if supported_by_ids.contains(id) {
                        continue;
                    }
                    supported_by_ids.push(*id);
                    continue;
                }
                base_height = *height;
                supported_by_ids.clear();
                supported_by_ids.push(*id);
                continue;
            }
        }

        for p in brick.iter() {
            let relative_height = p.z - min_z + 1;
            height_map
                .entry((p.x, p.y))
                .and_modify(|boxes_in_pos| {
                    boxes_in_pos.retain(|(i, _)| *i != id);
                })
                .or_default()
                .push((id, base_height + relative_height));
        }
    }

    for id in 0..bricks.len() {
        for supporter_id in supported_by[id].iter() {
            if supports[*supporter_id].contains(&id) {
                continue;
            }
            supports[*supporter_id].push(id);
        }
    }

    for id in 0..bricks.len() {
        for supporter_id in supported_by[id].iter() {
            assert!(supports[*supporter_id].contains(&id));
        }
        for supported_id in supports[id].iter() {
            assert!(supported_by[*supported_id].contains(&id));
        }
    }

    Relations {
        supports,
        supported_by,
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    let relations = get_relations(parse(file_content));
    relations
        .ids()
        .filter(|brick_id| relations.get_fallen_if_removed(*brick_id).count() == 0)
        .count()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let relations = get_relations(parse(file_content));

    relations
        .ids()
        .map(move |id| relations.get_fallen_count_if_removed_recursive(id))
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = Brick> + '_ {
    input.lines().map(|line| {
        let mut coordinates = line
            .split(|x| x == ',' || x == '~')
            .map(|s| s.parse::<usize>().expect("invalid number"));
        let p0 = P3::from_iter(&mut coordinates).unwrap();
        let p1 = P3::from_iter(&mut coordinates).unwrap();

        Brick { points: [p0, p1] }
    })
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(solve_part_1(EXAMPLE), 5);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(solve_part_1(ACTUAL), 490);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part_2(EXAMPLE), 7);
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(solve_part_2(ACTUAL), 96356);
    }
}
