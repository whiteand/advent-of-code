#[tracing::instrument(skip(input))]
pub fn solve_part_1<const N: usize>(mut input: State<N>) -> usize
where
    State<N>: std::fmt::Display,
{
    println!("{input}");
    input = input.goto(1, &[], &[0]).unwrap();
    println!("{input}");
    input = input.goto(2, &[0], &[0]).unwrap();
    println!("{input}");
    input = input.goto(1, &[], &[0]).unwrap();
    println!("{input}");
    input = input.goto(0, &[], &[0]).unwrap();
    println!("{input}");
    input = input.goto(1, &[], &[0, 1]).unwrap();
    println!("{input}");
    input = input.goto(2, &[], &[0, 1]).unwrap();
    println!("{input}");
    input = input.goto(3, &[], &[0, 1]).unwrap();
    println!("{input}");
    println!("{input}");
    input = input.goto(2, &[], &[0]).unwrap();
    println!("{input}");
    input = input.goto(3, &[0, 1], &[]).unwrap();
    println!("{input}");
    input = input.goto(2, &[], &[1]).unwrap();
    println!("{input}");
    input = input.goto(3, &[], &[0, 1]).unwrap();
    println!("{input}");
    assert!(input.all_on_level(3));
    todo!("part 1 is not implemented yet")
}
#[tracing::instrument(skip(input))]
pub fn solve_part_2<const N: usize>(input: State<N>) -> usize
where
    State<N>: std::fmt::Display,
{
    println!("{input}");
    todo!("part 2 is not implemented yet")
}

#[derive(Debug, Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2, State};

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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(super::ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(super::ACTUAL)), "0");
    }
}
