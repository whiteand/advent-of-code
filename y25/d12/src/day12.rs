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
use itertools::{Itertools, iproduct};
use rustsat::{
    instances::SatInstance,
    solvers::{Solve, SolverResult},
    types::{Lit, TernaryVal, constraints::CardConstraint},
};

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let (_, (shapes, tasks)) = all_consuming(parse_input).parse(file_content).unwrap();

    tasks
        .into_iter()
        .filter(|t| {
            let area = t.area();
            let all_shapes_area = get_all_shapes_area(&t, &shapes);
            area >= all_shapes_area
        })
        .filter(|t| {
            let places = (t.width / 3) * (t.height / 3);
            let total_shapes_to_place = t.shapes_number.iter().copied().sum::<usize>();
            places >= total_shapes_to_place || can_pack(&shapes, &t)
        })
        .count()
}

fn get_all_shapes_area(task: &Task, shapes: &Grid<Shape>) -> usize {
    task.shapes_number
        .iter()
        .enumerate()
        .map(|(shape_index, n)| shapes.row(shape_index).unwrap()[0].area() * n)
        .sum()
}

struct VarsHolder<'t> {
    // 0..width * height = cell
    cells: Vec<Lit>,
    shapes: &'t Grid<Shape>,
    shape_at_position: Vec<Lit>,
    task: &'t Task,
}
impl<'t> VarsHolder<'t> {
    fn new(shapes: &'t Grid<Shape>, task: &'t Task) -> Self {
        Self {
            cells: Vec::with_capacity(task.width * task.height),
            shape_at_position: Vec::with_capacity(
                shapes.iter().count() * (task.width - 2) * (task.height - 2),
            ),
            task,
            shapes,
        }
    }
    fn build_cell_vars(&mut self, instance: &mut SatInstance) {
        for _ in 0..self.task.width * self.task.height {
            self.cells.push(instance.new_lit());
        }
        instance.add_card_constr(CardConstraint::new_eq(
            self.cells.iter().copied(),
            get_all_shapes_area(self.task, self.shapes),
        ));
    }

    fn build_shape_at_position(&mut self, instance: &mut SatInstance) {
        let mut lits = Vec::with_capacity(2000);
        for row in 0..(self.task.height - 2) {
            for col in 0..(self.task.width - 2) {
                let pos_start = self.shape_at_position.len();
                for shape in self.shapes.iter() {
                    let lit = instance.new_lit();
                    self.shape_at_position.push(lit);
                    for (r, c) in shape.iter() {
                        let pos_lit = self.cell_var(r + row, c + col);
                        lits.push(pos_lit)
                    }
                    instance.add_lit_impl_cube(lit, lits.as_slice());
                    lits.clear();
                }
                let pos_end = self.shape_at_position.len();
                // only a single shape can be placed at the same position
                instance.add_card_constr(CardConstraint::new_ub(
                    self.shape_at_position[pos_start..pos_end].iter().copied(),
                    1,
                ));
            }
        }

        // Only a specified number of shapes are allowed
        for (shape_index, n) in self.task.shapes_number.iter().copied().enumerate() {
            let variations = self.shapes.row(shape_index).unwrap();
            for (r, c, s) in iproduct!(
                0..(self.task.height - 2),
                0..(self.task.width - 2),
                variations.iter()
            ) {
                let lit = self.shape_at_position_var(*s, r, c);
                lits.push(lit)
            }
            instance.add_card_constr(CardConstraint::new_eq(lits.as_slice().iter().copied(), n));
            lits.clear();
        }

        // 0 ......
        // 1 ......
        // 2 ......
        // 3 ......

        // If a cell is full it means that some shape is present
        for r in 0..self.task.height {
            for c in 0..self.task.width {
                let cell_lit = self.cell_var(r, c);
                let shape_rows = r.saturating_sub(2)..=r.min(self.task.height - 3);
                let shape_cols = c.saturating_sub(2)..=c.min(self.task.width - 3);
                for (sr, sc, shape) in
                    iproduct!(shape_rows, shape_cols, self.shapes.all().iter().copied())
                {
                    if shape
                        .iter()
                        .map(|(r, c)| (r + sr, c + sc))
                        .any(|(r2, c2)| r2 == r && c2 == c)
                    {
                        let lit = self.shape_at_position_var(shape, sr, sc);
                        lits.push(lit)
                    }
                }
                instance.add_lit_impl_clause(cell_lit, lits.as_slice());
                instance.add_clause_impl_lit(lits.as_slice(), cell_lit);
                lits.clear();
            }
        }

        // shape cannot "not touch" something on the left or on the right
        for (shape_index, n) in self.task.shapes_number.iter().copied().enumerate() {
            if n == 0 {
                continue;
            }
            let variations = self.shapes.row(shape_index).unwrap();
            for (r, c, s) in iproduct!(
                0..(self.task.height - 2),
                0..(self.task.width - 2),
                variations.iter()
            ) {
                let shape_lit = self.shape_at_position_var(*s, r, c);
                for (r0, c0) in iproduct!(r.saturating_sub(1)..r, c.saturating_sub(1)..c) {
                    for (sr, sc) in s.iter() {
                        let cell_r = r0 + sr;
                        let cell_c = c0 + sc;
                        let cell_lit = self.cell_var(cell_r, cell_c);
                        lits.push(cell_lit);
                    }
                    instance.add_lit_impl_clause(shape_lit, lits.as_slice());
                    lits.clear();
                }
            }
        }
    }

