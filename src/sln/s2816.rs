/**
   2816. Double a Number Represented as a Linked List
   Link: https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/

   Medium

   You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.

   Return the head of the linked list after doubling it.

   Example 1:
   Input: head = [1,8,9]
   Output: [3,7,8]
   Explanation: The figure above corresponds to the given linked list which represents the number 189. Hence, the returned linked list represents the number 189 * 2 = 378.

   Example 2:
   Input: head = [9,9,9]
   Output: [1,9,9,8]
   Explanation: The figure above corresponds to the given linked list which represents the number 999. Hence, the returned linked list reprersents the number 999 * 2 = 1998.

   Constraints:
   The number of nodes in the list is in the range [1, 104]
   0 <= Node.val <= 9
   The input is generated such that the list represents a number that does not have leading zeros, except the number 0 itself.
*/

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
impl Solution {
    pub fn util(node: &mut Option<Box<ListNode>>) -> i32 {
        let mut carry: i32 = 0;
        if *node != None {
            carry = Solution::util(&mut node.as_mut().unwrap().next);
            let temp = (node.as_mut().unwrap().val * 2) + carry;
            node.as_mut().unwrap().val = temp % 10;
            carry = temp / 10;
        }
        return carry;
    }

    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let carry = Solution::util(&mut head);
        if carry == 1 {
            let mut new_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
            new_head.as_mut().unwrap().next = head;
            return new_head;
        }
        return head;
    }
}
