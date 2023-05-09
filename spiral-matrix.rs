impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.is_empty() || matrix[0].is_empty() {
            return result;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut top = 0;
        let mut left = 0;
        let mut bottom = m as i32 - 1;
        let mut right = n as i32 - 1;
        while top <= bottom && left <= right {
            // Traverse top row
            for j in left..=right {
                result.push(matrix[top as usize][j as usize]);
            }
            top += 1;
            // Traverse right column
            for i in top..=bottom {
                result.push(matrix[i as usize][right as usize]);
            }
            right -= 1;
            // Traverse bottom row
            if top <= bottom {
                for j in (left..=right).rev() {
                    result.push(matrix[bottom as usize][j as usize]);
                }
                bottom -= 1;
            }
            // Traverse left column
            if left <= right {
                for i in (top..=bottom).rev() {
                    result.push(matrix[i as usize][left as usize]);
                }
                left += 1;
            }
        }
        result
    }
}
