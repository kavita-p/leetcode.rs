// url: https://leetcode.com/problems/sum-of-even-numbers-after-queries/
// date: 24 sep. 2022
// do you remember~

fn sum_even_after_queries (mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut answer = vec![];
    for q in queries.iter() {
        let (figure, index) = (q[0], q[1]);
        nums[index as usize] += figure;
        let mut even_sums = 0;
        for val in &nums {
            if val % 2 == 0 { even_sums += val };
        }
        answer.push(even_sums);
    } 
    answer
}

#[cfg(test)]
mod test {
    use super::sum_even_after_queries;

    #[test]
    fn leetcode_e1 () {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![1,0],vec![-3,1],vec![-4,0],vec![2,3]];
        let output = vec![8, 6, 2, 4];

        assert_eq!(output, sum_even_after_queries(nums, queries))
    }
    #[test]
    fn leetcode_e2 () {
        let nums = vec![1];
        let queries = vec![vec![4, 0]];
        let output = vec![0];

        assert_eq!(output, sum_even_after_queries(nums, queries))
    }
}
