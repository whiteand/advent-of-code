use std::collections::{BTreeMap, HashMap};

use advent_utils::{glam::IVec3, math::imat3::IMat3};
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScannerId(i32);

impl std::fmt::Debug for ScannerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub struct ScannerMeasurements {
    pub id: ScannerId,
    pub measurements: Vec<IVec3>,
}

impl ScannerMeasurements {
    pub fn new(id: i32, measurements: Vec<IVec3>) -> Self {
        Self {
            id: ScannerId(id),
            measurements,
        }
    }

    pub fn distances(&self) -> ScannerDistances {
        let mut distances_squared: Vec<_> = self
            .measurements
            .iter()
            .copied()
            .enumerate()
            .array_combinations::<2>()
            .map(|[(i, a), (j, b)]| (i, j, a.distance_squared(b)))
            .collect();

        distances_squared.sort_by_key(|x| x.2);

        ScannerDistances {
            id: self.id,
            distances_squared: distances_squared,
        }
    }
}

impl std::fmt::Debug for ScannerMeasurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]:", self.id.0)?;
        for m in &self.measurements {
            write!(f, " <{},{},{}>", m.x, m.y, m.z)?;
        }
        Ok(())
    }
}

pub struct ScannerDistances {
    pub id: ScannerId,
    distances_squared: Vec<(usize, usize, i32)>,
}

#[derive(Clone, Debug)]
pub struct MatchingDistances {
    pub scanner_ids: [ScannerId; 2],
    pub matches: [Vec<[usize; 2]>; 2],
}

#[derive(Copy, Clone)]
pub struct Alias {
    pub indexes: [usize; 2],
    pub count: usize,
}

impl std::fmt::Debug for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}->{}(n=${})",
            self.indexes[0], self.indexes[1], self.count
        )
    }
}
pub struct Aliases {
    pub scanner_ids: [ScannerId; 2],
    pub aliases: Vec<Alias>,
}

impl std::fmt::Debug for Aliases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}->{:?}] {:?}",
            self.scanner_ids[0], self.scanner_ids[1], self.aliases
        )
    }
}

impl MatchingDistances {
    pub fn matches(&self) -> impl Iterator<Item = [[usize; 2]; 2]> + '_ {
        self.matches[0]
            .iter()
            .copied()
            .zip(self.matches[1].iter().copied())
            .map(|(a, b)| [a, b])
    }

    pub fn get_aliases(&self) -> Aliases {
        let mut possible_aliases_map = HashMap::<usize, BTreeMap<usize, usize>>::new();
        for [a, b] in
            self.matches()
                .flat_map(|[[left_start, left_end], [right_start, right_end]]| {
                    [
                        [left_start, right_end],
                        [left_start, right_start],
                        [left_end, right_end],
                        [left_end, right_start],
                    ]
                })
        {
            *possible_aliases_map
                .entry(a)
                .or_default()
                .entry(b)
                .or_default() += 1;
        }

        let mut aliases = Vec::new();

        while !possible_aliases_map.is_empty() {
            let most_sure_alias = possible_aliases_map
                .keys()
                .copied()
                .flat_map(|left| {
                    possible_aliases_map
                        .get(&left)
                        .unwrap()
                        .iter()
                        .max_by_key(|e| e.1)
                        .map(|(right, n)| Alias {
                            indexes: [left, *right],
                            count: *n,
                        })
                })
                .max_by_key(|x| x.count)
                .unwrap();

            aliases.push(most_sure_alias);

            possible_aliases_map.remove(&most_sure_alias.indexes[0]);

            let mut entries_to_remove = Vec::new();

            for x in possible_aliases_map.iter_mut() {
                x.1.remove(&most_sure_alias.indexes[1]);
                if x.1.is_empty() {
                    entries_to_remove.push(*x.0);
                }
            }
            for e in entries_to_remove.into_iter() {
                possible_aliases_map.remove(&e);
            }
        }

        Aliases {
            aliases,
            scanner_ids: self.scanner_ids,
        }
    }
}

