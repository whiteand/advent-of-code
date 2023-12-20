use std::{cmp, collections::BTreeMap, ops::Range, str::FromStr};

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

#[derive(Clone, PartialEq, Eq)]
enum Condition {
    Any,
    Never,
    FieldBased {
        field: Field,
        order: cmp::Ordering,
        value: usize,
    },
}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "true"),
            Self::Never => write!(f, "false"),
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
        }
    }
}
impl Condition {
    fn matches(&self, obj: &[usize; 4]) -> bool {
        match self {
            Condition::Any => true,
            Condition::Never => false,
            Condition::FieldBased {
                field,
                order,
                value,
            } => {
                let obj_value = obj[*field as usize];
                let actual_cmp = obj_value.cmp(value);
                actual_cmp == *order
            }
        }
    }
    fn negation(&self) -> Self {
        match self {
            Condition::Any => Condition::Never,
            Condition::FieldBased {
                field,
                order,
                value,
            } => match order {
                cmp::Ordering::Less => Condition::FieldBased {
                    field: *field,
                    order: cmp::Ordering::Greater,
                    value: *value - 1,
                },
                cmp::Ordering::Equal => unreachable!("We do not have equality condition"),
                cmp::Ordering::Greater => Condition::FieldBased {
                    field: *field,
                    order: cmp::Ordering::Less,
                    value: *value + 1,
                },
            },
            Condition::Never => Condition::Any,
        }
    }
}

#[derive(Clone)]
struct Rule {
    condition: Condition,
    outcome: Outcome,
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.condition {
            Condition::Any => write!(f, "always {:?}", self.outcome),
            Condition::Never => write!(f, "never {:?}", self.outcome),
            Condition::FieldBased {
                field,
                order,
                value,
            } => write!(
                f,
                "{}{}{} -> {:?}",
                char::from(&field),
                match order {
                    cmp::Ordering::Less => "<",
                    cmp::Ordering::Equal => "=",
                    cmp::Ordering::Greater => ">",
                },
                value,
                self.outcome
            ),
        }
    }
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

            let condition = parse_condition(&condition_str);

            rules.push(Rule { condition, outcome })
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

fn parse_condition(input: &str) -> Condition {
    let (input, f) = nom::character::complete::one_of::<_, _, (&str, ErrorKind)>("xmas")
        .map(|c| Field::from(c))
        .parse(input)
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

    Condition::FieldBased {
        field: f,
        order: o,
        value: v,
    }
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

#[derive(Clone, PartialEq, Eq)]
struct Space {
    values: [Range<usize>; 4],
}

impl Space {
    fn all<const MIN: usize, const MAX: usize>() -> Self {
        Self {
            values: [MIN..MAX, MIN..MAX, MIN..MAX, MIN..MAX],
        }
    }
    fn is_empty(&self) -> bool {
        self.values.iter().any(|r| r.start == r.end)
    }

    fn combinations(&self) -> usize {
        self.values.iter().map(|r| r.end - r.start).product()
    }

    fn choose(&self, condition: &Condition) -> Self {
        match condition {
            Condition::Any => self.clone(),
            Condition::Never => Self::all::<0, 0>(),
            Condition::FieldBased {
                field,
                order,
                value,
            } => match order {
                cmp::Ordering::Less => {
                    let mut prev_range = self.values[(*field) as usize].clone();
                    prev_range.end = prev_range.end.min(*value);
                    let mut new_values = self.values.clone();
                    new_values[(*field) as usize] = prev_range;

                    Self { values: new_values }
                }
                cmp::Ordering::Greater => {
                    let mut prev_range = self.values[(*field) as usize].clone();
                    prev_range.start = prev_range.start.max(value + 1);
                    let mut new_values = self.values.clone();
                    new_values[(*field) as usize] = prev_range;

                    Self { values: new_values }
                }
                cmp::Ordering::Equal => unreachable!(),
            },
        }
    }
}

impl std::fmt::Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{x={:?},m={:?},a={:?},s={:?}}}",
            self.values[0], self.values[1], self.values[2], self.values[3]
        )
    }
}

