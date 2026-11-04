use itertools::Itertools;
use md5::{Digest, Md5};
use std::ops::RangeFrom;

#[tracing::instrument]
pub fn solve_part_1<const I: usize>(salt: &str) -> usize {
    solve::<I, Part1HashAlgo>(salt)
}

#[tracing::instrument(skip(salt))]
pub fn solve_part_2<const I: usize>(salt: &str) -> usize {
    solve::<I, Part2HashAlgo>(salt)
}

trait HashAlgo {
    type Out;
    fn digest(salt: &str, i: usize) -> Self::Out;
}

#[derive(Debug)]
struct Keys<'t, H: HashAlgo> {
    next_index: RangeFrom<usize>,
    salt: &'t str,
    remembered: Vec<H::Out>,
    fives: Vec<(u8, usize)>,
    last_five_check: usize,
}

struct Part1HashAlgo;

impl HashAlgo for Part1HashAlgo {
    type Out = [u8; 16];

    fn digest(salt: &str, i: usize) -> Self::Out {
        Md5::digest(format!("{salt}{i}")).into()
    }
}

struct Part2HashAlgo;

impl HashAlgo for Part2HashAlgo {
    type Out = [u8; 16];

    fn digest(salt: &str, i: usize) -> Self::Out {
        let mut buf = hex::encode(Md5::digest(format!("{salt}{i}")));

        for _ in 0..2015 {
            buf = hex::encode(Md5::digest(buf.as_bytes()));
        }
        Md5::digest(buf.as_bytes()).into()
    }
}

impl<'t, H: HashAlgo> Keys<'t, H>
where
    H::Out: AsRef<[u8]>,
{
    fn new(salt: &'t str) -> Self {
        Self {
            next_index: 0..,
            salt,
            remembered: Vec::with_capacity(30000),
            fives: Vec::with_capacity(30000),
            last_five_check: 0,
        }
    }
    fn stream_at(&mut self, ind: usize) -> &H::Out {
        for i in (self.remembered.len())..=ind {
            let digest = H::digest(self.salt, i);

            self.remembered.push(digest);
        }
        self.remembered.get(ind).unwrap()
    }
    fn has_five_of(&mut self, value: u8, start: usize) -> bool {
        let mut buf = Vec::with_capacity(10);
        for i in start..(start + 1000) {
            if self.last_five_check < i {
                let digits = self
                    .stream_at(i)
                    .as_ref()
                    .iter()
                    .flat_map(|x| [x >> 4, x & 0b1111]);
                let mut last = 0;
                let mut cnt = 0;
                for x in digits {
                    if x == last {
                        cnt += 1;
                    } else {
                        if cnt >= 5 {
                            buf.push(last);
                        }
                        last = x;
                        cnt = 1;
                    }
                }
                if cnt >= 5 {
                    buf.push(last);
                }

                for x in buf.drain(0..) {
                    self.fives.push((x, i));
                }
                self.last_five_check = i;
            }
            if self
                .fives
                .iter()
                .rev()
                .take_while(|(_, j)| *j >= i)
                .any(|(x, _)| *x == value)
            {
                return true;
            }
        }
        false
    }
}

impl<H: HashAlgo> Iterator for Keys<'_, H>
where
    H::Out: AsRef<[u8]>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let i = self.next_index.next()?;
            let dig: &H::Out = self.stream_at(i);

            let Some(tripple) = dig
                .as_ref()
                .iter()
                .flat_map(|&x| [x >> 4, x & 0b1111])
                .tuple_windows()
                .find_map(|(a, b, c)| (a == b && b == c).then_some(a))
            else {
                continue;
            };

            if self.has_five_of(tripple, i + 1) {
                return Some(i);
            }
        }
    }
}

#[tracing::instrument(skip(salt))]
fn solve<const I: usize, H>(salt: &str) -> usize
where
    H: HashAlgo,
    H::Out: AsRef<[u8]>,
{
    Keys::<H>::new(salt.trim()).nth(I).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1::<0>(EXAMPLE)), "39");
        assert_eq!(format!("{}", solve_part_1::<63>(EXAMPLE)), "22728");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1::<63>(ACTUAL)), "23890");
    }

    #[test]
    #[ignore] // runs 15s in realease
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2::<63>(EXAMPLE)), "22551");
    }

    #[test]
    #[ignore] // runs 15s in realease
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2::<63>(ACTUAL)), "22696");
    }
}
