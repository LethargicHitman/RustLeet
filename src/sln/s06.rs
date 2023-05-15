/**
   6. Zigzag Conversion
   Link: https://leetcode.com/problems/zigzag-conversion/

   Medium

   The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

   P   A   H   N
   A P L S I I G
   Y   I   R
   And then read line by line: "PAHNAPLSIIGYIR"

   Write the code that will take a string and make this conversion given a number of rows:

   string convert(string s, int numRows);


   Example 1:

   Input: s = "PAYPALISHIRING", numRows = 3
   Output: "PAHNAPLSIIGYIR"

   Example 2:

   Input: s = "PAYPALISHIRING", numRows = 4
   Output: "PINALSIGYAHRPI"
   Explanation:
   P     I    N
   A   L S  I G
   Y A   H R
   P     I

   Example 3:

   Input: s = "A", numRows = 1
   Output: "A"


   Constraints:

   1 <= s.length <= 1000
   s consists of English letters (lower-case and upper-case), ',' and '.'.
   1 <= numRows <= 1000
*/
impl Solution {
    pub fn print2dvec(x: &Vec<Vec<u8>>) {
        for i in 0..x.len() {
            for j in 0..x[i].len() {
                print!("{} ", x[i][j] as char);
            }
            println!("");
        }
    }

    pub fn convert(s: String, num_rows: i32) -> String {
        if (num_rows == 1) {
            return s;
        }

        let mut x: Vec<Vec<u8>> = Vec::with_capacity(s.len() * num_rows as usize);

        for _i in 0..num_rows {
            let mut y: Vec<u8> = Vec::with_capacity(s.len());
            for _j in 0..s.len() {
                y.push(0);
            }
            x.push(y);
        }

        let mut down: bool = true;
        let xb = s.as_bytes();
        let mut res: Vec<u8> = Vec::with_capacity(s.len());
        let (mut row, mut col) = (0 as usize, 0 as usize);

        for i in xb {
            x[row][col] = *i;

            // if we are going down
            if (down) {
                row += 1;

                // check if we are at end and need to switch
                if (row == num_rows as usize) {
                    row -= 2;
                    col += 1;
                    down = false;
                }
            }
            // if we are going up zig-zag
            else {
                if (row == 0) {
                    row = 1;
                    down = true;
                    continue;
                }

                row -= 1;
                col += 1;
            }
        }

        // Solution::print2dvec(&x);

        for i in 0..x.len() {
            for j in 0..x[i].len() {
                if x[i][j] != 0 {
                    res.push(x[i][j]);
                }
            }
        }

        let final_str = String::from_utf8(res).expect("Found invalid format");

        return final_str;
    }
}
