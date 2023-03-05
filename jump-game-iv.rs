use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 1 {
            return 0;
        }

        // Store the indices of each distinct value in the array
        let mut index_map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &x) in arr.iter().enumerate() {
            index_map.entry(x).or_default().push(i);
        }

        // BFS traversal of the array, starting from the first index
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = vec![false; n];
        visited[0] = true;
        q.push_back((0, 0));

        while let Some((i, steps)) = q.pop_front() {
            if i == n - 1 {
                return steps as i32;
            }

            for &j in &[i - 1, i + 1] {
                if j < 0 || j >= n || visited[j] {
                    continue;
                }
                visited[j] = true;
                q.push_back((j, steps + 1));
            }

            if let Some(indices) = index_map.get(&arr[i]) {
                for &j in indices {
                    if j != i && !visited[j] {
                        visited[j] = true;
                        q.push_back((j, steps + 1));
                    }
                }
                // Remove the entry to avoid revisiting the same indices
                index_map.remove(&arr[i]);
            }
        }

        -1 // No path found
    }
}
