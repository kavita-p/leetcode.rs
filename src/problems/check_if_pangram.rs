// 17 oct. 2022
// from https://leetcode.com/problems/check-if-the-sentence-is-pangram/

use std::collections::HashSet;

pub fn check_if_pangram(sentence: String) -> bool {
    sentence.bytes().collect::<HashSet<u8>>().len() == 26
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        assert!(check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
    }

    #[test]
    fn leetcode_e2() {
        assert!(!check_if_pangram("leetcode".to_string()));
    }

    #[test]
    fn leetcode_t49() {
        let case_string = "jwtucoucmdfwxxqnxzkaxoglszmfrcvjoiunqqausaxxaaijyqdqgvdnqcaihwilqkpivenpnekioyqujrdrovqrlxovcucjqzjsxmllfgndfprctxvxwlzjtciqxgsxfwhmuzqvlksyuztoetyjugmswfjtawwaqmwyxmvo".to_string();
        assert!(!check_if_pangram(case_string))
    }
}
