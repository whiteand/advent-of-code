use advent_utils::nom::{self, Parser};
use itertools::Itertools;

type RegularType = i32;

#[derive(Debug, Copy, Clone)]
pub struct PairIndex(usize);

#[derive(Clone, Copy)]
struct Pair([PairValue; 2]);

impl Pair {
    fn left(self) -> PairValue {
        self.0[0]
    }

    fn right(self) -> PairValue {
        self.0[1]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PairValue {
    Regular(RegularType),
    Pair(PairIndex),
}

impl PairValue {
    fn as_regular(self) -> Option<RegularType> {
        if let PairValue::Regular(x) = self {
            return Some(x);
        }
        None
    }
    fn as_pair(self) -> Option<PairIndex> {
        if let PairValue::Pair(x) = self {
            return Some(x);
        }
        None
    }
}

#[derive(Default)]
pub struct Pairs {
    pairs: Vec<Pair>,
}

#[derive(Debug)]
struct ExplosionEffect {
    left: Option<RegularType>,
    right: Option<RegularType>,
    replace_with: PairValue,
}

#[derive(Debug)]
struct SplitEffect {
    replace_with: PairIndex,
}

impl Pairs {
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    pub fn get<'t>(&'t self, ind: PairIndex) -> PairRef<'t> {
        assert!(self.pairs.len() >= ind.0);
        PairRef {
            numbers: self,
            index: ind,
        }
    }

    fn get_pair(&self, ind: PairIndex) -> Pair {
        self.pairs[ind.0]
    }

    pub fn alloc_pair(&mut self, left: PairValue, right: PairValue) -> PairIndex {
        let ind = PairIndex(self.pairs.len());
        self.pairs.push(Pair([left, right]));

        ind
    }

    fn explode_left_most_nested(
        &mut self,
        pair_index: PairIndex,
        allowed_nested_levels: usize,
    ) -> Option<ExplosionEffect> {
        let pair = self.get_pair(pair_index);

        if allowed_nested_levels == 0 {
            // println!("Exploding {}", self.get(root));
            let [left_regular, right_regular] = pair.0.map(|x| x.as_regular().unwrap());
            return Some(ExplosionEffect {
                left: Some(left_regular),
                right: Some(right_regular),
                replace_with: PairValue::Regular(0),
            });
        }
        if let PairValue::Pair(left) = pair.left()
            && let Some(left_explosion) =
                self.explode_left_most_nested(left, allowed_nested_levels - 1)
        {
            // println!("Left explosition happened {:?}", left_explosion);
            let add_to_right_part = left_explosion.right.unwrap_or_default();
            let new_right = self.add_left_most_regular(pair.right(), add_to_right_part);

            let new_root = self.alloc_pair(left_explosion.replace_with, new_right);

            // println!("New value: {}", self.get(new_root));

            return Some(ExplosionEffect {
                left: left_explosion.left,
                right: None,
                replace_with: PairValue::Pair(new_root),
            });
        }

        if let PairValue::Pair(right) = pair.right()
            && let Some(right_explosion) =
                self.explode_left_most_nested(right, allowed_nested_levels - 1)
        {
            // println!("Left explosition happened {:?}", right_explosion);
            let add_to_left_part = right_explosion.left.unwrap_or_default();

            let new_left = self.add_right_most_regular(pair.left(), add_to_left_part);

            let new_root = self.alloc_pair(new_left, right_explosion.replace_with);

            // println!("New value: {}", self.get(new_root));

            return Some(ExplosionEffect {
                left: None,
                right: right_explosion.right,
                replace_with: PairValue::Pair(new_root),
            });
        }

        return None;
    }

