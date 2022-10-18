// url: https://leetcode.com/problems/reverse-words-in-a-string-iii/
// date: 25 sep. 2022

struct Solution {}

impl Solution {
    fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        let input = "Let's take LeetCode contest".to_string();
        let output = "s'teL ekat edoCteeL tsetnoc".to_string();

        assert_eq!(Solution::reverse_words(input), output);
    }

    #[test]
    fn leetcode_e2() {
        let input = "God Ding".to_string();
        let output = "doG gniD".to_string();

        assert_eq!(Solution::reverse_words(input), output);
    }
}
