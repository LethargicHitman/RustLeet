/**
   18. 4Sum
   Link: https://leetcode.com/problems/4sum/

   Medium

   Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:

   0 <= a, b, c, d < n
   a, b, c, and d are distinct.
   nums[a] + nums[b] + nums[c] + nums[d] == target
   You may return the answer in any order.

   Example 1:
   Input: nums = [1,0,-1,0,-2,2], target = 0
   Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]

   Example 2:
   Input: nums = [2,2,2,2,2], target = 8
   Output: [[2,2,2,2]]


   Constraints:
   1 <= nums.length <= 200
   -10^9 <= nums[i] <= 10^9
   -10^9 <= target <= 10^9
*/
use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums_orig: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums_orig.len() < 4 {
            return vec![];
        }
        let mut hs: HashSet<Vec<i32>> = HashSet::new();
        let mut nums: Vec<i32> = nums_orig.clone();
        nums.sort();
        for i in 0..nums.len() - 3 {
            for j in i + 1..nums.len() - 2 {
                let mut l = j + 1;
                let mut r = nums.len() - 1;

                while l < r {
                    // println!("{}:{}, {}:{}, {}:{}, {}:{}", i, nums[i], j, nums[j], l, nums[l], r, nums[r]);
                    let mut curr_sum: i128 = i128::from(nums[i])
                        + i128::from(nums[j])
                        + i128::from(nums[l])
                        + i128::from(nums[r]);
                    if curr_sum as i32 == target {
                        if curr_sum as i32 as i128 == curr_sum {
                            hs.insert(vec![nums[i], nums[j], nums[l], nums[r]]);
                        }
                        l += 1;
                        r -= 1;
                    } else if (curr_sum as i32) < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }

        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in hs {
            res.push((*i).to_vec());
        }

        return res;
    }
}
