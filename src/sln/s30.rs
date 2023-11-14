/**
    30. Substring with Concatenation of All Words
    Link: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
    Hard

    You are given a string s and an array of strings words. All the strings of words are of the same length.
    A concatenated substring in s is a substring that contains all the strings of any permutation of words concatenated.

    For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings. "acdbef" is not a concatenated substring because it is not the concatenation of any permutation of words.
    Return the starting indices of all the concatenated substrings in s. You can return the answer in any order.

    Example 1:
    Input: s = "barfoothefoobarman", words = ["foo","bar"]
    Output: [0,9]
    Explanation: Since words.length == 2 and words[i].length == 3, the concatenated substring has to be of length 6.
    The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
    The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.
    The output order does not matter. Returning [9,0] is fine too.

    Example 2:
    Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
    Output: []
    Explanation: Since words.length == 4 and words[i].length == 4, the concatenated substring has to be of length 16.
    There is no substring of length 16 in s that is equal to the concatenation of any permutation of words.
    We return an empty array.

    Example 3:
    Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
    Output: [6,9,12]
    Explanation: Since words.length == 3 and words[i].length == 3, the concatenated substring has to be of length 9.
    The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"] which is a permutation of words.
    The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"] which is a permutation of words.
    The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"] which is a permutation of words.

    Constraints:
    1 <= s.length <= 104
    1 <= words.length <= 5000
    1 <= words[i].length <= 30
    s and words[i] consist of lowercase English letters.
**/
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut results = Vec::new();

        // Return empty vector if input string or words are empty
        if s.is_empty() || words.is_empty() {
            return results;
        }

        // Length of each word and total number of words
        let word_length = words[0].len();
        let total_words = words.len();

        // HashMap to store frequency of each word in 'words'
        let mut words_dict: HashMap<&String, i32> = HashMap::new();
        for word in &words {
            *words_dict.entry(word).or_insert(0) += 1;
        }

        // Iterate through the string 's' with different starting points
        for i in 0..word_length {
            let mut start = i; // Start index of the sliding window
            let mut end = i; // End index of the sliding window
            let mut current_dict = HashMap::new(); // HashMap to keep track of words in the current window

            // Extend the window
            while end + word_length <= s.len() {
                let word = &s[end..end + word_length];
                end += word_length;

                // If the word is not part of 'words', reset the current window
                if !words_dict.contains_key(&word.to_string()) {
                    current_dict.clear();
                    start = end;
                } else {
                    // Add the word to the current window's word count
                    *current_dict.entry(word.to_string()).or_insert(0) += 1;

                    // If there are more occurrences of the word than required, shrink the window
                    while current_dict[&word.to_string()] > words_dict[&word.to_string()] {
                        let left_word = &s[start..start + word_length];
                        *current_dict.get_mut(left_word).unwrap() -= 1;
                        start += word_length;
                    }

                    // If the current window's size matches the total length of all words, add the start index to results
                    if end - start == word_length * total_words {
                        results.push(start as i32);
                    }
                }
            }
        }

        results
    }
}
