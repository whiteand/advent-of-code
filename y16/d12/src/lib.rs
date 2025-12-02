use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1},
    combinator::{all_consuming, value},
    multi::separated_list1,
    parse_usize,
    sequence::{preceded, separated_pair},
    Parser,
};

type Int = usize;

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
enum Jump {
    Forward(usize),
    Backward(usize),
}
impl Jump {
    fn apply(&self, ip: &mut usize) {
        match self {
            Jump::Forward(f) => {
                *ip += f;
            }
            Jump::Backward(b) => {
                *ip -= b;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    /// cpy x y copies x (either an integer or the value of a register) into register y.
    Cpy(Operand, Register),
    /// inc x increases the value of register x by one.
    Inc(Register),
    Dec(Register),
    Jnz(Operand, Jump),
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
    fn execute(&mut self, program: &[Instruction]) {
        while let Some(command) = program.get(self.ip) {
            match *command {
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
                Instruction::Jnz(operand, jump) => {
                    let value = self.get_operand_value(operand);
                    if value == 0 {
                        self.ip += 1;
                    } else {
                        jump.apply(&mut self.ip);
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

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> Int {
    let (_, instructions) = all_consuming(parse_instructions)
        .parse(file_content.trim())
        .unwrap();

    let mut cpu = Cpu::default();

    cpu.execute(&instructions);

    cpu.a
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> Int {
    let (_, instructions) = all_consuming(parse_instructions)
        .parse(file_content.trim())
        .unwrap();

    let mut cpu = Cpu {
        c: 1,
        ..Default::default()
    };

    cpu.execute(&instructions);

    cpu.a
}

fn parse_instructions(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction).parse(input)
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((parse_cpy, parse_inc, parse_dec, parse_jnz)).parse(input)
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
        parse_usize.map(Operand::Integer),
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
fn parse_jnz(input: &str) -> nom::IResult<&str, Instruction> {
    preceded(
        (tag("jnz"), multispace1),
        separated_pair(parse_operand, multispace1, parse_jump)
            .map(|(operand, jump)| Instruction::Jnz(operand, jump)),
    )
    .parse(input)
}
fn parse_jump(input: &str) -> nom::IResult<&str, Jump> {
    alt((
        preceded(tag("-"), parse_usize).map(Jump::Backward),
        parse_usize.map(Jump::Forward),
    ))
    .parse(input)
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "42");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "318007");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "9227661");
    }
}
