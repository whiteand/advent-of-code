use itertools::Itertools;
use std::fmt::Write;
use std::io::Write as IOWrite;
use std::ops::{Deref, RangeFrom};
use tracing::info;

#[tracing::instrument]
pub fn solve_part_1<const I: usize>(salt: &str) -> usize {
    Keys::<Part1HashAlgo>::new(salt.trim()).nth(I).unwrap()
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
}

struct Part1HashAlgo;

impl HashAlgo for Part1HashAlgo {
    type Out = md5::Digest;

    fn digest(salt: &str, i: usize) -> Self::Out {
        let mut ctx = md5::Context::new();
        write!(ctx, "{}{}", salt, i).unwrap();
        ctx.compute()
    }
}

struct Part2HashAlgo;

impl HashAlgo for Part2HashAlgo {
    type Out = md5::Digest;

    fn digest(salt: &str, i: usize) -> Self::Out {
        let mut buf = String::with_capacity(32);
        let mut ctx = md5::Context::new();
        write!(ctx, "{}{}", salt, i).unwrap();
        let dig = ctx.compute();

        write!(buf, "{:x}", dig).unwrap();

        for _ in 0..2015 {
            let mut ctx = md5::Context::new();
            ctx.write(buf.as_bytes());
            let dig = ctx.compute();
            buf.clear();
            write!(buf, "{:x}", dig);
        }
        let mut ctx = md5::Context::new();
        ctx.write(buf.as_bytes());
        let dig = ctx.compute();
        dig
    }
}

impl<'t, H: HashAlgo> Keys<'t, H>
where
    H::Out: core::ops::Deref<Target = [u8; 16]>,
{
    fn new(salt: &'t str) -> Self {
        Self {
            next_index: 0..,
            salt,
            remembered: Vec::with_capacity(15000),
        }
    }
    fn stream_at(&mut self, ind: usize) -> &H::Out {
        for i in (self.remembered.len())..=ind {
            let digest = H::digest(self.salt, i);

            info!(?ind, hex = hex::encode(digest.deref()));
            self.remembered.push(digest);
        }
        self.remembered.get(ind).unwrap()
    }
}

impl<'t, H: HashAlgo> Iterator for Keys<'t, H>
where
    H::Out: core::ops::Deref<Target = [u8; 16]>,
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

            for j in (i + 1)..(i + 1001) {
                let dig = self.stream_at(j);
                if dig
                    .as_ref()
                    .iter()
                    .flat_map(|&x| [x >> 4, x & 0b1111])
                    .tuple_windows()
                    .any(|(a, b, c, d, e)| (tripple == a && a == b && b == c && c == d && d == e))
                {
                    return Some(i);
                }
            }
        }
    }
}

#[tracing::instrument(skip(salt))]
pub fn solve_part_2<const I: usize>(salt: &str) -> usize {
    Keys::<Part2HashAlgo>::new(salt.trim()).nth(I).unwrap()
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
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2::<64>(EXAMPLE)), "22551");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2::<63>(ACTUAL)), "0");
    }
}
