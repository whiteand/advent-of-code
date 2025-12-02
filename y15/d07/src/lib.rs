use std::collections::HashMap;

use itertools::Itertools;
use nom::Parser;

pub fn solve_part_1(file_content: &str) -> u16 {
    resolve(&mut parse_wire_maps(file_content), "a")
}

pub fn solve_part_2(file_content: &str) -> u16 {
    let mut wires = parse_wire_maps(file_content);
    let a = resolve(&mut wires.clone(), "a");
    wires.insert(
        "b",
        vec![Wire {
            source: Source::Value(a),
            destination: "b",
        }],
    );

    resolve(&mut wires, "a")
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Source<'i> {
    Value(u16),
    Id(&'i str),
    Not(&'i str),
    Or((&'i str, &'i str)),
    And((&'i str, &'i str)),
    AndConst((&'i str, u16)),
    RShift((&'i str, usize)),
    LShift((&'i str, usize)),
}

#[derive(Debug, Clone)]
struct Wire<'i> {
    source: Source<'i>,
    destination: &'i str,
}

fn resolve<'t>(wires: &mut HashMap<&'t str, Vec<Wire<'t>>>, id: &'t str) -> u16 {
    let wire = wires
        .get_mut(id)
        .and_then(|r| r.first_mut())
        .unwrap_or_else(|| panic!("{id} is absent"))
        .clone();

    let value = match wire.source {
        Source::Value(value) => value,
        Source::Id(value) => resolve(wires, value),
        Source::Not(x) => !resolve(wires, x),
        Source::Or((a, b)) => resolve(wires, a) | resolve(wires, b),
        Source::And((a, b)) => resolve(wires, a) & resolve(wires, b),
        Source::AndConst((a, b)) => resolve(wires, a) & b,
        Source::RShift((a, shift)) => resolve(wires, a) >> shift,
        Source::LShift((a, shift)) => resolve(wires, a) << shift,
    };

    let x = wires.get_mut(id).unwrap();
    x.clear();
    x.push(Wire {
        source: Source::Value(value),
        destination: id,
    });

    value
}

fn parse_wire_maps(input: &str) -> HashMap<&str, Vec<Wire<'_>>> {
    let (rest, wires) = parse_wires(input.trim()).unwrap();
    assert_eq!(rest, "");
    wires.into_iter().into_group_map_by(|x| x.destination)
}

fn parse_and_const_source(input: &str) -> nom::IResult<&str, Source<'_>> {
    let (input, value) = nom::character::complete::u16(input)?;
    let (input, _) = nom::bytes::complete::tag(" AND ").parse(input)?;
    let (input, id) = nom::character::complete::alpha1(input)?;

    Ok((input, Source::AndConst((id, value))))
}

fn parse_wires(input: &str) -> nom::IResult<&str, Vec<Wire<'_>>> {
    let parse_id = || nom::character::complete::alpha1;
    let parse_value_source = || nom::character::complete::u16.map(Source::Value);

    let parse_not_source =
        || nom::sequence::preceded(nom::bytes::complete::tag("NOT "), parse_id()).map(Source::Not);
    let parse_and_source = || {
        nom::sequence::separated_pair(parse_id(), nom::bytes::complete::tag(" AND "), parse_id())
            .map(|(id, id2)| Source::And((id, id2)))
    };
    let parse_or_source = || {
        nom::sequence::separated_pair(parse_id(), nom::bytes::complete::tag(" OR "), parse_id())
            .map(|(id, id2)| Source::Or((id, id2)))
    };

    let parse_lshift_source = || {
        nom::sequence::separated_pair(
            parse_id(),
            nom::bytes::complete::tag(" LSHIFT "),
            nom::character::complete::u8,
        )
        .map(|(id, shift)| Source::LShift((id, shift as usize)))
    };
    let parse_rshift_source = || {
        nom::sequence::separated_pair(
            parse_id(),
            nom::bytes::complete::tag(" RSHIFT "),
            nom::character::complete::u8,
        )
        .map(|(id, shift)| Source::RShift((id, shift as usize)))
    };

    let parse_source = || {
        nom::branch::alt((
            parse_not_source(),
            parse_and_source(),
            parse_and_const_source,
            parse_or_source(),
            parse_lshift_source(),
            parse_rshift_source(),
            parse_value_source(),
            parse_id().map(Source::Id),
        ))
    };
    let parse_wire = || {
        nom::sequence::separated_pair(
            parse_source(),
            nom::bytes::complete::tag(" -> "),
            parse_id(),
        )
        .map(|(s, d)| Wire {
            source: s,
            destination: d,
        })
    };
    let mut parse_wires = nom::sequence::terminated(
        nom::multi::separated_list1(nom::character::complete::newline, parse_wire()),
        nom::combinator::eof,
    );

    parse_wires.parse(input)
}
#[cfg(test)]
mod tests {
    use crate::Source;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_parse_and_const() {
        assert_eq!(
            super::parse_and_const_source("1 AND fi").unwrap(),
            ("", Source::AndConst(("fi", 1)))
        );
    }
    #[test]
    fn test_parse() {
        let text = "NOT di -> dj\n1 AND fi -> fj\nkf LSHIFT 15 -> kj";
        for line in text.lines() {
            let _ = super::parse_wires(line).unwrap();
        }
    }

    #[test]
    fn test_part1() {
        let mut wires = super::parse_wire_maps(EXAMPLE);
        assert_eq!(super::resolve(&mut wires, "d"), 72);
        assert_eq!(super::resolve(&mut wires, "e"), 507);
        assert_eq!(super::resolve(&mut wires, "f"), 492);
        assert_eq!(super::resolve(&mut wires, "g"), 114);
        assert_eq!(super::resolve(&mut wires, "h"), 65412);
        assert_eq!(super::resolve(&mut wires, "i"), 65079);
        assert_eq!(super::resolve(&mut wires, "x"), 123);
        assert_eq!(super::resolve(&mut wires, "y"), 456);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "3176");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "14710");
    }
}
