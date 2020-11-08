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
//
//      B64             B64
//    ┌──┴──┐            └──┐
//   R32   R96     =>      B96
//    ↑
//   DEL
#[test]
fn test_delete_2() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert(&KV32::same(32));
    tree.insert(&KV32::same(96));
    assert_eq!("RBTree{size:3,tree:((R:32),B:64,(R:96))}", tree.to_string());
    tree.validate();
    assert_eq!(true, tree.delete(&32));
    tree.validate();
    assert_eq!("RBTree{size:2,tree:(B:64,(R:96))}", tree.to_string());
}

// delete case 3: deleted a black node which has a red child
//
//             B64                      B64
//         ┌────┴────┐              ┌────┴────┐
//  DEL → B32       B96     =>     B48       B96
//         └──┐
//           R48
#[test]
fn test_delete_3() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_right(64, 96, BLACK);
    tree.insert_right(32, 48, RED);
    assert_eq!("RBTree{size:4,tree:((B:32,(R:48)),B:64,(B:96))}", tree.to_string());
    tree.validate();
    assert_eq!(true, tree.delete(&32));
    assert_eq!("RBTree{size:3,tree:((B:48),B:64,(B:96))}", tree.to_string());
    tree.validate();
}

// delete case 4: deleted the root node which has no children
//   B64
//    ↑
//   DEL
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
//
//        B64                       B96
//    ┌────┴────┐               ┌────┴────┐
//   R32       B96      =>     B64       B112
//    ↑      ┌──┴──┐            └──┐
//   DEL    B80   B112            R80
#[test]
fn test_delete_5_left() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_right(64, 96, RED);
    tree.insert_left(96, 80, BLACK);
    tree.insert_right(96, 112, BLACK);
    assert_eq!("RBTree{size:5,tree:((B:32),B:64,((B:80),R:96,(B:112)))}", tree.to_string());
    tree.validate();
    tree.delete(&32);
    assert_eq!("RBTree{size:4,tree:((B:64,(R:80)),B:96,(B:112))}", tree.to_string());
    tree.validate();
}

// delete case 5: the replacement and the parent are black, the sibling is red
//
//           B64                    B32
//       ┌────┴────┐            ┌────┴────┐
//      R32       B96    =>    B16       B64
//    ┌──┴──┐      ↑                   ┌──┘
//   B16   B48    DEL                 R48
#[test]
fn test_delete_5_right() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_right(64, 96, BLACK);
    tree.insert_left(64, 32, RED);
    tree.insert_right(32, 48, BLACK);
    tree.insert_left(32, 16, BLACK);
    assert_eq!("RBTree{size:5,tree:(((B:16),R:32,(B:48)),B:64,(B:96))}", tree.to_string());
    tree.validate();
    tree.delete(&96);
    assert_eq!("RBTree{size:4,tree:((B:16),B:32,((R:48),B:64))}", tree.to_string());
    tree.validate();
}

// delete case 6
//
//      B64            B64
//    ┌──┴──┐           └──┐
//   B32   B96   =>       R96
//    ↑
//   DEL
#[test]
fn test_delete_6() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_right(64, 96, BLACK);
    assert_eq!("RBTree{size:3,tree:((B:32),B:64,(B:96))}", tree.to_string());
    tree.validate();
    tree.delete(&32);
    assert_eq!("RBTree{size:2,tree:(B:64,(R:96))}", tree.to_string());
    tree.validate();
}

// delete case 7
//
//            B64                         B64
//       ┌─────┴─────┐               ┌─────┴─────┐
//      B32         B96      =>     B32         B96
//    ┌──┴──┐     ┌──┴──┐            └──┐     ┌──┴──┐
//   B16   B48   B80   B112            B48   B80   B112
//    ↑
//   DEL
#[test]
fn test_delete_7() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_right(64, 96, BLACK);
    tree.insert_left(32, 16, BLACK);
    tree.insert_right(32, 48, BLACK);
    tree.insert_left(96, 80, BLACK);
    tree.insert_right(96, 112, BLACK);
    assert_eq!("RBTree{size:7,tree:(((B:16),B:32,(B:48)),B:64,((B:80),B:96,(B:112)))}", tree.to_string());
    tree.validate();
    tree.delete(&16);
    assert_eq!("RBTree{size:6,tree:((B:32,(R:48)),B:64,((B:80),R:96,(B:112)))}", tree.to_string());
    tree.validate();
}

// delete case 8
//
//       B64                     B96
//    ┌───┴───┐               ┌───┴───┐
//   B32     B96      =>     B64     B112
//    ↑       └──┐
//   DEL        R112
#[test]
fn test_delete_8_left() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_right(64, 96, BLACK);
    tree.insert_right(96, 112, RED);
    assert_eq!("RBTree{size:4,tree:((B:32),B:64,(B:96,(R:112)))}", tree.to_string());
    tree.validate();
    tree.delete(&32);
    assert_eq!("RBTree{size:3,tree:((B:64),B:96,(B:112))}", tree.to_string());
    tree.validate();
}

// delete case 8
//
//          B64                     B32
//       ┌───┴───┐               ┌───┴───┐
//      B32     B96      =>     B16     B64
//    ┌──┘       ↑
//   R16        DEL
#[test]
fn test_delete_8_right() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_left(32, 16, RED);
    tree.insert_right(64, 96, BLACK);
    assert_eq!("RBTree{size:4,tree:(((R:16),B:32),B:64,(B:96))}", tree.to_string());
    tree.validate();
    tree.delete(&96);
    assert_eq!("RBTree{size:3,tree:((B:16),B:32,(B:64))}", tree.to_string());
    tree.validate();
}

// delete case 9
//
//        B64                      B80
//    ┌────┴────┐               ┌───┴───┐
//   B32       B96      =>     B64     B96
//    ↑      ┌──┘
//   DEL    R80
#[test]
fn test_delete_9_left() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_right(64, 96, BLACK);
    tree.insert_left(96, 80, RED);
    assert_eq!("RBTree{size:4,tree:((B:32),B:64,((R:80),B:96))}", tree.to_string());
    tree.validate();
    tree.delete(&32);
    assert_eq!("RBTree{size:3,tree:((B:64),B:80,(B:96))}", tree.to_string());
    tree.validate();
}


// delete case 9
//
//        B64                      B48
//    ┌────┴────┐               ┌───┴───┐
//   B32       B96      =>     B32     B64
//    └──┐      ↑
//      R48    DEL
#[test]
fn test_delete_9_right() {
    let mut tree: RBTree<KV32> = RBTree::new();
    tree.insert(&KV32::same(64));
    tree.insert_left(64, 32, BLACK);
    tree.insert_right(64, 96, BLACK);
    tree.insert_right(32, 48, RED);
    assert_eq!("RBTree{size:4,tree:((B:32,(R:48)),B:64,(B:96))}", tree.to_string());
    tree.validate();
    tree.delete(&96);
    assert_eq!("RBTree{size:3,tree:((B:32),B:48,(B:64))}", tree.to_string());
    tree.validate();
}
