#[derive(Debug)]
pub struct File<'input> {
    pub name: &'input str,
    pub size: usize,
}

impl File<'_> {
    pub fn has_name(&self, name: &str) -> bool {
        self.name == name
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChangeDirArgument<'input> {
    Root,
    Parent,
    Directory(&'input str),
}

#[derive(Debug)]
pub enum Node<'input> {
    Directory(&'input str),
    File(File<'input>),
}

#[derive(Debug)]
pub enum IO<'input> {
    List(Vec<Node<'input>>),
    ChangeDir(ChangeDirArgument<'input>),
}
