mod io;
mod parse;
mod tree;

use self::{parse::parse_io, tree::Tree};

fn file_content_to_tree(file_content: &str) -> Tree {
    parse_io(file_content).collect()
}

// 1581595
pub fn solve_task1(file_content: &str) -> usize {
    file_content_to_tree(file_content)
        .into_iter()
        .filter_map(|x| {
            if !x.is_dir() {
                return None;
            }
            let size = x.total_size();
            if size > 100000 {
                return None;
            }
            Some(size)
        })
        .sum()
}

// 1544176
pub fn solve_task2(file_content: &str) -> usize {
    let tree = file_content_to_tree(file_content);

    const TOTAL: usize = 70000000;
    const REQUIRED: usize = 30000000;

    let currently_free = TOTAL - tree.total_size();

    let to_delete = REQUIRED - currently_free;

    let mut possible: Vec<usize> = tree
        .into_iter()
        .filter_map(move |x| {
            if !x.is_dir() {
                return None;
            }
            let size = x.total_size();
            if size < to_delete {
                return None;
            }
            Some(size)
        })
        .collect();

    possible.sort_by(|a, b| b.cmp(a));
    possible.pop().unwrap_or_default()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "95437");
    }
    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "24933642");
    }
}
