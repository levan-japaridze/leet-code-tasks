impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let haystack_len = haystack_bytes.len();
        let needle_len = needle_bytes.len();

        if needle_len == 0 {
            return 0;
        }

        let mut lps = vec![0; needle_len];
        let mut i = 0;
        let mut j = 1;

        while j < needle_len {
            if needle_bytes[i] == needle_bytes[j] {
                i += 1;
                lps[j] = i as i32;
                j += 1;
            } else if i > 0 {
                i = lps[i-1] as usize;
            } else {
                lps[j] = 0;
                j += 1;
            }
        }

        let mut i = 0;
        let mut j = 0;

        while i < haystack_len {
            if haystack_bytes[i] == needle_bytes[j] {
                i += 1;
                j += 1;
                if j == needle_len {
                    return (i - j) as i32;
                }
            } else if j > 0 {
                j = lps[j-1] as usize;
            } else {
                i += 1;
            }
        }

        -1
}

}
