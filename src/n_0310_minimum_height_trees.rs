struct Solution {}

use core::panic;
use std::collections::VecDeque;

struct GraphStore<TItem> {
    pub max_n: u16,
    pub edges: Vec<GraphNode>,
    pub node_data: Vec<TItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GraphNode {
    pub pointed_nodes: Vec<u16>,
}

impl<TItem> GraphStore<TItem>
where
    TItem: Clone,
{
    fn new(n: u16, default_node_val: TItem) -> Self {
        let default_node = GraphNode {
            pointed_nodes: vec![],
        };
        let node_count = (n + 1) as usize;
        Self {
            max_n: n,
            edges: vec![default_node; node_count],
            node_data: vec![default_node_val; node_count],
        }
    }

    fn new_from_vec(n: u16, edges: Vec<Vec<i32>>, default_node: TItem) -> Self {
        let mut tree = GraphStore::new(n - 1, default_node);
        for edge in edges {
            tree.add_edge(edge[0].try_into().unwrap(), edge[1].try_into().unwrap());
        }
        tree
    }

    fn new_from_vec_max(edges: Vec<Vec<i32>>, default_node: TItem) -> Self {
        let n = edges.iter().flat_map(|x| x.iter()).max().unwrap() + 1;
        Self::new_from_vec(n.try_into().unwrap(), edges, default_node)
    }

    fn add_edge(&mut self, a: u16, b: u16) {
        #[cfg(test)]
        {
            if a > self.max_n || b > self.max_n {
                panic!(
                    "nodes must be within bounds. a: {}, b: {}, max_n: {}",
                    a, b, self.max_n
                )
            }
        }
        self.edges[a as usize].connect_to(b);
        self.edges[b as usize].connect_to(a);
    }

    fn sort_leaf_first(&mut self) {
        self.edges.sort_unstable_by_key(|x| x.pointed_nodes.len());
    }

    fn get_leaf_indexes(&self) -> impl Iterator<Item = u16> + '_ {
        self.edges
            .iter()
            .enumerate()
            .filter(|x| x.1.pointed_nodes.len() <= 1)
            .map(|x| x.0 as u16)
    }
}

impl GraphNode {
    fn connect_to(&mut self, other: u16) {
        self.pointed_nodes.push(other);
    }

    fn new<T>(pointed_to: T) -> Self
    where
        T: IntoIterator<Item = u16>,
    {
        Self {
            pointed_nodes: pointed_to.into_iter().collect(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct NodeData {
    pub min_child_dist: Option<u16>,
    pub visited: bool,
}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n <= 0 {
            return vec![];
        }
        let mut tree = GraphStore::new_from_vec(
            n.try_into().unwrap(),
            edges,
            NodeData {
                min_child_dist: None,
                visited: false,
            },
        );
        let mut frontier: VecDeque<u16> = tree.get_leaf_indexes().collect();
        for leaf in frontier.iter() {
            let leaf_idx = *leaf as usize;
            tree.node_data[leaf_idx] = NodeData {
                min_child_dist: Some(0),
                visited: true,
            };
        }

        while !frontier.is_empty() {
            let next_idx = frontier.pop_front().unwrap() as usize;
            let my_data = tree.node_data.get_mut(next_idx).unwrap();
            my_data.visited = true;
            let min_dist = my_data.min_child_dist.unwrap();
            let next_dist = min_dist + 1;

            for neighbor in tree.edges[next_idx].pointed_nodes.iter() {
                let neighbor_idx = *neighbor as usize;
                let neighbor_data = tree.node_data.get_mut(neighbor_idx).unwrap();
                // in theory, we should only find exactly 1 non-visited neighbor, assuming this is
                // a tree data structure
                if neighbor_data.visited {
                    continue;
                }

                match &mut neighbor_data.min_child_dist {
                    // if the distance is greater from this node, replace with longer dist, then
                    // add neightbor to the frontier
                    Some(dst) if *dst > next_dist => {
                        *dst = next_dist;
                        frontier.push_back(*neighbor);
                    }
                    // if distance is longer from this node, noop. don't add to frontier again
                    Some(_) => {}
                    // if None, then Some
                    x => {
                        *x = Some(next_dist);
                        frontier.push_back(*neighbor);
                    }
                }
            }
        }

        let max_dist = tree
            .node_data
            .iter()
            .map(|x| x.min_child_dist.unwrap())
            .max()
            .unwrap();

        tree.node_data
            .iter()
            .enumerate()
            .filter(|x| x.1.min_child_dist.unwrap() == max_dist)
            .map(|x| x.0 as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::n_0310_minimum_height_trees::GraphNode;

    use super::{GraphStore, Solution};
    #[test]
    fn graph_store_sorts_leaf_first() {
        let edges = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]];
        let expected = [
            GraphNode::new([3]),
            GraphNode::new([3]),
            GraphNode::new([3]),
            GraphNode::new([4]),
            GraphNode::new([3, 5]),
            GraphNode::new([0, 1, 2, 4]),
        ];

        let edges = edges.map(|x| x.to_vec()).to_vec();
        let mut tree = GraphStore::new_from_vec_max(edges, 0);
        tree.sort_leaf_first();

        assert_eq!(expected.to_vec(), tree.edges);
    }
    #[test]
    fn single_minimum_tree() {
        let edges = [[1, 0], [1, 2], [1, 3]];
        let n = 4;
        let expected = [1];

        let edges = edges.map(|x| x.to_vec()).to_vec();
        let expected = expected.to_vec();
        let actual = Solution::find_min_height_trees(n, edges);
        assert_eq!(expected, actual)
    }

    #[test]
    fn multiple_minimum_tree() {
        let edges = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]];
        let n = 6;
        let expected = [3, 4];

        let edges = edges.map(|x| x.to_vec()).to_vec();
        let expected = expected.to_vec();
        let actual = Solution::find_min_height_trees(n, edges);
        assert_eq!(expected, actual)
    }

    #[test]
    fn tiny_tree() {
        let edges = [[0, 1]];
        let n = 2;
        let expected = [0, 1];

        let edges = edges.map(|x| x.to_vec()).to_vec();
        let expected = expected.to_vec();
        let actual = Solution::find_min_height_trees(n, edges);
        assert_eq!(expected, actual)
    }

    #[test]
    fn multiple_minimum_tree_two() {
        let edges = [[0, 1], [1, 2], [1, 3], [2, 4], [3, 5], [4, 6]];
        let n = 7;
        let expected = [1, 2];

        let edges = edges.map(|x| x.to_vec()).to_vec();
        let expected = expected.to_vec();
        let actual = Solution::find_min_height_trees(n, edges);
        assert_eq!(expected, actual)
    }
}
