use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression<'input> {
    Var(&'input str),
    Integer(u64),
    Sum(Box<(Expression<'input>, Expression<'input>)>),
    Product(Box<(Expression<'input>, Expression<'input>)>),
}

impl<'input> Expression<'input> {
    pub fn calculate(&self, vars: &HashMap<&'input str, u64>) -> u64 {
        match self {
            Expression::Var(v) => vars.get(v).copied().unwrap_or_default(),
            Expression::Integer(i) => *i,
            Expression::Sum(pair) => pair.0.calculate(vars) + pair.1.calculate(vars),
            Expression::Product(pair) => pair.0.calculate(vars) * pair.1.calculate(vars),
        }
    }
}
