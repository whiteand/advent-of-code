use advent_utils::{glam::IVec2, grid::Grid};
use derive_more::{IsVariant, TryFrom};

#[tracing::instrument(skip(input))]
pub fn part1(input: &str) -> usize {
    solve::<Splits>(input)
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &str) -> usize {
    solve::<Timelines>(input)
}

fn solve<S: TraceStrategy>(input: &str) -> S::Result {
    let (grid, start) = parse_input(input);
    trace::<S>(&grid, vec![Particle::new(start)])
}

fn parse_input(input: &str) -> (Grid<Cell>, IVec2) {
    let grid: Grid<Cell> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .copied()
                .map(|c| Cell::try_from(c).unwrap_or_default())
        })
        .collect();

    let start = grid
        .entries_copy()
        .find_map(|(p, c)| c.is_start().then_some(p))
        .expect("should be present");

    (grid, start)
}

fn trace<S: TraceStrategy>(grid: &Grid<Cell>, particles: Vec<Particle>) -> S::Result {
    let mut state = S::default();
    let mut multiverse = Multiverse::new(grid.max_column(), particles);
    let mut next_multiverse = Multiverse::new(grid.max_column(), []);

    while !multiverse.is_empty() {
        for particle in multiverse.take_particles() {
            match grid.get_copy(particle.look_forward()) {
                Some(Cell::Empty) => {
                    next_multiverse.add_particles(&[particle.move_forward()]);
                }
                Some(Cell::Splitter) => {
                    next_multiverse.add_particles(particle.split().as_slice());
                    state.on_split();
                }
                None => {
                    state.on_particle_reached_end(particle);
                }
                Some(Cell::Start) => unreachable!(),
            }
        }
        std::mem::swap(&mut multiverse, &mut next_multiverse);
    }

    state.get_result()
}

trait TraceStrategy: Default {
    type Result;
    fn on_split(&mut self) {}
    fn on_particle_reached_end(&mut self, _: Particle) {}
    fn get_result(self) -> Self::Result;
}

#[derive(Default)]
struct Splits {
    splits: usize,
}
impl TraceStrategy for Splits {
    type Result = usize;

    fn get_result(self) -> Self::Result {
        self.splits
    }

    fn on_split(&mut self) {
        self.splits += 1;
    }
}

#[derive(Default)]
struct Timelines {
    timelines: usize,
}

impl TraceStrategy for Timelines {
    type Result = usize;

    fn get_result(self) -> Self::Result {
        self.timelines
    }

    fn on_particle_reached_end(&mut self, particle: Particle) {
        self.timelines += particle.timelines;
    }
}

#[derive(Clone)]
struct Multiverse {
    particles: Vec<Option<Particle>>,
    present: Vec<usize>,
}

impl Multiverse {
    fn new<T: IntoIterator<Item = Particle>>(width: usize, particles: T) -> Self {
        let mut res = vec![None; width];
        let mut present = Vec::with_capacity(width);
        for p in particles.into_iter() {
            let ind = p.pos.x as usize;
            res[p.pos.x as usize] = Some(p);
            present.push(ind);
        }
        Self {
            particles: res,
            present,
        }
    }
    fn add_particles(&mut self, particles: &[Particle]) {
        for particle in particles {
            match self.particles.get_mut(particle.pos.x as usize) {
                Some(Some(next)) => {
                    next.timelines += particle.timelines;
                }
                Some(None) => {
                    let ind = particle.pos.x as usize;
                    self.particles[ind] = Some(*particle);
                    self.present.push(ind)
                }
                None => unreachable!(),
            }
        }
    }
    fn take_particles(&mut self) -> impl Iterator<Item = Particle> {
        self.present
            .drain(..)
            .filter_map(|i| self.particles[i].take())
    }
    fn is_empty(&self) -> bool {
        self.present.is_empty()
    }
}

#[derive(TryFrom, Default, Copy, Clone, IsVariant)]
#[try_from(repr)]
#[repr(u8)]
enum Cell {
    #[default]
    Empty = b'.',
    Splitter = b'^',
    Start = b'S',
}

#[derive(Copy, Clone)]
struct Particle {
    pos: IVec2,
    timelines: usize,
}

impl Particle {
    fn new(pos: IVec2) -> Self {
        Self { pos, timelines: 1 }
    }
    fn split(&self) -> [Self; 2] {
        [
            Self {
                pos: self.pos + IVec2::Y + IVec2::NEG_X,
                timelines: self.timelines,
            },
            Self {
                pos: self.pos + IVec2::Y + IVec2::X,
                timelines: self.timelines,
            },
        ]
    }
    fn move_forward(&self) -> Self {
        Self {
            pos: self.pos + IVec2::Y,
            timelines: self.timelines,
        }
    }
    fn look_forward(&self) -> IVec2 {
        self.pos + IVec2::Y
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "21")]
    #[case::actual(ACTUAL, "1537")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "40")]
    #[case::actual(ACTUAL, "18818811755665")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
