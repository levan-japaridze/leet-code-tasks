impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = false;
        let mut result = String::new();

        // Iterate over the strings from right to left.
        let mut a_iter = a.chars().rev();
        let mut b_iter = b.chars().rev();

        loop {
            let (a_bit, b_bit) = match (a_iter.next(), b_iter.next()) {
                (None, None) => break,
                (Some(x), None) => (x, '0'),
                (None, Some(y)) => ('0', y),
                (Some(x), Some(y)) => (x, y),
            };

            let (sum, carry_out) = match (a_bit, b_bit, carry) {
                ('0', '0', false) => ('0', false),
                ('0', '1', false) => ('1', false),
                ('1', '0', false) => ('1', false),
                ('1', '1', false) => ('0', true),
                ('0', '0', true) => ('1', false),
                ('0', '1', true) => ('0', true),
                ('1', '0', true) => ('0', true),
                ('1', '1', true) => ('1', true),
                _ => panic!("Invalid input"),
            };

            result.push(sum);
            carry = carry_out;
        }

        if carry {
            result.push('1');
        }

        result.chars().rev().collect()
    }
}
