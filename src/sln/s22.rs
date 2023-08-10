/**
   22. Generate Parentheses
   Link: https://leetcode.com/problems/generate-parentheses

   Medium
   Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

   Example 1:
   Input: n = 3
   Output: ["((()))","(()())","(())()","()(())","()()()"]

   Example 2:
   Input: n = 1
   Output: ["()"]

   Constraints:
   1 <= n <= 8
*/
impl Solution {
    pub fn util(current_string: &String, open: i32, close: i32, vec: &mut Vec<String>) {
        let open_brace = String::from("(");
        let close_brace = String::from(")");

        // base condition
        if open == 0 && close == 0 {
            vec.push(current_string.to_string());
        }

        // edge case for returns
        if open < 0 || close < 0 {
            return;
        }

        let new_string1 = current_string.to_owned() + &open_brace;
        Solution::util(&new_string1, open - 1, close, vec);
        if open != close {
            let new_string2 = current_string.to_owned() + &close_brace;
            Solution::util(&new_string2, open, close - 1, vec);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let current_string: String = String::new();
        Solution::util(&current_string, n, n, &mut result);
        return result;
    }
}
