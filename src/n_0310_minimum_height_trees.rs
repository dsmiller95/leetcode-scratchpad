struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

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
