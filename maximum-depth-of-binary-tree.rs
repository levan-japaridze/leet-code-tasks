// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = Vec::new();
        let mut node_depths = Vec::new();
        let mut max_depth = 0;

        if let Some(node) = root {
            stack.push(node);
            node_depths.push(1);
        }

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            let node_depth = node_depths.pop().unwrap();
            max_depth = max_depth.max(node_depth);

            {
                let borrow = node.borrow();
                if let Some(left) = borrow.left.clone() {
                    let left_node = left.clone();
                    stack.push(left_node);
                    node_depths.push(node_depth + 1);
                }

                if let Some(right) = borrow.right.clone() {
                    let right_node = right.clone();
                    stack.push(right_node);
                    node_depths.push(node_depth + 1);
                }
            }
        }

        max_depth
    }
}
