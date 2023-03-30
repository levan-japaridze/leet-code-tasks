impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        if n != s2.len() || s1.chars().collect::<Vec<char>>().sort() != s2.chars().collect::<Vec<char>>().sort() {
            return false;
        }

        if n <= 1 {
            return s1 == s2;
        }

        let s1_chars = s1.chars().collect::<Vec<char>>();
        let s2_chars = s2.chars().collect::<Vec<char>>();

        let mut dp = vec![vec![vec![false; n]; n]; n];

        for i in 0..n {
            for j in 0..n {
                if s1_chars[i] == s2_chars[j] {
                    dp[i][j][0] = true;
                }
            }
        }

        for len in 1..n {
            for i in 0..n - len {
                for j in 0..n - len {
                    for k in 0..len {
                        if (dp[i][j][k] && dp[i + k + 1][j + k + 1][len - k - 1]) ||
                            (dp[i][j + len - k][k] && dp[i + k + 1][j][len - k - 1]) {
                            dp[i][j][len] = true;
                            break;
                        }
                    }
                }
            }
        }

        dp[0][0][n - 1]
    }
}
