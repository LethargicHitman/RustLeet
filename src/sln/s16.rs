/**
   16. 3Sum Closest
   Link: https://leetcode.com/problems/3sum-closest/

   Medium

   Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.

   Return the sum of the three integers.

   You may assume that each input would have exactly one solution.



   Example 1:

   Input: nums = [-1,2,1,-4], target = 1
   Output: 2
   Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
   Example 2:

   Input: nums = [0,0,0], target = 1
   Output: 0
   Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).


   Constraints:

   3 <= nums.length <= 500
   -1000 <= nums[i] <= 1000
   -104 <= target <= 104
*/
impl Solution {
    pub fn three_sum_closest(nums_orig: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums_orig.clone();
        nums.sort();
        if nums.len() < 3 {
            return -1i32;
        }
        let mut closest_sum = nums[0] + nums[1] + nums[2];

        for i in 0..nums.len() - 2 {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let x = nums[i] + nums[l] + nums[r];
                if x == target {
                    return target;
                } else if (x - target).abs() < (closest_sum - target).abs() {
                    closest_sum = x;
                }

                if x < target {
                    l += 1;
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                } else {
                    r -= 1;
                }
            }
        }

        return closest_sum;
    }
}
