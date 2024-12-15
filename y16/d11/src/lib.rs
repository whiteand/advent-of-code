use advent_utils::{declare_array, declare_field};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct State<const N: usize>(usize);

impl<const N: usize> State<N> {
    declare_field!(usize, elevator, set_elevator, (N * 2 * 2), 0b11);
    declare_array!(usize, microchip, set_microchip, 0, 2, 0b11);
    declare_array!(usize, generator, set_generator, (N * 2), 2, 0b11);

    fn done(&self) -> bool {
        let mask = usize::MAX >> (size_of::<usize>() * 8 - N * 2 * 2);
        self.0 & mask == mask
    }

    fn current_generators(&self) -> impl Iterator<Item = usize> {
        let s = *self;
        let elevator = s.elevator();
        (0..N).filter(move |i| s.generator(*i) == elevator)
    }
    fn current_microchips(&self) -> impl Iterator<Item = usize> + '_ {
        let s = *self;
        let elevator = s.elevator();
        (0..N).filter(move |i| s.microchip(*i) == elevator)
    }

    fn validate(self) -> Result<Self, ()> {
        for ty in 0..N {
            let level = self.microchip(ty);
            if self.generator(ty) == level {
                continue;
            }
            for ty2 in 0..N {
                if ty2 == ty {
                    continue;
                }
                if self.generator(ty2) == level {
                    return Err(());
                }
            }
        }

        Ok(self)
    }

    fn goto(self, level: usize, generators: &[usize], microchips: &[usize]) -> Result<Self, ()>
    where
        Self: std::fmt::Display,
    {
        if cfg!(debug_assertions) {
            let elevator = self.elevator();
            if !generators.iter().all(|i| self.generator(*i) == elevator) {
                return Err(());
            }
            if !microchips.iter().all(|i| self.microchip(*i) == elevator) {
                return Err(());
            }
        }
        let mut new_state = self;

        for i in 0..N {
            if generators.contains(&i) {
                new_state = new_state.set_generator(i, level);
            }
            if microchips.contains(&i) {
                new_state = new_state.set_microchip(i, level);
            }
        }

        new_state = new_state.set_elevator(level);

        new_state.validate()
    }
}

const TARGET_LEVEL: usize = 3;
#[tracing::instrument(skip(input))]
pub fn solve<const N: usize>(input: State<N>) -> usize
where
    State<N>: std::fmt::Display,
{
    let max_state = usize::MAX >> (size_of::<usize>() * 8 - N * 2 * 2 - 2);
    let mut min_steps_to = vec![usize::MAX; max_state + 1];
    let mut to_visit = VecDeque::new();
    to_visit.push_back(input);
    min_steps_to[input.0] = 0;
    while let Some(state) = to_visit.pop_front() {
        if state.done() {
            return min_steps_to[state.0];
        }
        let prev = min_steps_to[state.0];
        for s in next_states(&state) {
            let steps = prev.saturating_add(1);
            if min_steps_to[s.0] <= steps {
                continue;
            }
            min_steps_to[s.0] = steps;
            to_visit.push_back(s);
        }
    }
    min_steps_to[max_state]
}

fn next_states<const N: usize>(state: &State<N>) -> impl Iterator<Item = State<N>>
where
    State<N>: std::fmt::Display,
{
    let generators = state.current_generators().collect_vec();
    let chips = state.current_microchips().collect_vec();
    let mut res = Vec::new();
    for level in 0..=TARGET_LEVEL {
        if level.abs_diff(state.elevator()) != 1 {
            continue;
        }
        // taking 2 chips
        for i in 0..N {
            if i >= chips.len() {
                break;
            }
            for j in (i + 1)..N {
                if j >= chips.len() {
                    break;
                }
                let c1 = chips[i];
                let c2 = chips[j];
                let Ok(next_state) = state.goto(level, &[], &[c1, c2]) else {
                    continue;
                };
                res.push(next_state);
            }
        }
        // taking 2 generators
        for i in 0..N {
            if i >= generators.len() {
                break;
            }
            for j in (i + 1)..N {
                if j >= generators.len() {
                    break;
                }
                let g1 = generators[i];
                let g2 = generators[j];
                let Ok(next_state) = state.goto(level, &[g1, g2], &[]) else {
                    continue;
                };
                res.push(next_state);
            }
        }
        // taking 1 chip + 1 generator
        for i in 0..N {
            if i >= chips.len() {
                break;
            }
            for j in 0..N {
                if j >= generators.len() {
                    break;
                }
                let c1 = chips[i];
                let g1 = generators[j];
                let Ok(next_state) = state.goto(level, &[g1], &[c1]) else {
                    continue;
                };
                res.push(next_state);
            }
        }
        // taking 1 generator
        for i in 0..N {
            if i >= generators.len() {
                break;
            }
            let g1 = generators[i];
            let Ok(next_state) = state.goto(level, &[g1], &[]) else {
                continue;
            };
            res.push(next_state);
        }

        // taking 1 chip
        for i in 0..N {
            if i >= chips.len() {
                break;
            }
            let c1 = chips[i];
            let Ok(next_state) = state.goto(level, &[], &[c1]) else {
                continue;
            };
            res.push(next_state);
        }
    }

    res.into_iter()
}

