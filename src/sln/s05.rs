/**
   5. Longest Palindromic Substring

   Medium

   Given a string s, return the longest
   palindromic substring in s.

   Example 1:

   Input: s = "babad"
   Output: "bab"
   Explanation: "aba" is also a valid answer.

   Example 2:

   Input: s = "cbbd"
   Output: "bb"


   Constraints:

   1 <= s.length <= 1000
   s consist of only digits and English letters.
*/

impl Solution {
    pub fn print2d_vec(x: &Vec<Vec<bool>>) {
        for i in 0..x.len() {
            for j in 0..x[i].len() {
                print!("{} ", x[i][j]);
            }
            println!();
        }
    }

    pub fn longest_palindrome(s: String) -> String {
        // Comparing in char format is costlier in rust
        // Converting to bytes makes it easier to traverse the array and compare
        let sb = s.as_bytes();

        let mut x: Vec<Vec<bool>> = Vec::with_capacity(s.len() * s.len());

        for i in 0..s.len() {
            let mut y: Vec<bool> = Vec::with_capacity(s.len());
            for j in 0..s.len() {
                y.push(false);
            }
            x.push(y);
        }

        // all chars are palindromes on their own
        for i in 0..s.len() {
            x[i][i] = true;
        }

        let mut start_index: usize = 0;
        let mut max_len: usize = 1;
        for i in 0..s.len() - 1 {
            if sb[i] == sb[i + 1] {
                x[i][i + 1] = true;
                start_index = i;
                max_len = 2;
            }
        }

        let mut k = 2;
        let mut palindrome: bool = true;
        while k <= s.len() {
            for i in 0..s.len() - k {
                if sb[i] == sb[i + k] {
                    x[i][i + k] = x[i + 1][i + k - 1];
                    if x[i][i + k] {
                        start_index = i;
                        max_len = k + 1;
                    }
                }
            }
            k += 1;
        }

        // println!("Start_index: {}, max_len: {}, big_string: {}", start_index, max_len, &s[start_index .. start_index + max_len]);

        // print the 2d vector
        // Solution::print2d_vec(&x);

        return (&s[start_index..start_index + max_len]).to_string();
    }
}
