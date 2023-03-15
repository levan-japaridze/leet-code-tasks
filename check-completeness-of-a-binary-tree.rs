use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut end_of_level = false;
        
        while let Some(node) = queue.pop_front() {
            let node = node.unwrap();
            let node_ref = node.borrow();
            
            if let Some(left) = &node_ref.left {
                if end_of_level {
                    return false;
                }
                queue.push_back(Some(Rc::clone(left)));
            } else {
                end_of_level = true;
            }
            
            if let Some(right) = &node_ref.right {
                if end_of_level {
                    return false;
                }
                queue.push_back(Some(Rc::clone(right)));
            } else {
                end_of_level = true;
            }
        }
        
        true
    }
}
