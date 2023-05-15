/**
    7. Reverse Integer

    Medium


    Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

    Assume the environment does not allow you to store 64-bit integers (signed or unsigned).



    Example 1:

    Input: x = 123
    Output: 321
    Example 2:

    Input: x = -123
    Output: -321
    Example 3:

    Input: x = 120
    Output: 21


    Constraints:

    -2^31 <= x <= 2^31 - 1
*/

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut sign: bool = if x >= 0 { true } else { false };
        let mut y: i64 = x.abs() as i64;
        let mut z: i64 = 0;

        while y > 0 {
            z = (z * 10) + (y % 10);
            y = y / 10;
        }

        if z != z as i32 as i64 {
            return 0;
        }

        let mut res: i32 = z as i32;

        if sign {
            return res;
        } else {
            return -res;
        }
    }
}
