use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct State<const N: usize>(usize);

macro_rules! declare_field {
    ( $f:ident,$set_f:ident, $offset:expr, $mask:expr) => {
        #[inline(always)]
        fn $f(&self) -> usize {
            (self.0 >> $offset) & $mask
        }
        #[inline(always)]
        fn $set_f(&self, value: usize) -> Self {
            Self((!($mask << $offset) & self.0) | (value << $offset))
        }
    };
}
macro_rules! declare_array {
    ($f:ident, $set_f:ident, $offset:expr, $elem_bits:expr, $elem_mask:expr) => {
        #[inline(always)]
        fn $f(&self, i: usize) -> usize {
            (self.0 >> ((i * $elem_bits) + $offset)) & $elem_mask
        }
        #[inline(always)]
        fn $set_f(&self, i: usize, value: usize) -> Self {
            Self(
                (!($elem_mask << ((i * $elem_bits) + $offset)) & self.0)
                    | (value << ((i * $elem_bits) + $offset)),
            )
        }
    };
}

impl<const N: usize> State<N> {
    declare_field!(elevator, set_elevator, (N * 2 * 2), 0b11);
    declare_array!(microchip, set_microchip, 0, 2, 0b11);
    declare_array!(generator, set_generator, (N * 2), 2, 0b11);

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
impl std::fmt::Display for State<2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        wr!(
            f,
            F4,
            3,
            E at self.elevator(),
            HG at self.generator(0), HM at self.microchip(0),
            LG at self.generator(1), LM at self.microchip(1),
        );
        wr!(
            f,
            F3,
            2,
            E at self.elevator(),
          HG at self.generator(0), HM at self.microchip(0),
            LG at self.generator(1), LM at self.microchip(1),
        );
        wr!(
            f,
            F2,
            1,
            E at self.elevator(),
          HG at self.generator(0), HM at self.microchip(0),
            LG at self.generator(1), LM at self.microchip(1),
        );
        wr!(
            f,
            F1,
            0,
            E at self.elevator(),
          HG at self.generator(0), HM at self.microchip(0),
            LG at self.generator(1), LM at self.microchip(1),
        );

        Ok(())
    }
}
impl std::fmt::Display for State<5> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        wr!(
            f,
            F4,
            3,
            E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
        );
        wr!(
            f,
            F3,
            2,
            E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
        );
        wr!(
            f,
            F2,
            1,
         E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
        );
        wr!(
            f,
            F1,
            0,
         E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
        );

        Ok(())
    }
}
impl std::fmt::Display for State<7> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        wr!(
            f,
            F4,
            3,
            E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
            FG at self.generator(5), FM at self.microchip(5),
            GG at self.generator(6), GM at self.microchip(6),
        );
        wr!(
            f,
            F3,
            2,
            E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
            FG at self.generator(5), FM at self.microchip(5),
            GG at self.generator(6), GM at self.microchip(6),
        );
        wr!(
            f,
            F2,
            1,
            E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
            FG at self.generator(5), FM at self.microchip(5),
            GG at self.generator(6), GM at self.microchip(6),
        );
        wr!(
            f,
            F1,
            0,
            E at self.elevator(),
            AG at self.generator(0), AM at self.microchip(0),
            BG at self.generator(1), BM at self.microchip(1),
            CG at self.generator(2), CM at self.microchip(2),
            DG at self.generator(3), DM at self.microchip(3),
            EG at self.generator(4), EM at self.microchip(4),
            FG at self.generator(5), FM at self.microchip(5),
            GG at self.generator(6), GM at self.microchip(6),
        );

        Ok(())
    }
}

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
