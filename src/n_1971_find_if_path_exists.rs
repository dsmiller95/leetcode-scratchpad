struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn valid_path(_n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            return true;
        }
        let mut edges = edges;
        let mut reachable_from_source = HashSet::new();
        let mut reachable_from_dest = HashSet::new();
        loop {
            let mut modified = false;
            for edge in edges.iter_mut().filter(|x| x[0] != x[1]) {
                if edge == &[source, destination] {
                    return true;
                }
                if edge == &[destination, source] {
                    return true;
                }
                if edge[0] == source {
                    reachable_from_source.insert(edge[1]);
                    edge[1] = source;
                    modified = true;
                } else if edge[1] == source {
                    reachable_from_source.insert(edge[0]);
                    edge[0] = source;
                    modified = true;
                }
                if edge[0] == destination {
                    reachable_from_dest.insert(edge[1]);
                    edge[1] = destination;
                    modified = true;
                } else if edge[1] == destination {
                    reachable_from_dest.insert(edge[0]);
                    edge[0] = destination;
                    modified = true;
                }
            }
            if !modified {
                return false;
            }
            if reachable_from_source
                .intersection(&reachable_from_dest)
                .any(|_| true)
            {
                return true;
            }
            for vertex in edges.iter_mut().filter(|x| x[0] != x[1]).flatten() {
                let reach_dst_src = (
                    reachable_from_dest.contains(vertex),
                    reachable_from_source.contains(vertex),
                );

                match reach_dst_src {
                    (true, true) => {
                        return true;
                    }
                    (true, false) => {
                        *vertex = destination;
                    }
                    (false, true) => {
                        *vertex = source;
                    }
                    _ => {}
                }
            }
            reachable_from_source.clear();
            reachable_from_dest.clear();
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn test_triangle_has_path() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let has_path = Solution::valid_path(3, edges, 0, 2);
        assert!(has_path)
    }

    #[test]
    fn test_disconnected_path_has_no_path() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let has_path = Solution::valid_path(6, edges, 0, 5);
        assert!(!has_path)
    }

    #[test]
    fn test_no_edges_same_node() {
        let edges = vec![];
        let has_path = Solution::valid_path(1, edges, 0, 0);
        assert!(has_path)
    }

    #[test]
    fn test_no_edges_other_node() {
        let edges = vec![];
        let has_path = Solution::valid_path(1, edges, 0, 1);
        assert!(!has_path)
    }

    #[test]
    fn test_big_boi() {
        let edges = vec![
            vec![41, 40],
            vec![9, 32],
            vec![48, 14],
            vec![6, 44],
            vec![18, 41],
            vec![41, 1],
            vec![7, 41],
            vec![38, 41],
            vec![19, 4],
            vec![16, 41],
            vec![41, 43],
            vec![41, 22],
            vec![41, 21],
            vec![9, 0],
            vec![41, 48],
            vec![32, 36],
            vec![24, 44],
            vec![39, 41],
            vec![48, 17],
            vec![49, 15],
            vec![47, 41],
            vec![19, 31],
            vec![11, 41],
            vec![41, 23],
            vec![41, 49],
            vec![45, 44],
            vec![2, 49],
            vec![13, 41],
            vec![35, 41],
            vec![30, 0],
            vec![5, 44],
            vec![8, 0],
            vec![41, 20],
            vec![41, 28],
            vec![12, 11],
            vec![12, 41],
            vec![49, 10],
            vec![19, 0],
            vec![0, 37],
            vec![34, 41],
            vec![42, 48],
            vec![27, 41],
            vec![0, 41],
            vec![19, 44],
            vec![41, 26],
            vec![41, 29],
            vec![33, 41],
            vec![49, 46],
            vec![41, 25],
            vec![3, 41],
        ];

        let has_path = Solution::valid_path(50, edges, 40, 3);
        assert!(has_path)
    }

    #[test]
    fn test_single_chain() {
        let edges = vec![
            vec![0, 1],
            vec![2, 3],
            vec![5, 6],
            vec![4, 5],
            vec![3, 4],
            vec![1, 2],
        ];
        let has_path = Solution::valid_path(7, edges, 0, 6);
        assert!(has_path)
    }
}
