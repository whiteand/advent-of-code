#[derive(Debug, Clone)]
pub(super) enum Step {
    GoTo(usize),
    Open,
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::GoTo(s) => write!(f, "->{}", s),
            Step::Open => write!(f, "OPEN"),
        }
    }
}
