const ALPHABET_SIZE: usize = 26;
const BOTH_CASES_ALPHABET_SIZE: usize = ALPHABET_SIZE * 2;
struct Node {
    is_leaf: bool,
    children: [usize; BOTH_CASES_ALPHABET_SIZE],
}

impl Node {
    fn letter_index(ch: u8) -> usize {
        if ch >= b'a' && ch <= b'z' {
            (ch - b'a') as usize
        } else if ch >= b'A' && ch <= b'Z' {
            (ch - b'A') as usize + ALPHABET_SIZE
        } else {
            panic!("Unkonwn char: {:?}", ch as char);
        }
    }
    fn has_leaf(&self, prefix: &[u8], trie: &Trie) -> bool {
        if prefix.is_empty() {
            return self.is_leaf;
        }
        let letter = prefix[0];
        let letter_ind = Node::letter_index(letter);
        let child = self.children[letter_ind];
        if child == 0 {
            return false;
        }

        trie.nodes.get(child).unwrap().has_leaf(&prefix[1..], trie)
    }
}

pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node {
                is_leaf: false,
                children: [0usize; BOTH_CASES_ALPHABET_SIZE],
            }],
        }
    }
    fn allocate_node(&mut self) -> (&mut Node, usize) {
        let node = Node {
            is_leaf: false,
            children: [0usize; BOTH_CASES_ALPHABET_SIZE],
        };
        let index = self.nodes.len();
        self.nodes.push(node);
        (self.nodes.get_mut(index).unwrap(), index)
    }
    pub fn contains(&self, text: &str) -> bool {
        self.nodes[0].has_leaf(text.as_bytes(), self)
    }
    pub fn push(&mut self, text: &str) {
        let mut current = 0;
        let mut prefix = text.as_bytes();
        while prefix.len() > 1 {
            let letter_index = Node::letter_index(prefix[0]);
            let child_ind = self.nodes[current].children[letter_index];
            if child_ind == 0 {
                let (_, ind) = self.allocate_node();
                self.nodes[current].children[letter_index] = ind;
                current = ind;
                prefix = &prefix[1..];
                continue;
            }
            current = child_ind;
            prefix = &prefix[1..];
            continue;
        }
        let letter_idx = Node::letter_index(prefix[0]);
        let leaf_idx = self.nodes[current].children[letter_idx];
        if leaf_idx != 0 {
            self.nodes[leaf_idx].is_leaf = true;
            return;
        }
        let (leaf_node, idx) = self.allocate_node();
        leaf_node.is_leaf = true;
        self.nodes[current].children[letter_idx] = idx;
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        assert_eq!(trie.contains("a"), false);
        assert_eq!(trie.contains("b"), false);
        trie.push("a");
        assert_eq!(trie.contains("a"), true);
        assert_eq!(trie.contains("b"), false);
        trie.push("b");
        assert_eq!(trie.contains("a"), true);
        assert_eq!(trie.contains("b"), true);
    }
}
