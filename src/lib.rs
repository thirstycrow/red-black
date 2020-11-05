use std::cmp::Ordering;
use std::fmt::Debug;

mod kv;

#[cfg(test)]
mod tests;

pub trait Node: Sized {
    type Key: Ord + Debug;
    type Ptr: NodePtr<Self>;

    fn new(node: &Self) -> Self::Ptr;
    fn left(&self) -> &Self::Ptr;
    fn left_mut(&mut self) -> &mut Self::Ptr;
    fn right(&self) -> &Self::Ptr;
    fn right_mut(&mut self) -> &mut Self::Ptr;
    fn key(&self) -> &Self::Key;
    fn update(&mut self, node: &Self);

    fn is_black(&self) -> bool;
    fn is_red(&self) -> bool {
        !self.is_black()
    }

    fn set_black(&mut self);
    fn set_red(&mut self);
}

pub trait NodePtr<N: Node<Ptr = Self>>: Copy {
    const NIL: Self;
    fn is_nil(&self) -> bool;
    fn node<'a>(&self) -> &'a N;
    fn node_mut<'a>(&self) -> &'a mut N;

    fn is_black(&self) -> bool {
        self.is_nil() || self.node().is_black()
    }

    fn is_red(&self) -> bool {
        !self.is_black()
    }
}

pub struct RBTree<N: Node> {
    size: usize,
    root: N::Ptr
}

impl<N: Node> RBTree<N> {
    pub fn new() -> RBTree<N> {
        RBTree {
            size: 0,
            root: N::Ptr::NIL
        }
    }

    pub fn search(&self, key: &N::Key) -> Option<&N> {
        let mut ptr = &self.root;
        loop {
            if ptr.is_nil() {
                return None;
            }
            let node = ptr.node();
            match node.key().cmp(key) {
                Ordering::Equal => { return Some(node) }
                Ordering::Less => { ptr = node.right() }
                Ordering::Greater => { ptr = node.left() }
            }
        }
    }

    pub fn insert(&mut self, node: &N) {
        let mut ptr = &mut self.root;
        loop {
            if ptr.is_nil() {
                *ptr = N::new(node);
                self.size += 1;
                return
            }
            let current_node = ptr.node_mut();
            match current_node.key().cmp(node.key()) {
                Ordering::Equal => { return current_node.update(node) }
                Ordering::Less => { ptr = current_node.right_mut() }
                Ordering::Greater => { ptr = current_node.left_mut() }
            }
        }
    }
}

pub type KeyValue<K, V> = kv::KeyValue<K, V>;