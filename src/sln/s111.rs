/*
    111. Minimum Depth of Binary Tree
    Link: https://leetcode.com/problems/minimum-depth-of-binary-tree/description/

    Easy

    Given a binary tree, find its minimum depth.
    The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
    Note: A leaf is a node with no children.

    Example 1:
    Input: root = [3,9,20,null,null,15,7]
    Output: 2

    Example 2:
    Input: root = [2,null,3,null,4,null,5,null,6]
    Output: 5

    Constraints:
    The number of nodes in the tree is in the range [0, 105].
    -1000 <= Node.val <= 1000
*/

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
use std::collections::VecDeque;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            queue.push_back((node, 1));
        }

        while !queue.is_empty() {
            let (curr_node, level) = queue.pop_front().unwrap();
            let mut pushed: bool = false;

            let left = curr_node.borrow().left.clone();
            if left.is_some() {
                queue.push_back((left.unwrap(), level + 1));
                pushed = true;
            }
            let right = curr_node.borrow().right.clone();
            if right.is_some() {
                queue.push_back((right.unwrap(), level + 1));
                pushed = true;
            }

            if pushed == false {
                return (level as i32);
            }
        }

        return (0 as i32); 
    }
}