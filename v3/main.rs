use std::collections::{HashSet, VecDeque};

struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<Vec<usize>>,
}

impl<T> Graph<T> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, node: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        self.edges.push(Vec::new());
        index
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
    }

    fn bfs<F>(&self, start: usize, mut visit: F)
    where
        F: FnMut(usize),
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }

            visit(node);
            visited.insert(node);

            for &next in &self.edges[node] {
                if !visited.contains(&next) {
                    queue.push_back(next);
                }
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new();

    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");
    let e = graph.add_node("e");

    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    graph.add_edge(d, e);

    graph.bfs(a, |node| println!("{}", node));
}
