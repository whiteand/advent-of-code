#[derive(Debug, Clone, Copy)]
pub enum Command {
    Noop,
    Addx(i32),
}
