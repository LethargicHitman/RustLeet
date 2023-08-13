/**
   53. Maximum Subarray
   Link: https://leetcode.com/problems/maximum-subarray/
   Medium

   Given an integer array nums, find the subarray with the largest sum, and return its sum.

   Example 1:
   Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
   Output: 6
   Explanation: The subarray [4,-1,2,1] has the largest sum 6.

   Example 2:
   Input: nums = [1]
   Output: 1
   Explanation: The subarray [1] has the largest sum 1.

   Example 3:
   Input: nums = [5,4,-1,7,8]
   Output: 23
   Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.


   Constraints:
   1 <= nums.length <= 105
   -104 <= nums[i] <= 104
**/
use std::cmp;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_sum: i32 = nums[0];
        let mut max_sum: i32 = nums[0];

        // Simple implementation of Kadane's algorithm
        for i in 1..nums.len() {
            current_sum = cmp::max(nums[i], current_sum + nums[i]);
            max_sum = cmp::max(current_sum, max_sum);
        }
        max_sum
    }
}