    fn shape_at_position_var(&self, shape: Shape, row: usize, col: usize) -> Lit {
        let ind =
            (row * (self.task.width - 2) + col) * self.shapes.all().len() + shape.global_index();
        self.shape_at_position[ind]
    }

    fn cell_var(&self, row: usize, col: usize) -> Lit {
        let ind = row * self.task.width + col;
        self.cells[ind]
    }
}

fn can_pack(shapes: &Grid<Shape>, task: &Task) -> bool {
    tracing::info!(
        "{}x{}: {}",
        task.width,
        task.height,
        task.shapes_number.iter().join(" ")
    );

    let mut instance: SatInstance = SatInstance::new();

    let mut vars_holder = VarsHolder::new(&shapes, task);

    tracing::info!("building cell vars");
    vars_holder.build_cell_vars(&mut instance);

    tracing::info!("build_shape_at_position");
    vars_holder.build_shape_at_position(&mut instance);

    tracing::info!("init solver");
    let mut solver = rustsat_kissat::Kissat::default();

    solver
        .set_configuration(rustsat_kissat::Config::Unsat)
        .unwrap();

    solver.add_cnf(instance.into_cnf().0).unwrap();
    tracing::info!("solving");
    match solver.solve() {
        Ok(SolverResult::Sat) => {}
        res => {
            tracing::info!(?res, "solve");
            return false;
        }
    };
    tracing::info!("solved");

    let sol = match solver.full_solution() {
        Ok(res) => res,
        Err(err) => {
            tracing::warn!(?err, ?task, "{err:?}");
            return false;
        }
    };
    tracing::info!("full solution");

    let mut grid = Grid::new(IVec2::new(task.width as i32, task.height as i32), b'.');

    for r in 0..task.height {
        for c in 0..task.width {
            let lit = vars_holder.cell_var(r, c);
            let val = sol[lit.var()];
            if val == TernaryVal::True {
                grid.set_at(r, c, b'#');
            } else if val == TernaryVal::DontCare {
                println!("dont care about [{}, {}]", r, c);
            }
        }
    }

    // let mut i = 0;
    // for (d, lit) in position_literals.into_iter() {
    //     let val = sol[lit.var()];
    //     // tracing::info!(shape_index = ?d.shape_index, instance_id = ?d.instance_index, ?lit, ?val, "var");

    // }

    println!("Grid:\n{}", grid.render_ascii());

    true
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Shape {
    bitmask: usize,
}

impl Shape {
    fn new(shape_index: usize) -> Self {
        Self {
            bitmask: shape_index << 9,
        }
    }
    fn area(self) -> usize {
        (self.bitmask & 0b111111111).count_ones() as usize
    }
    fn has(&self, row: usize, col: usize) -> bool {
        assert!(row < 3);
        assert!(col < 3);
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

    fn shape_index(self) -> usize {
        (self.bitmask >> 9) & 0b111
    }

    fn flip(self) -> Self {
        (0..3)
            .flat_map(|r| (0..3).map(move |c| (r, c)))
            .fold(Shape::new(self.shape_index()), |s, (r, c)| {
                s.set(r, c, self.has(c, r))
            })
    }

    fn rotate(self) -> Self {
        Self::new(self.shape_index())
            .set(0, 0, self.has(2, 0))
            .set(0, 1, self.has(1, 0))
            .set(0, 2, self.has(0, 0))
            .set(1, 0, self.has(2, 1))
            .set(1, 1, self.has(1, 1))
            .set(1, 2, self.has(0, 1))
            .set(2, 0, self.has(2, 2))
            .set(2, 1, self.has(1, 2))
            .set(2, 2, self.has(0, 2))
    }

    fn set_global_index(self, global_index: usize) -> Self {
        Self {
            bitmask: (self.bitmask & 0b111111111111111) | (global_index << 15),
        }
    }
    fn global_index(self) -> usize {
        (self.bitmask >> 15) & 0b11111
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

fn parse_input(input: &str) -> IResult<&str, (Grid<Shape>, Vec<Task>)> {
    let (input, shapes) = parse_shapes(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, tasks) = parse_tasks(input)?;
    let mut all_ways_shapes = shapes
        .iter()
        .copied()
        .map(|s| s.variations())
        .collect::<Grid<_>>();

    for (i, x) in all_ways_shapes.all_mut().iter_mut().enumerate() {
        *x = x.set_global_index(i)
    }

    Ok((input, (all_ways_shapes, tasks)))
}
fn parse_shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    separated_list1((line_ending, line_ending), parse_shape).parse(input)
}
fn parse_shape(input: &str) -> IResult<&str, Shape> {
    let (input, index) = parse_usize(input)?;
    let (input, _) = (char(':'), line_ending).parse(input)?;
    let (input, lines) = separated_list1(line_ending, parse_bits).parse(input)?;
    let shape = lines
        .into_iter()
        .flat_map(|bits| bits.into_iter().map(|b| b == 1))
        .enumerate()
        .fold(Shape::new(index), |a, (i, b)| a.set(i / 3, i % 3, b));

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
    use super::part1;
    use rstest::rstest;
    // const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    // #[case::example(EXAMPLE, "2")] // too long
    #[case::actual(ACTUAL, "433")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
}
