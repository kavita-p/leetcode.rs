// 20 oct. 2022
// from https://leetcode.com/problems/integer-to-roman/

struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let dict: Vec<(i32, &str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut numerus = String::new();
        for &(value, roman) in &dict {
            let roman_digits = num / value;
            if roman_digits > 0 {
                numerus.push_str(&roman.repeat(roman_digits.try_into().unwrap()));
                num %= value;
            }
        }
        numerus
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string())
    }

    #[test]
    fn leetcode_e2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string())
    }

    #[test]
    fn leetcode_e3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string())
    }
}