/// ```ignore
/// F4 .  .  .  .  .  .  .  .  .  .  .
/// F3 .  .  .  .  .  .  .  DG DM EG EM
/// F2 .  .  .  .  BM .  CM .  .  .  .
/// F1 E  AG AM BG .  CG .  .  .  .  .
/// ```
pub const ACTUAL: State<5> = State(0b0010100000001010010100);

/// ```ignore
/// F4 .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  
/// F3 .  .  .  .  .  .  .  DG DM EG EM .  .  .  .
/// F2 .  .  .  .  BM .  CM .  .  .  .  .  .  .  .  
/// F1 E  AG AM BG .  CG .  .  .  .  .  FG FM GG GM  
/// ```
pub const ACTUAL2: State<7> = State(0b000000101000000000001010010100);

macro_rules! wr {
    ($f:ident, $label:ident, $cur:expr, $($id:ident at $actual:expr$(,)?)+) => {{
        write!($f, stringify!($label))?;
        write!($f, " ")?;
        $(
            if $actual == $cur {
                write!($f, stringify!($id))?;
                if stringify!($id).len() == 1 {
                    write!($f, "  ")?;
                } else {
                    write!($f, " ")?;
                }
            } else {
                write!($f, ".  ")?;
            }
        )+
        writeln!($f)?;
    }}
}

macro_rules! impl_display {
    ($n:expr,$self:ident, $($id:ident => $expr:expr),+) => {
        impl std::fmt::Display for State<$n> {
            fn fmt(&$self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                wr!(
                    f,
                    F4,
                    3,
                    $(
                        $id at $expr
                    ),+
                );
                wr!(
                    f,
                    F3,
                    2,
                    $(
                        $id at $expr
                    ),+
                );
                wr!(
                    f,
                    F2,
                    1,
                    $(
                        $id at $expr
                    ),+
                );
                wr!(
                    f,
                    F1,
                    0,
                    $(
                        $id at $expr
                    ),+
                );

                Ok(())
            }
        }
    }
}

impl_display!(
    2,
    self,
    E => self.elevator(),
    AG => self.generator(0),
    AM => self.microchip(0),
    BG => self.generator(1),
    BM => self.microchip(1)
);
impl_display!(
    5,
    self,
    E => self.elevator(),
    AG => self.generator(0),
    AM => self.microchip(0),
    BG => self.generator(1),
    BM => self.microchip(1),
    CG => self.generator(2),
    CM => self.microchip(2),
    DG => self.generator(3),
    DM => self.microchip(3),
    EG => self.generator(4),
    EM => self.microchip(4)
);
impl_display!(
    7,
    self,
    E => self.elevator(),
    AG => self.generator(0),
    AM => self.microchip(0),
    BG => self.generator(1),
    BM => self.microchip(1),
    CG => self.generator(2),
    CM => self.microchip(2),
    DG => self.generator(3),
    DM => self.microchip(3),
    EG => self.generator(4),
    EM => self.microchip(4),
    FG => self.generator(5),
    FM => self.microchip(5),
    GG => self.generator(6),
    GM => self.microchip(6)
);

#[cfg(test)]
mod tests {
    use super::{solve, State};

    const EXAMPLE: State<2> = State(0b0010010000);

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve(EXAMPLE)), "11");
    }

    #[test] // runs 1.79s
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve(super::ACTUAL)), "31");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve(super::ACTUAL2)), "55");
    }
}
