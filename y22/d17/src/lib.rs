mod chamber;
mod direction;
mod figure;
mod get_figures;
mod infinite;
mod parse;
mod vector;

use get_figures::get_figures;
use infinite::infinite;

use self::{
    chamber::Chamber,
    direction::Direction::{self, *},
    figure::Figure,
    vector::Vector,
};

struct FallingFigure<'i, Dirs>
where
    Dirs: Iterator<Item = Direction>,
{
    chamber: &'i Chamber,
    figure: &'i Figure,
    direction: &'i mut Dirs,
    position: Vector,
    finished: bool,
}

impl<'i, Dirs: Iterator<Item = Direction>> FallingFigure<'i, Dirs> {
    fn new(chamber: &'i Chamber, figure: &'i Figure, dirs: &'i mut Dirs) -> Self {
        Self {
            chamber,
            figure,
            direction: dirs,
            position: Vector::new(2, (chamber.height() + 3) as isize),
            finished: false,
        }
    }
}

impl<'i, Dirs: Iterator<Item = Direction>> Iterator for FallingFigure<'i, Dirs> {
    type Item = (&'i Figure, Vector);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // self.chamber.print(Some((self.figure, self.position)));
        match self.direction.next() {
            None => None,
            Some(dir) => {
                let step = match dir {
                    Left => Vector::new(-1, 0),
                    Right => Vector::new(1, 0),
                    Down => Vector::new(0, -1),
                };
                let new_pos = self.position.plus(&step);

                let can_move = new_pos.y >= 0
                    && new_pos.x >= 0
                    && new_pos.x + self.figure.width() as isize <= self.chamber.width() as isize
                    && self
                        .figure
                        .points
                        .iter()
                        .all(|p| !self.chamber.is_taken(&p.plus(&new_pos)));

                if can_move {
                    self.position = new_pos;
                } else if dir.is_down() {
                    self.finished = true;
                    return None;
                }
                Some((self.figure, self.position))
            }
        }
    }
}

pub fn solve_part_1<const W: usize>(file_content: &str, stop: usize) -> usize {
    let figures = get_figures();
    let dirs = parse::parse(file_content).collect::<Vec<_>>();
    let dirs = infinite(&dirs);
    let mut all_dirs = dirs.cloned().flat_map(|dir| [dir, Down]);
    let mut chamber = Chamber::new(W);
    let mut figures_it = infinite::infinite(&figures);
    let mut heights = std::iter::repeat_with(move || {
        let fig = figures_it.next().unwrap();
        let pos = FallingFigure::new(&chamber, fig, &mut all_dirs)
            .map(|p| p.1)
            .last()
            .unwrap();
        chamber.place(fig, pos);
        chamber.height()
    });
    heights.nth(stop - 1).unwrap()
}
pub fn solve_part_2<const N: usize>(file_content: &str) -> impl std::fmt::Display {
    const CYCLE: usize = 1725;
    const ADDITION: usize = 2659;
    let rem = N / CYCLE;
    let q = N % CYCLE;

    println!("rem = {rem}, q = {q}");
    ADDITION * rem + solve_part_1::<7>(file_content, q)
}
#[cfg(test)]
mod tests {
    use super::*;
    // COMMON: 35, addition: 53
    const INPUT: &str = include_str!("../example.txt");
    // COMMON: 1725, addition: 2659
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1::<7>(INPUT.trim(), 2022)), "3068");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(
            format!("{}", solve_part_1::<7>(ACTUAL.trim(), 2022)),
            "3109"
        );
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2::<2022>(ACTUAL.trim())), "3109");
        assert_eq!(
            format!("{}", solve_part_2::<1_000_000_000_000>(ACTUAL.trim())),
            "1541449275365"
        );
    }
}
