/**
    31. Next Permutation
    Link: https://leetcode.com/problems/next-permutation/
    Medium

    A permutation of an array of integers is an arrangement of its members into a sequence or linear order.

    For example, for arr = [1,2,3], the following are all the permutations of arr: [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
    The next permutation of an array of integers is the next lexicographically greater permutation of its integer. More formally, if all the permutations of the array are sorted in one container according to their lexicographical order, then the next permutation of that array is the permutation that follows it in the sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).

    For example, the next permutation of arr = [1,2,3] is [1,3,2].
    Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
    While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.
    Given an array of integers nums, find the next permutation of nums.

    The replacement must be in place and use only constant extra memory.

    Example 1:
    Input: nums = [1,2,3]
    Output: [1,3,2]

    Example 2:
    Input: nums = [3,2,1]
    Output: [1,2,3]

    Example 3:
    Input: nums = [1,1,5]
    Output: [1,5,1]

    Constraints:
    1 <= nums.length <= 100
    0 <= nums[i] <= 100
**/
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32 - 2;

        // Step 1: Find the first decreasing element
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }

        if i >= 0 {
            let mut j = nums.len() as i32 - 1;
            // Step 2: Find the element to swap
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            // Step 3: Swap
            nums.swap(i as usize, j as usize);
        }

        // Step 4: Reverse the sub-array
        nums[(i as usize + 1)..].reverse();
    }
}
