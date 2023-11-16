/**
    32. Longest Valid Parentheses
    Link: https://leetcode.com/problems/longest-valid-parentheses/
    Hard

    Companies
    Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses
    substring.

    Example 1:
    Input: s = "(()"
    Output: 2
    Explanation: The longest valid parentheses substring is "()".

    Example 2:
    Input: s = ")()())"
    Output: 4
    Explanation: The longest valid parentheses substring is "()()".

    Example 3:
    Input: s = ""
    Output: 0

    Constraints:
    0 <= s.length <= 3 * 104
    s[i] is '(', or ')'.
**/
use std::collections::VecDeque;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // implement stack using a double ended vector queue aka VecDeque
        let mut stack: VecDeque<usize> = VecDeque::new();

        // variable to track the max length of the stack
        let mut max_len = 0;

        // variable to track the start of the valid parantheses
        let mut start = 0;

        // iterate through the string
        for (i, char) in s.chars().enumerate() {
            // match the incoming brace
            match char {
                // always push to stack if it is an opening brace
                '(' => stack.push_back(i),
                // pop from stack if it is a closing brace and calculate the length of the valid parantheses
                ')' => {
                    if let Some(top) = stack.pop_back() {
                        if stack.is_empty() {
                            // if the stack is empty, find out where the current valid parantheses starts
                            max_len = max_len.max(i - start + 1);
                        } else {
                            // Stack is not empty, find the length from the first valid point
                            // example: (()() here when i=4, the length would be 4 because stack.back.unwrap() would return 0
                            max_len = max_len.max(i - stack.back().unwrap());
                        }
                    } else {
                        start = i + 1;
                    }
                }
                // default case to make RUST compiler happy
                _ => {}
            }
        }

        max_len as i32
    }
}
