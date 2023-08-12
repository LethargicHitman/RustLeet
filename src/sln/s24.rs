/**
   24. Swap Nodes in Pairs
   Link: https://leetcode.com/problems/swap-nodes-in-pairs/

   Medium

   Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)

   Example 1:
   Input: head = [1,2,3,4]
   Output: [2,1,4,3]

   Example 2:
   Input: head = []
   Output: []

   Example 3:
   Input: head = [1]
   Output: [1]

   Constraints:
   The number of nodes in the list is in the range [0, 100].
   0 <= Node.val <= 100

**/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::fmt;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut curr = &mut dummy;

        while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
            let mut first = curr.next.take().unwrap();
            let mut second = first.next.take().unwrap();

            first.next = second.next.take();
            second.next = Some(first);

            curr.next = Some(second);
            curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        return dummy.next.take();
    }
}