use std::fmt::Write;

use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    combinator::{all_consuming, value},
    multi::separated_list1,
    parse_usize,
    sequence::{preceded, separated_pair, tuple},
    Parser,
};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use tracing::{info, span, trace, Level};

struct Schema<'t> {
    rules: FxHashMap<&'t str, Rule<'t>>,
}

type Values<'t> = FxHashMap<&'t str, usize>;

impl<'t> Schema<'t> {
    fn calculate_single(
        &self,
        initials: &Values<'t>,
        visited: &mut FxHashSet<&'t str>,
        calculated: &mut Values<'t>,
        var: &'t str,
    ) -> Result<usize, VerificationError> {
        if let Some(x) = initials.get(var) {
            return Ok(*x);
        }
        if let Some(x) = calculated.get(var) {
            return Ok(*x);
        }
        if !visited.insert(var) {
            return Err(VerificationError::Loop);
        }
        let Some(rule) = self.rules.get(var) else {
            panic!("{var} is not defined");
        };
        let a = self.calculate_single(initials, visited, calculated, rule.a)?;
        let b = self.calculate_single(initials, visited, calculated, rule.b)?;
        let res = match rule.op {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        };
        calculated.insert(var, res);
        Ok(res)
    }
    fn calculate_many<'s, 'i>(
        &'s self,
        initials: &'i FxHashMap<&'t str, usize>,
        vars: impl Iterator<Item = &'t str> + 's,
    ) -> impl Iterator<Item = Result<(&'t str, usize), VerificationError>> + 's
    where
        'i: 's,
    {
        let mut calculated = FxHashMap::default();
        vars.into_iter().map(move |var| {
            self.calculate_single(initials, &mut FxHashSet::default(), &mut calculated, var)
                .map(|y| (var, y))
        })
    }
    fn calculate_usize(
        &self,
        initials: &Values,
        bits: &[&'t str],
    ) -> Result<usize, VerificationError> {
        let mut z = 0;
        self.calculate_many(initials, bits.iter().copied())
            .try_for_each(|r| {
                let (bit_name, bit) = r?;
                let offset = bit_name[1..].parse::<usize>().unwrap();
                z |= bit << offset;
                Ok(())
            })?;

        Ok(z)
    }

    fn get_all_dependents(&self, bit_names: impl Iterator<Item = &'t str>) -> Vec<&'t str> {
        let mut visited = FxHashSet::default();
        let mut children = Vec::new();
        for child in bit_names.flat_map(|x| self.get_dependents(x)) {
            if visited.insert(child) {
                children.push(child);
            }
        }
        children.reverse();

        let mut res = vec![];
        while let Some(child) = children.pop() {
            res.push(child);
            for x in self.get_dependents(child) {
                if visited.insert(x) {
                    children.push(x);
                }
            }
        }
        res
    }

    fn get_dependents<'u>(&'u self, bit_name: &'t str) -> Vec<&'t str> {
        self.rules
            .values()
            .filter_map(move |rule| {
                if rule.a == bit_name {
                    return Some(rule.dst);
                }
                if rule.b == bit_name {
                    return Some(rule.dst);
                }

                None
            })
            .collect()
    }

    fn swap_rules(&mut self, x: &'t str, y: &'t str) -> bool {
        if x == y {
            return false;
        }
        let mut rule_x = self.rules.remove(x).unwrap_or_else(|| {
            panic!("Failed to find {x} rule");
        });
        let mut rule_y = self.rules.remove(y).unwrap_or_else(|| {
            panic!("Failed to find {y} rule");
        });
        rule_x.dst = y;
        rule_y.dst = x;
        self.rules.insert(x, rule_y);
        self.rules.insert(y, rule_x);
        true
    }

    fn get_deps(&self, var_names: impl Iterator<Item = &'t str>) -> Vec<&'t str> {
        let mut to_visit = var_names.into_iter().unique().collect_vec();

        let mut res = Vec::new();

        while let Some(x) = to_visit.pop() {
            let Some(rule) = self.rules.get(x) else {
                continue;
            };

            if !res.contains(&rule.a) {
                res.push(rule.a);
                to_visit.push(rule.a);
            }
            if !res.contains(&rule.b) {
                res.push(rule.b);
                to_visit.push(rule.b);
            }
        }

        res.reverse();

        res
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let (_, _, zs, initials, schema) = parse_input(file_content);

    schema.calculate_usize(&initials, &zs).unwrap()
}

// #[derive(Debug)]
// struct Adder<'t> {
//     xs: Vec<&'t str>,
//     ys: Vec<&'t str>,
//     zs: Vec<&'t str>,
//     temp_sum: Vec<&'t str>,
//     cs: Vec<&'t str>,
// }

// fn find_rule_with_params<'t, 'u>(
//     values: &'u ValueMap<'t>,
//     a: &'t str,
//     b: &'t str,
//     op: Operation,
// ) -> impl Iterator<Item = Rule<'t>> + 'u
// where
//     'u: 't,
// {
//     values
//         .values()
//         .filter_map(|value| value.rule())
//         .filter(move |rule| rule.depends_on(a) && rule.depends_on(b) && rule.op == op)
// }
// fn find_one_rule_with_params<'t, 'u>(
//     values: &'u ValueMap<'t>,
//     a: &'t str,
//     b: &'t str,
//     op: Operation,
// ) -> Result<Rule<'t>, ()>
// where
//     'u: 't,
// {
//     find_rule_with_params(values, a, b, op).next().ok_or(())
// }
// fn find_rule_with_param_and_op<'t, 'u>(
//     values: &'u ValueMap<'t>,
//     a: &'t str,
//     op: Operation,
// ) -> impl Iterator<Item = Rule<'t>> + 'u
// where
//     'u: 't,
// {
//     values
//         .values()
//         .filter_map(|value| value.rule())
//         .filter(move |rule| rule.depends_on(a) && rule.op == op)
// }

// impl<'t> Adder<'t> {
//     fn first_bit<'u>(values: &'u ValueMap<'t>, x_bit: &'t str, y_bit: &'t str) -> Adder<'t>
//     where
//         'u: 't,
//     {
//         assert!(values.contains_key(x_bit));
//         assert!(values.contains_key(y_bit));

//         let Some(z0) = find_rule_with_params(values, x_bit, y_bit, Operation::Xor).next() else {
//             panic!("Failed to find z0");
//         };
//         let Some(c0) = find_rule_with_params(values, x_bit, y_bit, Operation::And).next() else {
//             panic!("Failed to find c0")
//         };
//         Adder {
//             xs: vec![x_bit],
//             ys: vec![y_bit],
//             cs: vec![c0.dst],
//             temp_sum: vec![z0.dst],
//             zs: vec![z0.dst],
//         }
//     }
//     fn try_add_bit<'u>(
//         &mut self,
//         values: &'u ValueMap<'t>,
//         x_bit: &'t str,
//         y_bit: &'t str,
//     ) -> Result<(), ()>
//     where
//         'u: 't,
//     {
//         let _span = span!(Level::INFO, "try_add", ?x_bit, ?y_bit).entered();
//         info!(?self);
//         if self.try_add_1(values, x_bit, y_bit).is_ok() {
//             return Ok(());
//         }

//         if self.try_add_2(values, x_bit, y_bit).is_ok() {
//             return Ok(());
//         }
//         if self.try_add_3(values, x_bit, y_bit).is_ok() {
//             return Ok(());
//         }

//         Err(())
//     }
//     fn try_add_1<'u>(
//         &mut self,
//         values: &'u ValueMap<'t>,
//         x_bit: &'t str,
//         y_bit: &'t str,
//     ) -> Result<(), ()>
//     where
//         'u: 't,
//     {
//         assert!(values.contains_key(x_bit));
//         assert!(values.contains_key(y_bit));
//         let last_carry = self.cs.last().copied().unwrap();
//         trace!(last_carry = ?values.get(last_carry).and_then(|x| x.rule()).unwrap());
//         let temp_sum = find_one_rule_with_params(values, x_bit, y_bit, Operation::Xor)?;
//         trace!(?temp_sum);
//         let new_sum = find_one_rule_with_params(values, last_carry, temp_sum.dst, Operation::Xor)?;
//         trace!(?new_sum);
//         let new_carry =
//             find_one_rule_with_params(values, last_carry, temp_sum.dst, Operation::And)?;
//         trace!(?new_carry);
//         self.xs.push(x_bit);
//         self.ys.push(y_bit);
//         self.cs.push(new_carry.dst);
//         self.temp_sum.push(temp_sum.dst);
//         self.zs.push(new_sum.dst);
//         Ok(())
//     }
//     fn try_add_2<'u>(
//         &mut self,
//         values: &'u ValueMap<'t>,
//         x_bit: &'t str,
//         y_bit: &'t str,
//     ) -> Result<(), ()>
//     where
//         'u: 't,
//     {
//         let _span = span!(Level::INFO, "try_add_2").entered();
//         assert!(values.contains_key(x_bit));
//         assert!(values.contains_key(y_bit));
//         if self.xs.len() <= 1 {
//             return Err(());
//         }
//         let prepre_x = self.xs.iter().rev().nth(1).copied().unwrap();
//         let prepre_y = self.ys.iter().rev().nth(1).copied().unwrap();
//         let pre_x = self.xs.last().copied().unwrap();
//         let pre_y = self.ys.last().copied().unwrap();
//         trace!(?prepre_x, ?prepre_y, ?pre_x, ?pre_y);
//         let overflow_self_prepre =
//             find_one_rule_with_params(values, &prepre_x, &prepre_y, Operation::And)?;
//         trace!(?overflow_self_prepre);
//         let overflow_self_pre = find_one_rule_with_params(values, &pre_x, &pre_y, Operation::And)?;
//         trace!(?overflow_self_pre);

//         let rem_self_pre = find_one_rule_with_params(values, &pre_x, &pre_y, Operation::Xor)?;
//         trace!(?rem_self_pre);
//         let rem_self = find_one_rule_with_params(values, &x_bit, &y_bit, Operation::Xor)?;
//         trace!(?rem_self);
//         let zero_carry_pre = find_one_rule_with_params(
//             values,
//             &overflow_self_prepre.dst,
//             &rem_self_pre.dst,
//             Operation::And,
//         )?;
//         trace!(?zero_carry_pre);
//         trace!(?overflow_self_pre, ?zero_carry_pre);
//         let prev_carry = find_one_rule_with_params(
//             values,
//             &overflow_self_pre.dst,
//             &zero_carry_pre.dst,
//             Operation::Or,
//         )?;
//         trace!(?prev_carry);
//         let new_sum =
//             find_one_rule_with_params(values, &prev_carry.dst, &rem_self.dst, Operation::Xor)?;
//         trace!(?new_sum);
//         let new_carry =
//             find_one_rule_with_params(values, &prev_carry.dst, &rem_self.dst, Operation::And)?;
//         trace!(?new_carry);

//         self.xs.push(x_bit);
//         self.ys.push(y_bit);
//         self.cs.push(new_carry.dst);
//         self.temp_sum.push(rem_self.dst);
//         self.zs.push(new_sum.dst);
//         Ok(())
//     }
//     fn try_add_3<'u>(
//         &mut self,
//         values: &'u ValueMap<'t>,
//         x_bit: &'t str,
//         y_bit: &'t str,
//     ) -> Result<(), ()>
//     where
//         'u: 't,
//     {
//         let _span = span!(Level::INFO, "try_add_3").entered();
//         assert!(values.contains_key(x_bit));
//         assert!(values.contains_key(y_bit));
//         if self.xs.len() < 3 {
//             return Err(());
//         }

//         // y00 AND x00 -> overflow_self_pre_pre_pre (ktt)
//         // x01 AND y01 -> overflow_self_pre_pre (kgp)
//         // x01 XOR y01 -> rem_self_pre_pre (rvb);
//         // y02 AND x02 -> overflow_self_pre (kwm)
//         // y02 XOR x02 -> rem_self_pre (ssq)
//         // y03 XOR x03 -> rem_self (fbk)

//         // overflow_self_pre_pre_pre AND rem_self_pre_pre -> carry_after_pre_pre (kmb)
//         // overflow_self_pre_pre OR carry_after_pre_pre -> (carry_pre) rkn
//         // carry_pre AND rem_self_pre -> zero_cary_pre (vsc)
//         // overflow_self_pre OR zero_cary_pre -> carry (ntj)
//         // rem_self XOR carry -> next_res
//         // carry AND rem_self -> next_carry

//         let (pre_x, prepre_x, preprepre_x) =
//             self.xs.iter().rev().take(3).collect_tuple().ok_or(())?;
//         let (pre_y, prepre_y, preprepre_y) =
//             self.ys.iter().rev().take(3).collect_tuple().ok_or(())?;

//         let overflow_self_pre_pre_pre =
//             find_one_rule_with_params(values, preprepre_x, preprepre_y, Operation::And)?;
//         info!(?overflow_self_pre_pre_pre);
//         let overflow_self_pre_pre =
//             find_one_rule_with_params(values, prepre_x, prepre_y, Operation::And)?;
//         info!(?overflow_self_pre_pre);
//         let rem_self_pre_pre =
//             find_one_rule_with_params(values, prepre_x, prepre_y, Operation::Xor)?;
//         info!(?rem_self_pre_pre);
//         let overflow_self_pre = find_one_rule_with_params(values, pre_x, pre_y, Operation::And)?;
//         info!(?overflow_self_pre);
//         let rem_self_pre = find_one_rule_with_params(values, pre_x, pre_y, Operation::Xor)?;
//         info!(?rem_self_pre);
//         let rem_self = find_one_rule_with_params(values, x_bit, y_bit, Operation::Xor)?;
//         info!(?rem_self);
//         let carry_after_pre_pre = find_one_rule_with_params(
//             values,
//             overflow_self_pre_pre_pre.dst,
//             rem_self_pre_pre.dst,
//             Operation::And,
//         )?;
//         info!(?carry_after_pre_pre);
//         let carry_pre = find_one_rule_with_params(
//             values,
//             overflow_self_pre_pre.dst,
//             carry_after_pre_pre.dst,
//             Operation::Or,
//         )?;
//         info!(?carry_pre);
//         let zero_cary_pre =
//             find_one_rule_with_params(values, carry_pre.dst, rem_self_pre.dst, Operation::And)?;

//         info!(?zero_cary_pre);
//         let carry = find_one_rule_with_params(
//             values,
//             overflow_self_pre.dst,
//             zero_cary_pre.dst,
//             Operation::Or,
//         )?;
//         info!(?carry);

//         let new_sum = find_one_rule_with_params(values, rem_self.dst, carry.dst, Operation::Xor)?;
//         info!(?new_sum);
//         let new_carry = find_one_rule_with_params(values, carry.dst, rem_self.dst, Operation::And)?;
//         info!(?new_carry);

//         self.xs.push(x_bit);
//         self.ys.push(y_bit);
//         self.cs.push(new_carry.dst);
//         self.temp_sum.push(rem_self.dst);
//         self.zs.push(new_sum.dst);
//         Ok(())
//     }
// }

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> String {
    let (xs, ys, zs, initials, mut schema) = parse_input(file_content);
    let x = schema.calculate_usize(&initials, &xs).expect("valid");
    let y = schema.calculate_usize(&initials, &ys).expect("valid");
    let z = schema.calculate_usize(&initials, &zs).expect("valid");
    let expected = x + y;
    info!(
        x = format!("{x:b}"),
        y = format!("{y:b}"),
        z = format!("{z:b}"),
        ez = format!("{expected:b}")
    );

    let mut swapped = FxHashSet::default();

    let swaps = fix_adder(&xs, &ys, &zs, &mut schema, zs.len(), &mut swapped)
        .expect("not failed to fix adder");

    swaps.into_iter().flatten().sorted().join(",")
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Swap<'t> {
    a: &'t str,
    b: &'t str,
}
impl<'t> Swap<'t> {
    pub fn new(a: &'t str, b: &'t str) -> Swap<'t> {
        if a.gt(b) {
            Self { b: a, a: b }
        } else {
            Self { a, b }
        }
    }
}
impl<'t> IntoIterator for Swap<'t> {
    type Item = &'t str;

    type IntoIter = std::array::IntoIter<&'t str, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.a, self.b].into_iter()
    }
}

fn check_bit_adder<'t>(
    xs: &[&'t str],
    ys: &[&'t str],
    zs: &[&'t str],
    schema: &mut Schema<'t>,
) -> Result<(), ()> {
    test_adder(schema, xs, ys, zs, 1, 0, 0, 0).map_err(|_| ())?;
    test_adder(schema, xs, ys, zs, 1, 0, 1, 1).map_err(|_| ())?;
    test_adder(schema, xs, ys, zs, 1, 1, 0, 1).map_err(|_| ())?;
    test_adder(schema, xs, ys, zs, 1, 0, 0, 0).map_err(|_| ())?;
    Ok(())
}
fn fix_adder<'t>(
    xs: &[&'t str],
    ys: &[&'t str],
    zs: &[&'t str],
    schema: &mut Schema<'t>,
    bits: usize,
    swapped: &mut FxHashSet<&'t str>,
) -> Result<Vec<Swap<'t>>, ()> {
    if bits == 1 {
        check_bit_adder(xs, ys, zs, schema)?;
        return Ok(vec![]);
    }

    let inner_swaps = {
        let res = fix_adder(xs, ys, zs, schema, bits - 1, swapped)?;
        res
    };

    if check_next_adder(xs, ys, zs, schema, bits).is_ok() {
        return Ok(inner_swaps);
    }

    if swapped.len() == 8 {
        return Err(());
    }

    let others =
        schema.get_all_dependents(xs[bits..].iter().copied().chain(ys[bits..].iter().copied()));

    let options = schema
        .get_all_dependents(
            xs[0..bits]
                .iter()
                .copied()
                .chain(ys[0..bits].iter().copied()),
        )
        .into_iter()
        .filter(|x| !swapped.contains(x))
        .filter(|x| !others.contains(x))
        .collect_vec();

    let mut min_possible_swaps: Option<Vec<_>> = None;
    for (a, b) in options.into_iter().tuple_combinations() {
        if !schema.swap_rules(a, b) {
            continue;
        }

        let _span = span!(Level::INFO, "swap", a, b).entered();

        if check_bit_adder(xs, ys, zs, schema).is_err() {
            trace!("fail single bit");
            schema.swap_rules(a, b);
            continue;
        }

        for i in 2..bits {
            if check_next_adder(xs, ys, zs, schema, bits).is_err() {
                trace!(?i, "fail");
                schema.swap_rules(a, b);
                continue;
            }
        }

        swapped.insert(a);
        swapped.insert(b);

        let Ok(mut inner_swaps) = fix_adder(xs, ys, zs, schema, bits, swapped) else {
            trace!("fail");
            swapped.remove(a);
            swapped.remove(b);
            schema.swap_rules(a, b);
            continue;
        };

        info!(?inner_swaps, "success");

        inner_swaps.push(Swap::new(a, b));

        min_possible_swaps = match min_possible_swaps {
            Some(x) if x.len() < inner_swaps.len() => Some(x),
            _ => Some(inner_swaps),
        };

        swapped.remove(a);
        swapped.remove(b);
        schema.swap_rules(a, b);
    }

    min_possible_swaps.ok_or(())
}

fn check_next_adder<'t>(
    xs: &[&'t str],
    ys: &[&'t str],
    zs: &[&'t str],
    schema: &Schema<'t>,
    bits: usize,
) -> Result<(), VerificationError> {
    let max_prev = (1usize << (bits - 1)) - 1;
    let min_current = max_prev + 1;
    let max_current = min_current + max_prev;
    macro_rules! check {
        ($a:expr, $b:expr, $res:expr) => {
            test_adder(schema, xs, ys, zs, bits, $a, $b, $res)
        };
    }

    let mask = usize::MAX & !(max_current);
    check!(1, 1, 2)?;
    check!(max_prev - 1, 1, max_prev)?;
    check!(max_prev, 1, min_current)?;
    check!(min_current, 1, min_current + 1)?;
    check!(max_current - 1, 1, max_current)?;
    check!(max_current, 1, 0)?;
    check!(mask | 1, mask | 1, 2)?;
    check!(mask | (max_prev - 1), mask | 1, max_prev)?;
    check!(mask | max_prev, mask | 1, min_current)?;
    check!(mask | min_current, mask | 1, min_current + 1)?;
    check!(mask | (max_current - 1), mask | 1, max_current)?;
    check!(mask | max_current, mask | 1, 0)?;
    Ok(())
}

fn test_adder<'t>(
    values: &Schema<'t>,
    xs: &[&'t str],
    ys: &[&'t str],
    zs: &[&'t str],
    bits: usize,
    x: usize,
    y: usize,
    expected_z: usize,
) -> Result<(), VerificationError> {
    let mut initials = FxHashMap::default();
    for (i, x_name) in xs.iter().copied().enumerate() {
        initials.insert(x_name, (x >> i) & 0b1);
    }
    for (i, y_name) in ys.iter().copied().enumerate() {
        initials.insert(y_name, (y >> i) & 0b1);
    }

    validate_z(values, &zs[0..bits], &initials, expected_z)
}

#[derive(Debug)]
enum VerificationError {
    Loop,
    InvalidBits(Vec<usize>),
}

fn validate_z<'t>(
    schema: &Schema<'t>,
    zs: &[&'t str],
    initials: &FxHashMap<&'t str, usize>,
    expected: usize,
) -> Result<(), VerificationError> {
    let z = schema.calculate_usize(initials, zs)?;
    let mut invalid = Vec::with_capacity(46);
    for (i, _) in zs.iter().copied().enumerate() {
        let actual_bit = (z >> i) & 0b1;
        let expected_bit = (expected >> i) & 0b1;
        if actual_bit == expected_bit {
            continue;
        }
        invalid.push(i);
    }

    if invalid.is_empty() {
        Ok(())
    } else {
        Err(VerificationError::InvalidBits(invalid))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, PartialEq, Eq)]
struct Rule<'t> {
    op: Operation,
    a: &'t str,
    b: &'t str,
    dst: &'t str,
}

impl<'t> Rule<'t> {
    fn depends_on(&self, x: &'t str) -> bool {
        self.a == x || self.b == x
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operation::And => '&',
            Operation::Or => '|',
            Operation::Xor => '^',
        };
        f.write_char(op)
    }
}
impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl std::fmt::Display for Rule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {} -> {});", self.a, self.op, self.b, self.dst)
    }
}
impl std::fmt::Debug for Rule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse_input(
    input: &str,
) -> (
    Vec<&str>,
    Vec<&str>,
    Vec<&str>,
    FxHashMap<&str, usize>,
    Schema<'_>,
) {
    let (_, (initials, rules)) = parse_initials_and_rules(input.trim()).expect("valid");
    let mut xs = vec![];
    let mut ys = vec![];
    let mut zs = vec![];
    let mut initials_map = FxHashMap::default();
    let mut rules_map = FxHashMap::default();
    for (name, initial) in initials {
        if name.starts_with("x") {
            xs.push(name);
        }
        if name.starts_with("y") {
            ys.push(name);
        }
        if name.starts_with("z") {
            zs.push(name);
        }
        initials_map.insert(name, initial);
    }

    for rule in rules {
        if rule.dst.starts_with("z") {
            zs.push(rule.dst);
        }
        if rule.dst.starts_with("x") {
            xs.push(rule.dst);
        }
        if rule.dst.starts_with("y") {
            ys.push(rule.dst);
        }
        rules_map.insert(rule.dst, rule);
    }
    xs.sort_unstable();
    ys.sort_unstable();
    zs.sort_unstable();

    (xs, ys, zs, initials_map, Schema { rules: rules_map })
}

fn parse_initials_and_rules<'t>(
    input: &str,
) -> nom::IResult<&str, (Vec<(&str, usize)>, Vec<Rule<'_>>)> {
    all_consuming(separated_pair(
        parse_initials,
        tuple((line_ending, line_ending)),
        parse_rules,
    ))(input)
}
fn parse_initials(input: &str) -> nom::IResult<&str, Vec<(&str, usize)>> {
    separated_list1(line_ending, parse_initial)(input)
}
fn parse_initial(input: &str) -> nom::IResult<&str, (&str, usize)> {
    separated_pair(alphanumeric1, tag(": "), parse_usize)(input)
}
fn parse_rules(input: &str) -> nom::IResult<&str, Vec<Rule<'_>>> {
    separated_list1(line_ending, parse_rule)(input)
}
fn parse_rule(input: &str) -> nom::IResult<&str, Rule> {
    tuple((
        alphanumeric1,
        alt((
            value(Operation::Or, tag(" OR ")),
            value(Operation::And, tag(" AND ")),
            value(Operation::Xor, tag(" XOR ")),
        )),
        alphanumeric1,
        preceded(tag(" -> "), alphanumeric1),
    ))
    .map(|(a, op, b, dst)| Rule { op, a, b, dst })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const ACTUAL: &str = include_str!("../input.txt");
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(part1(ACTUAL), 64755511006320);
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(part2(EXAMPLE).as_str(), "");
    }
    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(part2(ACTUAL).as_str(), "");
    }
}
