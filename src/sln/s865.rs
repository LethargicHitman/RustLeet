/*
    865. Smallest Subtree with all the Deepest Nodes
    Link: https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/

    Medium
    Given the root of a binary tree, the depth of each node is the shortest distance to the root.
    Return the smallest subtree such that it contains all the deepest nodes in the original tree.
    A node is called the deepest if it has the largest depth possible among any node in the entire tree.
    The subtree of a node is a tree consisting of that node, plus the set of all descendants of that node.

    Example 1:
    Input: root = [3,5,1,6,2,0,8,null,null,7,4]
    Output: [2,7,4]
    Explanation: We return the node with value 2, colored in yellow in the diagram.
    The nodes coloured in blue are the deepest nodes of the tree.
    Notice that nodes 5, 3 and 2 contain the deepest nodes in the tree but node 2 is the smallest subtree among them, so we return it.

    Example 2:
    Input: root = [1]
    Output: [1]
    Explanation: The root is the deepest node in the tree.

    Example 3:
    Input: root = [0,1,3,null,2]
    Output: [2]
    Explanation: The deepest node in the tree is 2, the valid subtrees are the subtrees of nodes 2, 1 and 0 but the subtree of node 2 is the smallest.

    Constraints:
    The number of nodes in the tree will be in the range [1, 500].
    0 <= Node.val <= 500
    The values of the nodes in the tree are unique.
*/
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
    // Helper function to find a path to a node with given value
    fn find_path(
        root: Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<Rc<RefCell<TreeNode>>>,
        k: i32
    ) -> bool {
        if let Some(node) = root {
            path.push(Rc::clone(&node));

            let val = node.borrow().val;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            if val == k ||
               Self::find_path(left, path, k) ||
               Self::find_path(right, path, k) {
                return true;
            }

            path.pop();
            return false;
        }

        false
    }

    // Find all the deepest nodes in the tree
    fn find_deepest_nodes(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> Vec<Rc<RefCell<TreeNode>>> {
        let mut deepest_nodes = Vec::new();
        let mut all_nodes = Vec::new();

        if root.is_none() {
            return deepest_nodes;
        }

        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back((node, 0));
        }

        while !queue.is_empty() {
            let (curr_node, level) = queue.pop_front().unwrap();

            let left = curr_node.borrow().left.clone();
            if left.is_some() {
                queue.push_back((left.unwrap(), level + 1));
            }

            let right = curr_node.borrow().right.clone();
            if right.is_some() {
                queue.push_back((right.unwrap(), level + 1));
            }

            all_nodes.push((curr_node, level));
        }

        // Get the last level
        let last_level = all_nodes.last().unwrap().1;

        // Collect all nodes at the deepest level
        for i in (0..all_nodes.len()).rev() {
            if all_nodes[i].1 != last_level {
                break;
            }
            deepest_nodes.push(Rc::clone(&all_nodes[i].0));
        }

        // // For debugging, print the values of deepest nodes
        // print!("Deepest Nodes: ");
        // for node in &deepest_nodes {
        //     print!("{} ", node.borrow().val);
        // }
        // println!();

        deepest_nodes
    }

    // Find lowest common ancestor of two nodes
    fn find_lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        n1: i32,
        n2: i32
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // println!("Try to find common ancestor for: {} and {}", n1, n2);

        if root.is_none() {
            return None;
        }

        let mut path1 = Vec::new();
        let mut path2 = Vec::new();

        // Try to find paths to both nodes
        if !Self::find_path(root.clone(), &mut path1, n1) ||
           !Self::find_path(root.clone(), &mut path2, n2) {
            return None;
        }

        // Find the point where paths diverge
        let mut lca = None;
        for i in 0..path1.len().min(path2.len()) {
            if i >= path2.len() || !Rc::ptr_eq(&path1[i], &path2[i]) {
                // println!("Common ancestor for: {} and {} is: {}", n1, n2, path1[i-1].borrow().val);
                lca = Some(Rc::clone(&path1[i-1]));
                break;
            }
            lca = Some(Rc::clone(&path1[i]));
        }

        lca
    }

    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let root_clone = root.clone();
        if let Some(node) = &root_clone {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            if left.is_none() && right.is_none() {
                return root;
            }
        }

        // Level order traverse the tree
        let deepest_nodes = Self::find_deepest_nodes(root.clone());

        // This should not happen
        if deepest_nodes.is_empty() {
            return None;
        }

        // If there's only one deepest node, return it
        if deepest_nodes.len() == 1 {
            return Some(Rc::clone(&deepest_nodes[0]));
        }

        // Find the lowest common ancestor of all deepest nodes
        let mut lca = Some(Rc::clone(&deepest_nodes[0]));

        for i in 1..deepest_nodes.len() {
            // If LCA is already the root, don't waste time
            if let Some(lca_node) = &lca {
                if let Some(root_node) = &root {
                    if Rc::ptr_eq(lca_node, root_node) {
                        return root;
                    }
                }
            }

            lca = Self::find_lowest_common_ancestor(
                root.clone(),
                lca.clone().as_ref().unwrap().borrow().val,
                deepest_nodes[i].borrow().val
            );
        }

        lca
    }
}