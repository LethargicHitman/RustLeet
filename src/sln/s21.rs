/**
   21. Merge Two Sorted Lists
   Link: https://leetcode.com/problems/merge-two-sorted-lists

   Easy

   You are given the heads of two sorted linked lists list1 and list2.
   Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
   Return the head of the merged linked list.

   Example 1:
   Input: list1 = [1,2,4], list2 = [1,3,4]
   Output: [1,1,2,3,4,4]

   Example 2:
   Input: list1 = [], list2 = []
   Output: []

   Example 3:
   Input: list1 = [], list2 = [0]
   Output: [0]


   Constraints:
   The number of nodes in both lists is in the range [0, 50].
   -100 <= Node.val <= 100
   Both list1 and list2 are sorted in non-decreasing order.
*/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
type List = Option<Box<ListNode>>;
impl Solution {
    pub fn merge_two_lists(list1: List, list2: List) -> List {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;

        while let (Some(node1), Some(node2)) = (list1.as_mut(), list2.as_mut()) {
            if node1.val < node2.val {
                current.next = list1.take();
                list1 = current.next.as_mut().unwrap().next.take();
            } else {
                current.next = list2.take();
                list2 = current.next.as_mut().unwrap().next.take();
            }
            current = current.next.as_mut().unwrap();
        }

        current.next = if list1.is_some() { list1 } else { list2 };

        dummy.next
    }
}
