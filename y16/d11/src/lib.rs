use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct State<const N: usize>(usize);

impl<const N: usize> State<N> {
    fn done(&self) -> bool {
        let mask = usize::MAX >> (size_of::<usize>() * 8 - N * 2 * 2);
        self.0 & mask == mask
    }
    fn elevator(&self) -> usize {
        (self.0 >> (N * 2 * 2)) & 0b11
    }
    fn set_elevator(&self, value: usize) -> Self {
        Self((!(0b11 << (N * 2 * 2)) & self.0) | (value << (N * 2 * 2)))
    }
    #[inline(always)]
    fn microchip(&self, i: usize) -> usize {
        (self.0 >> (i * 2)) & 0b11
    }
    #[inline(always)]
    fn set_microchip(&self, i: usize, value: usize) -> Self {
        Self((!(0b11 << (i * 2)) & self.0) | (value << (i * 2)))
    }
    #[inline(always)]
    fn generator(&self, i: usize) -> usize {
        (self.0 >> ((i * 2) + N * 2)) & 0b11
    }
    #[inline(always)]
    fn set_generator(&self, i: usize, value: usize) -> Self {
        Self((!(0b11 << ((i * 2) + N * 2)) & self.0) | (value << ((i * 2) + N * 2)))
    }
    fn current_generators(&self) -> impl Iterator<Item = usize> {
        let elevator = self.elevator();
        let b = self.0;
        (0..N).filter(move |i| ((b >> (i * 2 + N * 2)) & 0b11) == elevator)
    }
    fn current_microchips(&self) -> impl Iterator<Item = usize> + '_ {
        let elevator = self.elevator();
        let b = self.0;
        (0..N).filter(move |i| ((b >> (i * 2)) & 0b11) == elevator)
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

#[tracing::instrument(skip(input))]
pub fn solve<const N: usize>(input: State<N>) -> usize
where
    State<N>: std::fmt::Display,
{
    const TARGET_LEVEL: usize = 3;
    let (_, cost) = pathfinding::prelude::dijkstra(
        &input,
        |x| {
            tracing::info!("new state:\n{x}");
            let generators = x.current_generators().collect_vec();
            let chips = x.current_microchips().collect_vec();
            let mut res = Vec::new();
            for level in 0..=TARGET_LEVEL {
                if level.abs_diff(x.elevator()) != 1 {
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
                        let Ok(next_state) = x.goto(level, &[], &[c1, c2]) else {
                            continue;
                        };
                        res.push((next_state, 1));
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
                        let Ok(next_state) = x.goto(level, &[g1, g2], &[]) else {
                            continue;
                        };
                        res.push((next_state, 1));
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
                        let Ok(next_state) = x.goto(level, &[g1], &[c1]) else {
                            continue;
                        };
                        res.push((next_state, 1));
                    }
                }
                // taking 1 generator
                for i in 0..N {
                    if i >= generators.len() {
                        break;
                    }
                    let g1 = generators[i];
                    let Ok(next_state) = x.goto(level, &[g1], &[]) else {
                        continue;
                    };
                    res.push((next_state, 1));
                }

                // taking 1 chip
                for i in 0..N {
                    if i >= chips.len() {
                        break;
                    }
                    let c1 = chips[i];
                    let Ok(next_state) = x.goto(level, &[], &[c1]) else {
                        continue;
                    };
                    res.push((next_state, 1));
                }
            }

            res
        },
        |s| s.done(),
    )
    .unwrap();

    // for s in states {
    //     println!("{s}\n")
    // }

    cost
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
    #[ignore] // runs 17s in release
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve(super::ACTUAL2)), "0");
    }
}
