/**
   20. Valid Parentheses
   Link: https://leetcode.com/problems/valid-parentheses/

   Easy

   Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

   An input string is valid if:

   Open brackets must be closed by the same type of brackets.
   Open brackets must be closed in the correct order.
   Every close bracket has a corresponding open bracket of the same type.

   Example 1:
   Input: s = "()"
   Output: true

   Example 2:
   Input: s = "()[]{}"
   Output: true

   Example 3:
   Input: s = "(]"
   Output: false


   Constraints:

   1 <= s.length <= 104
   s consists of parentheses only '()[]{}'.
*/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v: Vec<u8> = Vec::new();
        let sb = s.as_bytes();
        let o1 = '{' as u8;
        let o2 = '[' as u8;
        let o3 = '(' as u8;
        let c1 = '}' as u8;
        let c2 = ']' as u8;
        let c3 = ')' as u8;

        for i in 0..sb.len() {
            if sb[i] == o1 || sb[i] == o2 || sb[i] == o3 {
                v.push(sb[i]);
            } else {
                let x = v[v.len() - 1];
                if (x == o1 && sb[i] == c1) || (x == o2 && sb[i] == c2) || (x == o3 && sb[i] == c3)
                {
                    v.pop();
                    continue;
                } else {
                    return false;
                }
            }
        }

        return true;
    }
}