impl ScannerDistances {
    pub fn get_same_distances(&self, another: &Self) -> MatchingDistances {
        let mut left_it = self.distances_squared.iter().copied();
        let mut right_it = another.distances_squared.iter().copied();
        let mut matching_distances = MatchingDistances {
            matches: [Vec::new(), Vec::new()],
            scanner_ids: [self.id, another.id],
        };

        let Some(mut last_left) = left_it.next() else {
            return matching_distances;
        };
        let Some(mut last_right) = right_it.next() else {
            return matching_distances;
        };

        loop {
            if last_left.2 == last_right.2 {
                matching_distances.matches[0].push([last_left.0, last_left.1]);
                matching_distances.matches[1].push([last_right.0, last_right.1]);

                let Some(new_last_left) = left_it.next() else {
                    return matching_distances;
                };
                last_left = new_last_left;
                let Some(new_last_right) = right_it.next() else {
                    return matching_distances;
                };
                last_right = new_last_right;
                continue;
            }
            if last_left.2 > last_right.2 {
                let Some(new_last_right) = right_it.next() else {
                    return matching_distances;
                };
                last_right = new_last_right;
                continue;
            } else {
                let Some(new_last_left) = left_it.next() else {
                    return matching_distances;
                };
                last_left.2 = new_last_left.2;
            }
        }
    }
}

pub struct Orientation {
    origin: IVec3,
    rotation: IMat3,
}

impl Orientation {
    pub fn new(origin: IVec3, rotation: IMat3) -> Self {
        Self { origin, rotation }
    }

    fn real_to_view(&self, real: IVec3) -> IVec3 {
        self.rotation.mul_vec3(real - self.origin)
    }

    fn view_to_real(&self, view: IVec3) -> IVec3 {
        self.rotation.inverse().unwrap().mul_vec3(view) + self.origin
    }

    pub fn identity() -> Self {
        Self {
            origin: IVec3::ZERO,
            rotation: IMat3::identity(),
        }
    }
}

pub struct ScannerPositions {
    entries: Vec<(ScannerId, Orientation)>,
}

impl ScannerPositions {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn set_position(&mut self, scanner_id: ScannerId, orientation: Orientation) {
        if let Some(x) = self.get_position_mut(scanner_id) {
            *x = orientation;
        } else {
            self.entries.push((scanner_id, orientation));
        }
    }

    pub fn get_position_mut(&mut self, scanner_id: ScannerId) -> Option<&mut Orientation> {
        self.entries
            .iter_mut()
            .find(|x| x.0 == scanner_id)
            .map(|r| &mut r.1)
    }

    pub fn get_position(&self, scanner_id: ScannerId) -> Option<&Orientation> {
        self.entries
            .iter()
            .find(|x| x.0 == scanner_id)
            .map(|r| &r.1)
    }
}

#[cfg(test)]
mod tests {

    use advent_utils::{glam::IVec3, math::imat3::IMat3};

    use crate::model::Orientation;

    #[test]
    fn test_orientation() {
        let orig = [
            IVec3::new(-618, -824, -621),
            IVec3::new(-537, -823, -458),
            IVec3::new(-447, -329, 318),
            IVec3::new(404, -588, -901),
            IVec3::new(544, -627, -890),
            IVec3::new(528, -643, 409),
            IVec3::new(-661, -816, -575),
            IVec3::new(390, -675, -793),
            IVec3::new(423, -701, 434),
            IVec3::new(-345, -311, 381),
            IVec3::new(459, -707, 401),
            IVec3::new(-485, -357, 347),
        ];

        let scanner2_orientation = Orientation::new(
            IVec3::new(68, -1246, -43),
            IMat3::from_axis(IVec3::NEG_X, IVec3::Y, IVec3::NEG_Z),
        );

        let transformed = orig.map(|v| scanner2_orientation.real_to_view(v));

        let expected = [
            IVec3::new(686, 422, 578),
            IVec3::new(605, 423, 415),
            IVec3::new(515, 917, -361),
            IVec3::new(-336, 658, 858),
            IVec3::new(-476, 619, 847),
            IVec3::new(-460, 603, -452),
            IVec3::new(729, 430, 532),
            IVec3::new(-322, 571, 750),
            IVec3::new(-355, 545, -477),
            IVec3::new(413, 935, -424),
            IVec3::new(-391, 539, -444),
            IVec3::new(553, 889, -390),
        ];

        assert_eq!(transformed, expected,);

        let transformed_back = transformed.map(|v| scanner2_orientation.view_to_real(v));

        assert_eq!(orig, transformed_back)
    }
}
