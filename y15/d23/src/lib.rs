mod parse;

use parse::parse_instructions;

#[derive(Default)]
struct Cpu {
    ip: usize,
    a: usize,
    b: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

#[derive(Copy, Clone, Debug)]
enum Offset {
    Forward(usize),
    Backward(usize),
}

#[derive(Debug)]
enum Instruction {
    Inc(Register),
    Tripple(Register),
    JumpIfOne(Register, Offset),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    Half(Register),
}

impl Cpu {
    fn jump(&mut self, offset: Offset) -> Result<(), ()> {
        match offset {
            Offset::Forward(x) => {
                if let Some(new_ip) = self.ip.checked_add(x) {
                    self.ip = new_ip;
                    Ok(())
                } else {
                    Err(())
                }
            }
            Offset::Backward(x) => {
                if let Some(new_ip) = self.ip.checked_sub(x) {
                    self.ip = new_ip;
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }

    fn register(&mut self, reg: Register) -> &mut usize {
        match reg {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), ()> {
        match instruction {
            Instruction::Inc(reg) => {
                *self.register(*reg) += 1;
                self.ip += 1;
            }
            Instruction::Tripple(reg) => {
                *self.register(*reg) *= 3;
                self.ip += 1;
            }
            Instruction::JumpIfOne(reg, offset) => {
                let value = *self.register(*reg);
                if value == 1 {
                    self.jump(*offset)?;
                } else {
                    self.ip += 1;
                }
            }
            Instruction::JumpIfEven(reg, offset) => {
                let value = *self.register(*reg);
                if value % 2 == 0 {
                    self.jump(*offset)?;
                } else {
                    self.ip += 1;
                }
            }
            Instruction::Jump(offset) => {
                self.jump(*offset)?;
            }
            Instruction::Half(register) => {
                *self.register(*register) /= 2;
                self.ip += 1;
            }
        }

        Ok(())
    }

    fn execute_source(&mut self, src: &str) {
        let program = parse_instructions(src.trim())
            .map(|x| {
                if !x.0.is_empty() {
                    unreachable!("{}", x.0);
                }
                x.1
            })
            .unwrap();

        loop {
            let Some(instruction) = program.get(self.ip) else {
                break;
            };
            if self.execute_instruction(instruction).is_err() {
                break;
            }
        }
    }
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let mut cpu = Cpu::default();
    cpu.execute_source(file_content);
    cpu.b
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let mut cpu = Cpu::default();
    cpu.a = 1;
    cpu.execute_source(file_content);
    cpu.b
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "307");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "160");
    }
}
