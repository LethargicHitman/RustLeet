/**
   14. Longest Common Prefix
   Link: https://leetcode.com/problems/longest-common-prefix/

   Easy

   Write a function to find the longest common prefix string amongst an array of strings.

   If there is no common prefix, return an empty string "".

   Example 1:

   Input: strs = ["flower","flow","flight"]
   Output: "fl"
   Example 2:

   Input: strs = ["dog","racecar","car"]
   Output: ""
   Explanation: There is no common prefix among the input strings.

   Constraints:
   1 <= strs.length <= 200
   0 <= strs[i].length <= 200
   strs[i] consists of only lowercase English letters.
*/
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs_vec: Vec<Vec<u8>> = Vec::with_capacity(200 * 200);

        for i in 0..strs.len() {
            let x: Vec<u8> = strs[i].as_bytes().to_vec();
            strs_vec.push(x);
        }

        let mut max_len = 0;

        'outer: for col in 0..strs_vec[0].len() {
            '_inner: for row in 1..strs_vec.len() {
                if col >= strs_vec[row].len() || strs_vec[0][col] != strs_vec[row][col] {
                    break 'outer;
                }
            }
            max_len += 1;
        }

        return strs[0][0..max_len].to_string();
    }
}
