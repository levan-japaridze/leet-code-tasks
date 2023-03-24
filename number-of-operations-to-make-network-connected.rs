use std::collections::HashSet;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if connections.len() < (n - 1) as usize {
            return -1;
        }

        let mut visited = vec![false; n as usize];
        let mut graph = vec![Vec::new(); n as usize];

        for connection in &connections {
            graph[connection[0] as usize].push(connection[1]);
            graph[connection[1] as usize].push(connection[0]);
        }

        let mut components = 0;

        for i in 0..n {
            if !visited[i as usize] {
                components += 1;
                Solution::dfs(i, &mut visited, &graph);
            }
        }

        components - 1
    }

    fn dfs(node: i32, visited: &mut Vec<bool>, graph: &Vec<Vec<i32>>) {
        visited[node as usize] = true;

        for &neighbor in &graph[node as usize] {
            if !visited[neighbor as usize] {
                Solution::dfs(neighbor, visited, graph);
            }
        }
    }
}
