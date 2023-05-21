/**
   17. Letter Combinations of a Phone Number
   Link: https://leetcode.com/problems/letter-combinations-of-a-phone-number/

   Medium

   Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
   A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

   Example 1:
   Input: digits = "23"
   Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

   Example 2:
   Input: digits = ""
   Output: []

   Example 3:
   Input: digits = "2"
   Output: ["a","b","c"]


   Constraints:
   0 <= digits.length <= 4
   digits[i] is a digit in the range ['2', '9'].
*/
impl Solution {
    pub fn letter_combs_util(res: Vec<String>, idx: usize) -> Vec<String> {
        let ntcm: Vec<Vec<char>> = vec![
            vec![' '],                // 0
            vec![' '],                // 1
            vec!['a', 'b', 'c'],      // 2
            vec!['d', 'e', 'f'],      // 3
            vec!['g', 'h', 'i'],      // 4
            vec!['j', 'k', 'l'],      // 5
            vec!['m', 'n', 'o'],      // 6
            vec!['p', 'q', 'r', 's'], // 7
            vec!['t', 'u', 'v'],      // 8
            vec!['w', 'x', 'y', 'z'], // 9
        ];
        let x: Vec<char> = ntcm[idx].clone();
        let mut y: Vec<String> = Vec::new();

        // if the first char is being entered
        if res.len() == 0 {
            for i in 0..x.len() {
                y.push(format!("{}", x[i]));
            }
            return y;
        }

        for i in 0..res.len() {
            for j in 0..x.len() {
                y.push(format!("{}{}", res[i], x[j]));
            }
        }

        return y;
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for i in digits.chars() {
            res = Solution::letter_combs_util(res.clone(), (i as u8 - '0' as u8) as usize);
        }

        return res;
    }
}
