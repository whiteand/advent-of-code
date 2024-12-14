use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    parse_usize,
    sequence::{preceded, separated_pair, tuple},
    Parser,
};
use itertools::Either;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let mut executor = parse_executor(file_content);

    executor
        .find_map(|x| match x {
            Either::Left(bot_state) if bot_state.contains(61) && bot_state.contains(17) => {
                return Some(bot_state.id)
            }
            _ => None,
        })
        .expect("should be valid")
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let executor = parse_executor(file_content);

    const N: usize = 3;
    let mut has = [0; N];
    let mut remaining = N;
    for OutputState { id, value } in executor.filter_map(|x| match x {
        Either::Right(x) if x.id < N => Some(x),
        _ => None,
    }) {
        let output_ref = has.get_mut(id).unwrap();
        if *output_ref == 0 {
            remaining -= 1;
        }
        *output_ref = value;
        if remaining == 0 {
            return has.into_iter().product();
        }
    }
    unreachable!()
}

struct Executor {
    bot_states: Vec<BotState>,
    dirty: Vec<bool>,
    dirty_bots: Vec<usize>,
    current_bots: Vec<usize>,
    pending_actions: Vec<Action>,
    bot_instructions: Vec<BotInstruction>,
}

impl Executor {
    fn new(bot_instructions: Vec<BotInstruction>) -> Self {
        Self {
            current_bots: Vec::with_capacity(bot_instructions.len()),
            pending_actions: Vec::with_capacity(bot_instructions.len() * 2),
            dirty: vec![false; bot_instructions.len()],
            dirty_bots: Vec::with_capacity(bot_instructions.len()),
            bot_states: (0..bot_instructions.len())
                .map(|id| BotState {
                    left: 0,
                    right: 0,
                    id,
                })
                .collect(),
            bot_instructions,
        }
    }
    fn action_to_execute(&mut self) -> Option<Action> {
        loop {
            if let Some(action) = self.pending_actions.pop() {
                return Some(action);
            };

            let id = self.bot_to_perform()?;

            self.pending_actions
                .extend(self.bot_states[id].perform(&self.bot_instructions[id]));
        }
    }

    fn bot_to_perform(&mut self) -> Option<usize> {
        if let Some(bot) = self.current_bots.pop() {
            return Some(bot);
        }
        std::mem::swap(&mut self.current_bots, &mut self.dirty_bots);
        for x in &self.current_bots {
            self.dirty[*x] = false;
        }
        self.current_bots.pop()
    }
    fn execute(&mut self, action: Action) -> Either<BotState, OutputState> {
        match action.target {
            Entity::Bot(bot) => {
                if !self.dirty[bot.id] {
                    self.dirty_bots.push(bot.id);
                    self.dirty[bot.id] = true;
                }
                self.bot_states[bot.id].add_chip(action.chip);
                Either::Left(self.bot_states[bot.id])
            }
            Entity::Output(output_id) => Either::Right(OutputState {
                id: output_id,
                value: action.chip,
            }),
        }
    }

    fn schedule_actions(&mut self, actions: impl Iterator<Item = Action>) {
        self.pending_actions.extend(actions)
    }
}

impl Iterator for Executor {
    type Item = Either<BotState, OutputState>;

    fn next(&mut self) -> Option<Self::Item> {
        self.action_to_execute().map(|action| self.execute(action))
    }
}

fn parse_executor(file_content: &str) -> Executor {
    let mut bot_instructions: Vec<BotInstruction> = Vec::with_capacity(211);
    let mut actions: Vec<Action> = Vec::with_capacity(32);
    for line in file_content.lines() {
        let instruction = parse_instruction(line).map(|(_, x)| x).unwrap();
        match instruction {
            Either::Left(bi) => {
                bot_instructions.push(bi);
            }
            Either::Right(action) => {
                actions.push(action);
            }
        }
    }

    bot_instructions.sort_unstable_by_key(|x| x.id);

    let mut executor = Executor::new(bot_instructions);

    executor.schedule_actions(actions.into_iter());

    executor
}

