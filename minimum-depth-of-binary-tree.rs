use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::VecDeque;

pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let mut queue = VecDeque::new();
                queue.push_back((1, Rc::clone(&root)));

                while let Some((depth, node)) = queue.pop_front() {
                    let node = node.borrow();

                    if node.left.is_none() && node.right.is_none() {
                        return depth;
                    }

                    if let Some(left) = &node.left {
                        queue.push_back((depth + 1, Rc::clone(left)));
                    }

                    if let Some(right) = &node.right {
                        queue.push_back((depth + 1, Rc::clone(right)));
                    }
                }

                0
            },
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_depth() {
        let mut root = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let mut right = TreeNode::new(20);
        right.left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.right = Some(Rc::new(RefCell::new(right)));
        assert_eq!(Solution::min_depth(Some(Rc::new(RefCell::new(root)))), 2);

        let mut root = TreeNode::new(2);
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.right.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        root.right.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        assert_eq!(Solution::min_depth(Some(Rc::new(RefCell::new(root)))), 5);
    }
}

