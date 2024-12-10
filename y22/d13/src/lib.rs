use advent_utils::nom::{
    self, bytes::complete::tag, character::complete, multi::separated_list0, sequence::delimited,
    IResult,
};

use advent_utils::reduces::Reduces;

#[derive(PartialEq, Eq, Clone)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(u32),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::List(left), PacketData::List(right)) => {
                for i in 0..(left.len().min(right.len())) {
                    let left_value = &left[i];
                    let right_value = &right[i];
                    match left_value.cmp(right_value) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }
                left.len().cmp(&right.len())
            }
            (PacketData::Integer(a), PacketData::Integer(b)) => (*a).cmp(b),
            (left, PacketData::Integer(right)) => {
                left.cmp(&PacketData::List(vec![PacketData::Integer(*right)]))
            }
            (PacketData::Integer(left), right) => {
                PacketData::List(vec![PacketData::Integer(*left)]).cmp(right)
            }
        }
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    parse_groups(file_content)
        .enumerate()
        .filter_map(|(ind, group)| {
            if is_greater_last_sorted(group) {
                Some(ind + 1)
            } else {
                None
            }
        })
        .sum()
}

fn is_greater_last_sorted<T: Ord>(list: Vec<T>) -> bool {
    for (left, right) in list.iter().zip(list.iter().skip(1)) {
        if left.gt(right) {
            return false;
        }
    }
    true
}

pub fn solve_part_2(file_content: &str) -> usize {
    let mut packets = file_content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_packet_data(line).unwrap().1)
        .collect::<Vec<_>>();

    let divider_packet_1 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    packets.push(divider_packet_1.clone());

    let divider_packet_2 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);
    packets.push(divider_packet_2.clone());

    packets.sort();

    let first = packets
        .iter()
        .position(|p| p == &divider_packet_1)
        .unwrap_or_default();
    let second = packets
        .iter()
        .position(|p| p == &divider_packet_2)
        .unwrap_or_default();
    (first + 1) * (second + 1)
}

fn parse_groups(file_content: &str) -> impl Iterator<Item = Vec<PacketData>> + '_ {
    file_content.lines().reduces(Vec::new(), |list, line| {
        if line.is_empty() {
            false
        } else {
            let packet_data = parse_packet_data(line).unwrap().1;
            list.push(packet_data);
            true
        }
    })
}

fn parse_packet_data(line: &str) -> IResult<&str, PacketData> {
    let parse_list = nom::combinator::map(
        delimited(
            tag("["),
            separated_list0(tag(","), parse_packet_data),
            tag("]"),
        ),
        PacketData::List,
    );

    let parse_integer = nom::combinator::map(complete::u32, PacketData::Integer);

    nom::branch::alt((parse_integer, parse_list))(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "13");
    }
    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "5003");
    }
    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "140");
    }
    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "20280");
    }
}
