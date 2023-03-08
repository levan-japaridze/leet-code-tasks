impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;
            let mut total_hours = 0;

            for bananas in piles.iter() {
                total_hours += (bananas + mid - 1) / mid;
            }

            if total_hours <= h {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}
