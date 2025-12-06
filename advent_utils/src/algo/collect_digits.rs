pub trait FromDecDigits: Sized {
    fn from_dec_digits(digits: impl Iterator<Item = u8>) -> Option<Self>;
}

pub trait CollectDecDigits: Iterator<Item = u8> {
    fn collect_dec_digits<T: FromDecDigits>(self) -> Option<T>
    where
        Self: Sized,
    {
        T::from_dec_digits(self)
    }
}

impl<T: Iterator<Item = u8>> CollectDecDigits for T {}

macro_rules! impl_for_ints {
    ($($t:ty),*) => {
        $(
            impl FromDecDigits for $t {
                fn from_dec_digits(digits: impl Iterator<Item = u8>) -> Option<Self> {
                    digits.map(|d| d as $t).reduce(|a,b| a * 10 + b)
                }
            }
        )*
    }
}

impl_for_ints! {
    u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,usize,isize
}
