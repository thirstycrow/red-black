use std::cmp::Ordering;
use std::fmt::Debug;

mod kv;

#[cfg(test)]
mod tests;

pub trait Node: Sized {
    type Key: Ord + Debug;
    type Ptr: NodePtr<Self>;

    fn new(node: &Self) -> Self::Ptr;
    fn free(&mut self);
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

    fn parent_ptr(&mut self) -> &mut N::Ptr {
        self.parent_ctx().ptr()
    }

    fn sibling_ptr(&mut self) -> &N::Ptr {
        if self.is_left_child() {
            self.parent_ptr().node().right()
        } else {
            self.parent_ptr().node().left()
        }
    }

    fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    fn is_left_child(&self) -> bool {
        self.parent.unwrap().1
    }

    fn parent_ctx(&mut self) -> &Self {
        unsafe { &*self.parent.unwrap().0 }
    }

    fn left_ctx(&mut self) -> Self {
        Context::<N> {
            parent: Some((self as *mut Self, true)),
            current: self.ptr().node_mut().left_mut()
        }
    }

    fn right_ctx(&mut self) -> Self {
        Context {
            parent: Some((self as *mut Self, false)),
            current: self.ptr().node_mut().right_mut()
        }
    }

    fn has_left(&self) -> bool {
        !self.ptr().node().left().is_nil()
    }

    fn has_left_and_right(&self) -> bool {
        let node = self.ptr().node();
        !node.left().is_nil() && !node.right().is_nil()
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

    pub fn size(&self) -> usize {
        self.size
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
            self.size += 1;
        }
        return inserted;
    }

    fn do_insert(mut ctx: Context<N>, node: &N) -> bool {
        let current_ptr = ctx.ptr();
        if current_ptr.is_nil() {
            *current_ptr = N::new(node);
            if ctx.is_root() {
                current_ptr.node_mut().set_black();
            }
            return true;
        }
        let current_node = current_ptr.node_mut();
        let next_ctx = match current_node.key().cmp(node.key()) {
            Ordering::Equal => {
                current_node.update(node);
                return false;
            }
            Ordering::Less => { ctx.right_ctx() }
            Ordering::Greater => { ctx.left_ctx() }
        };
        let inserted = Self::do_insert(next_ctx, node);
        if inserted && ctx.ptr().node().is_red() {
            if ctx.is_root() {
                ctx.ptr().node_mut().set_black();
            } else if next_ctx.ptr().node().is_red() {
                Self::insert_repair(ctx, next_ctx.is_left_child())
            }
        }
        inserted
    }

    fn insert_repair(mut ctx: Context<N>, inserted_at_left: bool) {
        if !ctx.sibling_ptr().is_nil() {
            let sibling_node = ctx.sibling_ptr().node_mut();
            if sibling_node.is_red() {
                ctx.ptr().node_mut().set_black();
                sibling_node.set_black();
                ctx.parent_ptr().node_mut().set_red();
                return;
            }
        }
        if ctx.is_left_child() {
            if !inserted_at_left {
                Self::rotate_left(ctx.ptr());
            }
            ctx.ptr().node_mut().set_black();
            ctx.parent_ptr().node_mut().set_red();
            Self::rotate_right(ctx.parent_ptr());
        } else {
            if inserted_at_left {
                Self::rotate_right(ctx.ptr());
            }
            ctx.ptr().node_mut().set_black();
            ctx.parent_ptr().node_mut().set_red();
            Self::rotate_left(ctx.parent_ptr());
        }
    }

    pub fn delete(&mut self, key: &N::Key) -> bool {
        let deleted = Self::do_delete(self.root_context(), key);
        if deleted {
            self.size -= 1;
        }
        return deleted
    }

    fn do_delete(mut ctx: Context<N>, key: &N::Key) -> bool {
        let current_ptr = ctx.ptr();
        if current_ptr.is_nil() {
            return false;
        }
        let current_node = current_ptr.node_mut();
        let next_ctx = match current_node.key().cmp(key) {
            Ordering::Equal => {
                let deleted_node = if ctx.has_left_and_right() {
                    let successor = Self::delete_left_most(ctx.right_ctx());
                    ctx.ptr().node_mut().update(successor);
                    successor
                } else {
                    Self::delete_node(ctx)
                };
                deleted_node.free();
                return true;
            }
            Ordering::Less => { ctx.right_ctx() }
            Ordering::Greater => { ctx.left_ctx() }
        };
        return Self::do_delete(next_ctx, key);
    }

    fn delete_left_most<'a>(mut ctx: Context<N>) -> &'a mut N {
        if ctx.has_left() {
            Self::delete_left_most(ctx.left_ctx())
        } else {
            Self::delete_node(ctx)
        }
    }

    fn delete_node<'a>(ctx: Context<N>) -> &'a mut N {
        let node = ctx.ptr().node_mut();
        ctx.ptr().clone_from(node.right());
        node
    }

    fn rotate_left(ptr: &mut N::Ptr) {
        let me = ptr.clone();
        let r = me.node().right().clone();
        let rl = r.node().left().clone();
        *ptr = r;
        *me.node_mut().right_mut() = rl;
        *r.node_mut().left_mut() = me;
    }

    fn rotate_right(ptr: &mut N::Ptr) {
        let me = ptr.clone();
        let l = me.node().left().clone();
        let lr = l.node().right().clone();
        *ptr = l;
        *me.node_mut().left_mut() = lr;
        *l.node_mut().right_mut() = me;
    }
}

pub type KeyValue<K, V> = kv::KeyValue<K, V>;