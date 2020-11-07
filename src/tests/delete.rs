use crate::RBTree;

use super::KV32;
use crate::kv::Color::{BLACK, RED};

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

// delete case 4: deleted the root node which has no children
#[test]
fn test_delete_4() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    assert_eq!("RBTree{size:1,tree:(B:64)}", tree.to_string());
    tree.validate();
    assert_eq!(true, tree.delete(&64));
    tree.validate();
    assert_eq!("RBTree{size:0}", tree.to_string());
}

// delete case 5: the replacement and the parent are black, the sibling is red
#[test]
fn test_delete_5_left() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.search_for_update(64).insert_left(&KV32::same(32)).color(BLACK);
    tree.search_for_update(64).insert_right(&KV32::same(96)).color(RED);
    tree.search_for_update(96).insert_left(&KV32::same(80)).color(BLACK);
    tree.search_for_update(96).insert_right(&KV32::same(112)).color(BLACK);
    tree.size = 5;
    assert_eq!("RBTree{size:5,tree:((B:32),B:64,((B:80),R:96,(B:112)))}", tree.to_string());
    tree.validate();
    tree.delete(&32);
    assert_eq!("RBTree{size:4,tree:((B:64,(R:80)),B:96,(B:112))}", tree.to_string());
    tree.validate();
}

// delete case 5: the replacement and the parent are black, the sibling is red
#[test]
fn test_delete_5_right() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.search_for_update(64).insert_right(&KV32::same(96)).color(BLACK);
    tree.search_for_update(64).insert_left(&KV32::same(32)).color(RED);
    tree.search_for_update(32).insert_right(&KV32::same(48)).color(BLACK);
    tree.search_for_update(32).insert_left(&KV32::same(16)).color(BLACK);
    tree.size = 5;
    assert_eq!("RBTree{size:5,tree:(((B:16),R:32,(B:48)),B:64,(B:96))}", tree.to_string());
    tree.validate();
    tree.delete(&96);
    assert_eq!("RBTree{size:4,tree:((B:16),B:32,((R:48),B:64))}", tree.to_string());
    tree.validate();
}
