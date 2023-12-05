use super::{figure::Figure, vector::Vector};

#[derive(Debug, Clone)]
pub struct Chamber {
    levels: Vec<u8>,
    width: usize,
    base: usize,
}

impl Chamber {
    pub fn new(width: usize) -> Self {
        Self {
            levels: Vec::new(),
            width,
            base: 0,
        }
    }
    pub fn place(&mut self, figure: &Figure, left_bottom: Vector) {
        if figure.points.is_empty() {
            return;
        }

        for Vector { x, y } in figure.points.iter() {
            let px = (x + left_bottom.x) as usize;
            let py = (y + left_bottom.y) as usize - self.base;
            while py >= self.levels.len() {
                self.levels.push(0);
            }
            self.levels[py] |= 1 << px;
        }
    }
    pub fn height(&self) -> usize {
        self.levels.len() + self.base
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn is_taken(&self, v: &Vector) -> bool {
        if v.y as usize >= self.height() {
            return false;
        }
        self.levels[v.y as usize - self.base] & (1 << (v.x as usize)) != 0
    }
    // pub fn print(&self, moved_figure: Option<(&Figure, Vector)>) {
    //     let height = match &moved_figure {
    //         Some((fig, pos)) => self.height.max(fig.height() + pos.y as usize),
    //         None => self.height,
    //     };
    //     let mut screen = vec![vec!['.'; self.width]; height];

    //     for Vector { x, y } in self.taken() {
    //         screen[y as usize][x as usize] = '#';
    //     }

    //     for Vector { x, y } in moved_figure
    //         .into_iter()
    //         .flat_map(|(fig, pos)| fig.points.iter().map(move |v| v.plus(&pos)))
    //     {
    //         screen[y as usize][x as usize] = '@';
    //     }

    //     screen.reverse();

    //     let mut s: String = String::with_capacity(
    //         self.width * self.height + self.height * 2 + self.width + 2 + self.height + 1,
    //     );
    //     for line in screen {
    //         s.push('|');
    //         for ch in line {
    //             s.push(ch);
    //         }
    //         s.push('|');
    //         s.push('\n');
    //     }
    //     s.push('+');
    //     for _ in 0..self.width {
    //         s.push('-');
    //     }
    //     s.push('+');
    //     println!("{s}");
    // }
}
