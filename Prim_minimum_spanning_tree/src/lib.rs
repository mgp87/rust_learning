use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, wt: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, wt);
    graph.entry(v2).or_insert_with(BTreeMap::new).insert(v1, wt);
}

pub fn prim<V: Ord + Copy, E: Ord + Copy>(graph: &Graph<V, E>, start: V) -> Graph<V, E> {
    let mut mst = Graph::new();
    let mut visited = BinaryHeap::new();

    mst.insert(start, BTreeMap::new());

    for (v, wt) in &graph[&start] {
        visited.push(Reverse((*wt, v, start)));
    }

    while let Some(Reverse((wt, vt, prev))) = visited.pop() {
        if mst.contains_key(vt) {
            continue;
        }

        add_edge(&mut mst, prev, *vt, wt);

        for (v, wt) in &graph[vt] {
            if !mst.contains_key(v) {
                visited.push(Reverse((*wt, v, *vt)));
            }
        }
    }
    mst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prims() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 3);
        add_edge(&mut graph, 0, 2, 4);
        add_edge(&mut graph, 1, 2, 1);
        add_edge(&mut graph, 1, 4, 6);
        add_edge(&mut graph, 2, 4, 3);
        add_edge(&mut graph, 2, 3, 5);
        add_edge(&mut graph, 3, 4, 2);

        let mut ans = BTreeMap::new();
        add_edge(&mut ans, 0, 1, 3);
        add_edge(&mut ans, 1, 2, 1);
        add_edge(&mut ans, 2, 4, 3);
        add_edge(&mut ans, 4, 3, 2);

        assert_eq!(prim(&graph, 0), ans);
    }
}
