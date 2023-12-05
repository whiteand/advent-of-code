use super::command::Command;

#[derive(Clone, Copy, Debug)]
pub struct Current {
    command: Command,
    start: usize,
}

pub struct Cpu<Cmds>
where
    Cmds: Iterator<Item = Command>,
{
    commands: Cmds,
    cycle: usize,
    current: Option<Current>,
    x: i32,
}

impl<Cmd> Cpu<Cmd>
where
    Cmd: Iterator<Item = Command>,
{
    pub fn new(cmds: Cmd) -> Self {
        Self {
            commands: cmds,
            cycle: 0,
            current: None,
            x: 1,
        }
    }
}

impl<Cmds> Iterator for Cpu<Cmds>
where
    Cmds: Iterator<Item = Command>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(Current { command, start }) => match command {
                Command::Noop => {
                    self.cycle += 1;
                    self.current = None;
                    Some(self.x)
                }
                Command::Addx(delta) => {
                    self.cycle += 1;
                    let before = self.x;
                    if start + 2 == self.cycle {
                        self.x += delta;
                        self.current = None;
                    }

                    Some(before)
                }
            },
            None => match self.commands.next() {
                Some(cmd) => {
                    self.current = Some(Current {
                        command: cmd,
                        start: self.cycle,
                    });
                    self.next()
                }
                None => None,
            },
        }
    }
}
