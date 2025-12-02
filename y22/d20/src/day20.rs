use itertools::Itertools;

struct Loop<T> {
    values: Vec<T>,
    next_ptr: Vec<usize>,
    prev_ptr: Vec<usize>,
    start: usize,
}

struct LoopIterator<'l, T> {
    loop_ref: &'l Loop<T>,
    next_ptr: usize,
}

impl<'t, T> Iterator for LoopIterator<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_ptr = self.next_ptr;
        self.next_ptr = self.loop_ref.next_ptr[self.next_ptr];
        self.loop_ref.values.get(prev_ptr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

impl<T> Loop<T> {
    fn iter(&self) -> LoopIterator<'_, T> {
        LoopIterator {
            loop_ref: self,
            next_ptr: self.start,
        }
    }
    fn len(&self) -> usize {
        self.values.len()
    }
    fn do_shift(&mut self, ptr: usize, mut shift: isize) {
        if shift == 0 {
            return;
        }
        let leni = self.len() as isize - 1;
        if shift > leni {
            shift %= leni;
        } else if shift < -leni {
            shift = -((-shift) % leni)
        }
        if shift > 0 {
            self.do_shift_right(ptr, shift as usize);
        } else {
            self.do_shift_left(ptr, (-shift) as usize);
        }
    }
    fn do_shift_right(&mut self, ptr: usize, shift: usize) {
        for _ in 0..shift {
            self.do_single_shift_right(ptr);
        }
    }
    #[inline]
    fn connect(&mut self, prev: usize, next: usize) {
        self.next_ptr[prev] = next;
        self.prev_ptr[next] = prev;
    }
    fn do_single_shift_right(&mut self, ptr: usize) {
        //   a -> b -> c -> d
        //        ^ptr
        //   a -> c -> b -> d
        //             ^ptr
        let b = ptr;
        let c = self.next_ptr[b];
        let a = self.prev_ptr[b];
        let d = self.next_ptr[c];

        self.connect(a, c);
        self.connect(c, b);
        self.connect(b, d);

        if self.start == b {
            self.start = c;
        }
    }
    fn do_shift_left(&mut self, ptr: usize, shift: usize) {
        for _ in 0..shift {
            self.do_single_shift_left(ptr);
        }
    }
    fn do_single_shift_left(&mut self, ptr: usize) {
        //   a -> c -> b -> d
        //             ^ptr
        //   a -> b -> c -> d
        //        ^ptr
        let b = ptr;
        let c = self.prev_ptr[b];
        let a = self.prev_ptr[c];
        let d = self.next_ptr[b];

        self.connect(a, b);
        self.connect(b, c);
        self.connect(c, d);
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Loop<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.iter().enumerate().take(self.len()) {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", x)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<A> FromIterator<A> for Loop<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let values = iter.into_iter().collect_vec();
        let n = values.len();
        let mut next_ptr = Vec::with_capacity(n);
        let mut prev_ptr = Vec::with_capacity(n);
        next_ptr.extend(1..values.len());
        next_ptr.push(0);
        prev_ptr.push(n - 1);
        prev_ptr.extend(0..(n - 1));
        Self {
            values,
            next_ptr,
            prev_ptr,
            start: 0,
        }
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> isize {
    let mut nums = advent_utils::parse::nums::<isize>(file_content).collect::<Loop<_>>();
    for i in 0..nums.len() {
        let v = nums.values[i];
        nums.do_shift(i, v);
    }
    let nums = nums.iter().take(nums.len()).copied().collect_vec();
    let zero_ind = nums.iter().position(|x| *x == 0).unwrap();
    let a = nums[(zero_ind + 1000) % nums.len()];
    let b = nums[(zero_ind + 2000) % nums.len()];
    let c = nums[(zero_ind + 3000) % nums.len()];
    a + b + c
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> isize {
    let mut nums = advent_utils::parse::nums::<isize>(file_content)
        .map(|x| x * 811589153)
        .collect::<Loop<_>>();
    for i in (0..nums.len()).cycle().take(nums.len() * 10) {
        let v = nums.values[i];
        nums.do_shift(i, v);
    }
    let nums = nums.iter().take(nums.len()).copied().collect_vec();
    let zero_ind = nums.iter().position(|x| *x == 0).unwrap();
    let a = nums[(zero_ind + 1000) % nums.len()];
    let b = nums[(zero_ind + 2000) % nums.len()];
    let c = nums[(zero_ind + 3000) % nums.len()];
    a + b + c
}

#[cfg(test)]
mod tests {
    use crate::day20::Loop;

    use super::{part1, part2};
    use itertools::Itertools;
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    // cargo test --package y22d20 --lib -- tests::test_part1::case_1_example --exact --show-output
    // cargo test --package y22d20 --lib -- day20::tests::test_part1::case_1_example --exact --nocapture

    #[rstest]
    #[case("1, 2, -3, 3, -2, 0, 4", 0, 1, "2, 1, -3, 3, -2, 0, 4")]
    #[case("2, 1, -3, 3, -2, 0, 4", 0, 2, "1, -3, 2, 3, -2, 0, 4")]
    #[case("1, -3, 2, 3, -2, 0, 4", 1, -3, "1, 2, 3, -2, -3, 0, 4")]
    #[case("1, 2, 3, -2, -3, 0, 4", 2, 3, "1, 2, -2, -3, 0, 3, 4")]
    #[case("1, 2, -2, -3, 0, 3, 4", 2, -2, "1, 2, -3, 0, 3, 4, -2")]
    #[case("1, 2, -3, 0, 3, 4, -2", 3, 0, "1, 2, -3, 0, 3, 4, -2")]
    #[case("1, 2, -3, 0, 3, 4, -2", 5, 4, "1, 2, -3, 4, 0, 3, -2")]
    fn test_do_shift(
        #[case] input: &str,
        #[case] ptr: usize,
        #[case] shift: isize,
        #[case] expected: &str,
    ) {
        let mut nums = advent_utils::parse::nums::<isize>(input).collect::<Loop<_>>();
        println!("{:?}", nums);
        nums.do_shift(ptr, shift);
        let output = nums.iter().take(nums.len()).join(", ");
        assert_eq!(output.as_str(), expected);
    }

    #[rstest]
    #[case::example(EXAMPLE, "3")]
    #[case::actual(ACTUAL, "13289")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "1623178306")]
    // #[case::actual(ACTUAL, "2865721299243")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
