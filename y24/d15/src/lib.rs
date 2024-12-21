use advent_utils::{glam::IVec2, grid::Grid, parse, vec_on_stack};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let (mut grid, robot_pos, moves) = parse_grid(file_content);
    play(&mut grid, robot_pos, &moves);

    gps_total(
        grid.coords()
            .filter(|x| grid.get(*x).copied() == Some(b'O')),
    )
}

fn play(grid: &mut Grid<u8>, mut player: IVec2, moves: &[IVec2]) -> IVec2 {
    for m in moves {
        let m = *m;
        if grid.get(player + m).copied().unwrap() == b'.' {
            player += m;
            continue;
        }

        let boxes = (1i32..)
            .take_while(|i| {
                let p = player + m * *i;
                grid.get(p).copied().map_or(false, |x| x == b'O')
            })
            .count();
        if boxes == 0 {
            continue;
        }
        if grid
            .get(player + m * (1 + boxes as i32))
            .copied()
            .unwrap_or_default()
            == b'#'
        {
            continue;
        }
        grid.set(player + m * (1 + boxes as i32), b'O');
        grid.set(player + m, b'.');
        player += m;
    }
    player
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let (mut grid, moves) = parse_large_grid(file_content);
    for m in moves {
        grid.move_player(m);
    }

    grid.boxes
        .into_iter()
        .map(|p| (p.y * 100 + p.x) as usize)
        .sum::<usize>()
}

fn gps_total(it: impl Iterator<Item = IVec2>) -> usize {
    it.map(|p| (p.y * 100 + p.x) as usize).sum::<usize>()
}

struct LargeGrid {
    boxes: Vec<IVec2>,
    entities: HashMap<IVec2, Entity>,
    player: IVec2,
}

#[derive(Copy, Clone)]
enum Entity {
    Wall,
    Box(usize),
}
impl Entity {
    fn as_box(&self) -> Option<usize> {
        match self {
            Entity::Wall => None,
            Entity::Box(box_id) => Some(*box_id),
        }
    }
}
impl LargeGrid {
    fn get(&self, p: IVec2) -> Option<Entity> {
        self.entities.get(&p).cloned()
    }

    fn move_player(&mut self, m: IVec2) {
        // tracing::info!("\n{}\nm={m}", self.render());

        let Some(e) = self.get(self.player + m) else {
            self.player += m;
            return;
        };
        let box_idx = match e {
            Entity::Wall => {
                return;
            }
            Entity::Box(idx) => idx,
        };

        if m.y == 0 {
            let mut p = self.player + m;

            vec_on_stack! {
                let (mut boxes_to_move: Vec<usize>, mut slice) = Vec::with_capacity(5);
            }

            loop {
                let Some(e) = self.get(p) else {
                    break;
                };
                if let Entity::Box(id) = e {
                    if boxes_to_move.last().map_or(true, |x| *x != id) {
                        boxes_to_move.push(id);
                    }
                    p += m;
                    continue;
                }
                return;
            }
            for id in boxes_to_move.iter().rev() {
                self.move_box(*id, m);
            }
            self.player += m;
            return;
        }

        let mut moved_boxes_idx = vec![];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(box_idx);
        while let Some(box_idx) = to_visit.pop_front() {
            moved_boxes_idx.push(box_idx);
            let b = &self.boxes[box_idx];
            let top_left = *b + m;
            if let Some(top_left_idx) = self.get(top_left).and_then(|b| b.as_box()) {
                if !to_visit.contains(&top_left_idx) {
                    to_visit.push_back(top_left_idx);
                }
            }
            let top_right = *b + m + IVec2::X;
            if let Some(top_right_idx) = self.get(top_right).and_then(|b| b.as_box()) {
                if !to_visit.contains(&top_right_idx) {
                    to_visit.push_back(top_right_idx);
                }
            }
        }

        let cannot_move = moved_boxes_idx.iter().copied().any(|box_id| {
            [m, m + IVec2::X]
                .map(|offset| self.boxes[box_id] + offset)
                .into_iter()
                .any(|p| self.get(p).map_or(false, |e| matches!(e, Entity::Wall)))
        });

        if cannot_move {
            return;
        }

        for b_id in moved_boxes_idx.into_iter().rev() {
            self.move_box(b_id, m);
        }
        self.player += m;
    }

    fn move_box(&mut self, box_id: usize, m: IVec2) {
        let b = &mut self.boxes[box_id];
        self.entities.remove(b);
        self.entities.remove(&(*b + IVec2::X));
        *b += m;
        self.entities.insert(*b, Entity::Box(box_id));
        self.entities.insert(*b + IVec2::X, Entity::Box(box_id));
    }
}

impl From<Grid<u8>> for LargeGrid {
    fn from(grid: Grid<u8>) -> Self {
        let mut boxes = Vec::with_capacity(grid.elements_len());
        let mut entities: HashMap<IVec2, Entity> = HashMap::new();
        let mut player: IVec2 = IVec2::new(2, 1);
        for c in grid.coords() {
            let cell = grid.get(c).copied().unwrap();
            if cell == b'.' {
                continue;
            }
            let large_coords = c * IVec2::new(2, 1);
            if cell == b'#' {
                entities.insert(large_coords, Entity::Wall);
                entities.insert(large_coords + IVec2::X, Entity::Wall);
                continue;
            }
            if cell == b'@' {
                player = large_coords;
                continue;
            }
            let box_idx = boxes.len();
            boxes.push(large_coords);
            entities.insert(large_coords, Entity::Box(box_idx));
            entities.insert(large_coords + IVec2::X, Entity::Box(box_idx));
            continue;
        }

        Self {
            boxes,
            entities,
            player,
        }
    }
}

fn parse_grid(file_content: &str) -> (Grid<u8>, IVec2, Vec<IVec2>) {
    let (first, second) = file_content.split_once("\n\n").unwrap();
    let mut grid = parse::ascii_grid(first);
    let robot_pos = grid
        .coords()
        .find(|x| grid.get(*x).copied() == Some(b'@'))
        .unwrap();

    let moves = second
        .as_bytes()
        .iter()
        .filter_map(|x| match x {
            // >>vv<v>>v<
            b'<' => Some(IVec2::NEG_X),
            b'v' => Some(IVec2::Y),
            b'>' => Some(IVec2::X),
            b'^' => Some(IVec2::NEG_Y),
            _ => None,
            //
        })
        .collect_vec();
    grid.set(robot_pos, b'.').unwrap();

    (grid, robot_pos, moves)
}

fn parse_large_grid(file_content: &str) -> (LargeGrid, Vec<IVec2>) {
    let (mut grid, robot_pos, moves) = parse_grid(file_content.trim());
    grid.set(robot_pos, b'@');
    let grid: LargeGrid = grid.into();
    (grid, moves)
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "10092");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1430439");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "9021");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        let res = solve_part_2(ACTUAL);
        assert_eq!(res, 1458740);
    }
}
