impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![std::collections::HashMap::new(); n];

        let mut res = 0;

        for i in 1..n {
            for j in 0..i {
                let diff = nums[i] - nums[j];
                let cnt = dp[j].get(&diff).unwrap_or(&0) + 1;
                *dp[i].entry(diff).or_insert(0) = cnt;
                res = res.max(cnt);
            }
        }

        res + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_arith_seq_length() {
        assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
        assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
        assert_eq!(Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]), 4);
    }
}
