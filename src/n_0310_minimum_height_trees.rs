use core::panic;

struct Solution {}

struct GraphStore {
    pub max_n: u16,
    pub edges: Vec<GraphNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GraphNode {
    pub pointed_nodes: Vec<u16>,
}

impl GraphStore {
    fn new(n: u16) -> Self {
        let default_node = GraphNode {
            pointed_nodes: vec![],
        };
        Self {
            max_n: n,
            edges: vec![default_node; (n + 1) as usize],
        }
    }

    fn new_from_vec(n: u16, edges: Vec<Vec<i32>>) -> Self {
        let mut tree = GraphStore::new(n - 1);
        for edge in edges {
            tree.add_edge(edge[0].try_into().unwrap(), edge[1].try_into().unwrap());
        }
        tree
    }

    fn new_from_vec_max(edges: Vec<Vec<i32>>) -> Self {
        let n = edges.iter().flat_map(|x| x.iter()).max().unwrap() + 1;
        Self::new_from_vec(n.try_into().unwrap(), edges)
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

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n <= 0 {
            return vec![];
        }
        let mut tree = GraphStore::new_from_vec(n.try_into().unwrap(), edges);
        tree.sort_leaf_first();
        todo!()
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
        let mut tree = GraphStore::new_from_vec_max(edges);
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
}
