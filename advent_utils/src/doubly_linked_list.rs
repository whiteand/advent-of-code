pub struct DoublyLinkedList<T> {
    nodes: Vec<Node<T>>,
    meta: Option<Meta>,
    len: usize,
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub prev: Option<NodeIndex>,
    pub next: Option<NodeIndex>,
}

#[derive(Clone, Debug)]
struct Meta {
    first: NodeIndex,
    last: NodeIndex,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NodeIndex(usize);

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DoublyLinkedList<T> {
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            meta: None,
            len: 0,
        }
    }
    pub fn get(&self, node_index: NodeIndex) -> Option<&Node<T>> {
        self.nodes.get(node_index.0)
    }
    pub fn get_mut(&mut self, node_index: NodeIndex) -> Option<&mut Node<T>> {
        self.nodes.get_mut(node_index.0)
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn first(&self) -> Option<NodeIndex> {
        self.meta.as_ref().map(|x| x.first)
    }
    pub fn push_back(&mut self, value: T) -> NodeIndex {
        self.len += 1;
        match &mut self.meta {
            Some(meta) => {
                let id = NodeIndex(self.nodes.len());
                self.nodes[meta.last.0].next = Some(id);
                self.nodes.push(Node {
                    value,
                    prev: Some(meta.last),
                    next: None,
                });
                meta.last = id;

                return id;
            }
            None => {
                let id = NodeIndex(0);
                self.meta = Some(Meta {
                    first: id,
                    last: id,
                });
                self.nodes.push(Node {
                    next: None,
                    prev: None,
                    value,
                });
                return id;
            }
        }
    }

    pub fn unlink(&mut self, node_index: NodeIndex) {
        debug_assert!(
            self.nodes[node_index.0].next.is_some() || self.nodes[node_index.0].prev.is_some()
        );
        let node_index = node_index.0;
        if let Some(NodeIndex(prev)) = self.nodes[node_index].prev {
            self.nodes[prev].next = self.nodes[node_index].next;
        }
        if let Some(NodeIndex(next)) = self.nodes[node_index].next {
            self.nodes[next].prev = self.nodes[node_index].prev;
        }
        self.len -= 1;
        if self.len == 0 {
            self.meta = None;
        } else {
            match &mut self.meta {
                Some(meta) => {
                    if meta.first.0 == node_index {
                        meta.first = self.nodes[node_index].next.unwrap();
                    } else if meta.last.0 == node_index {
                        meta.last = self.nodes[node_index].prev.unwrap();
                    }
                }
                None => {}
            }
        }

        self.nodes[node_index].prev = None;
        self.nodes[node_index].next = None;
    }
    pub fn insert_node_before(&mut self, node_index: NodeIndex, new_node_index: NodeIndex) {
        debug_assert_eq!(self.nodes[new_node_index.0].next, None);
        debug_assert_eq!(self.nodes[new_node_index.0].prev, None);
        match &mut self.meta {
            None => unreachable!(),
            Some(meta) => {
                if meta.first == node_index {
                    self.nodes[new_node_index.0].next = Some(meta.first);
                    meta.first = new_node_index;
                    return;
                }
                let prev = self.nodes[node_index.0].prev.unwrap();
                self.nodes[new_node_index.0].next = Some(node_index);
                self.nodes[new_node_index.0].prev = Some(prev);
                self.nodes[prev.0].next = Some(new_node_index);
                self.nodes[node_index.0].prev = Some(new_node_index);
            }
        }
    }

    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter {
            list: self,
            node_index: self.meta.as_ref().map(|x| x.first),
        }
    }
}

pub struct ListIter<'t, T> {
    list: &'t DoublyLinkedList<T>,
    node_index: Option<NodeIndex>,
}

impl<'t, T> Iterator for ListIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node_index.and_then(|x| self.list.get(x)) {
            Some(x) => {
                let res = Some(&x.value);
                self.node_index = x.next;
                res
            }
            None => None,
        }
    }
}
