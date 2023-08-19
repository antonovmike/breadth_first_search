use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct Graph<T> {
    adjacency_list: HashMap<T, Vec<T>>,
}

impl<T: Eq + Hash + Clone> Graph<T> {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: T, dst: T) {
        self.adjacency_list.entry(src.clone()).or_insert_with(Vec::new).push(dst.clone());
        self.adjacency_list.entry(dst).or_insert_with(Vec::new).push(src);
    }

    pub fn bfs(&self, start: T) -> Vec<T> {
        let mut visited = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());
        while let Some(node) = queue.pop_front() {
            if !visited.contains(&node) {
                visited.push(node.clone());
                if let Some(neighbors) = self.adjacency_list.get(&node) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }
        visited
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.add_edge("a", "b");
    graph.add_edge("b", "c");
    graph.add_edge("c", "d");
    graph.add_edge("d", "e");
    graph.add_edge("e", "f");

    let result = graph.bfs("c");
    println!("{result:?}")
}