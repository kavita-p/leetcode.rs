// 21 oct. 2022
// from https://leetcode.com/problems/contains-duplicate-ii/
// adapted from forum solution

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        nums.iter()
            .enumerate()
            .scan(HashMap::new(), |map, (i, num)| match map.insert(num, i) {
                Some(j) => Some(i - j <= k as usize),
                None => Some(false),
            })
            .any(|is_valid_dupe| is_valid_dupe)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3))
    }

    #[test]
    fn leetcode_e2() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 1, 0, 1], 2))
    }

    #[test]
    fn leetcode_e3() {
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ))
    }
}
