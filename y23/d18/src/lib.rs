use std::cmp::Ordering;

#[derive(Debug, Eq, Clone, PartialEq, PartialOrd, Ord)]
struct IVec2(isize, isize);
impl std::fmt::Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
impl IVec2 {
    fn new() -> Self {
        IVec2(0, 0)
    }
    fn set(&mut self, other: &IVec2) -> &mut Self {
        self.0 = other.0;
        self.1 = other.1;
        self
    }

    fn add_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 += other.0;
        self.1 += other.1;
        self
    }
    fn sub_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 -= other.0;
        self.1 -= other.1;
        self
    }
    fn right(&self) -> Self {
        if self.0 == 0 {
            if self.1 == 0 {
                IVec2(0, 1)
            } else if self.1 > 0 {
                IVec2(-1, 0)
            } else {
                IVec2(1, 0)
            }
        } else if self.1 == 0 {
            if self.0 > 0 {
                IVec2(0, 1)
            } else {
                IVec2(0, -1)
            }
        } else {
            panic!("invalid direction")
        }
    }

    fn mul_mut(&mut self, distance: usize) -> &mut Self {
        self.0 *= distance as isize;
        self.1 *= distance as isize;
        self
    }
    fn max_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 = self.0.max(other.0);
        self.1 = self.1.max(other.1);
        self
    }
    fn min_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 = self.0.min(other.0);
        self.1 = self.1.min(other.1);
        self
    }
}

struct Edge {
    start: IVec2,
    end: IVec2,
}

impl Edge {
    fn contains(&self, pos: &IVec2) -> bool {
        if pos.0 == self.start.0 && pos.0 == self.end.0 {
            pos.1 >= self.start.1.min(self.end.1) && pos.1 <= self.start.1.max(self.end.1)
        } else if pos.1 == self.start.1 && pos.1 == self.end.1 {
            pos.0 >= self.start.0.min(self.end.0) && pos.0 <= self.start.0.max(self.end.0)
        } else {
            false
        }
    }
    fn dir(&self) -> IVec2 {
        if self.is_horizontal() {
            if self.start.1 > self.end.1 {
                IVec2(0, -1)
            } else if self.start.1 < self.end.1 {
                IVec2(0, 1)
            } else {
                IVec2(0, 0)
            }
        } else {
            if self.start.0 > self.end.0 {
                IVec2(-1, 0)
            } else if self.start.0 < self.end.0 {
                IVec2(1, 0)
            } else {
                IVec2(0, 0)
            }
        }
    }
    fn len(&self) -> usize {
        if self.is_horizontal() {
            (self.start.1 - self.end.1).abs() as usize + 1
        } else {
            (self.start.0 - self.end.0).abs() as usize + 1
        }
    }
    fn iter(&self) -> impl Iterator<Item = IVec2> {
        let mut p = self.start.clone();
        let e = self.end.clone();
        let e2 = e.clone();
        let dp = self.dir();
        std::iter::from_fn(move || {
            if &p == &e {
                None
            } else {
                p.add_mut(&dp);
                Some(p.clone())
            }
        })
        .chain(std::iter::once(e2))
    }
    fn is_horizontal(&self) -> bool {
        self.start.0 == self.end.0
    }
    fn is_end(&self, pos: &IVec2) -> bool {
        &self.start == pos || &self.end == pos
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_horizontal() {
            write!(f, "({}, {}..={})", self.start.0, self.start.1, self.end.1)
        } else if self.start.1 == self.end.1 {
            write!(f, "({}..={}, {})", self.start.0, self.end.0, self.start.1)
        } else {
            write!(
                f,
                "({}, {})..=({}, {})",
                self.start.0, self.start.1, self.end.0, self.end.1
            )
        }
    }
}
impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn get_doubled_area(a: &IVec2, b: &IVec2) -> isize {
    a.0 * b.1 - a.1 * b.0
}

fn get_gcd(mut a: isize, mut b: isize) -> isize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
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

fn get_line_internal(a: &IVec2, b: &IVec2) -> isize {
    let dx = (b.0 - a.0).abs();
    let dy = (b.1 - a.1).abs();
    let g = get_gcd(dx, dy);
    2 + g - 1
}

fn get_internal_dots(a: &IVec2, b: &IVec2, c: &IVec2) -> isize {
    let mut ab = b.clone();
    ab.sub_mut(&a);
    let mut ac = c.clone();
    ac.sub_mut(&a);

    let s = get_doubled_area(&ab, &ac);
    let double_internal =
        s.abs() - get_line_internal(&a, &b) - get_line_internal(&b, &c) - get_line_internal(&a, &c)
            + 3
            + 2;

    // S = internal + external / 2 - 1
    // 2 * S =  + external - 2
    // internal * 2 = 2S - external + 2

    debug_assert!(double_internal % 2 == 0);

    double_internal / 2 * s.signum()
}

