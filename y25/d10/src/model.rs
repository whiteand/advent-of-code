use std::collections::VecDeque;

use derive_more::{Constructor, From};
use good_lp::{Expression, Solution, SolverModel, VariableDefinition, highs};
use itertools::Itertools;

#[derive(Copy, Clone, From, Eq, PartialEq, Constructor)]
pub struct Indicators {
    bitmask: u32,
    size: usize,
}
impl Indicators {
    fn len(self) -> usize {
        self.size
    }
    fn states(self) -> usize {
        1 << self.size
    }
    fn empty(size: usize) -> Self {
        Self { bitmask: 0, size }
    }
}

impl std::fmt::Debug for Indicators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len() {
            let bit = 1 << i;
            if (self.bitmask & bit) != 0 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Copy, Clone, From, PartialEq, Eq)]
pub struct Button(u32);

impl Button {
    fn iter(self) -> impl Iterator<Item = usize> {
        let cnt = self.0.count_ones();
        (0usize..(u32::BITS as usize))
            .filter(move |x| (self.0 & (1 << x)) != 0)
            .take(cnt as usize)
    }
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cnt = self.0.count_ones();
        let mut c = 0;
        let mut bit = 0..u32::BITS;
        write!(f, "(")?;
        loop {
            if let Some(bit_index) = bit.next()
                && c < cnt
            {
                let bit = 1 << bit_index;
                let bit_value = self.0 & bit;
                if bit_value != 0 {
                    if c != 0 {
                        write!(f, ",{}", bit_index)?;
                    } else {
                        write!(f, "{}", bit_index)?;
                    }
                    c += 1;
                }
            } else {
                write!(f, ")")?;
                break Ok(());
            }
        }
    }
}

pub type JoltageVec = Vec<u16>;

#[derive(Clone, Constructor, Eq, PartialEq)]
pub struct Machine {
    target: Indicators,
    buttons: Vec<Button>,
    target_joltage: Vec<u16>,
}

impl Machine {
    pub fn prepare_fewest_button_clicks_to_target() -> FewestButtonClicksToIndicatorsState {
        FewestButtonClicksToIndicatorsState {
            visited: Vec::with_capacity(2048),
            to_visit: VecDeque::with_capacity(2048),
        }
    }

    pub fn get_fewest_button_clicks_to_target(
        &self,
        dp: &mut FewestButtonClicksToIndicatorsState,
    ) -> usize {
        let visited = &mut dp.visited;
        let to_visit = &mut dp.to_visit;
        visited.clear();
        visited.resize(self.target.states(), false);

        to_visit.clear();
        to_visit.push_back((Indicators::empty(self.target.size), 0));

        while let Some((state, steps)) = to_visit.pop_front() {
            let ind = state.bitmask as usize;
            if visited[ind] {
                continue;
            }
            visited[ind] = true;
            if state == self.target {
                return steps;
            }

            for b in &self.buttons {
                let new_state = Indicators::new(b.0 ^ state.bitmask, state.len());
                let new_ind = new_state.bitmask as usize;
                if !visited[new_ind] {
                    to_visit.push_back((new_state, steps + 1));
                }
            }
        }

        usize::MAX
    }

    pub fn get_fewest_button_clicks_to_joltage(&self) -> usize {
        let mut vars = good_lp::ProblemVariables::new();

        let button_clicks_vars: Vec<_> =
            vars.add_all(self.buttons.iter().enumerate().map(|(i, b)| {
                VariableDefinition::default()
                    .name(format!("button[{i}]"))
                    .integer()
                    .min(0)
                    .max(
                        b.iter()
                            .map(|i| self.target_joltage[i])
                            .min()
                            .unwrap_or_default(),
                    )
            }));

        let mut clicks_sum = Expression::with_capacity(button_clicks_vars.len());
        for v in &button_clicks_vars {
            clicks_sum.add_mul(1., v);
        }

        let mut problem = vars.minimise(clicks_sum.clone()).using(highs);

        // Ensuring that all clicks are creating target_joltage at the end
        for place in 0..self.target_joltage.len() {
            let buttons_for_place = std::iter::zip(button_clicks_vars.iter(), &self.buttons)
                .filter(|(_, b)| b.iter().contains(&place))
                .collect_vec();
            let mut expr = Expression::with_capacity(buttons_for_place.len());
            for (v, _) in buttons_for_place {
                expr.add_mul(1., *v);
            }
            problem = problem.with(expr.eq(self.target_joltage[place]));
        }

        let solution = problem.solve().unwrap();

        let clicks = solution.eval(clicks_sum);

        clicks.round() as usize
    }
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.target)?;
        for b in &self.buttons {
            write!(f, " {:?}", b)?;
        }
        write!(f, "{{")?;
        for (i, x) in self.target_joltage.iter().copied().enumerate() {
            if i != 0 {
                write!(f, ",{}", x)?;
            } else {
                write!(f, "{}", x)?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

pub struct FewestButtonClicksToIndicatorsState {
    visited: Vec<bool>,
    to_visit: VecDeque<(Indicators, usize)>,
}
