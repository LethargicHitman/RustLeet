/**
    29. Divide Two Integers
    Link: https://leetcode.com/problems/divide-two-integers/
    Medium

    Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.

    The integer division should truncate toward zero, which means losing its fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be truncated to -2.

    Return the quotient after dividing dividend by divisor.

    Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [−2^31, 2^31 − 1]. For this problem, if the quotient is strictly greater than 2^31 - 1, then return 2^31 - 1, and if the quotient is strictly less than -2^31, then return -2^31.

    Example 1:
    Input: dividend = 10, divisor = 3
    Output: 3
    Explanation: 10/3 = 3.33333.. which is truncated to 3.

    Example 2:
    Input: dividend = 7, divisor = -3
    Output: -2
    Explanation: 7/-3 = -2.33333.. which is truncated to -2.

    Constraints:
    -2^31 <= dividend, divisor <= 2^31 - 1
    divisor != 0
**/
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut sign = 1;

        // calculate the sign
        if (dividend < 0) ^ (divisor < 0) {
            sign = -1;
        }

        // Get the absolute value of the dividend and the divisor
        let dividend = dividend.abs() as u32 as u64;
        let divisor = divisor.abs() as u32 as u64;

        // if dividend is lesser than the divisor return 0 straight away
        if sign == 1 && dividend < divisor {
            return 0;
        }

        // if the dividend is equal to the divisor return sign
        // 2/2 = 1, 2/-2 = -1, -2/2 = -1, -2/-2 = 1
        if dividend == divisor {
            return sign;
        }

        // Setup quotient and temporary variables
        let mut temp: u64 = 0;
        let mut quotient: i64 = 0;

        // Iterate over each of the bits
        for i in (0..32).rev() {
            if (temp + (divisor << i)) <= dividend {
                temp += divisor << i;
                quotient |= (1 << i);
            }
        }

        // fix the sign
        if sign == -1 {
            quotient = -quotient;
        }

        // corner cases
        if quotient > 2147483647 {
            return 2147483647;
        } else if quotient < -2147483648 {
            return -2147483648;
        }

        // return the quotient as i32
        quotient as i32
    }
}
