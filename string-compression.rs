impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut i = 0;
        let mut j = 0;

        while i < chars.len() {
            let c = chars[i];
            let mut count = 1;
            while i + 1 < chars.len() && chars[i + 1] == c {
                count += 1;
                i += 1;
            }
            chars[j] = c;
            j += 1;
            if count > 1 {
                let count_str = count.to_string();
                for digit in count_str.chars() {
                    chars[j] = digit;
                    j += 1;
                }
            }
            i += 1;
        }

        j as i32
    }
}
