/**
   300. Longest Increasing Subsequence
   Link: https://leetcode.com/problems/longest-increasing-subsequence/

   Medium
   Given an integer array nums, return the length of the longest strictly increasing subsequence.

   Example 1:
   Input: nums = [10,9,2,5,3,7,101,18]
   Output: 4
   Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.

   Example 2:
   Input: nums = [0,1,0,3,2,3]
   Output: 4

   Example 3:
   Input: nums = [7,7,7,7,7,7,7]
   Output: 1

   Constraints:
   1 <= nums.length <= 2500
   -104 <= nums[i] <= 104

   Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
*/
/*
/// DP approach
use std::cmp;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lis: Vec<i32> = vec![1; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            for j in i..nums.len() {
                if nums[i] < nums[j] {
                    lis[i] = cmp::max(1 + lis[j], lis[i]);
                }
            }
        }

        let mut res: i32 = lis[0];
        for i in 1..nums.len() {
            res = cmp::max(res, lis[i]);
        }

        res
    }
}
*/
/// O(n log(n)) solution
use std::cmp;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lis = Vec::new();
        for i in 0..nums.len() {
            // Binary search to find where the current element would sit in our LIS
            let mut l: usize = 0;
            let mut r: usize = lis.len();
            while l < r {
                let mid: usize = (l + r) / 2;
                if lis[mid] < nums[i] {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }

            if l == lis.len() {
                lis.push(nums[i]);
            } else {
                lis[l] = nums[i];
            }
        }

        return lis.len() as i32;
    }
}
