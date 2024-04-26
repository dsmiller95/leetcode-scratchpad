struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::n_1289_minimum_falling_path_sum_II::Solution;

    #[test]
    fn single_grid_single_path() {
        let grid = [[7]];
        let expected = 7;

        let grid = grid.map(|x| x.to_vec()).to_vec();
        assert_eq!(expected, Solution::min_falling_path_sum(grid))
    }

    #[test]
    fn small_grid_best_path() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected = 13;

        let grid = grid.map(|x| x.to_vec()).to_vec();
        assert_eq!(expected, Solution::min_falling_path_sum(grid))
    }
}
