use std::marker::PhantomData;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct ImmutableLists<T> {
    nodes: Vec<Node<T>>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct ImmutableList<T>(usize, PhantomData<T>);

impl<T> Clone for ImmutableList<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for ImmutableList<T> {}

impl<T> ImmutableLists<T> {
    pub fn with_capacity(nodes_len_capacity: usize) -> ImmutableLists<T> {
        Self {
            nodes: Vec::with_capacity(nodes_len_capacity),
        }
    }
    /// Creates a single-item linked list and returns it's index
    pub fn singleton(&mut self, value: T) -> ImmutableList<T> {
        self.alloc_node(Node {
            value,
            rest: None,
            len: 1,
        })
    }
    pub fn all_nodes(&self) -> impl Iterator<Item = &Node<T>> + '_ {
        self.nodes.iter()
    }
    pub fn get(&self, ptr: ImmutableList<T>) -> Option<&Node<T>> {
        self.nodes.get(ptr.0)
    }
    pub fn len(&self, list: ImmutableList<T>) -> usize {
        self.get(list).map_or(0, |x| x.len)
    }
    pub fn head(&self, ptr: ImmutableList<T>) -> Option<&T> {
        self.get(ptr).map(|x| &x.value)
    }
    fn alloc_node(&mut self, node: Node<T>) -> ImmutableList<T> {
        let idx = self.nodes.len();

        self.nodes.push(node);

        ImmutableList(idx, PhantomData)
    }
    pub fn suffixes(&self, list: ImmutableList<T>) -> impl Iterator<Item = ImmutableList<T>> + '_ {
        std::iter::successors(Some(list), |x| self.get(*x).and_then(|n| n.rest))
    }
    pub fn nodes(&self, list: ImmutableList<T>) -> impl Iterator<Item = &Node<T>> {
        self.suffixes(list).flat_map(|x| self.get(x))
    }
    pub fn iter(&self, list: ImmutableList<T>) -> impl Iterator<Item = &T> {
        self.nodes(list).map(|x| &x.value)
    }
    pub fn prepend(&mut self, list: ImmutableList<T>, value: T) -> ImmutableList<T> {
        self.alloc_node(Node {
            value,
            len: self.nodes[list.0].len + 1,
            rest: Some(list),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node<T> {
    value: T,
    rest: Option<ImmutableList<T>>,
    len: usize,
}
