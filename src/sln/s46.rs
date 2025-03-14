/**
    46. Permutations
    Link: https://leetcode.com/problems/permutations/description/

    Medium

    Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.

    Example 1:
    Input: nums = [1,2,3]
    Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

    Example 2:
    Input: nums = [0,1]
    Output: [[0,1],[1,0]]

    Example 3:
    Input: nums = [1]
    Output: [[1]]

    Constraints:
    1 <= nums.length <= 6
    -10 <= nums[i] <= 10

    All the integers of nums are unique.
**/

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        if nums.len() == 1 {
            return vec![nums];
        }

        let mut res = Vec::new();

        for (i, &curr_num) in nums.iter().enumerate() {
            let mut rem_vec = nums[..i].to_vec();
            rem_vec.extend_from_slice(&nums[i + 1..]);

            for mut perm in Solution::permute(rem_vec) {
                perm.insert(0, curr_num);
                res.push(perm);
            }
        }

        res
    }
}
