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

// insert case 2: update existing nodes
#[test]
fn test_insert_2() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert(&KV32::same(32));
    tree.insert(&KV32::same(96));
    tree.validate();
    assert_eq!("RBTree{size:3,tree:((R:32),B:64,(R:96))}", tree.to_string());

    assert_eq!(64, *tree.search(&64).unwrap().value());
    assert_eq!(32, *tree.search(&32).unwrap().value());
    assert_eq!(96, *tree.search(&96).unwrap().value());

    tree.insert(&KV32::new(64, 0));
    tree.insert(&KV32::new(32, 0));
    tree.insert(&KV32::new(96, 0));
    assert_eq!("RBTree{size:3,tree:((R:32),B:64,(R:96))}", tree.to_string());

    assert_eq!(0, *tree.search(&64).unwrap().value());
    assert_eq!(0, *tree.search(&32).unwrap().value());
    assert_eq!(0, *tree.search(&96).unwrap().value());
}

// insert case 3: black parent
#[test]
fn test_insert_3() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert(&KV32::same(32));
    tree.validate();
    assert_eq!("RBTree{size:2,tree:((R:32),B:64)}", tree.to_string());
}

// insert case 4: red parent and uncle
#[test]
fn test_insert_4() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert(&KV32::same(32));
    tree.insert(&KV32::same(96));
    tree.insert(&KV32::same(16));
    assert_eq!("RBTree{size:4,tree:(((R:16),B:32),B:64,(B:96))}", tree.to_string());
    tree.validate();
}
