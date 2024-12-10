use super::io::ChangeDirArgument;
use super::io::File;
use super::io::Node;
use super::io::IO;

#[derive(Debug)]
pub enum Tree<'input> {
    Directory(&'input str, Vec<Tree<'input>>),
    File(File<'input>),
}

impl<'input> Default for Tree<'input> {
    fn default() -> Self {
        Tree::Directory("", Vec::default())
    }
}

impl<'input> Tree<'input> {
    pub fn is_dir(&self) -> bool {
        match self {
            Tree::Directory(_, _) => true,
            Tree::File(_) => false,
        }
    }
    pub fn has_name(&self, name: &str) -> bool {
        match self {
            Tree::Directory(dir_name, _) => *dir_name == name,
            Tree::File(f) => f.has_name(name),
        }
    }
    pub fn add_file(&mut self, path: &[&'input str], f: File<'input>) {
        if path.is_empty() {
            match self {
                Tree::Directory(_, children) => {
                    children.push(Tree::File(f));
                }
                Tree::File(_) => todo!(),
            };
            return;
        }
        let first_name = path[0];
        let mut to_insert = Some(f);
        match self {
            Tree::Directory(_, children) => {
                for child in children {
                    if child.has_name(first_name) {
                        if let Some(f) = to_insert.take() {
                            child.add_file(&path[1..], f);
                        }
                    }
                }
            }
            Tree::File(_) => {}
        }
    }
    pub fn add_directory(&mut self, path: &[&'input str]) {
        if path.is_empty() {
            return;
        }
        if path.len() == 1 {
            let dir_name = path[0];
            return match self {
                Tree::Directory(_, children) => {
                    if children.iter().any(|x| x.has_name(dir_name) && x.is_dir()) {
                        return;
                    }
                    children.push(Tree::Directory(dir_name, Vec::new()));
                }
                Tree::File(_) => unreachable!(),
            };
        }
        let first_name = path[0];
        match self {
            Tree::Directory(_, children) => {
                let mut added = false;
                for x in children.iter_mut() {
                    if !x.is_dir() {
                        continue;
                    }
                    if !x.has_name(first_name) {
                        continue;
                    }
                    added = true;
                    x.add_directory(&path[1..]);
                }
                if !added {
                    children.push(Tree::Directory(first_name, Vec::new()));
                }
            }
            Tree::File(_) => todo!(),
        }
    }

    pub fn total_size(&self) -> usize {
        match self {
            Tree::Directory(_, children) => children.iter().fold(0, |a, b| a + b.total_size()),
            Tree::File(f) => f.size,
        }
    }
}

pub struct TreeIter<'input> {
    to_visit: Vec<&'input Tree<'input>>,
}

impl<'input> Iterator for TreeIter<'input> {
    type Item = &'input Tree<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_visit.is_empty() {
            return None;
        }
        let new_visit = self.to_visit.pop().unwrap();
        match new_visit {
            Tree::Directory(_, children) => self.to_visit.extend(children.iter()),
            Tree::File(_) => {}
        }
        Some(new_visit)
    }
}

impl<'input> IntoIterator for &'input Tree<'input> {
    type Item = &'input Tree<'input>;

    type IntoIter = TreeIter<'input>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter {
            to_visit: vec![self],
        }
    }
}

impl<'input> FromIterator<IO<'input>> for Tree<'input> {
    fn from_iter<T: IntoIterator<Item = IO<'input>>>(iter: T) -> Self {
        let mut tree = Tree::default();
        let mut current_path = Vec::new();
        for io in iter {
            match io {
                IO::List(children) => {
                    for child in children {
                        match child {
                            Node::Directory(d) => {
                                let mut full_path = current_path.clone();
                                full_path.push(d);
                                tree.add_directory(&full_path);
                            }
                            Node::File(f) => tree.add_file(&current_path, f),
                        }
                    }
                }
                IO::ChangeDir(arg) => match arg {
                    ChangeDirArgument::Root => current_path.clear(),
                    ChangeDirArgument::Parent => {
                        current_path.pop().unwrap();
                    }
                    ChangeDirArgument::Directory(d) => {
                        current_path.push(d);
                        tree.add_directory(&current_path);
                    }
                },
            }
        }
        tree
    }
}
