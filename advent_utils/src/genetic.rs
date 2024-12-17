use std::marker::PhantomData;

use derive_more::From;
use rand::prelude::Distribution;

/// Each bps is 0.01%. Total BPS is 10000
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, From)]
pub struct Bps(#[from] u16);

pub trait BpsAble {
    fn calculate_bps(&self, bps: Bps) -> Self;
}

macro_rules! impl_uint {
    ($($ty:ty as $ty2:ty),+) => {
        $(
            impl BpsAble for $ty {
                fn calculate_bps(&self, bps: Bps) -> Self {
                    ((*self as $ty2) * (bps.0 as $ty2) / 10_000) as Self
                }
            }
        )+
    };
}

impl_uint!(
    u8 as usize,
    u16 as usize,
    u32 as usize,
    usize as usize,
    u64 as u64,
    u128 as u128
);

impl Bps {
    pub fn of<T: BpsAble>(&self, value: &T) -> T {
        BpsAble::calculate_bps(value, *self)
    }
}

pub struct Genetic<G, E, GE, B, BI, R, M>
where
    B: Fn(&G, &G, &mut R) -> BI,
    R: rand::Rng,
    BI: Iterator<Item = G>,
{
    population: Vec<(usize, G, E)>,
    generation: usize,
    /// Max value 10000 = 100%
    mutation_rate_bps: Bps,
    /// Bps of the organisms that should pass to next generation
    preserve_best_bps: Bps,
    children: usize,
    target_population_size: usize,
    /// (generation, gene, error)
    best_gene: Option<(usize, G, E)>,
    get_error: GE,
    sex: B,
    rng: PhantomData<R>,
    mutate: M,
}

impl<G, E, GE, B, BI, R, M> Genetic<G, E, GE, B, BI, R, M>
where
    B: Fn(&G, &G, &mut R) -> BI,
    BI: Iterator<Item = G>,
    GE: FnMut(&G) -> E,
    E: Ord + Copy + Default,
    R: rand::Rng,
    G: PartialEq + Eq,
    M: FnMut(&mut G, &mut R),
{
    pub fn new(genes: Vec<G>, mut get_error: GE, sex: B, mutate: M) -> Self
    where
        E: Ord + Copy,
        G: Copy,
        GE: FnMut(&G) -> E,
    {
        let mut population = Vec::with_capacity(genes.len());
        for g in genes {
            let error = get_error(&g);
            population.push((0, g, error))
        }
        population.sort_unstable_by_key(|x| x.2);
        let best_gene = population.iter().max_by_key(|x| x.2).copied();
        Self {
            target_population_size: population.len(),
            population,
            generation: 0,
            mutation_rate_bps: 200.into(),
            preserve_best_bps: 5000.into(),
            best_gene: best_gene,
            children: 2,
            get_error,
            sex,
            rng: PhantomData,
            mutate,
        }
    }

    pub fn best(&self) -> Option<(usize, &G, E)> {
        self.best_gene.as_ref().map(|(ind, g, e)| (*ind, g, *e))
    }
    pub fn set_mutation_bps(&mut self, mutation_rate_bps: impl Into<Bps>) -> &mut Self {
        self.mutation_rate_bps = mutation_rate_bps.into();
        self
    }
    pub fn set_preserve_bps(&mut self, preserve_best_bps: impl Into<Bps>) -> &mut Self {
        self.preserve_best_bps = preserve_best_bps.into();
        self
    }

    pub fn population(&self) -> &[(usize, G, E)] {
        self.population.as_slice()
    }
    /// Calculates score
    /// Selects top N
    /// Interbreeds top
    ///
    /// Returns
    pub fn select<'x>(&'x mut self, rng: &mut R) -> Option<(usize, &'x G, E)>
    where
        G: Clone,
    {
        let preserved = self.preserve_best_bps.of(&self.population.len());
        self.population.truncate(preserved);

        while self.population.len() < self.target_population_size {
            let Ok(distribution) = rand::distributions::Slice::new(&self.population[0..preserved])
            else {
                break;
            };
            let father = distribution.sample(rng);
            let mother = distribution.sample(rng);

            for mut child in (self.sex)(&father.1, &mother.1, rng).take(self.children) {
                let bps: Bps = rng.gen_range(0..=10000u16).into();
                if bps < self.mutation_rate_bps {
                    (self.mutate)(&mut child, rng);
                }
                let error = (self.get_error)(&child);
                self.population.push((self.generation + 1, child, error));
                if self.population.len() >= self.target_population_size {
                    break;
                }
            }
        }

        self.population.sort_unstable_by_key(|x| x.2);

        self.best_gene = match (self.best_gene.take(), self.population.first()) {
            (None, None) => None,
            (None, Some((i, g, e))) => Some((*i, g.clone(), *e)),
            (Some(b), Some(f)) if f.2.lt(&b.2) => Some((f.0, f.1.clone(), f.2)),
            (Some(b), _) => Some(b),
        };

        let res = self.best_gene.as_ref().map(|(ind, g, s)| (*ind, g, *s));
        self.generation += 1;
        res
    }
}
