#[derive(Debug)]
pub struct Condition {
    pub divisor: u64,
    if_true: usize,
    if_false: usize,
}

impl Condition {
    pub fn new_division_condition(divisor: u64, if_true: usize, if_false: usize) -> Self {
        Condition {
            divisor,
            if_true,
            if_false,
        }
    }
    pub fn choose(&self, worry_level: u64) -> usize {
        if worry_level % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}
