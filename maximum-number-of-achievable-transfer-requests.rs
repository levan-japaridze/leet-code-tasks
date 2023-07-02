impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let total_requests = requests.len();
        let total_combinations = 1 << total_requests;
        for combination in 0..total_combinations {
            let mut buildings = vec![0; n as usize];
            let mut valid = true;
            let mut count = 0;
            for request in 0..total_requests {
                if (combination & (1 << request)) > 0 {
                    buildings[requests[request][0] as usize] -= 1;
                    buildings[requests[request][1] as usize] += 1;
                    count += 1;
                }
            }
            for i in 0..n as usize {
                if buildings[i] != 0 {
                    valid = false;
                    break;
                }
            }
            if valid && count > res {
                res = count;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_maximum_requests() {
        assert_eq!(
            Solution::maximum_requests(
                5,
                vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4]]
            ),
            5
        );
        assert_eq!(
            Solution::maximum_requests(3, vec![vec![0, 0], vec![1, 2], vec![2, 1]]),
            3
        );
        assert_eq!(
            Solution::maximum_requests(4, vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]]),
            4
        );
    }
}
