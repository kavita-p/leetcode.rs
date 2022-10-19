// url: https://leetcode.com/problems/find-k-closest-elements/
// date: 29 sep. 2022
// adapted from forum solution

struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut l = 0;
        let mut r = arr.len() - k;
        while l < r {
            let mid = l + (r - l) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        arr[l..(l + k)].into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_e1() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let output = vec![1, 2, 3, 4];

        assert_eq!(Solution::find_closest_elements(arr, k, x), output);
    }
}
