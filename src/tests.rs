use crate::{RBTree, NodePtr, Node};
use crate::kv::KeyValue;
use std::fmt::{Display, Formatter, Result, Write};

type KV32 = KeyValue<i32, i32>;

impl Display for KV32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('(')?;
        if !self.left().is_nil() {
            f.write_fmt(format_args!("{},", self.left().node()))?
        }
        f.write_fmt(format_args!("{}", self.key()))?;
        if !self.right().is_nil() {
            f.write_fmt(format_args!(",{}", self.right().node()))?
        }
        f.write_char(')')
    }
}

impl Display for RBTree<KV32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("RBTree{{size:{},tree:{}}}", self.size, self.root.node()))
    }
}

#[test]
fn it_works() {
    let mut tree: RBTree<KV32> = RBTree::new();
    for i in &[64, 32, 96, 16, 48, 80, 112, 8, 24, 40] {
        tree.insert(&KV32::new(*i, 0));
    }
    assert_eq!("RBTree{size:10,tree:((((8),16,(24)),32,((40),48)),64,((80),96,(112)))}", tree.to_string());
    println!("{}", tree);
    assert!(matches!(tree.search(&88), None));
    assert!(matches!(tree.search(&80).unwrap().key(), 80))
}
