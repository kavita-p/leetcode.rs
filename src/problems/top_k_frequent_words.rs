// 19 oct. 2022
// from https://leetcode.com/problems/top-k-frequent-words/
// adapted from forum solution

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        // vector of (frequency, word) tuples
        let mut word_counts: Vec<(i32, String)> = words
            .into_iter()
            .fold(HashMap::new(), |mut counts, word| {
                *counts.entry(word).or_insert(0) += 1;
                counts
            })
            .into_iter()
            // make counts negative for easy sorting
            .map(|(w, count)| (-count, w))
            .collect();

        word_counts.sort();

        word_counts
            .into_iter()
            // drop counts
            .map(|(_count, w)| w)
            // :)
            .take(k.try_into().unwrap_or_default())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        let words = ["i", "love", "leetcode", "i", "love", "coding"]
            .map(String::from)
            .to_vec();
        let k = 2;

        assert_eq!(
            Solution::top_k_frequent(words, k),
            ["i", "love"].map(String::from).to_vec()
        )
    }
}
