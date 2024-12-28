use core::str;
use std::marker::PhantomData;

use itertools::Itertools;
use nom::{AsBytes, AsChar};

use crate::grid::Grid;

/// Trims a string and returns a grid built over it.
pub fn ascii_grid(file_content: &str) -> Grid<u8> {
    file_content
        .trim()
        .lines()
        .map(|line| line.as_bytes().iter().copied())
        .collect()
}

pub fn digits<'t>(line: &'t str) -> impl Iterator<Item = u8> + 't {
    line.as_bytes()
        .iter()
        .copied()
        .filter(|x| x.is_ascii_digit())
        .map(|x| x - b'0')
}

pub fn nums<T>(line: &str) -> ParseNumsIter<'_, T> {
    ParseNumsIter {
        bytes: line.as_bytes(),
        phantom: PhantomData,
    }
}
pub struct ParseNumsIter<'t, T> {
    bytes: &'t [u8],
    phantom: PhantomData<T>,
}

impl<'t, T> ParseNumsIter<'t, T> {
    pub fn rest_bytes(&self) -> &'t [u8] {
        self.bytes
    }
    pub fn strip_prefix(&mut self, prefix: impl AsBytes) -> Result<&mut Self, &mut Self> {
        let bytes = prefix.as_bytes();
        match self.bytes.strip_prefix(bytes) {
            Some(rest) => {
                self.bytes = rest;
                Ok(self)
            }
            None => Err(self),
        }
    }

    pub fn rest_str(&self) -> &'t str {
        str::from_utf8(self.rest_bytes()).unwrap()
    }
    pub fn with_type<U>(self) -> ParseNumsIter<'t, U> {
        ParseNumsIter {
            bytes: self.bytes,
            phantom: PhantomData,
        }
    }
}

macro_rules! uint_nums {
    ($($typ:ty),+) => {
        $(
            impl<'t> Iterator for ParseNumsIter<'t, $typ> {
                type Item = $typ;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.bytes.is_empty() {
                        return None;
                    }
                    let Some((pos, digit)) = self.bytes.iter().find_position(|x| x.is_dec_digit()) else {
                        self.bytes = &self.bytes[self.bytes.len()..];
                        return None;
                    };
                    self.bytes = &self.bytes[(pos + 1)..];
                    let mut res = (*digit - b'0') as $typ;

                    while let Some((first, rest)) = self.bytes.split_first() {
                        if !first.is_dec_digit() {
                            break;
                        }
                        let num = (first - b'0') as $typ;
                        let Some(new_res) = res.checked_mul(10).and_then(|x| x.checked_add(num)) else {
                            panic!("Failed to parse: {res}{num} as {}", stringify!($typ));
                        };
                        res = new_res;
                        self.bytes = rest;
                    }

                    Some(res)
                }
            }
        )+
    }
}
macro_rules! int_nums {
    ($($typ:ty),+) => {
        $(
            impl<'t> Iterator for ParseNumsIter<'t, $typ> {
                type Item = $typ;

                fn next(&mut self) -> Option<Self::Item> {
                    let mut sign = 1;
                    let mut res: $typ;
                    loop {
                        let (pos, digit_or_minus) = self
                            .bytes
                            .iter()
                            .find_position(|x| **x == b'-' || x.is_dec_digit())?;
                        self.bytes = &self.bytes[(pos + 1)..];
                        if digit_or_minus.is_dec_digit() {
                            res = (digit_or_minus - b'0') as $typ;
                            break;
                        }
                        let next_char = self.bytes.get(0)?;
                        if next_char.is_dec_digit() {
                            sign = -1;
                            res = -((*next_char - b'0') as $typ);
                            self.bytes = &self.bytes[1..];
                            break;
                        }
                    }

                    if sign == 1 {
                        while let Some((first, rest)) = self.bytes.split_first() {
                            if !first.is_dec_digit() {
                                break;
                            }
                            let num = (first - b'0') as $typ;
                            let Some(new_res) = res.checked_mul(10).and_then(|x| x.checked_add(num)) else {
                                panic!("Failed to parse: {res}{num} as {}", stringify!($typ));
                            };
                            res = new_res;
                            self.bytes = rest;
                        }
                    } else {
                        while let Some((first, rest)) = self.bytes.split_first() {
                            if !first.is_dec_digit() {
                                break;
                            }
                            let num = (first - b'0') as $typ;
                            let Some(new_res) = res.checked_mul(10).and_then(|x| x.checked_sub(num)) else {
                                panic!("Failed to parse: {res}{num} as {}", stringify!($typ));
                            };
                            res = new_res;
                            self.bytes = rest;
                        }
                    }

                    Some(res)
                }
            }
        )+
    }
}

uint_nums! {u128, u64, u32, u16, u8, usize }
int_nums! {i128, i64, i32, i16, i8, isize }

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn test_nums() {
        assert_eq!(super::nums::<u8>("1,2,3").collect_vec(), vec![1, 2, 3]);
        assert_eq!(
            super::nums::<u8>("255,0,-10").collect_vec(),
            vec![255, 0, 10]
        );
    }

    #[test]
    #[should_panic(expected = "Failed to parse: 256 as u8")]
    fn test_fails_on_large_numbers() {
        super::nums::<u8>("256").collect_vec();
    }
    #[test]
    #[should_panic(expected = "Failed to parse: 128 as i8")]
    fn test_fails_on_signed_overflow() {
        super::nums::<i8>("128").collect_vec();
    }
    #[test]
    #[should_panic(expected = "Failed to parse: -129 as i8")]
    fn test_fails_on_signed_underflow() {
        assert_eq!(super::nums::<i8>("-129").collect_vec(), vec![]);
    }
    #[test]
    fn test_rest_str() {
        let mut it = super::nums::<i16>("-129andrew");
        assert_eq!(it.next(), Some(-129));
        assert_eq!(it.rest_str(), "andrew");

        let mut it = super::nums::<i16>("-129andrew10bohdan");
        assert_eq!(it.next(), Some(-129));
        assert_eq!(it.next(), Some(10));
        assert_eq!(it.rest_str(), "bohdan");

        let mut it = super::nums::<i16>("-129andrew10bohdan");
        assert_eq!(it.next(), Some(-129));
        assert_eq!(it.rest_str(), "andrew10bohdan");
    }
    #[test]
    fn test_with_type() {
        let mut it = super::nums::<u8>("10,1024rest");
        assert_eq!(it.next(), Some(10u8));
        let mut it = it.with_type::<u32>();
        assert_eq!(it.next(), Some(1024u32));
        assert_eq!(it.rest_str(), "rest");
    }
}
