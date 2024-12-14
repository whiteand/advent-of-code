use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn solve<const N: usize>(input: State<N>) -> usize
where
    State<N>: std::fmt::Display,
{
    const TARGET_LEVEL: usize = 3;
    let (states, cost) = pathfinding::prelude::dijkstra(
        &input,
        |x| {
            let generators = x.current_generators().collect_vec();
            let chips = x.current_microchips().collect_vec();
            let mut levels = vec![];
            if x.elevator < TARGET_LEVEL {
                levels.push(x.elevator + 1);
            }
            if x.elevator > 0 {
                levels.push(x.elevator - 1);
            }
            let mut res = Vec::new();
            for level in levels {
                for (cn, gn) in (0..=chips.len()).cartesian_product(0..=generators.len()) {
                    if cn == 0 && gn == 0 {
                        continue;
                    }
                    if cn + gn > 2 {
                        continue;
                    }
                    for cs in chips.iter().copied().combinations(cn) {
                        for gs in generators.iter().copied().combinations(gn) {
                            let Ok(next_state) = x.goto(level, &gs, &cs) else {
                                continue;
                            };
                            res.push((next_state, 1));
                        }
                    }
                }
            }
            res
        },
        |s| s.all_on_level(TARGET_LEVEL),
    )
    .unwrap();

    for s in states {
        println!("{s}\n")
    }

    cost
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State<const N: usize> {
    generators: [usize; N],
    microchips: [usize; N],
    elevator: usize,
}

impl<const N: usize> State<N> {
    fn all_on_level(&self, level: usize) -> bool {
        return self
            .generators
            .iter()
            .chain(self.microchips.iter())
            .all(|x| *x == level);
    }
    fn current_generators(&self) -> impl Iterator<Item = usize> + '_ {
        (0..N).filter(|i| self.generators[*i] == self.elevator)
    }
    fn current_microchips(&self) -> impl Iterator<Item = usize> + '_ {
        (0..N).filter(|i| self.microchips[*i] == self.elevator)
    }
    fn goto(self, level: usize, generators: &[usize], microchips: &[usize]) -> Result<Self, ()> {
        if cfg!(debug_assertions) {
            if !generators
                .iter()
                .all(|i| self.generators[*i] == self.elevator)
            {
                return Err(());
            }
            if !microchips
                .iter()
                .all(|i| self.microchips[*i] == self.elevator)
            {
                return Err(());
            }
        }

        let mut new_generators = self.generators;
        for g in generators {
            new_generators[*g] = level;
        }
        let mut new_microchips = self.microchips;
        for m in microchips {
            new_microchips[*m] = level;
        }

        let res = Self {
            elevator: level,
            microchips: new_microchips,
            generators: new_generators,
        };

        res.validate()
    }
    fn validate(self) -> Result<Self, ()> {
        for ty in 0..N {
            let level = self.microchips[ty];
            if self.generators[ty] == level {
                continue;
            }
            for ty2 in 0..N {
                if ty2 == ty {
                    continue;
                }
                if self.generators[ty2] == level {
                    return Err(());
                }
            }
        }

        Ok(self)
    }
}

/// ```ignore
/// F4 .  .  .  .  .  .  .  .  .  .  .
/// F3 .  .  .  .  .  .  .  DG DM EG EM
/// F2 .  .  .  .  BM .  CM .  .  .  .
/// F1 E  AG AM BG .  CG .  .  .  .  .
/// ```
pub const ACTUAL: State<5> = State {
    generators: [0, 0, 0, 2, 2],
    microchips: [0, 1, 1, 2, 2],
    elevator: 0,
};

/// ```ignore
/// F4 .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  
/// F3 .  .  .  .  .  .  .  DG DM EG EM .  .  .  .
/// F2 .  .  .  .  BM .  CM .  .  .  .  .  .  .  .  
/// F1 E  AG AM BG .  CG .  .  .  .  .  FG FM GG GM  
/// ```
pub const ACTUAL2: State<7> = State {
    generators: [0, 0, 0, 2, 2, 0, 0],
    microchips: [0, 1, 1, 2, 2, 0, 0],
    elevator: 0,
};

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
            E at self.elevator,
            HG at self.generators[0], HM at self.microchips[0],
            LG at self.generators[1], LM at self.microchips[1],
        );
        wr!(
            f,
            F3,
            2,
            E at self.elevator,
            HG at self.generators[0], HM at self.microchips[0],
            LG at self.generators[1], LM at self.microchips[1],
        );
        wr!(
            f,
            F2,
            1,
            E at self.elevator,
            HG at self.generators[0], HM at self.microchips[0],
            LG at self.generators[1], LM at self.microchips[1],
        );
        wr!(
            f,
            F1,
            0,
            E at self.elevator,
            HG at self.generators[0], HM at self.microchips[0],
            LG at self.generators[1], LM at self.microchips[1],
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
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
        );
        wr!(
            f,
            F3,
            2,
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
        );
        wr!(
            f,
            F2,
            1,
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
        );
        wr!(
            f,
            F1,
            0,
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
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
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
            FG at self.generators[5], FM at self.microchips[5],
            GG at self.generators[6], GM at self.microchips[6],
        );
        wr!(
            f,
            F3,
            2,
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
            FG at self.generators[5], FM at self.microchips[5],
            GG at self.generators[6], GM at self.microchips[6],
        );
        wr!(
            f,
            F2,
            1,
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
            FG at self.generators[5], FM at self.microchips[5],
            GG at self.generators[6], GM at self.microchips[6],
        );
        wr!(
            f,
            F1,
            0,
            E at self.elevator,
            AG at self.generators[0], AM at self.microchips[0],
            BG at self.generators[1], BM at self.microchips[1],
            CG at self.generators[2], CM at self.microchips[2],
            DG at self.generators[3], DM at self.microchips[3],
            EG at self.generators[4], EM at self.microchips[4],
            FG at self.generators[5], FM at self.microchips[5],
            GG at self.generators[6], GM at self.microchips[6],
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{solve, State};

    const EXAMPLE: State<2> = State {
        generators: [1, 2],
        microchips: [0, 0],
        elevator: 0,
    };

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
