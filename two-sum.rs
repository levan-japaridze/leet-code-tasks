impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(&index) => return vec![index as i32, i as i32],
                None => map.insert(num, i as i32),
            };
        }
        vec![]
    }
}
