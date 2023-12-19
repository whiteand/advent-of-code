use std::{cmp, collections::BTreeMap, str::FromStr};

use nom::{error::ErrorKind, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl From<&Field> for char {
    fn from(value: &Field) -> Self {
        match value {
            Field::X => 'x',
            Field::M => 'm',
            Field::A => 'a',
            Field::S => 's',
        }
    }
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("invalid field"),
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
struct WorkflowId(usize);

impl std::ops::Deref for WorkflowId {
    type Target = usize;
    fn deref(&self) -> &usize {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Accepted,
    Rejected,
    Workflow(WorkflowId),
}

impl FromStr for WorkflowId {
    type Err = ();
    fn from_str(s: &str) -> Result<WorkflowId, ()> {
        let id = s
            .bytes()
            .fold(0usize, |acc, b| acc * 26 + (b - b'a') as usize);

        Ok(Self(id))
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            name => Self::Workflow(name.parse::<WorkflowId>().unwrap()),
        })
    }
}

#[derive(Clone)]
enum Condition {
    Any,
    FieldBased {
        field: Field,
        order: cmp::Ordering,
        value: usize,
    },
    Never,
    Or(Box<Condition>, Box<Condition>),
    And(Box<Condition>, Box<Condition>),
}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "true"),
            Self::FieldBased {
                field,
                order,
                value,
            } => {
                write!(
                    f,
                    "{}{}{}",
                    char::from(field),
                    match order {
                        cmp::Ordering::Less => "<",
                        cmp::Ordering::Equal => "=",
                        cmp::Ordering::Greater => ">",
                    },
                    value
                )
            }
            Self::Never => write!(f, "false"),
            Self::Or(arg0, arg1) => {
                write!(f, "({:?}) || ({:?})", arg0, arg1)
            }
            Self::And(arg0, arg1) => write!(f, "({:?}) && ({:?})", arg0, arg1),
        }
    }
}
impl Condition {
    fn negate(&self) -> Self {
        match self {
            Self::Any => Self::Never,
            Self::Never => Self::Any,
            Self::FieldBased {
                field,
                order,
                value,
            } => match order {
                cmp::Ordering::Less => Self::FieldBased {
                    field: *field,
                    order: cmp::Ordering::Greater,
                    value: value - 1,
                },
                cmp::Ordering::Equal => unreachable!(),
                cmp::Ordering::Greater => Self::FieldBased {
                    field: *field,
                    order: cmp::Ordering::Less,
                    value: value + 1,
                },
            },
            Self::Or(a, b) => Self::And(a.negate().into(), b.negate().into()),
            Self::And(a, b) => Self::Or(a.negate().into(), b.negate().into()),
        }
    }

    fn or(&self, other: &Self) -> Self {
        match (self, other) {
            (Condition::Any, _) => Condition::Any,
            (_, Condition::Any) => Condition::Any,
            (Condition::Never, x) => x.clone(),
            (x, Condition::Never) => x.clone(),
            (
                Condition::FieldBased {
                    field: field_a,
                    order: order_a,
                    value: value_a,
                },
                Condition::FieldBased {
                    field: field_b,
                    order: order_b,
                    value: value_b,
                },
            ) => {
                if field_a != field_b {
                    return Condition::Or(self.clone().into(), other.clone().into());
                }
                if order_a != order_b {
                    return Condition::Or(self.clone().into(), other.clone().into());
                }
                todo!(
                    "merge conditions {:?} {:?} ({:?}) and ({:?})",
                    field_a,
                    order_a,
                    value_a,
                    value_b
                );
            }
            (Condition::And(a, b), Condition::And(c, d)) => Condition::Or(
                Condition::And(a.clone().into(), b.clone().into()).into(),
                Condition::And(c.clone().into(), d.clone().into()).into(),
            ),
            (Condition::Or(a, b), cond) => Condition::Or(
                Condition::Or(a.clone().into(), cond.clone().into()).into(),
                b.clone().into(),
            ),
            (a, b) => {
                todo!("Not implemented or for\n  {:?}\nand\n  {:?}\n", self, other)
            }
        }
    }

