use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use super::{parse::parse_id, valve::Valve};

pub fn precalculate_shortest_paths(
    valves_map: &BTreeMap<usize, Valve>,
) -> BTreeMap<(usize, usize), Vec<usize>> {
    let reachable = get_reachable_valves(valves_map, parse_id("AA").unwrap().1);

    let mut memory: BTreeMap<(usize, usize), Option<Vec<usize>>> = BTreeMap::new();

    for &from in valves_map.keys().filter(|k| reachable.contains(*k)) {
        let mut tasks: BinaryHeap<ShortestPath> = BinaryHeap::new();
        tasks.push(Vec::new().into());
        while let Some(ShortestPath(path)) = tasks.pop() {
            let current = *path.iter().last().unwrap_or(&from);
            if !memory.contains_key(&(from, current)) && from != current {
                memory.insert((from, current), Some(path.clone()));
            }
            for &neighbour in &valves_map.get(&current).unwrap().paths {
                if path.contains(&neighbour) {
                    continue;
                }
                if memory.contains_key(&(from, neighbour)) {
                    continue;
                }
                let mut new_task = path.clone();
                new_task.push(neighbour);
                tasks.push(new_task.into());
            }
        }
    }

    memory
        .into_iter()
        .flat_map(|(k, mp)| mp.map(|p| (k, p)))
        .collect()
}

#[derive(Debug)]
struct ShortestPath(Vec<usize>);

impl PartialEq for ShortestPath {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}
impl Eq for ShortestPath {}

impl PartialOrd for ShortestPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ShortestPath {
    fn cmp(&self, other: &Self) -> Ordering {
        let n = self.0.len();
        let m = other.0.len();
        match n.cmp(&m) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            x => x,
        }
    }
}
impl From<Vec<usize>> for ShortestPath {
    fn from(v: Vec<usize>) -> Self {
        ShortestPath(v)
    }
}

fn get_reachable_valves(valves_map: &BTreeMap<usize, Valve>, from: usize) -> BTreeSet<usize> {
    let mut visited = BTreeSet::new();
    let mut tasks = vec![from];
    while let Some(valve) = tasks.pop() {
        visited.insert(valve);
        for neighbour in &valves_map.get(&valve).unwrap().paths {
            if !visited.contains(neighbour) {
                tasks.push(*neighbour)
            }
        }
    }
    visited
}
