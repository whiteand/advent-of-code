use std::str::FromStr;

use advent_utils::ImmutableLists;
use itertools::Itertools;
use petgraph::prelude::{UnGraph, *};

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer(usize);
impl Computer {
    fn first_u8(&self) -> u8 {
        ((self.0 / 26) as u8) + b'a'
    }
}

impl From<Computer> for NodeIndex<usize> {
    fn from(value: Computer) -> Self {
        NodeIndex::new(value.0)
    }
}

impl FromStr for Computer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(());
        }
        let mut res = 0usize;
        for x in s.as_bytes() {
            if x.is_ascii_lowercase() {
                res *= 26;
                res += (x - b'a') as usize;
            } else {
                return Err(());
            }
        }
        Ok(Self(res))
    }
}
impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = self.first_u8();
        let second = (self.0 % 26) as u8;
        write!(f, "{}{}", first as char, (second + b'a') as char)
    }
}
impl std::fmt::Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let connections = parse_connections(file_content).collect_vec();
    let computers = connections
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<fxhash::FxHashSet<_>>();

    let graph: UnGraph<NodeIndex<usize>, (), usize> =
        UnGraph::from_edges(connections.iter().copied());

    let mut total = 0;

    for x in computers {
        let x_ind: NodeIndex<usize> = x.into();
        for y in graph.neighbors(x_ind) {
            if y <= x_ind {
                continue;
            }
            for z in graph.neighbors(y) {
                if z <= y {
                    continue;
                }
                if !graph.contains_edge(z, x_ind) {
                    continue;
                }
                if [x_ind, y, z]
                    .iter()
                    .any(|id| Computer(id.index()).first_u8() == b't')
                {
                    total += 1;
                }
            }
        }
    }

    total
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> String {
    let connections = parse_connections(file_content).collect_vec();
    let computers = connections
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<fxhash::FxHashSet<_>>();

    let graph: UnGraph<Computer, (), usize> = UnGraph::from_edges(connections.iter().copied());

    let mut lists = ImmutableLists::<Computer>::with_capacity(248068);

    let mut to_visit = Vec::with_capacity(550);

    for x in &computers {
        let singleton_list = lists.singleton(*x);
        to_visit.push(singleton_list);
    }

    let mut max_vertices = to_visit[0];

    while let Some(vertices) = to_visit.pop() {
        if lists.len(vertices) > lists.len(max_vertices) {
            max_vertices = vertices;
        }

        let last = lists.head(vertices).copied().unwrap().into();
        for x in graph.neighbors(last) {
            if x <= last {
                continue;
            }
            let neighbors = graph.neighbors(x).collect_vec();
            if lists
                .iter(vertices)
                .skip(1)
                .copied()
                .any(|n| !neighbors.contains(&NodeIndex::from(n)))
            {
                continue;
            }
            to_visit.push(lists.prepend(vertices, Computer(x.index())))
        }
    }
    lists
        .iter(max_vertices)
        .collect_vec()
        .into_iter()
        .rev()
        .join(",")
}

fn parse_connections(input: &str) -> impl Iterator<Item = (Computer, Computer)> + '_ {
    input.lines().filter_map(|x| {
        let (a, b) = x.split_once('-')?;
        let a = Computer::from_str(a).ok()?;
        let b = Computer::from_str(b).ok()?;
        Some((a, b))
    })
}

#[cfg(test)]
mod tests {
    use crate::day23::Computer;

    use super::{part1, part2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_parse_computer() {
        assert_eq!(
            format!("{}", "aa".parse::<Computer>().unwrap()).as_str(),
            "aa"
        );
        assert_eq!("aa".parse::<Computer>().unwrap(), Computer(0));
        assert_eq!("zz".parse::<Computer>().unwrap(), Computer(26 * 26 - 1));
    }

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(EXAMPLE)), "7");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(ACTUAL)), "1284");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).as_str(), "co,de,ka,ta");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(
            part2(ACTUAL).as_str(),
            "bv,cm,dk,em,gs,jv,ml,oy,qj,ri,uo,xk,yw"
        );
    }
}