fn print_edges<'t, T: IntoIterator<Item = &'t Edge>>(it: T) {
    let mut min = IVec2(0, 0);
    let mut max = IVec2(0, 0);
    let mut edges = Vec::new();
    for edge in it.into_iter() {
        min.min_mut(&edge.start);
        min.min_mut(&edge.end);
        max.max_mut(&edge.start);
        max.max_mut(&edge.end);
        edges.push(edge);
    }

    let w = match edges.len() {
        0..=9 => 1,
        10..=99 => 3,
        _ => 3,
    };

    for r in min.0 - 1..=max.0 + 1 {
        for c in min.1 - 1..=max.1 + 1 {
            if let Some((ind, _)) = edges
                .iter()
                .enumerate()
                .find(|(_, edge)| edge.contains(&IVec2(r, c)))
            {
                print!("{:<w$}", ind);
            } else {
                print!("{:<w$}", '.');
            }
        }
        println!()
    }
}

struct EdgesBuilder {
    points: Vec<IVec2>,
}
impl EdgesBuilder {
    fn new() -> Self {
        Self { points: Vec::new() }
    }
    fn push(&mut self, point: IVec2) {
        self.points.push(point);
    }
    fn build(self) -> Vec<Edge> {
        let mut edges = Vec::new();
        for i in 0..self.points.len() - 1 {
            let a = &self.points[i];
            let b = &self.points[i + 1];
            assert!(a.0 == b.0 || a.1 == b.1, "Not a line");
            let edge = Edge {
                start: a.clone(),
                end: b.clone(),
            };
            edges.push(edge);
        }
        edges
    }
}