    fn split_left_most(&mut self, root: PairIndex) -> Option<SplitEffect> {
        let pair = self.get_pair(root);

        match pair.left() {
            PairValue::Regular(left_value) if left_value >= 10 => {
                let new_left_value = left_value / 2;
                let new_left = self.alloc_pair(
                    PairValue::Regular(new_left_value),
                    PairValue::Regular(left_value - new_left_value),
                );
                return Some(SplitEffect {
                    replace_with: self.alloc_pair(PairValue::Pair(new_left), pair.right()),
                });
            }
            PairValue::Pair(p) if let Some(split) = self.split_left_most(p) => {
                let new_left = PairValue::Pair(split.replace_with);
                let new_pair = self.alloc_pair(new_left, pair.right());
                return Some(SplitEffect {
                    replace_with: new_pair,
                });
            }
            _ => {}
        }

        match pair.right() {
            PairValue::Regular(right_value) if right_value >= 10 => {
                let new_right_value = right_value / 2;
                let new_right = self.alloc_pair(
                    PairValue::Regular(new_right_value),
                    PairValue::Regular(right_value - new_right_value),
                );
                return Some(SplitEffect {
                    replace_with: self.alloc_pair(pair.left(), PairValue::Pair(new_right)),
                });
            }
            PairValue::Pair(p) if let Some(split) = self.split_left_most(p) => {
                let new_right = PairValue::Pair(split.replace_with);
                let new_pair = self.alloc_pair(pair.left(), new_right);
                return Some(SplitEffect {
                    replace_with: new_pair,
                });
            }
            _ => {}
        }
        None
    }

    pub fn reductions(&mut self, root: PairIndex) -> impl Iterator<Item = PairIndex> + '_ {
        let mut res = Some(root);
        return std::iter::from_fn(move || {
            let taken = res.take()?;
            // println!("Reduction: {}", self.get(taken));
            let response = Some(taken);

            if let Some(explosion) = self.explode_left_most_nested(taken, 4) {
                _ = res.insert(explosion.replace_with.as_pair().unwrap());
            } else if let Some(split) = self.split_left_most(taken) {
                _ = res.insert(split.replace_with);
            }

            return response;
        });
    }
    pub fn reduce(&mut self, root: PairIndex) -> PairIndex {
        self.reductions(root).last().unwrap()
    }

    pub fn add(&mut self, a: PairIndex, b: PairIndex) -> PairIndex {
        let pair = self.alloc_pair(PairValue::Pair(a), PairValue::Pair(b));

        self.reduce(pair)
    }

    fn add_left_most_regular(&mut self, root: PairValue, increment: i32) -> PairValue {
        if increment == 0 {
            return root;
        }
        match root {
            PairValue::Regular(r) => PairValue::Regular(r + increment),
            PairValue::Pair(pair_index) => {
                let pair = self.get_pair(pair_index);
                let new_left = self.add_left_most_regular(pair.left(), increment);

                let new_pair_index = self.alloc_pair(new_left, pair.right());
                return PairValue::Pair(new_pair_index);
            }
        }
    }
    fn add_right_most_regular(&mut self, root: PairValue, increment: i32) -> PairValue {
        if increment == 0 {
            return root;
        }
        match root {
            PairValue::Regular(r) => PairValue::Regular(r + increment),
            PairValue::Pair(pair_index) => {
                let pair = self.get_pair(pair_index);
                let new_right = self.add_right_most_regular(pair.right(), increment);

                let new_pair_index = self.alloc_pair(pair.left(), new_right);
                return PairValue::Pair(new_pair_index);
            }
        }
    }

    pub fn parse<'t, 'i>(&mut self, input: &'i str) -> nom::IResult<&'i str, PairValue> {
        if let Some(remaining) = input.strip_prefix(' ') {
            return self.parse(remaining);
        }
        if let Some(remaining) = input.trim().strip_prefix("[") {
            let (input, left) = self.parse(remaining)?;
            let (input, _) = nom::sequence::terminated(
                nom::bytes::tag(","),
                nom::character::complete::multispace0,
            )
            .parse(input)?;
            let (input, right) = self.parse(input)?;
            let (input, _) = nom::sequence::preceded(
                nom::character::complete::multispace0,
                nom::bytes::tag("]"),
            )
            .parse(input)?;
            let result = self.alloc_pair(left, right);
            return Ok((input.trim_start(), PairValue::Pair(result)));
        }
        let (input, v) = nom::character::complete::i32(input)?;
        let result = PairValue::Regular(v);
        Ok((input.trim_start(), result))
    }

    pub fn parse_number<'i>(&mut self, input: &'i str) -> Option<PairIndex> {
        let (_, v) = self.parse(input).ok()?;
        if let PairValue::Pair(v) = v {
            return Some(v);
        }
        None
    }
}

