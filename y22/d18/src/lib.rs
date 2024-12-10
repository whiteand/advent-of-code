use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

pub fn get_neighbours(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

pub fn find_exterior_area(dots: impl Iterator<Item = (i32, i32, i32)>) -> usize {
    let mut d: BTreeMap<(i32, i32, i32), u8> = Default::default();
    for (x, y, z) in dots {
        let mut open_edges = 6;

        for neighbour in get_neighbours(x, y, z) {
            if !d.contains_key(&neighbour) {
                continue;
            }
            open_edges -= 1;
            *d.entry(neighbour).or_insert(6) -= 1;
        }

        d.insert((x, y, z), open_edges);
    }
    d.values().map(|x| *x as usize).sum()
}

fn find_internal_dots(dots: &[(i32, i32, i32)]) -> Vec<(i32, i32, i32)> {
    let min_x = dots.iter().map(|p| p.0).min().unwrap_or(i32::MAX);
    let max_x = dots.iter().map(|p| p.0).max().unwrap_or(i32::MIN);
    let min_y = dots.iter().map(|p| p.1).min().unwrap_or(i32::MAX);
    let max_y = dots.iter().map(|p| p.1).max().unwrap_or(i32::MIN);
    let min_z = dots.iter().map(|p| p.2).min().unwrap_or(i32::MAX);
    let max_z = dots.iter().map(|p| p.2).max().unwrap_or(i32::MIN);

    let mut grid = vec![
        vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
        (max_z - min_z + 1) as usize
    ];

    for dot in dots {
        grid[(dot.2 - min_z) as usize][(dot.1 - min_y) as usize][(dot.0 - min_x) as usize] =
            u32::MAX;
    }

    let mut color = 1;
    loop {
        let mut initial = None;
        for z in min_z..=max_z {
            if initial.is_some() {
                break;
            }
            for y in min_y..=max_y {
                if initial.is_some() {
                    break;
                }
                for x in min_x..=max_x {
                    if initial.is_some() {
                        break;
                    }
                    let x_ind = (x - min_x) as usize;
                    let y_ind = (y - min_y) as usize;
                    let z_ind = (z - min_z) as usize;
                    let v = grid[z_ind][y_ind][x_ind];
                    if v == 0 {
                        initial = Some((x, y, z));
                    }
                }
            }
        }
        if initial.is_none() {
            break;
        }
        let initial = initial.unwrap();
        let mut visited = BTreeSet::new();
        let mut tasks = vec![initial];
        while let Some((x, y, z)) = tasks.pop() {
            grid[(z - min_z) as usize][(y - min_y) as usize][(x - min_x) as usize] = color;
            visited.insert((x, y, z));
            for neighbour in get_neighbours(x, y, z) {
                if visited.contains(&neighbour) {
                    continue;
                }
                if neighbour.0 < min_x {
                    continue;
                }
                if neighbour.1 < min_y {
                    continue;
                }
                if neighbour.2 < min_z {
                    continue;
                }
                let x_ind = (neighbour.0 - min_x) as usize;
                let y_ind = (neighbour.1 - min_y) as usize;
                let z_ind = (neighbour.2 - min_z) as usize;
                let neighbour_v = grid
                    .get(z_ind)
                    .and_then(|g| g.get(y_ind).and_then(|g| g.get(x_ind)));
                if neighbour_v.is_none() {
                    continue;
                }
                let neighbour_v = *neighbour_v.unwrap();
                if neighbour_v == 0 {
                    tasks.push(neighbour);
                }
            }
        }
        color += 1;
    }

    let mut not_internal_colors = BTreeSet::new();
    not_internal_colors.insert(u32::MAX);
    for z in min_z..=max_z {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == min_x || x == max_x || y == min_y || y == max_y || z == max_z || z == min_z
                {
                    let value =
                        grid[(z - min_z) as usize][(y - min_y) as usize][(x - min_x) as usize];

                    not_internal_colors.insert(value);
                }
            }
        }
    }
    let mut res = Vec::new();
    for z in min_z..=max_z {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let value = grid[(z - min_z) as usize][(y - min_y) as usize][(x - min_x) as usize];
                if not_internal_colors.contains(&value) {
                    continue;
                }
                res.push((x, y, z));
            }
        }
    }
    res
}

pub fn solve_part_1(file_content: &str) -> usize {
    find_exterior_area(parse(file_content))
}
pub fn solve_part_2(file_content: &str) -> impl std::fmt::Display {
    let dots = parse(file_content).collect_vec();
    let external_and_internal = find_exterior_area(dots.iter().cloned());
    let internal_dots = find_internal_dots(&dots);
    let external_area_of_internal = find_exterior_area(internal_dots.into_iter());
    external_and_internal - external_area_of_internal
}

fn parse(file_content: &str) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
    file_content
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2]))
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "64");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "3432");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "58");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "2042");
    }
}
