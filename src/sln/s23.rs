/**
   23. Merge k Sorted Lists
   Hard

   You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.

   Merge all the linked-lists into one sorted linked-list and return it.

   Example 1:
   Input: lists = [[1,4,5],[1,3,4],[2,6]]
   Output: [1,1,2,3,4,4,5,6]
   Explanation: The linked-lists are:
   [
   1->4->5,
   1->3->4,
   2->6
   ]
   merging them into one sorted list:
   1->1->2->3->4->4->5->6

    Example 2:
   Input: lists = []
   Output: []

   Example 3:
   Input: lists = [[]]
   Output: []


   Constraints:
   k == lists.length
   0 <= k <= 104
   0 <= lists[i].length <= 500
   -104 <= lists[i][j] <= 104

   lists[i] is sorted in ascending order.
   The sum of lists[i].length will not exceed 104.
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

use std::collections::BinaryHeap;
use std::cmp::Ordering;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap: BinaryHeap<ListNode> = BinaryHeap::new();

        // Push the heads of each linked list onto the min-heap
        for list in lists {
            if let Some(node) = list {
                min_heap.push(*node);
            }
        }

        // Create a dummy node to build the merged list
        let mut dummy = ListNode { val: 0, next: None };
        let mut current = &mut dummy;

        // Continue until the min-heap is empty
        while let Some(mut node) = min_heap.pop() {
            current.next = Some(Box::new(ListNode { val: node.val, next: None }));
            current = current.next.as_mut().unwrap();

            if let Some(next) = node.next.take() {
                min_heap.push(*next);
            }
        }
        dummy.next
    }
}