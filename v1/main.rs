use std::collections::VecDeque;

struct Graph {
    nodes: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self { nodes: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.nodes[u].push(v);
        self.nodes[v].push(u);
    }

    fn bfs(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.nodes.len()];
        let mut parent = vec![None; self.nodes.len()];

        queue.push_back(start);
        visited[start] = true;

        while let Some(node) = queue.pop_front() {
            if node == end {
                let mut path = Vec::new();
                let mut current = node;
                while let Some(parent_node) = parent[current] {
                    path.push(current);
                    current = parent_node;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            for neighbor in &self.nodes[node] {
                if !visited[*neighbor] {
                    visited[*neighbor] = true;
                    parent[*neighbor] = Some(node);
                    queue.push_back(*neighbor);
                }
            }
        }

        None
    }
}

fn main() {
    let mut graph = Graph::new(6);

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);

    if let Some(path) = graph.bfs(0, 5) {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found");
    }
}
