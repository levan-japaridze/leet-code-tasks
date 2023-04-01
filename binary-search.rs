impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if let Some(index) = nums.iter().position(|&x| x == target) {
            index as i32
        } else {
            -1
        }
    }   
}
