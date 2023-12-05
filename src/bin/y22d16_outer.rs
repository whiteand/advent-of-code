use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use advent::y22::y22d16::parse::{self, parse_id};
use advent::y22::y22d16::valve::Valve;

const MAX_DISTANCE: u8 = 26;
const PART_COUNT: usize = 2;

#[derive(Copy, Clone, Debug)]
struct Node {
    v_id: u8,
    distance: [u8; 64],
}

impl Node {
    fn new(v_id: u8) -> Self {
        Self {
            v_id,
            distance: [u8::MAX; 64],
        }
    }

    fn build_node(&mut self, edge_map: &[u64]) {
        let mut queue = VecDeque::new();
        let mut visited = 0u64;

        queue.push_back((self.v_id, 1));
        while let Some((c_v_id, distance)) = queue.pop_front() {
            if (visited & (1 << c_v_id)) != 0 {
                continue;
            }

            visited |= 1 << c_v_id;
            self.distance[c_v_id as usize] = distance;

            let edges = edge_map[c_v_id as usize];
            for i in 0..64 {
                if (edges & (1 << i)) != 0 {
                    queue.push_back((i, distance + 1));
                }
            }
        }
    }
}

struct Cache {
    backing_array: Vec<u16>,
    node_count: usize,
}

impl Cache {
    fn new(node_count: usize) -> Self {
        let mut array_size = PART_COUNT;
        array_size *= (MAX_DISTANCE + 1) as usize;
        array_size *= 1 << node_count;
        array_size *= node_count;

        println!("Cache size: {}", array_size);

        Self {
            backing_array: vec![u16::MAX; array_size],
            node_count,
        }
    }

    fn to_index(
        &self,
        participant: usize,
        rem_distance: u8,
        current_node: usize,
        turned_valves: usize,
    ) -> usize {
        let mut array_index = participant;
        array_index = (array_index * ((MAX_DISTANCE + 1) as usize)) + (rem_distance as usize);
        array_index = (array_index * (1 << self.node_count)) + turned_valves;
        array_index = (array_index * self.node_count) + current_node;
        array_index
    }

    fn get_value(&self, index: usize) -> u16 {
        unsafe { *self.backing_array.get_unchecked(index) }
    }

    fn set_value(&mut self, index: usize, value: u16) {
        unsafe {
            *self.backing_array.get_unchecked_mut(index) = value;
        }
    }
}

fn get_max_flow(
    participant: usize,
    rem_distance: u8,
    current_node: usize,
    turned_valves: usize,
    nodes: &[Node],
    v_rates: &[u16],
    cache: &mut Cache,
) -> u16 {
    let index = cache.to_index(participant, rem_distance, current_node, turned_valves);
    if cache.get_value(index) == u16::MAX {
        let mut max_flow = 0u16;

        let node = &nodes[current_node];
        for i in 1..cache.node_count {
            if (turned_valves & (1 << i)) != 0 {
                continue;
            }

            if node.distance[i] >= rem_distance {
                continue;
            }

            let next_flow = get_max_flow(
                participant,
                rem_distance - node.distance[i],
                i,
                turned_valves | (1 << i),
                nodes,
                v_rates,
                cache,
            );

            max_flow = std::cmp::max(max_flow, next_flow);
        }

        if participant != 0 {
            let next_flow = get_max_flow(
                participant - 1,
                MAX_DISTANCE,
                0,
                turned_valves,
                nodes,
                v_rates,
                cache,
            );

            max_flow = std::cmp::max(max_flow, next_flow);
        }

        max_flow += v_rates[current_node] * (rem_distance as u16);
        cache.set_value(index, max_flow);
    }

    cache.get_value(index)
}

fn main() {
    let input = include_str!("../y22/y22d16/example.txt");

    let mut input = parse::parse(input);

    let aa = parse_id("AA").unwrap().1;

    input.sort_by(|l, r| {
        if l.name == aa {
            return Ordering::Less;
        } else if r.name == aa {
            return Ordering::Greater;
        }

        r.rate.cmp(&l.rate).then(l.name.cmp(&r.name))
    });

    let mut v_id_map = HashMap::new();
    let mut v_rates = Vec::new();
    let mut v_edges = Vec::new();
    let mut nodes = Vec::new();
    let mut interesting_nodes = 1;
    for Valve {
        name: v_id,
        rate,
        paths: edges,
    } in input
    {
        nodes.push(Node::new(v_id_map.len() as u8));
        v_id_map.insert(v_id, v_id_map.len() as u8);
        v_rates.push(rate);
        v_edges.push(edges);
        if rate != 0 {
            interesting_nodes += 1;
        }
    }

    let v_edges = v_edges
        .into_iter()
        .map(|edges| {
            let mut edge_mask = 0;
            for edge in edges {
                let edge_id = v_id_map.get(&edge).unwrap();
                edge_mask |= 1 << edge_id;
            }

            edge_mask
        })
        .collect::<Vec<_>>();

    let start = Instant::now();
    let mut cache = Cache::new(interesting_nodes);
    for node in &mut nodes[0..interesting_nodes] {
        node.build_node(&v_edges);
    }

    let max_flow = get_max_flow(
        PART_COUNT - 1,
        MAX_DISTANCE,
        0,
        0,
        &nodes,
        &v_rates,
        &mut cache,
    );

    println!("{}", max_flow);

    let end = Instant::now();
    println!("{:?}", end - start);
}