fn collapse_tripple(edge_a: Edge, edge_b: Edge, edge_c: Edge) -> (Vec<Edge>, isize) {
    let da = edge_a.dir();
    let db = edge_b.dir();
    let dc = edge_c.dir();
    let mut res = EdgesBuilder::new();

    if da == IVec2(0, 1) && db == IVec2(1, 0) && dc == IVec2(0, -1) {
        match edge_a.len().cmp(&edge_c.len()) {
            Ordering::Less => {
                let additional_area = ((edge_a.len() - 1) * edge_b.len()) as isize;
                let a = &edge_a.start;
                let b = &edge_b.start;
                let c = &edge_c.start;
                let d = &edge_c.end;
                let mut new_b = c.clone();
                new_b.add_mut(&a).sub_mut(&b);

                // .....a0b
                // .......1
                // ..d2222c
                // Should become:
                // .....0..
                // .....0..
                // ..1110..

                res.push(a.clone());
                res.push(new_b);
                res.push(d.clone());
                let edges = res.build();
                println!("Additional area = {}", additional_area);
                print_edges(edges.iter());

                return (edges, additional_area);
            }
            Ordering::Equal => {
                // a######
                // ______#
                // ______#
                // ______#
                // ______#
                // c######
                // Should become:
                // #______
                // #______
                // #______
                // #______
                // #______
                // #______
                dbg!(&edge_a, &da, &edge_b, &db, &edge_c, &dc);
                todo!("first equal");
            }
            Ordering::Greater => {
                let additional_area = ((edge_c.len() - 1) * edge_b.len()) as isize;

                let a = edge_a.start.clone();
                let b = edge_b.start.clone();
                let d = edge_c.end.clone();
                let c = edge_c.start.clone();

                let mut new_b = b.clone();
                new_b.add_mut(&d).sub_mut(&c);
                res.push(a);
                res.push(new_b);
                res.push(d);

                return (res.build(), additional_area);
            }
        }
    }

    if &da == &IVec2(1, 0) && &db == &IVec2(0, -1) && &dc == &IVec2(-1, 0) {
        match edge_a.len().cmp(&edge_c.len()) {
            Ordering::Less => {
                let additional_area = ((edge_a.len() - 1) * edge_b.len()) as isize;
                let mut new_b = edge_b.end;
                new_b.add_mut(&edge_a.start).sub_mut(&edge_a.end);
                res.push(edge_a.start);
                res.push(new_b);
                res.push(edge_c.end);
                return (res.build(), additional_area);
            }
            Ordering::Equal => {
                let additional_area = ((edge_a.len() - 1) * edge_b.len()) as isize;
                let Edge { start: a, .. } = edge_a;
                let Edge { end: d, .. } = edge_c;
                res.push(a);
                res.push(d);
                return (res.build(), additional_area);
            }
            Ordering::Greater => {
                println!("Not implemented for this case: a > c");
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a > c")
            }
        }
    }

    if &da == &IVec2(0, -1) && &db == &IVec2(-1, 0) && &dc == &IVec2(0, 1) {
        match edge_a.len().cmp(&edge_c.len()) {
            Ordering::Less => {
                let additional_area = ((edge_a.len() - 1) * edge_b.len()) as isize;
                let mut new_b = edge_b.end;
                new_b.add_mut(&edge_a.start).sub_mut(&edge_a.end);

                res.push(edge_a.start);
                res.push(new_b);
                res.push(edge_c.end);
                return (res.build(), additional_area);
            }
            Ordering::Equal => {
                let additional_area = ((edge_a.len() - 1) * edge_b.len()) as isize;
                let Edge { start: a, .. } = edge_a;
                let Edge { end: d, .. } = edge_c;
                res.push(a);
                res.push(d);
                return (res.build(), additional_area);
            }
            Ordering::Greater => {
                println!("Not implemented for this case: a > c");
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a > c")
            }
        }
    }

    if &da == &IVec2(1, 0) && &db == &IVec2(0, 1) && &dc == &IVec2(-1, 0) {
        match edge_a.len().cmp(&edge_c.len()) {
            Ordering::Less => {
                println!("Not implemented for this case: a > c");
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
            Ordering::Equal => {
                let additional_area = -(((edge_a.len() - 1) * edge_b.len()) as isize);
                let Edge { start: a, .. } = edge_a;
                let Edge { end: d, .. } = edge_c;
                res.push(a);
                res.push(d);
                return (res.build(), additional_area);
            }
            Ordering::Greater => {
                // .....  .....
                // ..0..  ..0..
                // ..0..  ..0..
                // ..02.  ..11.
                // ..02.  .....
                // ..02.  .....
                // ..01.  .....
                // .....  .....

                let additional_area = -(((edge_c.len() - 1) * edge_b.len()) as isize);
                let mut new_b = edge_a.end;
                new_b.add_mut(&edge_c.end).sub_mut(&edge_c.start);
                res.push(edge_a.start);
                res.push(new_b);
                res.push(edge_c.end);

                return (res.build(), additional_area);
            }
        }
    }
    if &da == &IVec2(0, 1) && &db == &IVec2(1, 0) && &dc == &IVec2(-1, 0) {
        match edge_a.len().cmp(&edge_c.len()) {
            Ordering::Less => {
                println!("Not implemented for this case: a < c");
                dbg!([&edge_a, &edge_b, &edge_c]);
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
            Ordering::Equal => {
                println!("Not implemented for this case: a == c");
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
            Ordering::Greater => {
                println!("Not implemented for this case: a > c");
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
        }
    }

    if &da == &IVec2(0, 1) && &db == &IVec2(-1, 0) && &dc == &IVec2(0, -1) {
        match edge_a.len().cmp(&edge_c.len()) {
            Ordering::Less => {
                println!("Not implemented for this case: a < c");
                dbg!([&edge_a, &edge_b, &edge_c]);
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
            Ordering::Equal => {
                let additional_area = -(((edge_a.len() - 1) * edge_b.len()) as isize);
                res.push(edge_a.start);
                res.push(edge_c.end);
                return (res.build(), additional_area);
            }
            Ordering::Greater => {
                println!("Not implemented for this case: a > c");
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
        }
    }

    if &da == &IVec2(0, -1) && &db == &IVec2(1, 0) && &dc == &IVec2(0, 1) {
        match edge_a.len().cmp(&edge_c.len()) {
            Ordering::Less => {
                println!("Not implemented for this case: a < c");
                dbg!([&edge_a, &edge_b, &edge_c]);
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
            Ordering::Equal => {
                let additional_area = -(((edge_a.len() - 1) * edge_b.len()) as isize);
                res.push(edge_a.start);
                res.push(edge_c.end);
                return (res.build(), additional_area);
            }
            Ordering::Greater => {
                println!("Not implemented for this case: a > c");
                print_edges([&edge_a, &edge_b, &edge_c].into_iter());
                todo!("a < c")
            }
        }
    }

    println!("Not implemented for this case: ");
    dbg!([&edge_a, &edge_b, &edge_c]);
    print_edges([edge_a, edge_b, edge_c].iter());
    println!(
        "if &da == &IVec2({}, {}) && &db == &IVec2({}, {}) && &dc == &IVec2({}, {}) {{}}",
        da.0, da.1, db.0, db.1, dc.0, dc.1
    );
    todo!("Don't know how to collapse them");
}

fn collapse_some_tripple(edges: &mut Vec<Edge>, total_area: &mut isize) {
    // 0 1 2 3 4
    let ptr = (0..(edges.len() - 2))
        .rev()
        .find_map(|i| {
            let a = &edges[i];
            let b = &edges[i + 1];
            let c = &edges[i + 2];

            let da = a.dir();
            let dc = c.dir();

            (a.is_horizontal() == c.is_horizontal()
                && a.is_horizontal() != b.is_horizontal()
                && dc.0 == -da.0
                && dc.1 == -da.1)
                .then_some(i)
        })
        .expect("There should be at least one tripple of П shape");

    let c = edges.remove(ptr + 2);
    let b = edges.remove(ptr + 1);
    let a = edges.remove(ptr);

    println!("Total Area: {}", *total_area);
    println!("To collapse:");
    print_edges([&a, &b, &c].into_iter());

    let (new_edges, additional_area) = collapse_tripple(a, b, c);
    println!("Additional area: {}", additional_area);

    for edge in new_edges.into_iter().rev() {
        edges.insert(ptr, edge);
    }
    *total_area += additional_area;
}

fn collapse_continuations(edges: &mut Vec<Edge>) {
    let mut ptr = edges.len() - 1;

    while ptr > 1 && ptr < edges.len() {
        if edges[ptr].is_horizontal() == edges[ptr - 1].is_horizontal() {
            let b = edges.remove(ptr);
            edges[ptr - 1].end.set(&b.end);
            ptr -= 1;
        } else {
            ptr -= 1;
        }
    }
}

fn solve(instructions: impl Iterator<Item = Instruction>) -> usize {
    let mut edges = instructions
        .scan(IVec2(0isize, 0isize), |pos, instruction| {
            let prev_pos = pos.clone();
            pos.add_mut(&IVec2::from(instruction.direction).mul_mut(instruction.distance));

            let edge = Edge {
                start: prev_pos,
                end: pos.clone(),
            };

            Some(edge)
        })
        .collect::<Vec<_>>();

    let mut res = 0isize;

    while edges.len() > 4 {
        println!("Before collapse tripple state:");
        print_edges(edges.iter());
        collapse_some_tripple(&mut edges, &mut res);
        println!("Before collapse continuations:");
        print_edges(edges.iter());
        collapse_continuations(&mut edges);
        println!("After collapse:\n");
        print_edges(&edges);
        println!();
    }

    dbg!(&edges, res);
    todo!("I am not sure how to calculate the are of this shape");
    0
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve(parse_instructions(file_content))
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve(
        parse_instructions(file_content).map(|instruction| Instruction {
            color: Color(instruction.distance),
            distance: instruction.color.0,
            direction: instruction.direction,
        }),
    )
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for IVec2 {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => IVec2(-1, 0),
            Right => IVec2(0, 1),
            Down => IVec2(1, 0),
            Left => IVec2(0, -1),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        match self {
            Up => write!(f, "U"),
            Right => write!(f, "R"),
            Down => write!(f, "D"),
            Left => write!(f, "L"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Color(usize);

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Color(color) = self;
        write!(f, "#{:06x}", color)
    }
}
impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Color(color) = self;
        write!(f, "#{:06x}", color)
    }
}

struct Instruction {
    direction: Direction,
    distance: usize,
    color: Color,
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.direction, self.distance, self.color)
    }
}

fn parse_instructions(file_content: &str) -> impl Iterator<Item = Instruction> + '_ {
    file_content.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let direction = match parts.next().unwrap() {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("invalid direction"),
        };
        let distance = parts.next().unwrap().parse().unwrap();
        let color_str = parts.next().unwrap();
        let color = usize::from_str_radix(&color_str[2..color_str.len() - 1], 16)
            .map(Color)
            .unwrap();
        Instruction {
            color,
            distance,
            direction,
        }
    })
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::{get_doubled_area, solve_part_1, solve_part_2, IVec2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case(IVec2(0, 4), IVec2(3, 0), -12)]
    #[case(IVec2(3, 0), IVec2(0, 4), 12)]
    #[case(IVec2(1, 3), IVec2(3, 1), -8)]
    #[case(IVec2(0, 1), IVec2(1, 0), -1)]
    #[case(IVec2(1, 0), IVec2(0, 1), 1)]
    #[ignore]

    fn test_get_area(#[case] a: IVec2, #[case] b: IVec2, #[case] expected: isize) {
        assert_eq!(get_doubled_area(&a, &b), expected);
    }

    #[rstest]
    #[case(IVec2(0, 0), IVec2(3, 0), IVec2(0, 4), 3)]
    #[case(IVec2(0, 0), IVec2(0, 4), IVec2(3, 0), -3)]
    #[ignore]
    fn test_get_internal_dots(
        #[case] a: IVec2,
        #[case] b: IVec2,
        #[case] c: IVec2,
        #[case] expected: isize,
    ) {
        assert_eq!(super::get_internal_dots(&a, &b, &c), expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "62");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        assert_eq!(solve_part_1(ACTUAL), 56923);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "952408144115");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
