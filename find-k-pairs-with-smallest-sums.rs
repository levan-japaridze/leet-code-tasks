use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut res = Vec::new();

        let n = nums1.len();
        let m = nums2.len();

        for i in 0..n.min(k as usize) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }

        for _ in 0..k as usize {
            if heap.is_empty() {
                break;
            }

            let Reverse((_, i, j)) = heap.pop().unwrap();
            res.push(vec![nums1[i], nums2[j]]);

            if j + 1 < m {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_k_smallest_pairs() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
            vec![vec![1, 3], vec![2, 3]]
        );
    }
}
