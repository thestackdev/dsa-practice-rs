use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    val: i32,
    neighbours: Vec<i32>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            neighbours: vec![],
        }
    }
}

type Graph = HashMap<i32, Node>;

pub struct Solution;

impl Solution {
    fn clone_graph(self, graph: &Graph, start: i32) -> Graph {
        let mut result: Graph = HashMap::new();
        Self::dfs(graph, start, &mut result);
        result
    }

    fn dfs(graph: &Graph, val: i32, result: &mut Graph) {
        if result.contains_key(&val) {
            return;
        }

        if let Some(node) = graph.get(&val) {
            result.insert(val, node.clone());

            for nbr in node.neighbours.clone() {
                Self::dfs(graph, nbr, result);
            }
        }
    }
}
