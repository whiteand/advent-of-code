use std::collections::HashMap;

use advent_utils::{
    glam::IVec2,
    grid::Grid,
    nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, line_ending},
        combinator::{all_consuming, value},
        multi::{many1, separated_list1},
        parse_usize,
    },
};
use itertools::Itertools;
use rustsat::{
    instances::SatInstance,
    solvers::{Solve, SolverResult},
    types::{Lit, TernaryVal, Var, constraints::CardConstraint},
};

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let (_, (shapes, tasks)) = all_consuming(parse_input).parse(file_content).unwrap();
    tracing::info!(tasks = ?tasks.len(), "parsed");
    tasks
        .into_iter()
        .filter(|t| {
            let area = t.area();
            let all_shapes_area = t
                .shapes_number
                .iter()
                .enumerate()
                .map(|(shape_index, n)| shapes[shape_index][0].area() * n)
                .sum();
            area >= all_shapes_area
        })
        .filter(|t| can_pack2(&shapes, &t))
        .count()
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct VarDescription {
    shape: Shape,
    shape_index: usize,
    instance_index: usize,
    row: usize,
    col: usize,
}

impl VarDescription {
    fn instance_id(self) -> usize {
        (self.instance_index << 9) | self.shape.normalize().bitmask
    }
    fn coords(self) -> impl Iterator<Item = (usize, usize)> {
        let Self { row, col, .. } = self;
        self.shape.iter().map(move |(r, c)| (row + r, col + c))
    }
}

impl PartialOrd for VarDescription {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VarDescription {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.instance_id()
            .cmp(&other.instance_id())
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| self.shape.bitmask.cmp(&other.shape.bitmask))
    }
}

fn can_pack2(shapes: &[Vec<Shape>], task: &Task) -> bool {
    tracing::info!(
        "{}x{}: {}",
        task.width,
        task.height,
        task.shapes_number.iter().join(" ")
    );

    let mut instance: SatInstance = SatInstance::new();

    let mut position_literals = rustc_hash::FxHashMap::<VarDescription, Lit>::default();

    for d in build_var_descriptions(shapes, task) {
        // tracing::info!(
        //     "\nShape:\n{:?}\nat [{}, {}]",
        //     description.shape,
        //     description.row,
        //     description.col
        // );
        let v = instance.new_lit();
        // tracing::info!(shape_index = ?d.shape_index, instance_index = ?d.instance_index, dinstance_id = ?d.instance_id());
        position_literals.insert(d, v);
    }

    for (_, single_instance_placements) in &position_literals
        .iter()
        .sorted_unstable_by_key(|(k, _)| k.instance_id())
        .chunk_by(|(k, _)| k.instance_id())
    {
        // tracing::info!(?k, "Ensuring that only one is present");
        let vars = single_instance_placements.map(|(_, l)| *l).collect_vec();
        instance.add_card_constr(CardConstraint::new_eq(vars, 1));
    }

    for ((a_def, a_lit), (b_def, b_lit)) in position_literals
        .iter()
        .map(|(k, v)| (*k, *v))
        .cartesian_product(position_literals.iter().map(|(k, v)| (*k, *v)))
        .filter(|((a, _), (b, _))| a.instance_id() != b.instance_id())
    {
        if a_def.coords().any(|c| b_def.coords().contains(&c)) {
            instance.add_card_constr(CardConstraint::new_ub([a_lit, b_lit], 1));
        }
    }

    // tracing::info!(?instance, "instance");

    let mut solver = rustsat_kissat::Kissat::default();

    solver.add_cnf(instance.into_cnf().0).unwrap();
    match solver.solve() {
        Ok(SolverResult::Sat) => {}
        res => {
            tracing::info!(?res, "solve");
            return false;
        }
    };

    let sol = match solver.full_solution() {
        Ok(res) => res,
        Err(err) => {
            tracing::warn!(?err, ?task, "{err:?}");
            return false;
        }
    };

    let mut grid = Grid::new(IVec2::new(task.width as i32, task.height as i32), b'.');

    let mut i = 0;
    for (d, lit) in position_literals.into_iter() {
        let val = sol[lit.var()];
        // tracing::info!(shape_index = ?d.shape_index, instance_id = ?d.instance_index, ?lit, ?val, "var");

        if val == TernaryVal::True {
            let char = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[i];
            i += 1;
            set_shape_at(d.shape, &mut grid, d.row, d.col, char, b'.');
        } else if val == TernaryVal::DontCare {
            println!("dont care about {:?} at [{}, {}]", d.shape, d.row, d.col);
        }
    }

    println!("Grid:\n{}", grid.render_ascii());

    true
}

