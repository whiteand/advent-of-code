use advent_utils::{glam::IVec2, grid::Grid};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content.trim());
    let mut colors = grid.map(|_, _| 0);
    let mut color = 0;
    let mut total = 0;
    for p in grid.coords() {
        let value = grid.get(p).copied().unwrap();
        if colors.get(p).copied().unwrap_or_default() == 0 {
            color += 1;
            let mut area = 0;
            let mut perimeter = 0;
            calculate_area_and_perimeter(
                &grid,
                &mut colors,
                p,
                value,
                color,
                &mut area,
                &mut perimeter,
            );
            total += area * perimeter;
        }
    }
    total
}

const DIRS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];
fn calculate_area_and_sides(
    grid: &Grid<u8>,
    colors: &mut Grid<usize>,
    p: IVec2,
    region_value: u8,
    color: usize,
    area: &mut usize,
    sides: &mut usize,
) {
    if colors.get(p).copied().unwrap() == color {
        return;
    }
    *area += 1;
    colors.set(p, color);
    let neighbours = {
        let mut res = 0;
        let mut it = FULL_NEIGHBOURS.iter().map(|d| colors.get(p + *d));
        for _ in 0..FULL_NEIGHBOURS.len() {
            res <<= 1;
            res |= if it.next().flatten().map(|v| *v == color).unwrap_or_default() {
                1
            } else {
                0
            };
        }
        res
    };

    match neighbours {
        0b11010110 | 0b11111000 | 0b01101011 | 0b00011111 => {
            *sides -= 4;
        }
        0b00001000 | 0b00000010 | 0b00010000 | 0b01000000 | 0b11010100 | 0b11101000
        | 0b00101011 | 0b00010111 | 0b11110000 | 0b10010110 | 0b00001111 | 0b01101001 => {
            *sides += 0;
        }
        0b00000011 | 0b00010100 | 0b11000000 | 0b00101000 | 0b00001001 | 0b01100000
        | 0b10010000 | 0b00000110 => {
            *sides += 2;
        }
        0b00000000 | 0b10010100 | 0b11100000 | 0b00101001 | 0b00000111 | 0b10010101
        | 0b11100100 | 0b10101001 | 0b00100111 | 0b11100001 | 0b10110100 | 0b10000111
        | 0b00101101 => {
            *sides += 4;
        }

        0b11011111 | 0b11111110 | 0b11111011 | 0b01111111 => {
            // ###
            // #?#
            // .##
            *sides -= 4;
        }
        0b00010110 | 0b11010000 | 0b01101000 | 0b00001011 => {
            // .##
            // .?#
            // ...
            *sides -= 2;
        }
        0b10011111 | 0b11110110 | 0b11111001 | 0b01101111 | 0b11101011 | 0b11111100
        | 0b11010111 | 0b00111111 => {
            // ###
            // #?#
            // ..#
            *sides -= 2;
        }
        0b10010111 | 0b11110100 | 0b11101001 | 0b00101111 => {
            // ###
            // .?#
            // ..#
            *sides += 2;
        }
        0b01100001 | 0b00001101 | 0b10000110 | 0b10110000 | 0b00010101 | 0b00100011
        | 0b10101000 | 0b11000100 => {
            // #..
            // .?.
            // ##.
            *sides += 2;
        }
        0b01000100 | 0b10001000 | 0b00100010 | 0b00010001 | 0b00110000 | 0b10000010
        | 0b00001100 | 0b01000001 => {
            // ..#
            // .?.
            // .#.
            *sides += 0;
        }
        0b01110000 | 0b11001000 | 0b01010100 | 0b10010010 | 0b00101010 | 0b00001110
        | 0b01001001 | 0b00010011 => {
            // ..#
            // .?#
            // .#.
            *sides += 0;
        }
        0b11110101 | 0b11101101 | 0b10110111 | 0b10101111 => {
            // ###
            // .?#
            // #.#
            *sides += 2;
        }
        0b11101100 | 0b10110110 | 0b11110001 | 0b11010101 | 0b01101101 | 0b10101011
        | 0b00110111 | 0b10001111 => {
            // #.#
            // .?#
            // .##
            *sides += 0;
        }
        0b11101110 | 0b10111110 | 0b11011101 | 0b01111101 | 0b11110011 | 0b10111011
        | 0b01110111 | 0b11001111 => {
            // #.#
            // #?#
            // .##
            *sides += 0;
        }
        0b11111111 => {
            // ###
            // #?#
            // ###
            *sides -= 4;
        }
        0b11111101 | 0b11110111 | 0b11101111 | 0b10111111 => {
            // ###
            // #?#
            // #.#
            *sides += 0;
        }
        0b11011000 | 0b01111000 | 0b11010010 | 0b01101010 | 0b01010110 | 0b00011110
        | 0b01001011 | 0b00011011 => {
            // .##
            // .?#
            // .#.
            *sides -= 4;
        }
        0b10111100 | 0b11100110 | 0b10111001 | 0b10011101 | 0b00111101 | 0b11100011
        | 0b11000111 | 0b01100111 => {
            // #.#
            // #?#
            // #..
            *sides += 2;
        }
        0b01101100 | 0b00110110 | 0b11010001 | 0b10001011 => {
            // #..
            // .?#
            // .##
            *sides -= 2;
        }
        0b10111000 | 0b11000110 | 0b00011101 | 0b01100011 => {
            // #.#
            // #?#
            // ...
            *sides += 0;
        }
        0b10111101 | 0b11100111 => {
            // #.#
            // #?#
            // #.#
            *sides += 4;
        }
        0b10011000 | 0b00111000 | 0b00011100 | 0b11000010 | 0b01100010 | 0b01000110
        | 0b00011001 | 0b01000011 => {
            // .##
            // .?.
            // .#.
            *sides -= 2;
        }
        0b01111100 | 0b01110110 | 0b01101110 | 0b00111110 | 0b11011001 | 0b11010011
        | 0b11001011 | 0b10011011 => {
            // ##.
            // #?#
            // ..#
            *sides -= 2;
        }
        0b01110100 | 0b00101110 | 0b11001001 | 0b10010011 => {
            // ##.
            // .?#
            // ..#
            *sides += 2;
        }
        0b11011100 | 0b11110010 | 0b11101010 | 0b10011110 | 0b01111001 | 0b00111011
        | 0b01010111 | 0b01001111 => {
            // .##
            // #?#
            // ..#
            *sides -= 2;
        }
        0b10011100 | 0b11100010 | 0b00111001 | 0b01000111 => {
            // ###
            // .?.
            // .#.
            *sides += 0;
        }
        0b00111100 | 0b01100110 | 0b10011001 | 0b11000011 => {
            // ##.
            // .?.
            // .##
            *sides += 0;
        }
        x => {
            panic!("Unreachable: {x:?}");
        }
    }

    for p in grid
        .neighbours(p, DIRS)
        .filter(|(_, v)| **v == region_value)
        .map(|(p, _)| p)
    {
        calculate_area_and_sides(grid, colors, p, region_value, color, area, sides);
    }
}
const FULL_NEIGHBOURS: [IVec2; 8] = [
    IVec2::splat(-1),
    IVec2::NEG_Y,
    IVec2::new(1, -1),
    IVec2::NEG_X,
    IVec2::X,
    IVec2::new(-1, 1),
    IVec2::Y,
    IVec2::splat(1),
];
fn calculate_area_and_perimeter(
    grid: &Grid<u8>,
    colors: &mut Grid<usize>,
    p: IVec2,
    region_value: u8,
    color: usize,
    area: &mut usize,
    perimeter: &mut usize,
) {
    if colors.get(p).copied().unwrap() == color {
        return;
    }
    *area += 1;
    colors.set(p, color);
    *perimeter = *perimeter + 4
        - colors
            .neighbours(p, DIRS)
            .filter(|(_, v)| **v == color)
            .count()
            * 2;

    for p in grid
        .neighbours(p, DIRS)
        .filter(|(_, v)| **v == region_value)
        .map(|(p, _)| p)
    {
        calculate_area_and_perimeter(grid, colors, p, region_value, color, area, perimeter);
    }
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content.trim());
    let mut colors = grid.map(|_, _| 0);
    let mut color = 0;
    let mut total = 0;
    for p in grid.coords() {
        let value = grid.get(p).copied().unwrap();
        if colors.get(p).copied().unwrap_or_default() == 0 {
            color += 1;
            let mut area = 0;
            let mut sides = 0;
            calculate_area_and_sides(&grid, &mut colors, p, value, color, &mut area, &mut sides);
            total += area * sides;
        }
    }
    total
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "1930");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1522850");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "1206");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "953738");
    }
}
