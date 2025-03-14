/**
    47. Permutations II
    Link: https://leetcode.com/problems/permutations-ii/

    Medium

    Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.

    Example 1:
    Input: nums = [1,1,2]
    Output:
    [[1,1,2],
    [1,2,1],
    [2,1,1]]

    Example 2:
    Input: nums = [1,2,3]
    Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

    Constraints:
    1 <= nums.length <= 8
    -10 <= nums[i] <= 10
**/

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false; nums.len()];

        nums.sort(); // Sort first to handle duplicates efficiently
        Self::backtrack(&nums, &mut used, &mut path, &mut res);

        res
    }

    fn backtrack(nums: &Vec<i32>, used: &mut Vec<bool>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue; // Skip if already used in this permutation
            }
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue; // Skip duplicates at the same level
            }

            // Choose
            used[i] = true;
            path.push(nums[i]);

            // Explore
            Self::backtrack(nums, used, path, res);

            // Un-choose
            path.pop();
            used[i] = false;
        }
    }
}
