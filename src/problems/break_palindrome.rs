// url: https://leetcode.com/problems/break-a-palindrome
// date: 10 oct. 2022

struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() <= 1 {
            return String::from("");
        }

        let mut bytes = palindrome.as_bytes().to_vec();

        if let Some(i) = (0..palindrome.len() / 2)
            .into_iter()
            .find(|&i| bytes[i] != b'a')
        {
            bytes[i] = b'a';
        } else {
            bytes[palindrome.len() - 1] = b'b';
        }

        String::from_utf8_lossy(&bytes).into_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        assert_eq!(
            Solution::break_palindrome("abccba".to_string()),
            "aaccba".to_string()
        )
    }

    #[test]
    fn leetcode_e2() {
        assert_eq!(Solution::break_palindrome("a".to_string()), "".to_string())
    }
}
