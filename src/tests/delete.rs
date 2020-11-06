use crate::RBTree;

use super::KV32;

// delete case 1: a non-exist key
#[test]
fn test_delete_1() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    assert_eq!(false, tree.delete(&32));
    assert_eq!("RBTree{size:1,tree:(B:64)}", tree.to_string());
}

// delete case 2: delete a red node
#[test]
fn test_delete_2() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert(&KV32::same(32));
    tree.insert(&KV32::same(96));
    tree.insert(&KV32::same(16));
    tree.insert(&KV32::same(80));
    tree.insert(&KV32::same(8));
    tree.insert(&KV32::same(24));
    assert_eq!("RBTree{size:7,tree:(((B:8),R:16,((R:24),B:32)),B:64,((R:80),B:96))}", tree.to_string());
    tree.validate();
    assert_eq!(true, tree.delete(&24));
    tree.validate();
    assert_eq!("RBTree{size:6,tree:(((B:8),R:16,(B:32)),B:64,((R:80),B:96))}", tree.to_string());
}

// delete case 3: deleted a black node which has a red child
#[test]
fn test_delete_3() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert(&KV32::same(32));
    tree.insert(&KV32::same(96));
    tree.insert(&KV32::same(16));
    tree.insert(&KV32::same(80));
    tree.insert(&KV32::same(8));
    tree.insert(&KV32::same(24));
    assert_eq!("RBTree{size:7,tree:(((B:8),R:16,((R:24),B:32)),B:64,((R:80),B:96))}", tree.to_string());
    tree.validate();
    assert_eq!(true, tree.delete(&32));
    tree.validate();
    assert_eq!("RBTree{size:6,tree:(((B:8),R:16,(B:24)),B:64,((R:80),B:96))}", tree.to_string());
}