    fn and(&self, other: &Self) -> Self {
        match (self, other) {
            (Condition::Any, x) => x.clone(),
            (_, Condition::Never) => Condition::Never,
            (
                Condition::FieldBased {
                    field: field_a,
                    order: order_a,
                    value: value_a,
                },
                Condition::FieldBased {
                    field: field_b,
                    order: order_b,
                    value: value_b,
                },
            ) => {
                if field_a != field_b {
                    return Condition::And(self.clone().into(), other.clone().into());
                }
                match (order_a, order_b) {
                    (cmp::Ordering::Less, cmp::Ordering::Less) => todo!(
                        "Not implemented and for {:?} and {:?}",
                        (field_a, order_a, value_a),
                        (field_b, order_b, value_b)
                    ),
                    (cmp::Ordering::Less, cmp::Ordering::Greater) => todo!(
                        "Not implemented and for {:?} and {:?}",
                        (field_a, order_a, value_a),
                        (field_b, order_b, value_b)
                    ),
                    (cmp::Ordering::Greater, cmp::Ordering::Less) => todo!(
                        "Not implemented and for {:?} and {:?}",
                        (field_a, order_a, value_a),
                        (field_b, order_b, value_b)
                    ),
                    (cmp::Ordering::Greater, cmp::Ordering::Greater) => Condition::FieldBased {
                        field: *field_a,
                        order: *order_a,
                        value: cmp::max(*value_a, *value_b),
                    },
                    (_, cmp::Ordering::Equal) | (cmp::Ordering::Equal, _) => unreachable!(),
                }
            }
            (cond, Condition::Or(a, b)) => cond.and(a).or(&cond.and(b)),
            (Condition::Or(a, b), cond) => cond.and(a).or(&cond.and(b)),
            (Condition::And(a, b), cond) => cond.and(a).and(&cond.and(b)),
            (cond, Condition::And(a, b)) => cond.and(a).and(&cond.and(b)),
            (a, b) => {
                todo!("Not implemented and for {:?} and {:?}", self, other)
            }
        }
    }

