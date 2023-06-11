/**
    19. Remove Nth Node From End of List
    Link: https://leetcode.com/problems/remove-nth-node-from-end-of-list/

    Medium
    Given the head of a linked list, remove the nth node from the end of the list and return its head.

    Example 1:
    Input: head = [1,2,3,4,5], n = 2
    Output: [1,2,3,5]

    Example 2:
    Input: head = [1], n = 1
    Output: []

    Example 3:
    Input: head = [1,2], n = 1
    Output: [1]


    Constraints:
    The number of nodes in the list is sz.
    1 <= sz <= 30
    0 <= Node.val <= 100
    1 <= n <= sz
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut length = 0;

        {
            let mut curr = dummy_head.as_ref();
            while curr.unwrap().next.is_some() {
                curr = curr.unwrap().next.as_ref();
                length += 1;
            }
        }

        let index = length - n;

        {
            let mut curr = dummy_head.as_mut();

            for _ in 0..index {
                curr = curr.unwrap().next.as_mut();
            }

            let next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = next;
        }

        return dummy_head.unwrap().next;
    }
}
