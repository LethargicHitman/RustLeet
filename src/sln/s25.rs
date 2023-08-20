/**
   25. Reverse Nodes in k-Group
   Link: https://leetcode.com/problems/reverse-nodes-in-k-group/

   Hard

   Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
   k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
   You may not alter the values in the list's nodes, only nodes themselves may be changed.

   Example 1:
   Input: head = [1,2,3,4,5], k = 2
   Output: [2,1,4,3,5]

   Example 2:
   Input: head = [1,2,3,4,5], k = 3
   Output: [3,2,1,4,5]

   Constraints:
   The number of nodes in the list is n.
   1 <= k <= n <= 5000
   0 <= Node.val <= 1000

   Follow-up: Can you solve the problem in O(1) extra memory space?
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
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        dummy_head.as_mut().unwrap().next = head;
        let mut head = dummy_head.as_mut();

        'outer: loop {
            let mut start = head.as_mut().unwrap().next.take();
            if start.is_none() {
                break 'outer;
            }

            let mut end = start.as_mut();
            for i in 0..(k - 1) {
                end = end.unwrap().next.as_mut();
                if end.is_none() {
                    head.as_mut().unwrap().next = start;
                    break 'outer;
                }
            }

            // the take() in the below line makes start a linked list with k elements
            let mut tail = end.unwrap().next.take();
            let end = Solution::reverse(start, tail);
            head.as_mut().unwrap().next = end;
            for _ in (0..k) {
                head = head.unwrap().next.as_mut();
            }
        }

        dummy_head.unwrap().next
    }

    fn reverse(
        mut start: Option<Box<ListNode>>,
        end: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev = end;
        let mut curr = start;
        while let Some(mut node) = curr {
            let mut next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            curr = next;
        }
        prev
    }
}
