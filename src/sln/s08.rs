/**
   8. String to Integer (atoi)
   link: https://leetcode.com/problems/string-to-integer-atoi/

   Medium

   Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).

   The algorithm for myAtoi(string s) is as follows:

   Read in and ignore any leading whitespace.
   Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
   Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
   Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
   If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
   Return the integer as the final result.
   Note:

   Only the space character ' ' is considered a whitespace character.
   Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.


   Example 1:

   Input: s = "42"
   Output: 42
   Explanation: The underlined characters are what is read in, the caret is the current reader position.
   Step 1: "42" (no characters read because there is no leading whitespace)
           ^
   Step 2: "42" (no characters read because there is neither a '-' nor '+')
           ^
   Step 3: "42" ("42" is read in)
             ^
   The parsed integer is 42.
   Since 42 is in the range [-231, 231 - 1], the final result is 42.

   Example 2:

   Input: s = "   -42"
   Output: -42
   Explanation:
   Step 1: "   -42" (leading whitespace is read and ignored)
               ^
   Step 2: "   -42" ('-' is read, so the result should be negative)
               ^
   Step 3: "   -42" ("42" is read in)
                 ^
   The parsed integer is -42.
   Since -42 is in the range [-231, 231 - 1], the final result is -42.

   Example 3:

   Input: s = "4193 with words"
   Output: 4193
   Explanation:
   Step 1: "4193 with words" (no characters read because there is no leading whitespace)
           ^
   Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
           ^
   Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
               ^
   The parsed integer is 4193.
   Since 4193 is in the range [-231, 231 - 1], the final result is 4193.


   Constraints:

   0 <= s.length <= 200
   s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
*/

#[derive(PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
}

impl Solution {
    pub fn is_num(c: u8) -> Option<u8> {
        let x = c - '0' as u8;
        if 0 <= x && x <= 9 {
            return Some(x);
        }
        return None;
    }

    pub fn is_sign(c: u8) -> Option<Sign> {
        if c as char == '+' {
            return Some(Sign::Positive);
        }
        if c as char == '-' {
            return Some(Sign::Negative);
        }
        return None;
    }

    pub fn my_atoi(s: String) -> i32 {
        let mut x = s.trim().as_bytes();
        let mut sign: Sign = Sign::Positive;
        let mut num_found = false;
        let mut big_num: i128 = 0;

        for i in x {
            // check if it is a sign
            match Solution::is_sign(*i) {
                Some(temp) => {
                    if num_found {
                        break;
                    }
                    sign = temp;
                    num_found = true;
                    continue;
                }
                None => {}
            }

            // if you find a number make both sign and number irrelevant
            match Solution::is_num(*i) {
                Some(temp) => {
                    num_found = true;
                    big_num = (big_num * 10) + temp as i128;
                    continue;
                }
                None => {
                    break;
                }
            }
        }

        big_num = if (sign == Sign::Negative) {
            -big_num
        } else {
            big_num
        };

        if big_num != big_num as i32 as i128 {
            if sign == Sign::Negative {
                return i32::MIN;
            } else {
                return i32::MAX;
            }
        }

        return big_num as i32;
    }
}
