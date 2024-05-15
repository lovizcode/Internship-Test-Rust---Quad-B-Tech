use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left.as_ref());
            let right_depth = max_depth(node.right.as_ref());
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
    // Example usage:
    let mut root = TreeNode::new(3);
    let left = TreeNode::new(9);
    let mut right = TreeNode::new(20);
    let right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);

    right.left = Some(Box::new(right_left));
    right.right = Some(Box::new(right_right));
    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));

    println!("Maximum depth of the tree: {}", max_depth(Some(&Box::new(root))));
}