#[macro_export]
macro_rules! snailfish_number_value {
    ($pairs:expr, $lit:literal) => {
        PairValue::Regular($lit)
    };
    ($pairs:expr, [ $left:tt, $right:tt ]) => {{
        let left_index = snailfish_number_value!($pairs, $left);
        let right_index = snailfish_number_value!($pairs, $right);
        PairValue::Pair($pairs.alloc_pair(left_index, right_index))
    }};
}
#[macro_export]
macro_rules! snailfish_number {
    ($pairs:expr, [ $left:tt, $right:tt ]) => {{
        let left_index = snailfish_number_value!($pairs, $left);
        let right_index = snailfish_number_value!($pairs, $right);
        $pairs.alloc_pair(left_index, right_index)
    }};
}

#[derive(Copy, Clone)]
pub struct PairRef<'t> {
    numbers: &'t Pairs,
    index: PairIndex,
}

impl<'t> PairRef<'t> {
    fn left(self) -> PairValueRef<'t> {
        PairValueRef {
            numbers: self.numbers,
            pair_index: self.index,
            value_index: 0,
        }
    }
    fn right(self) -> PairValueRef<'t> {
        PairValueRef {
            numbers: self.numbers,
            pair_index: self.index,
            value_index: 1,
        }
    }

    fn values(self) -> [PairValueRef<'t>; 2] {
        [self.left(), self.right()]
    }

    pub fn magnitude(self) -> RegularType {
        self.values()
            .into_iter()
            .enumerate()
            .map(|(i, v)| (3 - i) as RegularType * v.magnitude())
            .sum()
    }
}

#[derive(Copy, Clone)]
pub struct PairValueRef<'t> {
    numbers: &'t Pairs,
    pair_index: PairIndex,
    value_index: usize,
}

impl PairValueRef<'_> {
    fn magnitude(self) -> RegularType {
        let pair = self.numbers.get_pair(self.pair_index);
        let value = pair.0[self.value_index];
        match value {
            PairValue::Regular(r) => r,
            PairValue::Pair(pair_index) => self.numbers.get(pair_index).magnitude(),
        }
    }
}

impl<'t> PartialEq for PairRef<'t> {
    fn eq(&self, other: &Self) -> bool {
        self.values()
            .into_iter()
            .zip_eq(other.values())
            .all(|(a, b)| a.eq(&b))
    }
}

impl<'t> PartialEq for PairValueRef<'t> {
    fn eq(&self, other: &Self) -> bool {
        let self_pair = self.numbers.get_pair(self.pair_index);
        let self_value = self_pair.0[self.value_index];
        let other_pair = other.numbers.get_pair(other.pair_index);
        let other_value = other_pair.0[other.value_index];
        match (self_value, other_value) {
            (PairValue::Regular(a), PairValue::Regular(b)) => a == b,
            (PairValue::Pair(pair_index), PairValue::Pair(pair_index2)) => self
                .numbers
                .get(pair_index)
                .eq(&other.numbers.get(pair_index2)),
            _ => false,
        }
    }
}

impl<'t> std::fmt::Display for PairRef<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [left, right] = self.values();
        write!(f, "[{}, {}]", left, right)
    }
}
impl<'t> std::fmt::Display for PairValueRef<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pair = self.numbers.get_pair(self.pair_index);
        let value = pair.0[self.value_index];
        match value {
            PairValue::Regular(r) => write!(f, "{r}"),
            PairValue::Pair(pair_index) => write!(f, "{}", self.numbers.get(pair_index)),
        }
    }
}

