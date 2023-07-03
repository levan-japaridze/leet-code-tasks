impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() { return false; }
        if s == goal {
            let mut chars = vec![0; 26];
            for c in s.bytes() {
                chars[(c - b'a') as usize] += 1;
                if chars[(c - b'a') as usize] > 1 {
                    return true;
                }
            }
            return false;
        } else {
            let mut pairs = Vec::new();
            let (s, goal) = (s.as_bytes(), goal.as_bytes());
            for i in 0..s.len() {
                if s[i] != goal[i] {
                    pairs.push((s[i], goal[i]));
                    if pairs.len() > 2 {
                        return false;
                    }
                }
            }
            return pairs.len() == 2 && pairs[0] == (pairs[1].1, pairs[1].0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_buddy_strings() {
        assert_eq!(Solution::buddy_strings(String::from("ab"), String::from("ba")), true);
        assert_eq!(Solution::buddy_strings(String::from("ab"), String::from("ab")), false);
        assert_eq!(Solution::buddy_strings(String::from("aa"), String::from("aa")), true);
    }
}
