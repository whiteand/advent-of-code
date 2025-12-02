use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    combinator::{all_consuming, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser,
};
use tracing::{info, trace};

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
    Out(Register),
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
struct Cpu {
    a: Int,
    b: Int,
    c: Int,
    d: Int,
    ip: usize,
    gas: usize,
}
impl Cpu {
    fn execute(&mut self, program: &mut [Instruction], out: &mut Vec<Int>) {
        while let Some(command) = program.get(self.ip).copied() {
            if self.gas == 0 {
                break;
            }
            self.gas -= 1;
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
                Instruction::Out(register) => {
                    trace!("out");
                    out.push(*self.register(register));
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
                    trace!(ip = ?self.ip, a=?self.a,b=?self.b,c=?self.c,d=self.d,?command, ?value);

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
        Instruction::Out(register) => Some(Instruction::Inc(register)),
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

    info!(?instructions);
    for a in 0.. {
        let mut cpu = Cpu {
            a,
            gas: 10_000_000,
            ..Default::default()
        };

        let mut output = Vec::new();

        cpu.execute(&mut instructions, &mut output);

        if cpu.gas > 0 {
            continue;
        }
        if output.len() <= 10 {
            continue;
        }
        if output.iter().copied().step_by(2).any(|x| x != 0) {
            continue;
        }
        if output.iter().copied().skip(1).step_by(2).any(|x| x != 1) {
            continue;
        }
        return a;
    }
    0
}

fn parse_instructions(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction).parse(input)
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((
        parse_cpy, parse_inc, parse_dec, parse_jnz, parse_tgl, parse_out,
    ))
    .parse(input)
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
    ))
    .parse(input)
}
fn parse_operand(input: &str) -> nom::IResult<&str, Operand> {
    alt((
        parse_register.map(Operand::Register),
        complete::i64.map(Operand::Integer),
    ))
    .parse(input)
}
fn parse_dec(input: &str) -> nom::IResult<&str, Instruction> {
    preceded((tag("dec"), multispace1), parse_register)
        .map(Instruction::Dec)
        .parse(input)
}
fn parse_inc(input: &str) -> nom::IResult<&str, Instruction> {
    preceded((tag("inc"), multispace1), parse_register)
        .map(Instruction::Inc)
        .parse(input)
}
fn parse_out(input: &str) -> nom::IResult<&str, Instruction> {
    preceded((tag("out"), multispace1), parse_register)
        .map(Instruction::Out)
        .parse(input)
}
fn parse_tgl(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(
        (tag("tgl"), multispace1),
        parse_operand.map(Instruction::Tgl),
    )
    .parse(input)
}
fn parse_jnz(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(
        (tag("jnz"), multispace1),
        separated_pair(parse_operand, multispace1, parse_operand)
            .map(|(operand, jump)| Instruction::Jnz(operand, jump)),
    )
    .parse(input)
}
#[cfg(test)]
mod tests {
    use super::part1;
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::actual(ACTUAL, "196")]
    #[ignore] // slow
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
}
