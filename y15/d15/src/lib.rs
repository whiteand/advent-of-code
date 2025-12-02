use itertools::Itertools;
use nom::Parser;

pub fn solve_part_1(file_content: &str) -> i64 {
    let ingredients = parse_ingredients(file_content).unwrap();
    best_of_spoons(&ingredients, 100, get_total)
}
pub fn solve_part_2(file_content: &str) -> i64 {
    let ingredients = parse_ingredients(file_content).unwrap();
    best_of_spoons(&ingredients, 100, |a, b| get_total_2(a, b, 500))
}

fn best_of_spoons(
    ingredients: &[Ingredient],
    spoons: usize,
    get_total: fn(&[Ingredient], &[usize]) -> i64,
) -> i64 {
    let mut amount = ingredients.iter().map(|_| 0usize).collect_vec();
    let mut best_total = 0;

    go(
        ingredients,
        spoons,
        0,
        &mut amount,
        &mut best_total,
        get_total,
    );

    best_total
}

fn go(
    ingredients: &[Ingredient],
    spoons: usize,
    ind: usize,
    amount: &mut [usize],
    best_total: &mut i64,
    get_total: fn(&[Ingredient], &[usize]) -> i64,
) {
    let remaining = spoons - amount[..ind].iter().sum::<usize>();
    if ind == amount.len() - 1 {
        amount[ind] = remaining;
        let total = get_total(ingredients, amount);
        if total > *best_total {
            *best_total = total;
        }
        return;
    }
    for n in 0..=remaining {
        amount[ind] = n;
        go(ingredients, spoons, ind + 1, amount, best_total, get_total);
    }
}

fn get_total(ingredients: &[Ingredient], amounts: &[usize]) -> i64 {
    macro_rules! sum_with_amounts {
        ($ings:ident, $amounts:ident, $field:ident) => {
            $ings
                .iter()
                .map(|x| x.$field)
                .zip($amounts)
                .map(|(cap, n)| cap * (*n as i64))
                .sum::<i64>()
                .max(0)
        };
    }
    let capacity = sum_with_amounts!(ingredients, amounts, capacity);
    let durability = sum_with_amounts!(ingredients, amounts, durability);
    let flavor = sum_with_amounts!(ingredients, amounts, flavor);
    let texture = sum_with_amounts!(ingredients, amounts, texture);

    capacity * durability * flavor * texture
}
fn get_total_2(ingredients: &[Ingredient], amounts: &[usize], total_calories: i64) -> i64 {
    macro_rules! sum_with_amounts {
        ($ings:ident, $amounts:ident, $field:ident) => {
            $ings
                .iter()
                .map(|x| x.$field)
                .zip($amounts)
                .map(|(cap, n)| cap * (*n as i64))
                .sum::<i64>()
                .max(0)
        };
    }
    let capacity = sum_with_amounts!(ingredients, amounts, capacity);
    let durability = sum_with_amounts!(ingredients, amounts, durability);
    let flavor = sum_with_amounts!(ingredients, amounts, flavor);
    let texture = sum_with_amounts!(ingredients, amounts, texture);
    let calories = sum_with_amounts!(ingredients, amounts, calories);

    if calories != total_calories {
        return 0;
    }

    capacity * durability * flavor * texture
}

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

// \w+: capacity -?\d+, durability \d+, flavor -?\d+, texture -?\d+, calories \d+
fn parse_ingredient(input: &str) -> nom::IResult<&str, Ingredient> {
    let (input, (_, _, capacity, _, durability, _, flavor, _, texture, _, calories)) = (
        nom::character::complete::alpha1,
        nom::bytes::complete::tag(": capacity "),
        nom::character::complete::i64,
        nom::bytes::complete::tag(", durability "),
        nom::character::complete::i64,
        nom::bytes::complete::tag(", flavor "),
        nom::character::complete::i64,
        nom::bytes::complete::tag(", texture "),
        nom::character::complete::i64,
        nom::bytes::complete::tag(", calories "),
        nom::character::complete::i64,
    )
        .parse(input)?;

    Ok((
        input,
        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    ))
}
fn parse_ingredients(input: &str) -> Result<Vec<Ingredient>, nom::Err<nom::error::Error<&str>>> {
    let (_, ingredients) =
        nom::multi::separated_list1(nom::character::complete::newline, parse_ingredient)
            .parse(input)?;

    Ok(ingredients)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(solve_part_1(EXAMPLE), 62842880);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "222870");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "57600000");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "117936");
    }
}
