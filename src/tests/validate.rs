use crate::{Node, NodePtr, RBTree};

use super::KV32;

impl<N: Node> RBTree<N> {

    pub(crate) fn validate(&self) -> usize {
        if !self.root.is_nil() && !self.root.node().is_black() {
            panic!("The root node should be BLACK!");
        }
        self.validate_node(&self.root, None)
    }

    fn validate_node(&self, node_ptr: &N::Ptr, parent_ptr: Option<&N::Ptr>) -> usize {
        if node_ptr.is_nil() {
            return 1;
        }
        let node = node_ptr.node();
        if parent_ptr.is_some() && parent_ptr.unwrap().is_red() && node.is_red() {
            panic!("A node ({:?}) and its parent are both RED!", node.key());
        }
        if !node.left().is_nil() {
            let left_key = node.left().node().key();
            if node.key().le(left_key) {
                panic!("A node ({:?}) is less than or equal to its left child ({:?})!", &node.key(), left_key);
            }
        }
        if !node.right().is_nil() {
            let right_key = node.right().node().key();
            if node.key().ge(right_key) {
                panic!("A node ({:?}) is greater than or equal to its right child ({:?})!", &node.key(), right_key);
            }
        }
        let black_depth = self.validate_node(node.left(), Some(node_ptr));
        if self.validate_node(node.right(), Some(node_ptr)) != black_depth {
            panic!("A node ({:?}) has variant black depth!", node.key());
        }
        if node.is_black() { black_depth + 1 } else { black_depth }
    }
}

#[cfg(test)]
mod tests {
    use crate::{NodePtr, RBTree};
    use crate::kv::Color::{BLACK, RED};

    use super::KV32;

    #[test]
    #[should_panic(expected = "The root node should be BLACK!")]
    fn test_validate_1() {
        let mut tree: RBTree<KV32> = RBTree::new();
        tree.insert(&KV32::same(64));
        tree.root.node_mut().color(RED);
        tree.validate();
    }

    #[test]
    #[should_panic(expected = "A node (16) and its parent are both RED!")]
    fn test_validate_2() {
        let mut tree: RBTree<KV32> = RBTree::new();
        tree.insert(&KV32::same(64));
        tree.root.node_mut().color(BLACK)
            .insert_left(&KV32::same(32).color(RED))
            .insert_left(&KV32::same(16).color(RED));
        println!("{}", tree);
        tree.validate();
    }

    #[test]
    #[should_panic(expected = "A node (64) is less than or equal to its left child (65)!")]
    fn test_validate_3() {
        let mut tree: RBTree<KV32> = RBTree::new();
        tree.insert(&KV32::same(64));
        tree.root.node_mut().color(BLACK)
            .insert_left(&KV32::same(65).color(RED));
        println!("{}", tree);
        tree.validate();
    }

    #[test]
    #[should_panic(expected = "A node (64) is less than or equal to its left child (66)!")]
    fn test_validate_4() {
        let mut tree: RBTree<KV32> = RBTree::new();
        tree.insert(&KV32::same(64));
        tree.root.node_mut().color(BLACK)
            .insert_left(&KV32::same(66).color(RED));
        println!("{}", tree);
        tree.validate();
    }

    #[test]
    #[should_panic(expected = "A node (64) is greater than or equal to its right child (64)!")]
    fn test_validate_5() {
        let mut tree: RBTree<KV32> = RBTree::new();
        tree.insert(&KV32::same(64));
        tree.root.node_mut().color(BLACK)
            .insert_right(&KV32::same(64).color(RED));
        println!("{}", tree);
        tree.validate();
    }

    #[test]
    #[should_panic(expected = "A node (64) is greater than or equal to its right child (63)!")]
    fn test_validate_6() {
        let mut tree: RBTree<KV32> = RBTree::new();
        tree.insert(&KV32::same(64));
        tree.root.node_mut().color(BLACK)
            .insert_right(&KV32::same(63).color(RED));
        println!("{}", tree);
        tree.validate();
    }

    #[test]
    #[should_panic(expected = "A node (32) has variant black depth!")]
    fn test_validate_7() {
        let mut tree: RBTree<KV32> = RBTree::new();
        tree.insert(&KV32::same(64));
        let root_node = tree.root.node_mut().color(BLACK);
        root_node.insert_left(&KV32::same(32)).color(RED);
        root_node.insert_right(&KV32::same(96)).color(RED);
        tree.search_for_update(32).insert_left(&KV32::same(16)).color(BLACK);
        println!("{}", tree);
        tree.validate();
    }
}