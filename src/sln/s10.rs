/**
   10. Regular Expression Matching
   Link: https://leetcode.com/problems/regular-expression-matching

   Hard

   Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

   '.' Matches any single character.​​​​
   '*' Matches zero or more of the preceding element.
   The matching should cover the entire input string (not partial).



   Example 1:

   Input: s = "aa", p = "a"
   Output: false
   Explanation: "a" does not match the entire string "aa".
   Example 2:

   Input: s = "aa", p = "a*"
   Output: true
   Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
   Example 3:

   Input: s = "ab", p = ".*"
   Output: true
   Explanation: ".*" means "zero or more (*) of any character (.)".


   Constraints:

   1 <= s.length <= 20
   1 <= p.length <= 20
   s contains only lowercase English letters.
   p contains only lowercase English letters, '.', and '*'.
   It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
*/

/**
 * FYI:: This is a recursive solution and even this can be optimized so much further.
 * This can also be solved using DP. You are welcome to update this, if you have a better solution :)
 */

impl Solution {
    pub fn is_char(x: u8) -> bool {
        let temp = x as i8 - 'a' as i8;
        if 0 <= temp && temp <= 25 {
            return true;
        }
        return false;
    }

    pub fn prints(s: &Vec<u8>, p: &Vec<u8>) {
        // print pattern and string
        print!("Pattern: ");
        for i in 0..p.len() {
            print!("{}", p[i] as char);
        }
        println!();
        print!("String: ");
        for i in 0..s.len() {
            print!("{}", s[i] as char);
        }
        println!();
    }

    pub fn is_match_util(s: Vec<u8>, p: Vec<u8>) -> bool {
        // Solution::prints(&s, &p);

        // base cases
        // if both pattern and string is empty, then we have achieved our pattern
        if p.len() == 0 && s.len() == 0 {
            return true;
        }

        // if pattern is empty and string is not
        if p.len() == 0 && s.len() != 0 {
            return false;
        }

        let dot = '.' as u8;
        let star = '*' as u8;

        // if string is empty and pattern is not, check if pattern only has *s
        if s.len() == 0 {
            if p.len() & 1 == 1 {
                return false;
            }
            for i in (0..p.len()).step_by(2) {
                if (Solution::is_char(p[i]) || p[i] == dot) && p[i + 1] == star {
                    continue;
                }
                return false;
            }
            return true;
        }

        // Case 1: Any alphabet as alphabet
        if Solution::is_char(p[0]) {
            // If pattern does not match the string, no point comparing further
            if s[0] != p[0] {
                if p.len() > 1 && p[1] == star {
                    return Solution::is_match_util(s, p[2..p.len()].to_vec());
                }
                return false;
            }

            // if pattern ends with the current char
            if p.len() == 1 {
                // if the string is longer than the pattern, no point comparing further
                // this will not match the regex
                if s.len() > 1 {
                    return false;
                }

                // if the string and pattern is of equal length, it is already a match, return true
                return true;
            }

            // Case 1a: just the char
            if p[1] != star {
                // char matches with the string
                return Solution::is_match_util(s[1..s.len()].to_vec(), p[1..p.len()].to_vec());
            }

            // Case 1b: the char followed by a star
            if p[1] == star {
                if Solution::is_match_util(s[..].to_vec(), p[2..p.len()].to_vec()) {
                    return true;
                }

                for i in 0..s.len() {
                    // if the character matches with the pattern character
                    // try all possible regex combinations
                    if s[i] == p[0] {
                        if Solution::is_match_util(
                            s[i + 1..s.len()].to_vec(),
                            p[2..p.len()].to_vec(),
                        ) {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
                return false;
            }
        }

        // Case 2: Any alphabet as .
        if p[0] == dot {
            // If pattern does not match the string, no point comparing further
            if Solution::is_char(s[0]) == false {
                return false;
            }

            // if pattern ends with the current char
            if p.len() == 1 {
                // if the string is longer than the pattern, no point comparing further
                // this will not match the regex
                if s.len() > 1 {
                    return false;
                }

                // if the string and pattern is of equal length, it is already a match, return true
                return true;
            }

            // Case 2a: just the dot
            if p[1] != star {
                // char matches with the string
                return Solution::is_match_util(s[1..s.len()].to_vec(), p[1..p.len()].to_vec());
            }

            // Case 2b: the dot followed by a star
            if p[1] == star {
                if Solution::is_match_util(s[..].to_vec(), p[2..p.len()].to_vec()) {
                    return true;
                }
                for i in 0..s.len() {
                    // if the character matches with the pattern character
                    // try all possible regex combinations
                    if Solution::is_char(s[i]) {
                        if Solution::is_match_util(
                            s[i + 1..s.len()].to_vec(),
                            p[2..p.len()].to_vec(),
                        ) {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
                return false;
            }
        }

        return false;
    }
    pub fn is_match(s: String, p: String) -> bool {
        let su8: Vec<u8> = s.as_bytes().to_vec();
        let pu8: Vec<u8> = p.as_bytes().to_vec();
        return Solution::is_match_util(su8, pu8);
    }
}
