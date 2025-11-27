// GRAPHS:
// -------
// A graph is a data structure used to represent relationships.
// It consists of:
//     Nodes (vertices): the entities
//     Edges: the connections between them
// - Adjancent vertices: connects the nodes (direct edge)
// - Degree of node: # of edges going to a vertex
// - Path: how to get from one node to another
// - Connected Graph: path between every 2 nodes (vertices)
// - Complete Graph: direct edge between any 2 vertex
//
// Two main types:
//     - Undirected graph: connections go both ways (e.g., friendships)
//     - Directed graph (digraph): connections have a unique direction (e.g., web links)
// Edges may also have:
//     - weights (numbers) → distance, cost, time, etc.
//     - unweighted -> no weights
//
// Adjacency Matrix
//     Nodes: A B C D
//
//     A   B   C   D
//   A 0   1   1   0
//   B 1   0   1   0
//   C 0   1   0   1
//   D 0   0   1   0
//
use std::collections::HashMap;

#[derive(Debug)]
pub struct NodeNotInGraph; // Custom Error Type

pub struct DirectedGraph {
    adjacentcy_matrix: HashMap<String, Vec<(String, i32)>>, // Strings are vertex, i32 are the weights
}

pub struct UndirectedGraph {
    adjacentcy_matrix: HashMap<String, Vec<(String, i32)>>,
}

pub trait Graph {
    fn new() -> Self;
    fn adjacentcy_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacentcy_matrix().get(node) {
            None => {
                self.adjacentcy_matrix()
                    .insert((*node).to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacentcy_matrix()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    fn neighbors(&mut self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        match self.adjacentcy_matrix().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }
}

impl Graph for DirectedGraph {
    fn new() -> DirectedGraph {
        DirectedGraph {
            adjacentcy_matrix: HashMap::new(),
        }
    }

    fn adjacentcy_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacentcy_matrix
    }
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacentcy_matrix: HashMap::new(),
        }
    }

    fn adjacentcy_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacentcy_matrix
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacentcy_matrix()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });

        // Connection goes both ways
        self.adjacentcy_matrix()
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::*;

    #[test]
    fn test_neighbors() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbors("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7)]
        );
    }

    #[test]
    fn test_directed() {
        let mut graph = DirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "a", 5));

        assert_eq!(graph.neighbors("a").unwrap(), &vec![(String::from("b"), 5)]);
        assert_eq!(
            graph.neighbors("b").unwrap(),
            &vec![(String::from("c"), 10), (String::from("a"), 5)]
        );
    }
}
