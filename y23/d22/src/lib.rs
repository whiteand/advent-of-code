use std::{collections::BTreeMap, ops::Range};

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

#[allow(dead_code)]
fn print_height_map(h: &BTreeMap<(usize, usize), Vec<(usize, usize)>>) {
    if h.is_empty() {
        return;
    }
    let min_x = h.first_key_value().map(|x| x.0 .0).unwrap();
    let max_x = h.last_key_value().map(|x| x.0 .0).unwrap();
    let (min_y, max_y) = h.keys().map(|(_, y)| *y).minmax().into_option().unwrap();

    print!("{:3} ", "");
    for y in min_y..=max_y {
        print!("{:3} ", y);
    }
    println!();
    print!("----");
    for _ in min_y..=max_y {
        print!("----");
    }
    println!();

    for x in min_x..=max_x {
        print!("{:3}|", x);
        for y in min_y..=max_y {
            let height = h
                .get(&(x, y))
                .and_then(|v| v.last().map(|v| v.1))
                .unwrap_or(0);
            if height == 0 {
                print!("  . ");
                continue;
            }
            print!("{:3} ", height);
        }
        println!();
    }
    println!();
}

struct Relations {
    supported_by: Vec<Vec<usize>>,
    supports: Vec<Vec<usize>>,
}
impl Relations {
    fn ids(&self) -> Range<usize> {
        0..self.supports.len()
    }
    fn get_supporters_for(&self, id: &usize) -> &[usize] {
        &self.supported_by[*id]
    }
    fn get_supported_by(&self, id: &usize) -> &[usize] {
        &self.supports[*id]
    }
}

fn get_relations(file_content: &str) -> Relations {
    let mut bricks = parse(file_content).collect_vec();
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

    Relations {
        supports,
        supported_by,
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    let relations = get_relations(file_content);
    relations
        .ids()
        .filter(|brick_id| {
            relations
                .get_supported_by(brick_id)
                .iter()
                .all(|supported_brick_id| {
                    relations.get_supporters_for(supported_brick_id).len() != 1
                })
        })
        .count()
}
pub fn solve_part_2(_file_content: &str) -> usize {
    todo!("part 2 is not implemented yet")
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "5");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(solve_part_1(ACTUAL), 490);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
