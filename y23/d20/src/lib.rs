use std::{
    collections::{BTreeMap, VecDeque, BTreeSet},
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

    fn get_output_beam(&self, input: &[Beam]) -> Beam {
        todo!()
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

    fn get_output_beam(&self, input: &[Beam]) -> Beam {
        let mut low_frequency = Frequency::never();
        let mut high_frequency = Frequency::never();

        for inp in input {
            let low_inp = &inp.low;

            let mut new_low = Vec::new();
            let mut new_high = Vec::new();

            for (i, r) in low_inp.remainders.iter().enumerate() {
                if i % 2 == 0 {
                    new_high.push(*r);
                } else {
                    new_low.push(*r);
                }
            }

            for r in 0..new_low.len() {
                new_low.push(new_low[r] + low_inp.divisor);
            }
            for r in 0..new_high.len() {
                new_high.push(new_high[r] + low_inp.divisor);
            }

            if !new_low.is_empty() {
                let mut new_low_frequency = Frequency {
                    divisor: low_inp.divisor * 2,
                    remainders: new_low,
                };
                Frequency::unify(&mut low_frequency, &mut new_low_frequency);
                low_frequency = low_frequency.or(&new_low_frequency);
            }

            if !new_high.is_empty() {
                let mut new_high_frequency = Frequency {
                    divisor: low_inp.divisor * 2,
                    remainders: new_high,
                };

                Frequency::unify(&mut high_frequency, &mut new_high_frequency);
                high_frequency = high_frequency.or(&new_high_frequency);
            }
        }

        Beam {
            low: low_frequency,
            high: high_frequency,
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

    fn get_output_beam(&self, input: &[Beam]) -> Beam {
        let mut all_inputs = input
            .iter()
            .fold(Frequency::never(), |acc, b| acc.or(&b.low).or(&b.high));
        let mut all_high = input.iter().fold(Frequency::any(), |acc, b| acc.and(&b));
        let mut any_low = all_inputs.remainders
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

    fn get_output_beam(&self, input: &[Beam]) -> Beam {
        match self {
            Node::Broadcaster(b) => b.get_output_beam(input),
            Node::FlipFlow(f) => f.get_output_beam(input),
            Node::Conjunction(c) => c.get_output_beam(input),
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
    fn inputs(&self) -> &[NodeId] {
        match self {
            Node::Broadcaster(b) => &b.inputs,
            Node::FlipFlow(f) => &f.inputs,
            Node::Conjunction(c) => &c.inputs,
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

    if decl_str.starts_with('%') {
        let id = NodeId::from_str(&decl_str[1..]).unwrap();

        return Node::FlipFlow(FlipFlop {
            id,
            inputs: Vec::new(),
            outputs,
        });
    }
    if decl_str.starts_with('&') {
        let id = NodeId::from_str(&decl_str[1..]).unwrap();

        return Node::Conjunction(Conjunction {
            id,
            inputs: Vec::new(),
            outputs,
        });
    }

    let id = NodeId::from_str(decl_str).unwrap();

    return Node::Broadcaster(Broadcaster {
        id,
        inputs: Vec::new(),
        outputs,
    });
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
        let outputs = nodes
            .get(&node_id)
            .unwrap()
            .outputs()
            .iter()
            .copied()
            .collect::<Vec<_>>();

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

#[derive(Clone, PartialEq, Eq)]
struct Frequency {
    divisor: usize,
    remainders: Vec<usize>,
}

impl std::fmt::Debug for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, r) in self.remainders.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", r)?;
            } else {
                write!(f, ", {}", r)?;
            }
        }
        write!(f, "; {}]", self.divisor)
    }
}

fn get_gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    while a > 0 && b > 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    a.max(b)
}

impl Frequency {
    fn never() -> Frequency {
        Frequency {
            divisor: 1,
            remainders: vec![],
        }
    }
    fn any() -> Frequency {
        Frequency {
            divisor: 1,
            remainders: vec![0],
        }
    }

    fn and(&self, other: &Self) -> Self {
        assert_eq!(self.divisor, other.divisor);
        let mut current_remainders = self.remainders.iter().copied()
            .collect::<BTreeSet<_>>();
        let other_remainders = other.remainders.iter().copied()
            .collect::<BTreeSet<_>>();

        current_remainders.retain(|r| other_remainders.contains(r));


        let new_remainders = current_remainders.iter().copied().collect::<Vec<_>>();

        Self {
            divisor: self.divisor,
            remainders: new_remainders,
        }
    }
    fn or(&self, other: &Self) -> Self {
        assert_eq!(self.divisor, other.divisor);
        let mut remainders = self.remainders.clone();
        remainders.extend(other.remainders.iter().copied());
        remainders.sort_unstable();
        remainders.dedup();

        Self {
            divisor: self.divisor,
            remainders,
        }
    }

    fn upgrade(&mut self, higher_divisor: usize) {
        let gcd = get_gcd(higher_divisor, self.divisor);
        let new_divisor = higher_divisor / gcd * self.divisor;

        let mut new_remainders = Vec::new();

        for r in 0..(higher_divisor / gcd) {
            for &old_r in &self.remainders {
                new_remainders.push(r * self.divisor + old_r);
            }
        }

        self.divisor = new_divisor;
        self.remainders = new_remainders;
    }

    fn unify(first: &mut Frequency, second: &mut Frequency) {
        match first.divisor.cmp(&second.divisor) {
            std::cmp::Ordering::Less => first.upgrade(second.divisor),
            std::cmp::Ordering::Equal => return,
            std::cmp::Ordering::Greater => {
                second.upgrade(first.divisor);
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Beam {
    high: Frequency,
    low: Frequency,
}

impl Beam {
    fn never() -> Self {
        Self {
            high: Frequency::never(),
            low: Frequency::never(),
        }
    }
    fn is_empty(&self) -> bool {
        self.high.remainders.is_empty() && self.low.remainders.is_empty()
    }
}

pub fn solve_part_2(file_content: &str) -> usize {
    let nodes = parse_nodes(file_content);
    let node_ids = nodes.keys().copied().collect::<Vec<_>>();
    let broadcaster_id = NodeId::from_str("broadcaster").unwrap();
    let mut beams = BTreeMap::<NodeId, Beam>::new();

    beams.insert(
        broadcaster_id,
        Beam {
            high: Frequency::never(),
            low: Frequency::any(),
        },
    );

    for _ in 0..100 {
        dbg!(&beams);
        let mut found_new = false;
        for node_id in &node_ids {
            let previous_beam = beams
                .get(&node_id)
                .cloned()
                .unwrap_or_else(|| Beam::never());

            dbg!(node_id, &previous_beam);

            let node = nodes.get(&node_id).unwrap();

            let input_beams = node
                .inputs()
                .iter()
                .map(|id| beams.get(id).cloned().unwrap_or_else(|| Beam::never()))
                .collect::<Vec<_>>();

            let beam = node.get_output_beam(&input_beams);

            dbg!(&beam);

            if beam == previous_beam {
                continue;
            }
            if beam.is_empty() {
                continue;
            }

            beams.insert(*node_id, beam);
            found_new = true;
        }

        if !found_new {
            break;
        }
    }

    dbg!(beams);

    0
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
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "-1");
    }
}
