/**
    412. Fizz Buzz
    Link: https://leetcode.com/problems/fizz-buzz/
    Easy

    Given an integer n, return a string array answer (1-indexed) where:

    answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
    answer[i] == "Fizz" if i is divisible by 3.
    answer[i] == "Buzz" if i is divisible by 5.
    answer[i] == i (as a string) if none of the above conditions are true.

    Example 1:
    Input: n = 3
    Output: ["1","2","Fizz"]

    Example 2:
    Input: n = 5
    Output: ["1","2","Fizz","4","Buzz"]

    Example 3:
    Input: n = 15
    Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]

    Constraints:
    1 <= n <= 104
**/
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for i in (1 .. (n + 1)) {
            if i % 15 == 0 {
                res.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                res.push("Fizz".to_string());
            } else if i % 5 == 0 {
                res.push("Buzz".to_string());
            } else {
                res.push(i.to_string());
            }
        }
        res
    }
}