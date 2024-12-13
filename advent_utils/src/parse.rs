use std::marker::PhantomData;

use itertools::Itertools;
use nom::AsChar;

pub fn nums<'t, T>(line: &'t str) -> ParseNumsIter<'t, T> {
    ParseNumsIter {
        bytes: line.as_bytes(),
        phantom: PhantomData,
    }
}
pub struct ParseNumsIter<'t, T> {
    bytes: &'t [u8],
    phantom: PhantomData<T>,
}

impl<'t, T> ParseNumsIter<'t, T> {}

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
}
