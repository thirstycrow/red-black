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

struct Context<N: Node> {
    parent: Option<(*mut Self, bool)>,
    current: *mut N::Ptr
}

impl<N: Node> Clone for Context<N> {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent,
            current: self.current
        }
    }
}

impl<N: Node> Copy for Context<N> {
}

impl<N: Node> Context<N> {
    fn ptr(&self) -> &mut N::Ptr {
        unsafe { &mut *self.current }
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

    fn root_context(&mut self) -> Context<N> {
        Context {
            parent: None,
            current: &mut self.root as *mut N::Ptr
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

    pub fn insert(&mut self, node: &N) -> bool {
        let inserted = Self::do_insert(self.root_context(), node);
        if inserted {
            self.root.node_mut().set_black();
            self.size += 1;
        }
        return inserted;
    }

    fn do_insert(ctx: Context<N>, node: &N) -> bool {
        let current_ptr = ctx.ptr();
        if current_ptr.is_nil() {
            *current_ptr = N::new(node);
            return true;
        }
        false
    }
}

pub type KeyValue<K, V> = kv::KeyValue<K, V>;