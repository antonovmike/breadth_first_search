use std::collections::{HashMap, VecDeque};

fn bfs(graph: &HashMap<char, Vec<char>>, start: char) -> HashMap<char, usize> {
    let mut distances: HashMap<char, usize> = HashMap::new();
    let mut queue: VecDeque<char> = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    let current_distance = *distances.get(&current).unwrap();
                    distances.insert(neighbor, current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distances
}

fn main() {
    let mut graph: HashMap<char, Vec<char>> = HashMap::new();
    graph.insert('A', vec!['B', 'C']);
    graph.insert('B', vec!['A', 'D', 'E']);
    graph.insert('C', vec!['A', 'F']);
    graph.insert('D', vec!['B']);
    graph.insert('E', vec!['B', 'F']);
    graph.insert('F', vec!['C', 'E']);

    let distances = bfs(&graph, 'A');

    for (node, distance) in distances {
        println!("Distance from A to {}: {}", node, distance);
    }
}
