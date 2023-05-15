/**
   9. Palindrome Number
   Link: https://leetcode.com/problems/palindrome-number/

   Easy

   Given an integer x, return true if x is a
   palindrome, and false otherwise.



   Example 1:

   Input: x = 121
   Output: true
   Explanation: 121 reads as 121 from left to right and from right to left.
   Example 2:

   Input: x = -121
   Output: false
   Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
   Example 3:

   Input: x = 10
   Output: false
   Explanation: Reads 01 from right to left. Therefore it is not a palindrome.


   Constraints:

   -231 <= x <= 231 - 1


   Follow up: Could you solve it without converting the integer to a string?
*/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut n1: i32 = x;
        let mut n2: i32 = 0;

        while n1 > 0 {
            n2 = (n2 * 10) + (n1 % 10);
            n1 = n1 / 10;
        }

        if n2 == x {
            return true;
        }

        return false;
    }
}
