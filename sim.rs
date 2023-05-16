use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<char, Vec<char>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, start: char, end: char) {
        self.adj_list.entry(start).or_insert(Vec::new()).push(end);
    }

    fn topological_sort(&self) -> Option<Vec<char>> {
        let mut visited: HashSet<char> = HashSet::new();
        let mut result: Vec<char> = Vec::new();

        for &node in self.adj_list.keys() {
            if !visited.contains(&node) {
                if !self.dfs(node, &mut visited, &mut result) {
                    return None; // Cycle detected, topological sort not possible
                }
            }
        }

        result.reverse();
        Some(result)
    }

    fn dfs(&self, node: char, visited: &mut HashSet<char>, result: &mut Vec<char>) -> bool {
        visited.insert(node);

        if let Some(neighbors) = self.adj_list.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    if !self.dfs(neighbor, visited, result) {
                        return false; // Cycle detected
                    }
                }
            }
        }

        result.push(node);
        true
    }
}

// Example usage:
fn main() {
    let mut graph = Graph::new();
    graph.add_edge('A', 'B');
    graph.add_edge('A', 'C');
    graph.add_edge('B', 'D');
    graph.add_edge('C', 'D');
    graph.add_edge('C', 'E');
    graph.add_edge('D', 'E');

    if let Some(sorted_nodes) = graph.topological_sort() {
        println!("Topological Sort: {:?}", sorted_nodes);
    } else {
        println!("Cycle detected, topological sort not possible.");
    }
}
