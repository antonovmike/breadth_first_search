use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub struct Graph<T> {
    adjacency_list: HashMap<T, Vec<T>>,
}

impl<T: Eq + Hash + Copy> Graph<T> {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: T, dest: T) {
        self.adjacency_list.entry(src).or_insert_with(Vec::new).push(dest);
    }
}

impl<T: Eq + Hash + Copy + PartialEq> Graph<T> {
    pub fn bfs(&self, start: T, target: T) -> bool {
        let mut visited: HashSet<T> = HashSet::new();
        let mut queue: VecDeque<T> = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if node == target {
                return true;
            }
            if let Some(neighbors) = self.adjacency_list.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(*neighbor);
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
        false
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.add_edge("a", "b");
    graph.add_edge("b", "c");
    graph.add_edge("c", "d");
    graph.add_edge("d", "e");
    graph.add_edge("e", "f");

    let result = graph.bfs("c", "q");
    println!("{result:?}")
}
