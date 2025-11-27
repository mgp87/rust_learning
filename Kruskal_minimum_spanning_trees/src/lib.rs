// Minimum Spanning Tree (MST):
//-----------------------------
// Spanning Tree Rules:
//  1. Connected
//  2. No cycles
//  3. # of edges = # of vertices - 1
//
// --------------------------------------------
// MST = Sum of edges weights should be minimum
// --------------------------------------------

use disjoint_sets::UnionFind;

type Node = usize;
type Weight = usize;

struct Edge {
    destination: Node,
    weight: Weight,
}

type Graph = Vec<Vec<Edge>>;

fn edges_by_weight(graph: &Graph) -> Vec<(Node, Node, Weight)> {
    let mut edges = vec![];

    for (src, destination) in graph.iter().enumerate() {
        for edge in destination {
            edges.push((src, edge.destination, edge.weight));
        }
    }

    edges.sort_by_key(|&(_, _, weight)| weight);
    edges
}

// Kruskal's MST
fn mst(graph: &Graph) -> Vec<(Node, Node)> {
    let mut result = vec![];
    let mut union_find = UnionFind::new(graph.len());

    for (src, destination, _) in edges_by_weight(graph) {
        if !union_find.equiv(src, destination) {
            union_find.union(src, destination);
            result.push((src, destination));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mst() {
        let graph = vec![
            vec![
                Edge {
                    destination: 1,
                    weight: 3,
                },
                Edge {
                    destination: 3,
                    weight: 6,
                },
                Edge {
                    destination: 5,
                    weight: 1,
                },
            ],
            vec![
                Edge {
                    destination: 3,
                    weight: 5,
                },
                Edge {
                    destination: 5,
                    weight: 4,
                },
                Edge {
                    destination: 2,
                    weight: 1,
                },
            ],
            vec![
                Edge {
                    destination: 3,
                    weight: 2,
                },
                Edge {
                    destination: 4,
                    weight: 3,
                },
            ],
            vec![Edge {
                destination: 4,
                weight: 7,
            }],
            vec![Edge {
                destination: 5,
                weight: 2,
            }],
            vec![],
        ];

        assert_eq!(vec![(0, 5), (1, 2), (2, 3), (4, 5), (0, 1)], mst(&graph));
    }
}
