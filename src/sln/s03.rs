/**
    3. Longest Substring Without Repeating Characters

    Given a string s, find the length of the longest 
    substring
    without repeating characters.

    

    Example 1:

    Input: s = "abcabcbb"
    Output: 3
    Explanation: The answer is "abc", with the length of 3.
    Example 2:

    Input: s = "bbbbb"
    Output: 1
    Explanation: The answer is "b", with the length of 1.
    Example 3:

    Input: s = "pwwkew"
    Output: 3
    Explanation: The answer is "wke", with the length of 3.
    Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
*/

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s == "" {
            return 0;
        }


        let mut alph_map: [i32; 256] = [-1; 256];

        let mut curr_unchanged: i32 = 1;
        let mut max_unchanged: i32 = 1;
        let mut prev_unchanged: i32 = 0;

        alph_map[s.chars().nth(0).unwrap() as i32 as usize] = 0;

        for (index, chr) in s[1 .. s.len()].chars().enumerate() {
            let mut x: usize = chr as i32 as usize;
            // println!("index: {}, current_char: {}, prev unchanged index: {}, curr unchanged len: {}, max unchanged len: {}", index + 1, chr, prev_unchanged, curr_unchanged, max_unchanged);

            // println!("current lswrc: {}", s[prev_unchanged as usize .. (prev_unchanged + curr_unchanged) as usize]);
            if alph_map[x] < prev_unchanged {
                curr_unchanged += 1;
                // println!("Incrementing lswrc: {}", curr_unchanged);
            } else {
                prev_unchanged = alph_map[x] + 1;
                curr_unchanged = (index + 1) as i32 - prev_unchanged + 1;
                // println!("Breaking lswrc: curr_unchanged_len: {}, prev_unchanged: {}", curr_unchanged, prev_unchanged);
            }
            alph_map[x] = (index + 1) as i32;
            if curr_unchanged > max_unchanged {
                max_unchanged = curr_unchanged;
            }
        }

        return max_unchanged;     
    }
}