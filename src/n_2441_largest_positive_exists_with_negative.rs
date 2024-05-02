struct Solution();

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut num_table: HashSet<u16> = HashSet::with_capacity(nums.len() / 4);
        let mut max_out: Option<u16> = None;

        for num in nums {
            let num = num.unsigned_abs() as u16;

            let Some(existing_num) = num_table.replace(num) else {
                continue;
            };

            max_out = Some(match max_out {
                Some(x) => x.max(existing_num),
                None => existing_num,
            });
        }

        match max_out {
            Some(x) => x.into(),
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::n_2441_largest_positive_exists_with_negative::Solution;

    #[test]
    fn test_case_1() {
        let nums = [-1, 2, -3, 3];
        let expected = 3;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }

    #[test]
    fn test_case_2() {
        let nums = [-1, 10, 6, 7, -7, 1];
        let expected = 7;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }

    #[test]
    fn test_case_3() {
        let nums = [-10, 8, 6, 7, -2, -3];
        let expected = -1;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }

    #[test]
    fn test_case_4() {
        let nums = [-9, -43, 24, -23, -16, -30, -38, -30];
        let expected = -1;

        let nums = nums.to_vec();
        assert_eq!(Solution::find_max_k(nums), expected)
    }
}
