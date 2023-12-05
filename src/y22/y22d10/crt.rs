pub struct Crt {
    pub row: usize,
    pub col: usize,
}

impl Crt {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }
    pub fn draw(&mut self, register_x: i32) -> &'static str {
        let is_filled = (register_x - self.col as i32).abs() <= 1;
        let is_new_line = self.col == 39;

        self.col += 1;
        if self.col >= 40 {
            self.col = 0;
            self.row += 1;
        }

        match (is_filled, is_new_line) {
            (true, true) => "#\n",
            (true, false) => "#",
            (false, true) => ".\n",
            (false, false) => ".",
        }
    }
}
