/**
   28. Find the Index of the First Occurrence in a String
   Link: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
   Easy

   Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

   Example 1:
   Input: haystack = "sadbutsad", needle = "sad"
   Output: 0
   Explanation: "sad" occurs at index 0 and 6.
   The first occurrence is at index 0, so we return 0.

   Example 2:
   Input: haystack = "leetcode", needle = "leeto"
   Output: -1
   Explanation: "leeto" did not occur in "leetcode", so we return -1.

   Constraints:
   1 <= haystack.length, needle.length <= 104
   haystack and needle consist of only lowercase English characters.
**/
impl Solution {
    fn hash_string(bytes: &[u8], len: usize) -> u64 {
        let mut hash = 0;
        for i in 0..len {
            hash = hash + bytes[i] as u64;
        }
        hash
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        const PRIME: u64 = 101;
        let haystack_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let haystack_len = haystack.len();
        let needle_len = needle.len();

        if haystack_len < needle_len {
            return -1;
        }

        let needle_hash = Solution::hash_string(&needle_bytes, needle_len);
        let mut haystack_hash = Solution::hash_string(&haystack_bytes[..needle_len], needle_len);

        for i in 0..=(haystack_len - needle_len) {
            if haystack_hash == needle_hash && &haystack[i..i + needle_len] == needle.as_str() {
                return i as i32;
            }

            if i + needle_len < haystack_len {
                haystack_hash = haystack_hash - haystack_bytes[i] as u64
                    + haystack_bytes[i + needle_len] as u64;
            }
        }

        // default return case if nothing matches
        -1
    }
}
