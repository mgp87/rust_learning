use std::collections::{HashSet, VecDeque};

// Depth First Search
// 1. Starting vertex
// 2. Make sure to track visited vertex

pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u8>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u8> = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(root);

    while let Some(current_vertex) = queue.pop_front() {
        history.push(current_vertex.value());

        if current_vertex == objective {
            return Some(history);
        }

        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            if visited.insert(neighbor) {
                queue.push_front(neighbor)
            }
        }
    }

    None
}

// Breadth First Search
pub fn breadth_first_search(graph: &Graph, root: Vertex, target: Vertex) -> Option<Vec<u8>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u8> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);

    while let Some(current_vertex) = queue.pop_front() {
        history.push(current_vertex.value());
        if current_vertex == target {
            return Some(history);
        }

        for neighbor in current_vertex.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    None
}

// Dijkstra algorithm

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u8);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u8, u8);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<u8> for Vertex {
    fn from(item: u8) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u8, u8)> for Edge {
    fn from(item: (u8, u8)) -> Self {
        Edge(item.0, item.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 10;
        let correct = vec![1, 2, 4, 5, 3, 6, 7];
        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            None
        );
    }

    #[test]
    fn test_dfs() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 7;
        let correct = vec![1, 2, 4, 5, 3, 6, 7];
        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct)
        );
    }

    #[test]
    fn bfs_fail() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 10;
        let correct = vec![1, 2, 4, 5, 3, 6, 7];
        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            breadth_first_search(&graph, root.into(), objective.into()),
            None
        );
    }

    #[test]
    fn test_bfs() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 7;
        let path = vec![1, 2, 3, 4, 5, 6, 7];
        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            breadth_first_search(&graph, root.into(), objective.into()),
            Some(path)
        );
    }
}