fn build_var_descriptions(
    shapes: &[Vec<Shape>],
    task: &Task,
) -> impl Iterator<Item = VarDescription> {
    let Task { height, width, .. } = task;
    let height = *height;
    let width = *width;
    task.shapes_number
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(shape_index, n)| {
            (0..n).map(move |instance_index| (shape_index, instance_index))
        })
        .flat_map(move |(shape_index, instance_index)| {
            (0..height)
                .cartesian_product(0..width)
                .flat_map(move |(r, c)| {
                    shapes[shape_index]
                        .iter()
                        .copied()
                        .filter(move |shape| can_place_in_bounds(*shape, width, height, r, c))
                        .map(move |shape| VarDescription {
                            shape,
                            shape_index,
                            instance_index,
                            row: r,
                            col: c,
                        })
                })
        })
}
fn can_pack(shapes: &[Vec<Shape>], task: &Task) -> bool {
    tracing::info!(
        "{}x{}: {}",
        task.width,
        task.height,
        task.shapes_number.iter().join(" ")
    );
    let mut grid = Grid::new(IVec2::new(task.width as i32, task.height as i32), false);
    let mut shapes_number = task.shapes_number.to_vec();
    can_pack_into_grid(&shapes, &mut shapes_number, &mut grid)
}

fn can_pack_into_grid(
    shapes: &[Vec<Shape>],
    shapes_numbers: &mut [usize],
    grid: &mut Grid<bool>,
) -> bool {
    // println!("Grid:\n{}", grid.render_ascii());
    // tracing::info!(?shapes_numbers);
    if shapes_numbers.is_empty() || shapes.is_empty() {
        return true;
    }
    let last_shape_index = shapes_numbers.len() - 1;
    let last_shape_number = shapes_numbers[last_shape_index];

    if last_shape_number == 0 {
        return can_pack_into_grid(
            &shapes[0..last_shape_index],
            &mut shapes_numbers[0..last_shape_index],
            grid,
        );
    }

    let last_shape_variations = &shapes[last_shape_index];

    for r in 0..grid.rows_len() {
        for c in 0..grid.cols(r) {
            for shape in last_shape_variations.into_iter().copied() {
                if can_place_at(shape, grid, r, c) {
                    set_shape_at(shape, grid, r, c, true, false);
                    shapes_numbers[last_shape_index] -= 1;
                    if can_pack_into_grid(shapes, shapes_numbers, grid) {
                        return true;
                    }
                    shapes_numbers[last_shape_index] += 1;
                    set_shape_at(shape, grid, r, c, false, true);
                }
            }
        }
    }

    false
}

fn set_shape_at<T: Copy + Eq + std::fmt::Debug>(
    shape: Shape,
    grid: &mut Grid<T>,
    row: usize,
    col: usize,
    value: T,
    empty_value: T,
) {
    for (r, c) in shape.iter() {
        let row = row + r;
        let col = col + c;
        assert_eq!(grid.get_copy_at(row, col), Some(empty_value));
        grid.set_at(row, col, value);
    }
}

