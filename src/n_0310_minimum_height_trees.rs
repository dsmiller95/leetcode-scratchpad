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
    pub longest_children: [Option<u16>; 2],
    pub longest_chain_origin: u16,
}

struct DfsData {
    pub longest_children_len: [u16; 2],
    pub longest_chain_len: u16,
}

struct DfsNodeData {
    node: NodeData,
    dfs: DfsData,
}

struct ChainPointer {
    len: u16,
    next: u16,
}

impl DfsNodeData {
    fn default() -> Self {
        Self {
            node: NodeData {
                longest_children: [None; 2],
                longest_chain_origin: 0,
            },
            dfs: DfsData {
                longest_children_len: [0; 2],

                longest_chain_len: 0,
            },
        }
    }

    fn get_node_data(&self) -> NodeData {
        self.node
    }

    fn get_longest_idx(&self) -> usize {
        match self.dfs.longest_children_len {
            [a, b] if a > b => 0,
            [_, _] => 1,
        }
    }
    fn get_shortest_idx(&self) -> usize {
        match self.get_longest_idx() {
            0 => 1,
            1 => 0,
            _ => panic!("index must only be 0 or 1"),
        }
    }

    fn get_longest_child(&self) -> Option<ChainPointer> {
        let longest = self.get_longest_idx();
        let next = self.node.longest_children[longest]?;
        Some(ChainPointer {
            len: self.dfs.longest_children_len[longest],
            next,
        })
    }

    fn add_longer_child(&mut self, deepest_from_child: u16, from_child: u16) {
        let idx = self.get_shortest_idx();
        let len_from_self = deepest_from_child + 1;
        if self.dfs.longest_children_len[idx] >= len_from_self {
            // noop, the new child is shorter than the existing longest children
            return;
        }
        self.dfs.longest_children_len[idx] = len_from_self;
        self.node.longest_children[idx] = Some(from_child);
    }

    fn apply_child(&mut self, child: &DfsNodeData, child_id: u16) {
        let longest_from_child = child.get_longest_child();
        let child_depth = longest_from_child.map(|x| x.len).unwrap_or(0);
        self.add_longer_child(child_depth, child_id);

        if child.dfs.longest_chain_len > self.dfs.longest_chain_len {
            self.dfs.longest_chain_len = child.dfs.longest_chain_len;
            self.node.longest_chain_origin = child.node.longest_chain_origin;
        }
    }

    fn use_own_chain_if_better(&mut self, self_id: u16) {
        // include myself in the chain
        let my_chain_len = self.dfs.longest_children_len[0] + self.dfs.longest_children_len[1] + 1;

        if self.dfs.longest_chain_len >= my_chain_len {
            return;
        }

        self.dfs.longest_chain_len = my_chain_len;
        self.node.longest_chain_origin = self_id;
    }

    /// ensure the longest child is at index 0
    fn sort_longest_children(&mut self) {
        let will_swap = self.dfs.longest_children_len[0] < self.dfs.longest_children_len[1];
        if !will_swap {
            return;
        }

        self.dfs.longest_children_len.swap(0, 1);
        self.node.longest_children.swap(0, 1);
    }
}

struct LongestChain {
    pub left: Vec<u16>,
    pub right: Vec<u16>,
    pub origin: u16,
}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n <= 0 {
            return vec![];
        }
        let mut tree: GraphStore<Option<NodeData>> =
            GraphStore::new_from_vec(n.try_into().unwrap(), edges, None);

        let result_walk = Solution::dfs(0, u16::MAX, &mut tree);
        tree.node_data[0] = Some(result_walk.node);

        let chain_origin = result_walk.node.longest_chain_origin;
        let chain_len = result_walk.dfs.longest_chain_len;

        let mut chain = Solution::collect_chain(chain_origin, chain_len, &tree);
        let chain = chain.make_contiguous();
        let chain_len = chain_len as usize;
        assert_eq!(chain_len, chain.len());

        if chain_len % 2 == 0 {
            vec![
                chain[chain_len / 2].into(),
                chain[(chain_len / 2) - 1].into(),
            ]
        } else {
            vec![chain[chain_len / 2].into()]
        }
    }
    fn dfs(node: u16, parent: u16, tree: &mut GraphStore<Option<NodeData>>) -> DfsNodeData {
        let children: Vec<u16> = tree.edges[node as usize]
            .pointed_nodes
            .iter()
            .filter(|&&x| x != parent)
            .copied()
            .collect();

        let mut self_data = DfsNodeData::default();
        for child in children {
            let child_data = Solution::dfs(child, node, tree);
            self_data.apply_child(&child_data, child);

            tree.node_data[child as usize] = Some(child_data.node);
        }
        self_data.use_own_chain_if_better(node);
        self_data.sort_longest_children();

        self_data
    }
    fn collect_chain(
        chain_origin: u16,
        chain_len: u16,
        tree: &GraphStore<Option<NodeData>>,
    ) -> VecDeque<u16> {
        let mut res = VecDeque::with_capacity(chain_len as usize);
        let node_data = tree.node_data[chain_origin as usize].unwrap();
        res.push_front(chain_origin);

        let mut next_node = node_data.longest_children[0];
        while let Some(idx) = next_node {
            res.push_front(idx);
            let next_data = tree.node_data[idx as usize].unwrap();

            // the longest child will be in index 0
            next_node = next_data.longest_children[0]
        }

        next_node = node_data.longest_children[1];
        while let Some(idx) = next_node {
            res.push_back(idx);
            let next_data = tree.node_data[idx as usize].unwrap();

            // the longest child will be in index 0
            next_node = next_data.longest_children[0]
        }

        res
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
