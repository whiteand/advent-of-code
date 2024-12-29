use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    combinator::{all_consuming, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Parser,
};
use tracing::info;

type Int = i64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operand {
    Integer(Int),
    Register(Register),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    /// cpy x y copies x (either an integer or the value of a register) into register y.
    Cpy(Operand, Register),
    /// inc x increases the value of register x by one.
    Inc(Register),
    Dec(Register),
    Tgl(Operand),
    Jnz(Operand, Operand),
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
struct Cpu {
    a: Int,
    b: Int,
    c: Int,
    d: Int,
    ip: usize,
}
impl Cpu {
    fn execute(&mut self, program: &mut [Instruction]) {
        while let Some(command) = program.get(self.ip).copied() {
            match command {
                Instruction::Cpy(operand, register) => {
                    let value = self.get_operand_value(operand);
                    *self.register_mut(register) = value;
                    self.ip += 1;
                }
                Instruction::Inc(register) => {
                    *self.register_mut(register) += 1;
                    self.ip += 1;
                }
                Instruction::Dec(register) => {
                    *self.register_mut(register) -= 1;
                    self.ip += 1;
                }
                Instruction::Tgl(op) => {
                    let value = self.get_operand_value(op);
                    if value != 0 {
                        let ptr = (self.ip as Int) + value;
                        toggle_instruction(program, ptr as usize);
                    }
                    self.ip += 1;
                }
                Instruction::Jnz(operand, jump) => {
                    let value = self.get_operand_value(operand);
                    info!(ip = ?self.ip, a=?self.a,b=?self.b,c=?self.c,d=self.d,?command, ?value);

                    if value == 0 {
                        self.ip += 1;
                    } else {
                        let jump = self.get_operand_value(jump);
                        self.ip = ((self.ip as Int) + jump) as usize;
                    }
                }
            }
        }
    }
    fn get_operand_value(&self, operand: Operand) -> Int {
        match operand {
            Operand::Integer(value) => value,
            Operand::Register(reg) => *self.register(reg),
        }
    }
    fn register(&self, register: Register) -> &Int {
        match register {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
            Register::D => &self.d,
        }
    }
    fn register_mut(&mut self, register: Register) -> &mut Int {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }
}

fn toggle_instruction(program: &mut [Instruction], n: usize) {
    if let Some(new_instruction) = program.get(n).copied().and_then(toggled) {
        program[n] = new_instruction
    }
}
fn toggled(instruction: Instruction) -> Option<Instruction> {
    match instruction {
        Instruction::Cpy(operand, register) => {
            Some(Instruction::Jnz(operand, Operand::Register(register)))
        }
        Instruction::Inc(register) => Some(Instruction::Dec(register)),
        Instruction::Dec(register) => Some(Instruction::Inc(register)),
        Instruction::Tgl(Operand::Register(r)) => Some(Instruction::Inc(r)),
        Instruction::Tgl(_) => None,
        Instruction::Jnz(operand, Operand::Register(r)) => Some(Instruction::Cpy(operand, r)),
        Instruction::Jnz(_, _) => None,
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> Int {
    let (_, mut instructions) = all_consuming(parse_instructions)
        .parse(file_content.trim())
        .unwrap();

    let mut cpu = Cpu {
        a: 7,
        ..Default::default()
    };

    cpu.execute(&mut instructions);

    cpu.a
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str, start: Int) -> Int {
    let (_, mut instructions) = all_consuming(parse_instructions)
        .parse(file_content.trim())
        .unwrap();

    let mut cpu = Cpu {
        a: start,
        ..Default::default()
    };

    cpu.execute(&mut instructions);

    cpu.a
}

fn parse_instructions(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((parse_cpy, parse_inc, parse_dec, parse_jnz, parse_tgl))(input)
}
fn parse_cpy(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(
        tag("cpy "),
        separated_pair(parse_operand, multispace1, parse_register)
            .map(|(operand, register)| Instruction::Cpy(operand, register)),
    )
    .parse(input)
}
fn parse_register(input: &str) -> nom::IResult<&str, Register> {
    alt((
        value(Register::A, tag("a")),
        value(Register::B, tag("b")),
        value(Register::C, tag("c")),
        value(Register::D, tag("d")),
    ))(input)
}
fn parse_operand(input: &str) -> nom::IResult<&str, Operand> {
    alt((
        parse_register.map(Operand::Register),
        complete::i64.map(Operand::Integer),
    ))(input)
}
fn parse_dec(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(tuple((tag("dec"), multispace1)), parse_register)
        .map(Instruction::Dec)
        .parse(input)
}
fn parse_inc(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(tuple((tag("inc"), multispace1)), parse_register)
        .map(Instruction::Inc)
        .parse(input)
}
fn parse_tgl(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(
        tuple((tag("tgl"), multispace1)),
        parse_operand.map(|operand| Instruction::Tgl(operand)),
    )
    .parse(input)
}
fn parse_jnz(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(
        tuple((tag("jnz"), multispace1)),
        separated_pair(parse_operand, multispace1, parse_operand)
            .map(|(operand, jump)| Instruction::Jnz(operand, jump)),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "3")]
    #[case::actual(ACTUAL, "11662")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::actual(ACTUAL, "479008222")] // runs 9s in release
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input, 12)), expected);
    }
}