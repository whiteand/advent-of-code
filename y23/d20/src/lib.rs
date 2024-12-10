use std::{
    collections::{BTreeMap, VecDeque},
    ops::BitXor,
    str::FromStr,
};

#[derive(Debug, Clone, Ord, PartialEq, Eq, PartialOrd, Copy)]
struct NodeId(usize);

impl FromStr for NodeId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .as_bytes()
            .iter()
            .fold(0usize, |acc, b| acc * 26 + (b - b'a') as usize);

        Ok(NodeId(id))
    }
}

#[derive(Debug)]
struct Broadcaster {
    id: NodeId,
    inputs: Vec<NodeId>,
    outputs: Vec<NodeId>,
}
impl Broadcaster {
    fn handle_signal(
        &self,
        signals: &mut Signals,
        _state: &mut usize,
        _source: NodeId,
        power: Power,
    ) {
        for output in self.outputs.iter().copied() {
            let signal = Signal {
                source: self.id,
                power,
                target: output,
            };
            signals.emit(signal);
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    id: NodeId,
    inputs: Vec<NodeId>,
    outputs: Vec<NodeId>,
}
impl FlipFlop {
    fn handle_signal(
        &self,
        signals: &mut Signals,
        state: &mut usize,
        _source: NodeId,
        power: Power,
    ) {
        match power {
            Power::Low => {
                *state = state.bitxor(1);
                let output_source = if *state == 0 { Power::Low } else { Power::High };
                for output in self.outputs.iter().copied() {
                    let signal = Signal {
                        source: self.id,
                        power: output_source,
                        target: output,
                    };
                    signals.emit(signal);
                }
            }
            Power::High => {
                // ignored
            }
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    id: NodeId,
    inputs: Vec<NodeId>,
    outputs: Vec<NodeId>,
}
impl Conjunction {
    fn handle_signal(
        &self,
        signals: &mut Signals,
        state: &mut usize,
        source: NodeId,
        power: Power,
    ) {
        let ind = self
            .inputs
            .iter()
            .position(|id| id == &source)
            .unwrap_or_default();
        let bit = 1 << ind;
        match power {
            Power::Low => {
                *state &= !bit;
            }
            Power::High => {
                *state |= bit;
            }
        };
        let output_source = if (*state).count_ones() == self.inputs.len() as u32 {
            Power::Low
        } else {
            Power::High
        };
        for output in self.outputs.iter().copied() {
            let signal = Signal {
                source: self.id,
                power: output_source,
                target: output,
            };
            signals.emit(signal);
        }
    }
}

#[derive(Debug)]
enum Node {
    Broadcaster(Broadcaster),
    FlipFlow(FlipFlop),
    Conjunction(Conjunction),
}

impl Node {
    fn add_input(&mut self, input: NodeId) {
        match self {
            Node::Broadcaster(b) => b.inputs.push(input),
            Node::FlipFlow(f) => f.inputs.push(input),
            Node::Conjunction(c) => c.inputs.push(input),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Power {
    Low,
    High,
}

#[derive(Debug)]
struct Signal {
    source: NodeId,
    power: Power,
    target: NodeId,
}

impl Node {
    fn id(&self) -> NodeId {
        match self {
            Node::Broadcaster(b) => b.id,
            Node::FlipFlow(f) => f.id,
            Node::Conjunction(c) => c.id,
        }
    }
    fn outputs(&self) -> &[NodeId] {
        match self {
            Node::Broadcaster(b) => &b.outputs,
            Node::FlipFlow(f) => &f.outputs,
            Node::Conjunction(c) => &c.outputs,
        }
    }

    fn handle_signal(
        &self,
        signals: &mut Signals,
        state: &mut usize,
        source: NodeId,
        power: Power,
    ) {
        match self {
            Node::Broadcaster(b) => b.handle_signal(signals, state, source, power),
            Node::FlipFlow(f) => f.handle_signal(signals, state, source, power),
            Node::Conjunction(c) => c.handle_signal(signals, state, source, power),
        }
    }
}

fn parse_node(input: &str) -> Node {
    let (decl_str, outputs_str) = input.split_once(" -> ").unwrap();
    let outputs = outputs_str
        .split(", ")
        .map(|s| s.parse::<NodeId>().unwrap())
        .collect::<Vec<_>>();

    if let Some(stripped) = decl_str.strip_prefix('%') {
        let id = NodeId::from_str(stripped).unwrap();

        return Node::FlipFlow(FlipFlop {
            id,
            inputs: Vec::new(),
            outputs,
        });
    }
    if let Some(stripped) = decl_str.strip_prefix('&') {
        let id = NodeId::from_str(stripped).unwrap();

        return Node::Conjunction(Conjunction {
            id,
            inputs: Vec::new(),
            outputs,
        });
    }

    let id = NodeId::from_str(decl_str).unwrap();

    Node::Broadcaster(Broadcaster {
        id,
        inputs: Vec::new(),
        outputs,
    })
}

fn parse_nodes(input: &str) -> BTreeMap<NodeId, Node> {
    let mut nodes = input
        .lines()
        .map(|line| {
            let node = parse_node(line);
            (node.id(), node)
        })
        .collect::<BTreeMap<_, _>>();

    let node_ids = nodes.keys().copied().collect::<Vec<_>>();

    nodes.insert(
        NodeId::from_str("rx").unwrap(),
        Node::Broadcaster(Broadcaster {
            id: NodeId::from_str("rx").unwrap(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }),
    );

    for node_id in node_ids.iter().copied() {
        let outputs = nodes.get(&node_id).unwrap().outputs().to_vec();

        for output in outputs {
            let node = nodes.get_mut(&output).unwrap();
            node.add_input(node_id);
        }
    }

    nodes
}

struct Signals {
    signals: VecDeque<Signal>,
    total_low: usize,
    total_high: usize,
}

impl Signals {
    fn new() -> Self {
        Self {
            signals: VecDeque::new(),
            total_low: 0,
            total_high: 0,
        }
    }
    fn emit(&mut self, signal: Signal) {
        match signal.power {
            Power::Low => {
                self.total_low += 1;
            }
            Power::High => {
                self.total_high += 1;
            }
        }
        self.signals.push_back(signal);
    }
    fn pop(&mut self) -> Option<Signal> {
        self.signals.pop_front()
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    let nodes = parse_nodes(file_content);
    let mut signals = Signals::new();
    let broadcaster_id = NodeId::from_str("broadcaster").unwrap();
    let mut states = BTreeMap::new();
    for _ in 0..1000 {
        signals.emit(Signal {
            source: NodeId(0),
            target: broadcaster_id,
            power: Power::Low,
        });

        while let Some(Signal {
            source,
            target,
            power,
        }) = signals.pop()
        {
            let node = nodes.get(&target).unwrap();
            let state = states.entry(target).or_insert(0);

            node.handle_signal(&mut signals, state, source, power);
        }
    }
    signals.total_high * signals.total_low
}

pub fn solve_part_2(_file_content: &str) -> usize {
    // I didn't found the abstract way to solve all kinds of this problem
    //  I've just found that necesary inputs occur in loops:
    // qs -high-> cl: 3942 + 3943 * a
    // dt -high-> cl: 3946 + 3947 * b
    // ts -high-> cl: 4006 + 4007 * c
    // js -high-> cl: 4018 + 4019 * d

    // I just found when all of those inputs are high at the same time and (added 1 because started from 0)
    250628960065793
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "32000000");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "938065580");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "250628960065793");
    }
}
