use std::iter::Peekable;

use advent_utils::nom::{
    self, branch, bytes,
    character::{self, complete::space1},
    sequence, IResult,
};

use super::io::{ChangeDirArgument, File, Node, IO};

#[derive(Debug, PartialEq, Eq)]
enum Query<'input> {
    List,
    ChangeDir(ChangeDirArgument<'input>),
}

struct IOParser<'input, Lines>
where
    Lines: Iterator<Item = &'input str>,
{
    lines: Peekable<Lines>,
}

impl<'input, Lines> Iterator for IOParser<'input, Lines>
where
    Lines: Iterator<Item = &'input str>,
{
    type Item = IO<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => None,
            Some(line) => {
                let (_, query) = parse_query(line).expect("valid query is expected");
                match query {
                    Query::List => {
                        let mut res = Vec::new();
                        loop {
                            match self.lines.peek() {
                                None => return Some(IO::List(res)),
                                Some(line) => match parse_node(line) {
                                    Ok((_, r)) => {
                                        self.lines.next().unwrap();
                                        res.push(r);
                                    }
                                    Err(_) => return Some(IO::List(res)),
                                },
                            }
                        }
                    }
                    Query::ChangeDir(argument) => Some(IO::ChangeDir(argument)),
                }
            }
        }
    }
}

pub fn parse_io(file_content: &str) -> impl Iterator<Item = IO<'_>> {
    IOParser {
        lines: file_content.lines().peekable(),
    }
}

fn parse_change_dir_query(line: &str) -> IResult<&str, Query<'_>> {
    nom::combinator::map(
        sequence::preceded(
            bytes::complete::tag("cd "),
            nom::branch::alt((
                nom::combinator::map(character::complete::alpha1, |name| {
                    ChangeDirArgument::Directory(name)
                }),
                nom::combinator::map(bytes::complete::tag(".."), |_| ChangeDirArgument::Parent),
                nom::combinator::map(bytes::complete::tag("/"), |_| ChangeDirArgument::Root),
            )),
        ),
        Query::ChangeDir,
    )(line)
}
fn parse_list_query(line: &str) -> IResult<&str, Query<'_>> {
    nom::combinator::map(bytes::complete::tag("ls"), |_| Query::List)(line)
}
fn parse_query(line: &str) -> IResult<&str, Query<'_>> {
    sequence::preceded(
        bytes::complete::tag("$ "),
        branch::alt((parse_change_dir_query, parse_list_query)),
    )(line)
}

fn parse_file(line: &str) -> IResult<&str, Node<'_>> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::character::complete::u32,
            space1,
            nom::character::complete::not_line_ending,
        ),
        |(size, name)| {
            Node::File(File {
                name,
                size: size as usize,
            })
        },
    )(line)
}
fn parse_directory(line: &str) -> IResult<&str, Node<'_>> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("dir "),
            nom::character::complete::not_line_ending,
        ),
        Node::Directory,
    )(line)
}

fn parse_node(line: &str) -> IResult<&str, Node<'_>> {
    nom::branch::alt((parse_file, parse_directory))(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_query() {
        assert_eq!(parse_query("$ ls"), Ok(("", Query::List)));
    }

    #[test]
    fn test_parse_change_dir_query_parent() {
        assert_eq!(
            parse_query("$ cd .."),
            Ok(("", Query::ChangeDir(ChangeDirArgument::Parent)))
        );
    }
    #[test]
    fn test_parse_change_dir_query_root() {
        assert_eq!(
            parse_query("$ cd /"),
            Ok(("", Query::ChangeDir(ChangeDirArgument::Root)))
        );
    }
    #[test]
    fn test_parse_change_dir_query_directory() {
        assert_eq!(
            parse_query("$ cd a"),
            Ok(("", Query::ChangeDir(ChangeDirArgument::Directory("a"))))
        );
    }
}
