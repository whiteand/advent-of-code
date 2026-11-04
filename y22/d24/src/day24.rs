use std::{collections::VecDeque, ops::Deref};

use advent_utils::{
    glam::IVec2,
    grid::{Grid, NonDiagonal},
};
use rustc_hash::FxHashSet;

#[derive(Clone, Default)]
struct Map {
    size: IVec2,
    winds: Winds,
}

#[derive(Clone, Default)]
struct Winds {
    positions: Vec<IVec2>,
    directions: Vec<IVec2>,
}

impl Winds {
    fn get_wind_positions(&self, size: IVec2, t: usize, dst: &mut [IVec2]) {
        for (i, dir) in self.directions.iter().copied().enumerate() {
            let pos = self.positions.get(i).unwrap();

            let tv = IVec2::splat(t as i32);
            let sm2 = size - IVec2::splat(2);
            let tvm = tv.rem_euclid(sm2);
            let tvmd = tvm * dir;
            let pos_tvmd = tvmd + pos;
            let pos_tvmd_off = pos_tvmd - IVec2::splat(1);
            let pos_tvmd_off_m = pos_tvmd_off.rem_euclid(sm2);

            dst[i] = pos_tvmd_off_m + IVec2::splat(1)
        }
    }

    fn alloc_pos(&self) -> Vec<IVec2> {
        self.positions.clone()
    }

    fn get(&self, size: IVec2, temp_pos: &mut [IVec2], pos: IVec2, t: usize) -> u8 {
        self.get_wind_positions(size, t, temp_pos);

        let mut found_dir = IVec2::ZERO;
        let mut cnt = 0;
        for dir in temp_pos
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, p)| *p == pos)
            .map(|(i, _)| self.directions[i])
        {
            cnt += 1;
            found_dir = dir;
        }
        _ = temp_pos;
        if cnt == 0 {
            return b'.';
        }
        if cnt == 1 {
            return NonDiagonal::to_ascii_char(found_dir);
        }

        cnt + b'0'
    }

    fn spawn(&mut self, pos: IVec2, dir: IVec2) {
        self.positions.push(pos);
        self.directions.push(dir);
    }
}

#[derive(Copy, Clone)]
struct MapAt<'t> {
    map: &'t Map,
    t: usize,
}

impl<'t> Deref for MapAt<'t> {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        self.map
    }
}

impl<'t> MapAt<'t> {
    fn into_ascii_grid(self) -> Grid<u8> {
        let mut grid = Grid::new(self.map.size, b'.');
        let mut temp_pos = self.map.winds.alloc_pos();
        let t = self.t;
        for (p, v) in grid.entries_mut() {
            *v = self.map.get(&mut temp_pos, p, t);
        }
        grid
    }

    fn placed(self, pos: IVec2) -> MapAtPlaced<'t> {
        MapAtPlaced { map_at: self, pos }
    }

    fn next_tick(self) -> Self {
        Self {
            map: self.map,
            t: self.t + 1,
        }
    }

    fn get_wind_positions(&self, dst: &mut [IVec2]) -> &Self {
        self.winds.get_wind_positions(self.size, self.t, dst);
        self
    }
}

impl std::fmt::Display for MapAt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "t={}", self.t)?;
        write!(f, "{}", self.into_ascii_grid().render_ascii())
    }
}

#[derive(Copy, Clone)]
struct MapAtPlaced<'t> {
    map_at: MapAt<'t>,
    pos: IVec2,
}

impl<'t> MapAtPlaced<'t> {
    fn next_tick(self) -> Self {
        Self {
            map_at: self.map_at.next_tick(),
            pos: self.pos,
        }
    }

    fn with_pos(self, pos: IVec2) -> Self {
        Self {
            map_at: self.map_at,
            pos,
        }
    }

    fn key(self) -> (usize, IVec2) {
        (
            self.t % (((self.size.x - 2) * (self.size.y - 2)) as usize),
            self.pos,
        )
    }

    fn with_goal(self, it: usize) -> MapAtPlacedIteration<'t> {
        MapAtPlacedIteration {
            map_at_placed: self,
            goal_index: it,
        }
    }
}

impl<'t> Deref for MapAtPlaced<'t> {
    type Target = MapAt<'t>;

    fn deref(&self) -> &Self::Target {
        &self.map_at
    }
}

impl std::fmt::Display for MapAtPlaced<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = self.map_at.into_ascii_grid();
        grid.set(self.pos, b'E');
        writeln!(f, "t={}", self.map_at.t)?;
        write!(f, "{}", grid.render_ascii())
    }
}

#[derive(Copy, Clone)]
struct MapAtPlacedIteration<'t> {
    map_at_placed: MapAtPlaced<'t>,
    goal_index: usize,
}

impl<'t> Deref for MapAtPlacedIteration<'t> {
    type Target = MapAtPlaced<'t>;

    fn deref(&self) -> &Self::Target {
        &self.map_at_placed
    }
}

impl<'t> MapAtPlacedIteration<'t> {
    fn key(self) -> (usize, IVec2) {
        (
            (self.t % (((self.size.x - 2) * (self.size.y - 2)) as usize)) * 3 + self.goal_index,
            self.pos,
        )
    }

    fn goal(self) -> IVec2 {
        match self.goal_index {
            0 | 2 => self.size - IVec2::new(2, 1),
            1 => IVec2::new(1, 0),
            _ => unreachable!(),
        }
    }
}

