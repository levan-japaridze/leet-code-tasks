impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num_array = num;
        let mut carry = k;
        for i in (0..num_array.len()).rev() {
            let digit_sum = num_array[i] + carry;
            num_array[i] = digit_sum % 10;
            carry = digit_sum / 10;
        }
        while carry > 0 {
            num_array.insert(0, carry % 10);
            carry /= 10;
        }
        num_array
    }
}
