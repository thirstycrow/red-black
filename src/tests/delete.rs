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
