/**
   4. Median of Two Sorted Arrays
   Link: https://leetcode.com/problems/median-of-two-sorted-arrays/

   Hard

   Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

   The overall run time complexity should be O(log (m+n)).


   Example 1:

   Input: nums1 = [1,3], nums2 = [2]
   Output: 2.00000
   Explanation: merged array = [1,2,3] and median is 2.


   Example 2:

   Input: nums1 = [1,2], nums2 = [3,4]
   Output: 2.50000
   Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
*/
use std::cmp;
impl Solution {
    /*
        The solution to achieve O(log(m + n)) is maintaining equal number of elements on both LHS and RHS
        By doing this and checking if l1 < r2 and l2 < r1, we will arrive at the optimal answer
    */
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();

        // We always want num1 to be smaller
        if n1 > n2 {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let mut l = 0;
        let mut r = n1;
        let (mut l1, mut l2, mut r1, mut r2) = (0, 0, 0, 0);

        while l <= r {
            let cut1 = (l + r) / 2;
            let cut2 = ((n1 + n2 + 1) / 2) - cut1;

            l1 = if cut1 <= 0 { i32::MIN } else { nums1[cut1 - 1] };
            l2 = if cut2 <= 0 { i32::MIN } else { nums2[cut2 - 1] };
            r1 = if cut1 >= n1 { i32::MAX } else { nums1[cut1] };
            r2 = if cut2 >= n2 { i32::MAX } else { nums2[cut2] };

            // This means that all our elements on the left side is lesser than all our elements on the right
            // and number of elements on left side == number of elements on right side
            // So our median is here
            if l1 <= r2 && l2 <= r1 {
                // odd num of elements
                if ((n1 + n2) & 1) == 1 {
                    return cmp::max(l1, l2) as f64;
                }
                // even num of elements
                else {
                    return (cmp::max(l1, l2) as f64 + cmp::min(r1, r2) as f64) / 2.0;
                }
            }

            // if l1 > r2, then our right window should be narrowed
            if l1 > r2 {
                r = cut1 - 1;
            } else {
                l = cut1 + 1;
            }
        }
        return -1.0;
    }
}
