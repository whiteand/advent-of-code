#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct DisjointSet {
    pub member: usize,
    pub size: usize,
}

impl DisjointSet {
    fn new(member: usize, size: usize) -> Self {
        Self { member, size }
    }
    fn union(&self, other: DisjointSet) -> DisjointSet {
        if other.member == self.member {
            return *self;
        }
        DisjointSet {
            member: self.member.min(other.member),
            size: self.size + other.size,
        }
    }
}

pub struct DisjointSets {
    sets: Vec<DisjointSet>,
}

impl DisjointSets {
    pub fn new(size: usize) -> Self {
        Self {
            sets: (0..size).map(|i| DisjointSet::new(i, 1)).collect(),
        }
    }
    pub fn join(&mut self, from: usize, to: usize) -> DisjointSet {
        let from_set = self.get_set_of(from);
        let to_set = self.get_set_of(to);
        let sets = &mut self.sets;

        let new_set = from_set.union(to_set);
        if from_set.member != to_set.member {
            for i in [to_set.member, from_set.member, from, to] {
                sets[i] = new_set;
            }
        }
        new_set
    }
    pub fn get_set_of_shortening_path(&mut self, point: usize) -> DisjointSet {
        let exampler = &self.sets;
        let mut current = point;
        while exampler[current].member != current {
            current = exampler[current].member;
        }

        let set = exampler[current];
        self.sets[point] = set;
        set
    }
    pub fn get_set_of(&mut self, mut point: usize) -> DisjointSet {
        let sets = &self.sets;
        while sets[point].member != point {
            point = sets[point].member;
        }
        sets[point]
    }

    /**
     * Iterates over disjoint sets
     */
    pub fn iter(&self) -> impl Iterator<Item = DisjointSet> + '_ {
        self.sets
            .iter()
            .enumerate()
            .filter_map(|(i, s)| (i == s.member).then_some(*s))
    }

    /**
     * Iterates over the sets sizes
     */
    pub fn sizes(&mut self) -> impl Iterator<Item = usize> + '_ {
        self.iter().map(|x| x.size)
    }
}