impl<'t> std::fmt::Debug for PairRef<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_format() {
        let mut numbers = Pairs::default();
        let b = snailfish_number!(numbers, [1, 2]);
        let c = snailfish_number!(numbers, [[1, 2], [3, 4]]);

        assert_eq!("[1, 2]", format!("{}", numbers.get(b)).as_str());
        assert_eq!("[[1, 2], [3, 4]]", format!("{}", numbers.get(c)).as_str());
    }

    #[test]
    fn test_add() {
        let mut numbers = Pairs::default();
        let a = snailfish_number!(numbers, [1, 1]);
        let b = snailfish_number!(numbers, [2, 2]);
        let c = snailfish_number!(numbers, [3, 3]);
        let d = snailfish_number!(numbers, [4, 4]);
        let ab = numbers.add(a, b);
        let abc = numbers.add(ab, c);
        let abcd = numbers.add(abc, d);

        let expected = snailfish_number!(numbers, [[[[1, 1], [2, 2]], [3, 3]], [4, 4]]);

        assert_eq!(numbers.get(abcd), numbers.get(expected),)
    }

    #[rstest]
    #[case(
        r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"#,
        "[[[[5,0],[7,4]],[5,5]],[6,6]]"
    )]
    #[case(
        r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"#,
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    )]
    fn test_sums(#[case] lines: &str, #[case] expected: &str) {
        let mut numbers = Pairs::default();

        let nums = lines
            .trim()
            .lines()
            .map(|line| numbers.parse_number(line).unwrap())
            .collect_vec();

        let sum = nums.into_iter().reduce(|a, b| numbers.add(a, b)).unwrap();

        let expected_sum = numbers.parse_number(expected).unwrap();

        assert_eq!(numbers.get(sum), numbers.get(expected_sum))
    }

    #[test]
    fn test_add_2() {
        let mut numbers = Pairs::default();
        let a = snailfish_number!(numbers, [1, 1]);
        let b = snailfish_number!(numbers, [2, 2]);
        let c = snailfish_number!(numbers, [3, 3]);
        let d = snailfish_number!(numbers, [4, 4]);
        let e = snailfish_number!(numbers, [5, 5]);
        let ab = numbers.add(a, b);
        let abc = numbers.add(ab, c);
        let abcd = numbers.add(abc, d);
        let abcde = numbers.add(abcd, e);

        let expected = snailfish_number!(numbers, [[[[3, 0], [5, 3]], [4, 4]], [5, 5]]);

        assert_eq!(numbers.get(abcde), numbers.get(expected))
    }

    #[rstest]
    #[case("[[[[[9, 8], 1], 2], 3], 4]", "[[[[0, 9], 2], 3], 4]")]
    #[case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
    #[case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
    #[case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
    )]
    #[case(
        "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    )]
    fn test_reduce(#[case] input: &str, #[case] expected_output: &str) {
        let mut numbers = Pairs::default();
        let input = numbers.parse_number(input).unwrap();
        let actual = numbers.reduce(input);
        let expected = numbers.parse_number(expected_output).unwrap();

        assert_eq!(numbers.get(actual), numbers.get(expected))
    }

    #[test]
    fn test_parse() {
        let mut numbers = Pairs::default();
        let (remaining, actual) = numbers.parse("[  1,   2   ], 1").unwrap();
        let expected = snailfish_number!(numbers, [1, 2]);
        assert_eq!(remaining, ", 1");
        assert_eq!(
            numbers.get(actual.as_pair().unwrap()),
            numbers.get(expected)
        )
    }

    #[rstest]
    #[case("[[1,2],[[3,4],5]]", 143)]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    fn test_magnitude(#[case] input_str: &str, #[case] expected: RegularType) {
        let mut pairs = Pairs::default();

        let pair = pairs.parse_number(input_str).unwrap();

        assert_eq!(pairs.get(pair).magnitude(), expected)
    }
}
