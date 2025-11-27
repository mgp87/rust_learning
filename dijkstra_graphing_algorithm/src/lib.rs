use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(graph: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut distance: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    let mut visited = BinaryHeap::new();
    distance[start] = 0;
    visited.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = visited.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > distance[position] {
            continue;
        }
        for edge in &graph[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < distance[next.position] {
                visited.push(next);
                distance[next.position] = next.cost;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            // node 0
            vec![
                Edge { node: 1, cost: 6 },
                Edge { node: 2, cost: 4 },
                Edge { node: 3, cost: 1 },
            ],
            // node 1
            vec![Edge { node: 0, cost: 6 }, Edge { node: 2, cost: 3 }],
            // node 2
            vec![
                Edge { node: 0, cost: 4 },
                Edge { node: 1, cost: 3 },
                Edge { node: 3, cost: 1 },
            ],
            // node 3
            vec![Edge { node: 0, cost: 1 }, Edge { node: 2, cost: 1 }],
        ];

        assert_eq!(shortest_path(&graph, 0, 1), Some(5));
    }
}
