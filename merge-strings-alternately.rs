impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut merged_string = String::new();
        let word1_chars = word1.chars();
        let word2_chars = word2.chars();
        let mut word1_iter = word1_chars.enumerate();
        let mut word2_iter = word2_chars.enumerate();

        loop {
            let word1_next = word1_iter.next();
            let word2_next = word2_iter.next();

            if word1_next.is_none() && word2_next.is_none() {
                break;
            }

            if let Some((_idx, ch)) = word1_next {
                merged_string.push(ch);
            }

            if let Some((_idx, ch)) = word2_next {
                merged_string.push(ch);
            }
        }

        merged_string
    }
}
