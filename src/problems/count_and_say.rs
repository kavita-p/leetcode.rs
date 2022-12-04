// 18 oct. 2022
// from https://leetcode.com/problems/count-and-say/

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let mut say: Vec<char> = vec![];
        let mut digit_count: u8 = b'0';
        let mut prev = ' ';
        for char in Self::count_and_say(n - 1).chars() {
            if prev == ' ' {
                prev = char;
                digit_count = b'1';
            } else if char == prev {
                digit_count += 1;
            } else {
                say.push(digit_count as char);
                say.push(prev);
                prev = char;
                digit_count = b'1';
            }
        }
        say.push(digit_count as char);
        say.push(prev);
        say.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        assert_eq!(Solution::count_and_say(1), "1".to_string())
    }

    #[test]
    fn leetcode_e2() {
        assert_eq!(Solution::count_and_say(4), "1211".to_string())
    }
}
