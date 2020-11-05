use crate::RBTree;
use super::KV32;

// insert case 1: empty tree
#[test]
fn test_insert_1() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.validate();
    assert_eq!("RBTree{size:1,tree:(B:64)}", tree.to_string());
}