impl Map {
    fn alloc_pos(&self) -> Vec<IVec2> {
        self.winds.alloc_pos()
    }

    fn is_wall(&self, pos: IVec2) -> bool {
        match pos.to_array() {
            [1, 0] => false,
            [_, y] if y <= 0 => true,
            [x, _] if x <= 0 => true,
            [x, _] if x >= self.size.x - 1 => true,
            [x, y] if x == self.size.x - 2 && y == self.size.y - 1 => false,
            [_, y] if y >= self.size.y - 1 => true,
            _ => false,
        }
    }
    fn get(&self, temp_pos: &mut [IVec2], pos: IVec2, t: usize) -> u8 {
        if self.is_wall(pos) {
            return b'#';
        }
        match pos.to_array() {
            [1, 0] => b'.',
            [x, y] if x == self.size.x - 2 && y == self.size.y - 1 => b'.',
            _ => self.winds.get(self.size, temp_pos, pos, t),
        }
    }

    fn at(&self, t: usize) -> MapAt<'_> {
        MapAt { map: self, t }
    }
}

impl Map {
    fn new(size: IVec2) -> Self {
        Self {
            winds: Winds::default(),
            size,
        }
    }

    fn spawn_wind(&mut self, pos: IVec2, dir: IVec2) {
        self.winds.spawn(pos, dir)
    }
}

fn parse_map(file_content: &str) -> Map {
    let grid = advent_utils::parse::ascii_grid(file_content);
    let mut map = Map::new(grid.size());

    for (p, v) in grid.entries_copy() {
        match v {
            b'#' | b'.' => continue,
            x => {
                let Some(direction) = NonDiagonal::try_from_ascii_char(x) else {
                    unreachable!("{}", v as char);
                };
                map.spawn_wind(p, direction);
            }
        }
    }

    map
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let map = parse_map(file_content);
    let initial_state = map.at(0).placed(IVec2 { x: 1, y: 0 });
    let t_mod = ((map.size.x - 2) * (map.size.y - 2)) as usize;
    let positions = (0usize..t_mod)
        .map(|i| {
            let mut positions = initial_state.alloc_pos();
            initial_state.map.at(i).get_wind_positions(&mut positions);
            positions
        })
        .collect::<Vec<_>>();
    let mut opt_pos = Vec::new();

    let mut queue = VecDeque::new();

    let mut visited: FxHashSet<(usize, IVec2)> = Default::default();

    queue.push_back(initial_state);

    while let Some(s) = queue.pop_front() {
        if s.pos == s.size - IVec2::new(2, 1) {
            return s.t;
        }
        if visited.contains(&s.key()) {
            continue;
        }
        visited.insert(s.key());
        // println!("{}", s);
        let next_tick_state = s.next_tick();
        // println!("Next:\n{}", next_tick_state);
        let temp_pos = &positions[next_tick_state.t % t_mod];
        opt_pos.extend(
            NonDiagonal::directions()
                .map(|d| d + s.pos)
                .into_iter()
                .filter(|p| !next_tick_state.is_wall(*p))
                .chain(std::iter::once(s.pos)),
        );

        opt_pos.retain(|x| !temp_pos.contains(x));

        for new_pos in opt_pos.drain(..) {
            let next_state = next_tick_state.with_pos(new_pos);
            if visited.contains(&next_state.key()) {
                continue;
            }
            queue.push_back(next_state);
        }
    }

    0
}
#[tracing::instrument(skip(file_content))]
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let map = parse_map(file_content);
    let initial_state = map.at(0).placed(IVec2 { x: 1, y: 0 }).with_goal(0);
    let t_mod = ((map.size.x - 2) * (map.size.y - 2)) as usize;

    let positions = (0usize..t_mod)
        .map(|i| {
            let mut positions = initial_state.alloc_pos();
            initial_state.map.at(i).get_wind_positions(&mut positions);
            positions
        })
        .collect::<Vec<_>>();

    let mut opt_pos = Vec::new();

    let mut queue = VecDeque::new();

    let mut visited: FxHashSet<(usize, IVec2)> = Default::default();

    queue.push_back(initial_state);

    while let Some(s) = queue.pop_front() {
        if s.goal() == s.pos {
            let next_s = s.with_goal(s.goal_index + 1);
            if next_s.goal_index == 3 {
                return next_s.t;
            } else {
                queue.push_back(next_s);
            }
            continue;
        }

        if visited.contains(&s.key()) {
            continue;
        }
        visited.insert(s.key());
        // println!("{}", s);
        let next_tick_state = s.next_tick();
        // println!("Next:\n{}", next_tick_state);

        opt_pos.extend(
            NonDiagonal::directions()
                .map(|d| d + s.pos)
                .into_iter()
                .filter(|p| !next_tick_state.is_wall(*p))
                .chain(std::iter::once(s.pos)),
        );

        let temp_pos = &positions[next_tick_state.t % t_mod];

        opt_pos.retain(|x| !temp_pos.contains(x));

        for new_pos in opt_pos.drain(..) {
            queue.push_back(next_tick_state.with_pos(new_pos).with_goal(s.goal_index));
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    // const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "18")]
    // #[case::actual(ACTUAL, "277")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "54")]
    // #[case::actual(ACTUAL, "877")] // 46s
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
