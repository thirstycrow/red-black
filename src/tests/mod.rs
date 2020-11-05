use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result, Write};

use crate::{Node, NodePtr, RBTree};
use crate::kv::KeyValue;

mod validate;
mod insert;

type KV32 = KeyValue<i32, i32>;
type Color = crate::kv::Color;

impl Display for KV32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('(')?;
        if !self.left().is_nil() {
            f.write_fmt(format_args!("{},", self.left().node()))?
        }
        f.write_fmt(format_args!("{}:{}", if self.is_black() { 'B' } else { 'R' }, self.key()))?;
        if !self.right().is_nil() {
            f.write_fmt(format_args!(",{}", self.right().node()))?
        }
        f.write_char(')')
    }
}

impl KV32 {
    fn same(key: i32) -> KV32 {
        KV32::new(key, key)
    }

    fn color(&mut self, color: Color) -> &mut Self {
        if color == Color::BLACK {
            self.set_black()
        } else {
            self.set_red()
        }
        self
    }

    fn insert_left(&mut self, node: &KV32) -> &mut KV32 {
        if !self.left().is_nil() {
            panic!("{} already has a left child {}", self.key(), self.left().node().key())
        }
        *self.left_mut() = Node::new(node);
        self.left_mut().node_mut()
    }

    fn insert_right(&mut self, node: &KV32) -> &mut KV32 {
        if !self.right().is_nil() {
            panic!("{} already has a right child {}", self.key(), self.right().node().key())
        }
        *self.right_mut() = Node::new(node);
        self.right_mut().node_mut()
    }
}

impl Display for RBTree<KV32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("RBTree{{size:{},tree:{}}}", self.size, self.root.node()))
    }
}

impl RBTree<KV32> {
    fn search_for_update(&mut self, at: i32) -> &mut KV32 {
        let mut current_ptr = &self.root;
        loop {
            if current_ptr.is_nil() {
                panic!("Node {} does not exists", at)
            }
            let current_node = current_ptr.node_mut();
            match current_node.key().cmp(&at) {
                Ordering::Equal => { return current_node }
                Ordering::Less => { current_ptr = current_node.right_mut() }
                Ordering::Greater => { current_ptr = current_node.left_mut() }
            }
        }
    }
}