fn can_place_in_bounds(shape: Shape, width: usize, height: usize, row: usize, col: usize) -> bool {
    shape.iter().all(|(r, c)| {
        let row = row + r;
        let col = col + c;
        (0..width).contains(&col) && (0..height).contains(&row)
    })
}
fn can_place_at(shape: Shape, grid: &Grid<bool>, row: usize, col: usize) -> bool {
    shape
        .iter()
        .all(|(r, c)| grid.get_copy_at(r + row, c + col) == Some(false))
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content.len()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Shape {
    bitmask: usize,
}

impl Shape {
    fn empty() -> Self {
        Self { bitmask: 0 }
    }
    fn area(self) -> usize {
        self.bitmask.count_ones() as usize
    }
    fn has(&self, row: usize, col: usize) -> bool {
        (self.bitmask & (1 << (row * 3 + col))) != 0
    }
    fn set(&self, row: usize, col: usize, present: bool) -> Self {
        let bitmask = if present {
            self.bitmask | (1 << (row * 3 + col))
        } else {
            self.bitmask & (!(1 << (row * 3 + col)))
        };

        Self { bitmask }
    }

    fn normalize(self) -> Self {
        self.variations().min_by_key(|f| f.bitmask).unwrap()
    }

    fn flip(self) -> Self {
        (0..3)
            .flat_map(|r| (0..3).map(move |c| (r, c)))
            .fold(Shape::empty(), |s, (r, c)| s.set(r, c, self.has(c, r)))
    }

    fn rotate(self) -> Self {
        let [a, b, c] = [self.has(0, 0), self.has(0, 1), self.has(0, 2)];
        let [d, e, f] = [self.has(1, 0), self.has(1, 1), self.has(1, 2)];
        let [g, h, i] = [self.has(2, 0), self.has(2, 1), self.has(2, 2)];
        Shape::from_iter([[g, d, a], [h, e, b], [i, f, c]].into_iter().flatten())
    }

    fn variations(self) -> impl Iterator<Item = Self> {
        [self, self.flip()]
            .into_iter()
            .flat_map(|s| std::iter::successors(Some(s), |s| Some(s.rotate())).take(4))
            .unique()
    }

    // Iterates over (row, col) of the shape
    fn iter(self) -> impl Iterator<Item = (usize, usize)> {
        (0..3)
            .flat_map(|r| (0..3).map(move |c| (r, c)))
            .filter(move |(r, c)| self.has(*r, *c))
    }
}

impl FromIterator<bool> for Shape {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        std::iter::zip((0..3).flat_map(|r| (0..3).map(move |c| (r, c))), iter)
            .fold(Shape::empty(), |s, ((r, c), v)| s.set(r, c, v))
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            if i > 0 {
                writeln!(f, "")?;
            }
            for j in 0..3 {
                let bit = (self.bitmask & (1 << (i * 3 + j))) != 0;
                if bit {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Task {
    width: usize,
    height: usize,
    shapes_number: Vec<usize>,
}

impl Task {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<Shape>>, Vec<Task>)> {
    let (input, shapes) = parse_shapes(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, tasks) = parse_tasks(input)?;
    let all_ways_shapes = shapes
        .iter()
        .copied()
        .map(|s| s.variations().collect_vec())
        .collect_vec();

    Ok((input, (all_ways_shapes, tasks)))
}
fn parse_shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    separated_list1((line_ending, line_ending), parse_shape).parse(input)
}
fn parse_shape(input: &str) -> IResult<&str, Shape> {
    let (input, _) = parse_usize(input)?;
    let (input, _) = (char(':'), line_ending).parse(input)?;
    let (input, lines) = separated_list1(line_ending, parse_bits).parse(input)?;
    let shape = lines
        .into_iter()
        .flat_map(|bits| bits.into_iter().map(|b| b == 1))
        .collect::<Shape>();

    Ok((input, shape))
}
fn parse_bits(input: &str) -> IResult<&str, Vec<usize>> {
    many1(alt((value(1, char('#')), value(0, char('.'))))).parse(input)
}
fn parse_tasks(input: &str) -> IResult<&str, Vec<Task>> {
    separated_list1(line_ending, parse_task).parse(input)
}
fn parse_task(input: &str) -> IResult<&str, Task> {
    let (input, width) = parse_usize(input)?;
    let (input, _) = char('x').parse(input)?;
    let (input, height) = parse_usize(input)?;
    let (input, _) = tag(": ").parse(input)?;
    let (input, shapes_number) = separated_list1(char(' '), parse_usize).parse(input)?;

    Ok((
        input,
        Task {
            width,
            height,
            shapes_number,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "2")] // too long for now
    // #[case::actual(ACTUAL, "0")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