fn get_accepted_combinations_from_outcome(
    workflows: &BTreeMap<WorkflowId, Workflow>,
    outcome: Outcome,
    space: Space,
) -> usize {
    match outcome {
        Outcome::Accepted => space.combinations(),
        Outcome::Rejected => 0,
        Outcome::Workflow(id) => get_accepted_combinations_from_workflow(workflows, id, space),
    }
}

fn get_accepted_combinations_from_workflow(
    workflows: &BTreeMap<WorkflowId, Workflow>,
    id: WorkflowId,
    mut space: Space,
) -> usize {
    println!("Workflow {:?} for {:?}", id, &space);
    let workflow = workflows.get(&id).unwrap();
    let mut total = 0usize;
    for rule in &workflow.rules {
        println!("{:?}  Rule {:?}", id, rule);
        let applied_space = space.choose(&rule.condition);
        println!(
            "    {:?} | {:?} = {:?}",
            &space, &rule.condition, &applied_space
        );
        if applied_space.is_empty() {
            println!("  There is no combinations which satisfies this rule");
            space = space.choose(&rule.condition.negation());
            continue;
        }
        total += get_accepted_combinations_from_outcome(workflows, rule.outcome, applied_space);
        space = space.choose(&rule.condition.negation());
        println!(
            "{:?}  {:?} |! {:?} = {:?}",
            id, &space, &rule.condition, &space
        );
        if space.is_empty() {
            break;
        }
    }
    total
}

pub fn solve_part_2(file_content: &str) -> usize {
    let (workflows, _) = parse_input(file_content);

    let start_id = WorkflowId::from_str("in").unwrap();

    let initial_space = Space::all::<1, 4001>();

    get_accepted_combinations_from_workflow(&workflows, start_id, initial_space)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::parse_condition;

    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "19114");
    }

    #[rstest]
    #[case("x<10", "x>9")]
    #[case("m<10", "m>9")]
    #[case("a<10", "a>9")]
    #[case("s<10", "s>9")]
    #[case("x>9", "x<10")]
    #[case("m>9", "m<10")]
    #[case("a>9", "a<10")]
    #[case("s>9", "s<10")]
    fn test_negation(#[case] condition_str: &str, #[case] expected_condition_str: &str) {
        let condition = parse_condition(&condition_str);
        let expected_condition = parse_condition(&expected_condition_str);

        assert_eq!(condition.negation(), expected_condition);
    }

    #[rstest]
    #[case(Space::all::<1, 4001>(), Condition::Any, Space::all::<1, 4001>())]
    #[case(Space::all::<1, 4001>(), Condition::Never, Space::all::<0, 0>())]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::X, order: cmp::Ordering::Less, value: 10 }, Space { values: [1..10, 1..4001,1..4001,1..4001]})]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::X, order: cmp::Ordering::Greater, value: 10 }, Space { values: [11..4001, 1..4001,1..4001,1..4001]})]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::M, order: cmp::Ordering::Less, value: 10 }, Space { values: [ 1..4001,1..10,1..4001,1..4001]})]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::M, order: cmp::Ordering::Greater, value: 10 }, Space { values: [ 1..4001,11..4001,1..4001,1..4001]})]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::A, order: cmp::Ordering::Less, value: 10 }, Space { values: [ 1..4001,1..4001,1..10,1..4001]})]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::A, order: cmp::Ordering::Greater, value: 10 }, Space { values: [ 1..4001,1..4001,11..4001,1..4001]})]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::S, order: cmp::Ordering::Less, value: 10 }, Space { values: [ 1..4001,1..4001,1..4001,1..10,]})]
    #[case(Space::all::<1, 4001>(), Condition::FieldBased { field: Field::S, order: cmp::Ordering::Greater, value: 10 }, Space { values: [ 1..4001,1..4001,1..4001,11..4001,]})]
    fn test_choose(
        #[case] space: Space,
        #[case] condition: Condition,
        #[case] expected_space: Space,
    ) {
        assert_eq!(space.choose(&condition), expected_space);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "406934");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "167409079868000");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "131192538505367");
    }
}