#[derive(Debug, Copy, Clone)]
struct Action {
    chip: usize,
    target: Entity,
}

#[derive(Debug, Copy, Clone)]
struct BotState {
    left: usize,
    right: usize,
    id: usize,
}

#[derive(Debug, Copy, Clone)]
struct OutputState {
    id: usize,
    value: usize,
}

impl BotState {
    fn add_chip(&mut self, chip: usize) -> Either<(usize, usize), usize> {
        if self.left == 0 {
            self.left = chip;

            return Either::Right(chip);
        }
        if self.right == 0 {
            self.right = chip;
            return Either::Left((self.right, self.left));
        }
        unreachable!("Cannot add chip: {chip} to {self:?}");
    }
    fn contains(&self, chip: usize) -> bool {
        self.left == chip || self.right == chip
    }
    fn perform(&mut self, bot_instruction: &BotInstruction) -> impl Iterator<Item = Action> {
        if self.left != 0 && self.right != 0 {
            let low = self.left.min(self.right);
            let high = self.left.max(self.right);
            let actions = [
                Action {
                    chip: low,
                    target: bot_instruction.low,
                },
                Action {
                    chip: high,
                    target: bot_instruction.high,
                },
            ];
            self.left = 0;
            self.right = 0;

            Either::Left(actions.into_iter())
        } else {
            Either::Right(std::iter::empty())
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    id: usize,
}
#[derive(Debug, Copy, Clone)]
enum Entity {
    Bot(Bot),
    Output(usize),
}
impl From<Bot> for Entity {
    fn from(value: Bot) -> Self {
        Entity::Bot(value)
    }
}

type Instruction = Either<BotInstruction, Action>;

#[derive(Debug)]
struct BotInstruction {
    id: usize,
    low: Entity,
    high: Entity,
}

fn parse_bot(input: &str) -> nom::IResult<&str, Bot> {
    preceded(tag("bot "), parse_usize)
        .map(|id| Bot { id })
        .parse(input)
}
fn parse_entity(input: &str) -> nom::IResult<&str, Entity> {
    alt((
        parse_bot.map(Entity::from),
        preceded(tag("output "), complete::u64.map(|x| x as usize)).map(|id| Entity::Output(id)),
    ))(input)
}
fn parse_bot_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    tuple((
        parse_bot,
        tag(" gives low to "),
        parse_entity,
        tag(" and high to "),
        parse_entity,
    ))
    .map(|(bot, _, low, _, high)| BotInstruction {
        id: bot.id,
        low,
        high,
    })
    .map(Either::Left)
    .parse(input)
}
fn parse_value_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    separated_pair(
        preceded(tag("value "), complete::u64.map(|x| x as usize)),
        tag(" goes to "),
        parse_entity,
    )
    .map(|(chip, target)| Action { chip, target })
    .map(Either::Right)
    .parse(input)
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((parse_bot_instruction, parse_value_instruction))(input)
}

#[cfg(test)]
mod tests {
    use itertools::Either;
    use rstest::rstest;

    use crate::parse_executor;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case(EXAMPLE)]
    #[case(ACTUAL)]
    fn test_parse_instruction(#[case] input: &str) {
        input
            .lines()
            .try_for_each(|x| {
                let inp = super::parse_instruction(x)
                    .map(|(i, _)| i)
                    .map_err(|x| format!("{x}"))?;
                if !inp.is_empty() {
                    return Err(inp.to_string());
                }

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        let mut executor = parse_executor(EXAMPLE);
        let x = executor
            .find_map(|x| match x {
                Either::Left(bot_state) if bot_state.contains(2) && bot_state.contains(5) => {
                    return Some(bot_state.id)
                }
                _ => None,
            })
            .expect("should be valid");
        assert_eq!(x, 2);
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "181");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "30");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "12567");
    }
}