    fn matches(&self, obj: &[usize; 4]) -> bool {
        match self {
            Condition::Any => true,
            Condition::FieldBased {
                field,
                order,
                value,
            } => {
                let obj_value = obj[*field as usize];
                let actual_cmp = obj_value.cmp(value);
                actual_cmp == *order
            }
            Condition::Never => false,
            Condition::Or(a, b) => a.matches(obj) || b.matches(obj),
            Condition::And(a, b) => a.matches(obj) && b.matches(obj),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Condition,
    outcome: Outcome,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}
impl Workflow {
    fn outcome(&self, obj: &[usize; 4]) -> Outcome {
        for Rule { condition, outcome } in &self.rules {
            if condition.matches(&obj) {
                return *outcome;
            }
        }
        unreachable!("no default rule")
    }
}

fn parse_input(input: &str) -> (BTreeMap<WorkflowId, Workflow>, Vec<[usize; 4]>) {
    let (workflows_str, objects_str) = input.split_once("\n\n").unwrap();
    let mut workflows = BTreeMap::new();
    let mut objects = Vec::new();

    for line in workflows_str.lines() {
        let (name, rules) = line.split_once("{").unwrap();
        let rules_str = &rules[..rules.len() - 1];
        let mut rules = Vec::new();

        let id = name.parse::<WorkflowId>().unwrap();

        'rule_part: for rule_part in rules_str.split(',') {
            let (condition_str, outcome_str) = match rule_part.split_once(":") {
                Some(p) => p,
                None => {
                    let outcome = rule_part.parse::<Outcome>().unwrap();

                    rules.push(Rule {
                        condition: Condition::Any,
                        outcome,
                    });

                    continue 'rule_part;
                }
            };

            let outcome = outcome_str.parse::<Outcome>().unwrap();

            let (input, f) = nom::character::complete::one_of::<_, _, (&str, ErrorKind)>("xmas")
                .map(|c| Field::from(c))
                .parse(condition_str)
                .unwrap();
            let (input, o) = nom::character::complete::one_of::<_, _, (&str, ErrorKind)>("><")
                .map(|c| match c {
                    '>' => cmp::Ordering::Greater,
                    '<' => cmp::Ordering::Less,
                    _ => panic!("invalid ordering"),
                })
                .parse(input)
                .unwrap();
            let (input, v) = nom::character::complete::digit1::<_, (&str, ErrorKind)>
                .map(|s: &str| s.parse::<usize>().expect("invalid number"))
                .parse(input)
                .unwrap();

            assert_eq!(input, "");

            rules.push(Rule {
                condition: Condition::FieldBased {
                    field: f,
                    order: o,
                    value: v,
                },
                outcome,
            })
        }

        workflows.insert(id, Workflow { rules });
    }

    for line in objects_str.lines() {
        let line = &line[1..line.len() - 1];
        let mut fields = [0; 4];
        for f_str in line.split(',') {
            let (field_str, num_str) = f_str.split_once('=').unwrap();
            let field = Field::from(field_str.chars().next().unwrap());
            let num = num_str.parse::<usize>().unwrap();

            fields[field as usize] = num;
        }
        objects.push(fields);
    }

    (workflows, objects)
}

pub fn solve_part_1(file_content: &str) -> usize {
    let (workflows, objects) = parse_input(file_content);
    let mut accepted_total = 0usize;
    'obj: for obj in objects {
        let mut workflow_id = WorkflowId::from_str("in").unwrap();
        'workflow: loop {
            let workflow = workflows.get(&workflow_id).unwrap();
            let outcome = workflow.outcome(&obj);
            match outcome {
                Outcome::Accepted => {
                    accepted_total += obj.iter().sum::<usize>();
                    continue 'obj;
                }
                Outcome::Rejected => {
                    continue 'obj;
                }
                Outcome::Workflow(id) => {
                    workflow_id = id;
                    continue 'workflow;
                }
            }
        }
    }
    accepted_total
}

fn get_accepted_condition_from_workflow(
    workflows: &BTreeMap<WorkflowId, Workflow>,
    workflow_id: WorkflowId,
) -> Condition {
    let mut when_accepted = Condition::Never;

    let workflow = workflows.get(&workflow_id).unwrap();

    let mut when_previous_rule_is_not_applied = Condition::Any;

    for rule in workflow.rules.iter() {
        match rule.outcome {
            Outcome::Accepted => {
                when_accepted =
                    when_accepted.or(&when_previous_rule_is_not_applied.and(&rule.condition));
                when_previous_rule_is_not_applied =
                    when_previous_rule_is_not_applied.and(&rule.condition.negate())
            }
            Outcome::Rejected => {
                when_previous_rule_is_not_applied =
                    when_previous_rule_is_not_applied.and(&rule.condition.negate())
            }
            Outcome::Workflow(w_id) => {
                when_accepted = when_accepted.or(&when_previous_rule_is_not_applied.and(
                    &rule
                        .condition
                        .and(&get_accepted_condition_from_workflow(workflows, w_id)),
                ));
                when_previous_rule_is_not_applied =
                    when_previous_rule_is_not_applied.and(&rule.condition.negate())
            }
        }
    }

    when_accepted
}

pub fn solve_part_2(file_content: &str) -> usize {
    let (workflows, _) = parse_input(file_content);

    let start_id = WorkflowId::from_str("in").unwrap();

    let accepted_condition = get_accepted_condition_from_workflow(&workflows, start_id);

    dbg!(accepted_condition);

    0
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "19114");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "406934");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
